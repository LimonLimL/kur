use async_sqlite::{JournalMode, Pool, PoolBuilder};
use rusqlite::{params_from_iter, types::Value};
use std::path::PathBuf;
use time::format_description::well_known::Rfc3339;
// ИСПРАВЛЕНИЕ: правильный путь к Borrowed
use time::format_description::Borrowed;
use uuid::Uuid;

use crate::contracts::order_storage::OrderStorage;
use crate::models::order::Order;
// Заглушки для связанных объектов
use crate::models::item::Item;
use crate::models::online_store::OnlineStore;
use std::collections::HashMap;

pub struct OrderSqliteStorage {
    connection: Pool,
}

impl OrderSqliteStorage {
    pub async fn new(database_url: &str) -> Self {
        let path_buf = PathBuf::from(database_url);
        let builder = PoolBuilder::new()
            .path(path_buf)
            .journal_mode(JournalMode::Wal)
            .num_conns(10);

        let connection = builder.open().await.expect("Database creation error");
        Self { connection }
    }
}

impl OrderStorage for OrderSqliteStorage {
    async fn add(&self, order: Order) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "INSERT INTO orders (id, store_id, item_id, order_date, order_time, quantity, client_name, client_phone, confirmation) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)";

        let mut params: Vec<Value> = Vec::with_capacity(9);
        params.push(order.id.into());
        params.push(order.store_id.into());
        params.push(order.item_id.into());
        params.push(order.order_date.format(&Rfc3339)?.into());
        params.push(order.order_time.to_string().into());
        params.push(order.quantity.into());
        params.push(order.client_name.into());
        params.push(order.client_phone.into());
        params.push((order.confirmation as i32).into());

        self.connection
            .conn(move |conn| {
                conn.execute(QUERY, params_from_iter(params))?;
                Ok(())
            })
            .await?;
        Ok(())
    }

    async fn get(&self, id: Uuid) -> Result<Order, Box<dyn std::error::Error>> {
        const QUERY: &str = "SELECT id, store_id, item_id, order_date, order_time, quantity, client_name, client_phone, confirmation FROM orders WHERE id = ?1";

        let result = self
            .connection
            .conn(move |conn| {
                conn.query_row(QUERY, [id], |row| {
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

                    // ИСПРАВЛЕННЫЙ ПАРСИНГ ВРЕМЕНИ
                    let time_fmt = Borrowed("[hour]:[minute]:[second]");
                    let order_time_str: String = row.get(4)?;
                    let order_time =
                        time::Time::parse(&order_time_str, &time_fmt).map_err(|e| {
                            rusqlite::Error::FromSqlConversionFailure(
                                4,
                                rusqlite::types::Type::Text,
                                Box::new(e),
                            )
                        })?;

                    // Создание заглушек для связанных объектов
                    let dummy_store = OnlineStore::create(
                        store_id,
                        "".to_string(),
                        false,
                        time::UtcDateTime::now(),
                        None,
                        None,
                    );
                    let dummy_item = Item::create(
                        item_id,
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                        HashMap::new(),
                        0.0,
                        0,
                        "".to_string(),
                        time::UtcDateTime::now(),
                        None,
                        None,
                    );

                    Ok(Order::create(
                        id,
                        &dummy_store,
                        &dummy_item,
                        order_date,
                        order_time,
                        row.get(5)?,
                        row.get(6)?,
                        row.get(7)?,
                        row.get::<_, i32>(8)? != 0,
                    ))
                })
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
        params.push(order.id.into());
        params.push(order.store_id.into());
        params.push(order.item_id.into());
        params.push(order.order_date.format(&Rfc3339)?.into());
        params.push(order.order_time.to_string().into());
        params.push(order.quantity.into());
        params.push(order.client_name.clone().into());
        params.push(order.client_phone.clone().into());
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
