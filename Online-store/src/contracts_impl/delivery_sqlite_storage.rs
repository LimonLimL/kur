use async_sqlite::{JournalMode, Pool, PoolBuilder};
use rusqlite::{params_from_iter, types::Value};
use std::path::PathBuf;
use time::format_description::well_known::Rfc3339;
// ИСПРАВЛЕНИЕ: правильный путь к Borrowed
use time::format_description::Borrowed;
use uuid::Uuid;

use crate::contracts::delivery_storage::DeliveryStorage;
use crate::models::delivery::Delivery;
// Заглушки для связанных объектов
use crate::models::item::Item;
use crate::models::online_store::OnlineStore;
use crate::models::order::Order;
use std::collections::HashMap;

pub struct DeliverySqliteStorage {
    connection: Pool,
}

impl DeliverySqliteStorage {
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

impl DeliveryStorage for DeliverySqliteStorage {
    async fn add(&self, delivery: Delivery) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "INSERT INTO deliveries (id, order_id, delivery_date, delivery_time, address, client_name, courier_name) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)";

        let mut params: Vec<Value> = Vec::with_capacity(7);
        params.push(Uuid::new_v4().into());
        params.push(delivery.order_id.into());
        params.push(delivery.delivery_date.format(&Rfc3339)?.into());
        params.push(delivery.delivery_time.to_string().into());
        params.push(delivery.delivery_address.into());
        params.push(delivery.client_name.into());
        params.push(delivery.courier_name.into());

        self.connection
            .conn(move |conn| {
                conn.execute(QUERY, params_from_iter(params))?;
                Ok(())
            })
            .await?;
        Ok(())
    }

    async fn get(&self, order_id: Uuid) -> Result<Delivery, Box<dyn std::error::Error>> {
        const QUERY: &str = "SELECT id, order_id, delivery_date, delivery_time, address, client_name, courier_name FROM deliveries WHERE order_id = ?1";

        let result = self
            .connection
            .conn(move |conn| {
                conn.query_row(QUERY, [order_id], |row| {
                    // Чтение UUID с ручной конвертацией ошибки
                    let _id_bytes: Vec<u8> = row.get(0)?;
                    let order_id_bytes: Vec<u8> = row.get(1)?;
                    let order_id = Uuid::from_slice(&order_id_bytes).map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            1,
                            rusqlite::types::Type::Blob,
                            Box::new(e),
                        )
                    })?;

                    // Парсинг даты с ручной конвертацией ошибки
                    let delivery_date_str: String = row.get(2)?;
                    let delivery_date = time::UtcDateTime::parse(&delivery_date_str, &Rfc3339)
                        .map_err(|e| {
                            rusqlite::Error::FromSqlConversionFailure(
                                2,
                                rusqlite::types::Type::Text,
                                Box::new(e),
                            )
                        })?;

                    // ИСПРАВЛЕННЫЙ ПАРСИНГ ВРЕМЕНИ
                    let time_fmt = Borrowed("[hour]:[minute]:[second]");
                    let delivery_time_str: String = row.get(3)?;
                    let delivery_time =
                        time::Time::parse(&delivery_time_str, &time_fmt).map_err(|e| {
                            rusqlite::Error::FromSqlConversionFailure(
                                3,
                                rusqlite::types::Type::Text,
                                Box::new(e),
                            )
                        })?;

                    // Создание заглушек для связанных объектов
                    let dummy_store = OnlineStore::create(
                        Uuid::nil(),
                        "".to_string(),
                        false,
                        time::UtcDateTime::now(),
                        None,
                        None,
                    );
                    let dummy_item = Item::create(
                        Uuid::nil(),
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
                    let dummy_order = Order::create(
                        order_id,
                        &dummy_store,
                        &dummy_item,
                        time::UtcDateTime::now(),
                        time::Time::MIDNIGHT,
                        0,
                        "".to_string(),
                        "".to_string(),
                        false,
                    );

                    Ok(Delivery::create(
                        &dummy_order,
                        delivery_date,
                        delivery_time,
                        row.get(4)?,
                        row.get(5)?,
                        row.get(6)?,
                    ))
                })
            })
            .await?;
        Ok(result)
    }

    async fn remove(&self, order_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "DELETE FROM deliveries WHERE order_id = ?1";
        self.connection
            .conn(move |conn| {
                conn.execute(QUERY, [order_id])?;
                Ok(())
            })
            .await?;
        Ok(())
    }

    async fn update(&self, delivery: &Delivery) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "UPDATE deliveries SET delivery_date=?2, delivery_time=?3, address=?4, client_name=?5, courier_name=?6 WHERE order_id=?1";

        let mut params: Vec<Value> = Vec::with_capacity(6);
        params.push(delivery.order_id.into());
        params.push(delivery.delivery_date.format(&Rfc3339)?.into());
        params.push(delivery.delivery_time.to_string().into());
        params.push(delivery.delivery_address.clone().into());
        params.push(delivery.client_name.clone().into());
        params.push(delivery.courier_name.clone().into());

        self.connection
            .conn(move |conn| {
                conn.execute(QUERY, params_from_iter(params))?;
                Ok(())
            })
            .await?;
        Ok(())
    }
}
