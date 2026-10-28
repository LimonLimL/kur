use async_sqlite::Client;
use rusqlite::{params_from_iter, types::Value};
use time::format_description::well_known::Rfc3339;
use time::format_description::{self, FormatDescriptionV3};
use uuid::Uuid;

use crate::contracts::order_storage::OrderStorage;
use crate::models::order::Order;

use super::sqlite_validation_error;

/// Единый формат времени для записи и чтения. Дробные доли секунды при
/// записи отбрасываются, поэтому чтение всегда успешно.
const TIME_FORMAT: &str = "[hour]:[minute]:[second]";

/// Возвращает разобранное описание формата времени (парсится один раз).
fn time_format() -> &'static FormatDescriptionV3<'static> {
    static FORMAT: std::sync::OnceLock<FormatDescriptionV3<'static>> = std::sync::OnceLock::new();
    FORMAT.get_or_init(|| {
        format_description::parse_owned::<3>(TIME_FORMAT)
            .expect("Некорректное описание формата времени")
    })
}

pub struct OrderSqliteStorage {
    connection: Client,
}

impl OrderSqliteStorage {
    pub fn new(connection: Client) -> Self {
        Self { connection }
    }
}

impl OrderStorage for OrderSqliteStorage {
    async fn add(&self, order: Order) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "INSERT INTO orders (id, store_id, item_id, order_date, order_time, quantity, client_name, client_phone, confirmation) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)";

        let mut params: Vec<Value> = Vec::with_capacity(9);
        params.push(order.id.value().into());
        params.push(order.store_id.value().into());
        params.push(order.item_id.value().into());
        params.push(order.order_date.format(&Rfc3339)?.into());
        params.push(order.order_time.format(time_format())?.into());
        params.push(i64::from(order.quantity.value()).into());
        params.push(order.client_name.value().to_string().into());
        params.push(order.client_phone.value().to_string().into());
        params.push((order.confirmation as i32).into());

        self.connection
            .conn(move |conn| {
                conn.execute(QUERY, params_from_iter(params))?;
                Ok(())
            })
            .await?;
        Ok(())
    }

    async fn get(&self, id: Uuid) -> Result<Option<Order>, Box<dyn std::error::Error>> {
        const QUERY: &str = "SELECT id, store_id, item_id, order_date, order_time, quantity, client_name, client_phone, confirmation FROM orders WHERE id = ?1";

        let result = self
            .connection
            .conn(move |conn| {
                let order = match conn.query_row(QUERY, [id], |row| {
                    // Чтение UUID с ручной конвертацией ошибки
                    let id_bytes: Vec<u8> = row.get(0)?;
                    let id = Uuid::from_slice(&id_bytes).map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            0,
                            rusqlite::types::Type::Blob,
                            Box::new(e),
                        )
                    })?;

                    let store_id_bytes: Vec<u8> = row.get(1)?;
                    let store_id = Uuid::from_slice(&store_id_bytes).map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            1,
                            rusqlite::types::Type::Blob,
                            Box::new(e),
                        )
                    })?;

                    let item_id_bytes: Vec<u8> = row.get(2)?;
                    let item_id = Uuid::from_slice(&item_id_bytes).map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            2,
                            rusqlite::types::Type::Blob,
                            Box::new(e),
                        )
                    })?;

                    // Парсинг даты с ручной конвертацией ошибки
                    let order_date_str: String = row.get(3)?;
                    let order_date =
                        time::UtcDateTime::parse(&order_date_str, &Rfc3339).map_err(|e| {
                            rusqlite::Error::FromSqlConversionFailure(
                                3,
                                rusqlite::types::Type::Text,
                                Box::new(e),
                            )
                        })?;

                    // Парсинг времени по тому же формату, что используется при записи
                    let order_time_str: String = row.get(4)?;
                    let order_time =
                        time::Time::parse(&order_time_str, time_format()).map_err(|e| {
                            rusqlite::Error::FromSqlConversionFailure(
                                4,
                                rusqlite::types::Type::Text,
                                Box::new(e),
                            )
                        })?;

                    let quantity: i32 = row.get(5)?;
                    let client_name: String = row.get(6)?;
                    let client_phone: String = row.get(7)?;

                    Order::create(
                        id,
                        store_id,
                        item_id,
                        order_date,
                        order_time,
                        quantity,
                        client_name,
                        client_phone,
                        row.get::<_, i32>(8)? != 0,
                    )
                    .map_err(sqlite_validation_error)
                }) {
                    Ok(order) => order,
                    Err(rusqlite::Error::QueryReturnedNoRows) => return Ok(None),
                    Err(e) => return Err(e),
                };

                Ok(Some(order))
            })
            .await?;
        Ok(result)
    }

    async fn remove(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "DELETE FROM orders WHERE id = ?1";
        self.connection
            .conn(move |conn| {
                conn.execute(QUERY, [id])?;
                Ok(())
            })
            .await?;
        Ok(())
    }

    async fn update(&self, order: &Order) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "UPDATE orders SET store_id=?2, item_id=?3, order_date=?4, order_time=?5, quantity=?6, client_name=?7, client_phone=?8, confirmation=?9 WHERE id=?1";

        let mut params: Vec<Value> = Vec::with_capacity(9);
        params.push(order.id.value().into());
        params.push(order.store_id.value().into());
        params.push(order.item_id.value().into());
        params.push(order.order_date.format(&Rfc3339)?.into());
        params.push(order.order_time.format(time_format())?.into());
        params.push(i64::from(order.quantity.value()).into());
        params.push(order.client_name.value().to_string().into());
        params.push(order.client_phone.value().to_string().into());
        params.push((order.confirmation as i32).into());

        self.connection
            .conn(move |conn| {
                conn.execute(QUERY, params_from_iter(params))?;
                Ok(())
            })
            .await?;
        Ok(())
    }
}
