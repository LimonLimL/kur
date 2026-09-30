use async_sqlite::{JournalMode, Pool, PoolBuilder};
use rusqlite::{params_from_iter, types::Value};
use serde_json;
use std::path::PathBuf;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

use crate::contracts::item_storage::ItemStorage;
use crate::models::item::Item;

pub struct ItemSqliteStorage {
    connection: Pool,
}

impl ItemSqliteStorage {
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

impl ItemStorage for ItemSqliteStorage {
    async fn add(&self, item: Item) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "INSERT INTO items (id, name, company, model, characteristics, price, guarantee_period, image, created_at, updated_at, deleted_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)";

        let mut params: Vec<Value> = Vec::with_capacity(11);
        params.push(item.id.into());
        params.push(item.name.into());
        params.push(item.company.into());
        params.push(item.model.into());

        // Конвертация ошибки сериализации JSON
        let chars_json = serde_json::to_string(&item.characteristics)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        params.push(chars_json.into());

        params.push(item.price.into());
        params.push(item.guarantee_period.into());
        params.push(item.image.into());
        params.push(item.created_at.format(&Rfc3339)?.into());
        params.push(item.updated_at.map(|t| t.format(&Rfc3339).unwrap()).into());
        params.push(item.deleted_at.map(|t| t.format(&Rfc3339).unwrap()).into());

        self.connection
            .conn(move |conn| {
                conn.execute(QUERY, params_from_iter(params))?;
                Ok(())
            })
            .await?;
        Ok(())
    }

    async fn get(&self, id: Uuid) -> Result<Item, Box<dyn std::error::Error>> {
        const QUERY: &str = "SELECT id, name, company, model, characteristics, price, guarantee_period, image, created_at, updated_at, deleted_at FROM items WHERE id = ?1";

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

                    // Чтение JSON с ручной конвертацией ошибки
                    let chars_str: String = row.get(4)?;
                    let characteristics: std::collections::HashMap<String, String> =
                        serde_json::from_str(&chars_str).map_err(|e| {
                            rusqlite::Error::FromSqlConversionFailure(
                                4,
                                rusqlite::types::Type::Text,
                                Box::new(e),
                            )
                        })?;

                    // Парсинг времени
                    let created_at_str: String = row.get(8)?;
                    let created_at =
                        time::UtcDateTime::parse(&created_at_str, &Rfc3339).map_err(|e| {
                            rusqlite::Error::FromSqlConversionFailure(
                                8,
                                rusqlite::types::Type::Text,
                                Box::new(e),
                            )
                        })?;

                    let updated_at: Option<String> = row.get(9)?;
                    let updated_at = updated_at
                        .map(|s| {
                            time::UtcDateTime::parse(&s, &Rfc3339).map_err(|e| {
                                rusqlite::Error::FromSqlConversionFailure(
                                    9,
                                    rusqlite::types::Type::Text,
                                    Box::new(e),
                                )
                            })
                        })
                        .transpose()?;

                    let deleted_at: Option<String> = row.get(10)?;
                    let deleted_at = deleted_at
                        .map(|s| {
                            time::UtcDateTime::parse(&s, &Rfc3339).map_err(|e| {
                                rusqlite::Error::FromSqlConversionFailure(
                                    10,
                                    rusqlite::types::Type::Text,
                                    Box::new(e),
                                )
                            })
                        })
                        .transpose()?;

                    Ok(Item::create(
                        id,
                        row.get(1)?,
                        row.get(2)?,
                        row.get(3)?,
                        characteristics,
                        row.get(5)?,
                        row.get(6)?,
                        row.get(7)?,
                        created_at,
                        updated_at,
                        deleted_at,
                    ))
                })
            })
            .await?;
        Ok(result)
    }

    async fn remove(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "DELETE FROM items WHERE id = ?1";
        self.connection
            .conn(move |conn| {
                conn.execute(QUERY, [id])?;
                Ok(())
            })
            .await?;
        Ok(())
    }

    async fn update(&self, item: &Item) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "UPDATE items SET name=?2, company=?3, model=?4, characteristics=?5, price=?6, guarantee_period=?7, image=?8, updated_at=?9, deleted_at=?10 WHERE id=?1";

        let mut params: Vec<Value> = Vec::with_capacity(10);
        params.push(item.id.into());
        params.push(item.name.clone().into());
        params.push(item.company.clone().into());
        params.push(item.model.clone().into());

        let chars_json = serde_json::to_string(&item.characteristics)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        params.push(chars_json.into());

        params.push(item.price.into());
        params.push(item.guarantee_period.into());
        params.push(item.image.clone().into());
        params.push(item.updated_at.map(|t| t.format(&Rfc3339).unwrap()).into());
        params.push(item.deleted_at.map(|t| t.format(&Rfc3339).unwrap()).into());

        self.connection
            .conn(move |conn| {
                conn.execute(QUERY, params_from_iter(params))?;
                Ok(())
            })
            .await?;
        Ok(())
    }
}
