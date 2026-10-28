use async_sqlite::Client;
use rusqlite::{params_from_iter, types::Value};
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

use crate::contracts::online_store_storage::OnlineStoreStorage;
use crate::models::online_store::OnlineStore;

use super::sqlite_validation_error;

pub struct OnlineStoreSqliteStorage {
    connection: Client,
}

impl OnlineStoreSqliteStorage {
    pub fn new(connection: Client) -> Self {
        Self { connection }
    }
}

impl OnlineStoreStorage for OnlineStoreSqliteStorage {
    async fn add(&self, store: OnlineStore) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "INSERT INTO online_stores (id, email, delivery_payment, created_at, updated_at, deleted_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)";

        let mut params: Vec<Value> = Vec::with_capacity(6);
        params.push(store.id.value().into());
        params.push(store.email.value().to_string().into());
        params.push((store.delivery_payment as i32).into());
        params.push(store.record_lifetime.created_at().format(&Rfc3339)?.into());
        params.push(
            store
                .record_lifetime
                .updated_at()
                .map(|t| t.format(&Rfc3339))
                .transpose()?
                .into(),
        );
        params.push(
            store
                .record_lifetime
                .deleted_at()
                .map(|t| t.format(&Rfc3339))
                .transpose()?
                .into(),
        );

        self.connection
            .conn(move |conn| {
                conn.execute(QUERY, params_from_iter(params))?;
                Ok(())
            })
            .await?;
        Ok(())
    }

    async fn get(&self, id: Uuid) -> Result<Option<OnlineStore>, Box<dyn std::error::Error>> {
        const QUERY: &str = "SELECT id, email, delivery_payment, created_at, updated_at, deleted_at FROM online_stores WHERE id = ?1";

        let result = self
            .connection
            .conn(move |conn| {
                let store = match conn.query_row(QUERY, [id], |row| {
                    let id_bytes: Vec<u8> = row.get(0)?;
                    let id = Uuid::from_slice(&id_bytes).map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            0,
                            rusqlite::types::Type::Blob,
                            Box::new(e),
                        )
                    })?;

                    let created_at_str: String = row.get(3)?;
                    let created_at =
                        time::UtcDateTime::parse(&created_at_str, &Rfc3339).map_err(|e| {
                            rusqlite::Error::FromSqlConversionFailure(
                                3,
                                rusqlite::types::Type::Text,
                                Box::new(e),
                            )
                        })?;

                    let updated_at: Option<String> = row.get(4)?;
                    let updated_at = updated_at
                        .map(|s| {
                            time::UtcDateTime::parse(&s, &Rfc3339).map_err(|e| {
                                rusqlite::Error::FromSqlConversionFailure(
                                    4,
                                    rusqlite::types::Type::Text,
                                    Box::new(e),
                                )
                            })
                        })
                        .transpose()?;

                    let deleted_at: Option<String> = row.get(5)?;
                    let deleted_at = deleted_at
                        .map(|s| {
                            time::UtcDateTime::parse(&s, &Rfc3339).map_err(|e| {
                                rusqlite::Error::FromSqlConversionFailure(
                                    5,
                                    rusqlite::types::Type::Text,
                                    Box::new(e),
                                )
                            })
                        })
                        .transpose()?;

                    let email: String = row.get(1)?;

                    OnlineStore::create(
                        id,
                        email,
                        row.get::<_, i32>(2)? != 0,
                        created_at,
                        updated_at,
                        deleted_at,
                    )
                    .map_err(sqlite_validation_error)
                }) {
                    Ok(store) => store,
                    Err(rusqlite::Error::QueryReturnedNoRows) => return Ok(None),
                    Err(e) => return Err(e),
                };

                Ok(Some(store))
            })
            .await?;
        Ok(result)
    }

    async fn remove(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "DELETE FROM online_stores WHERE id = ?1";
        self.connection
            .conn(move |conn| {
                conn.execute(QUERY, [id])?;
                Ok(())
            })
            .await?;
        Ok(())
    }

    async fn update(&self, store: &OnlineStore) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "UPDATE online_stores SET email=?2, delivery_payment=?3, updated_at=?4, deleted_at=?5 WHERE id=?1";

        let mut params: Vec<Value> = Vec::with_capacity(5);
        params.push(store.id.value().into());
        params.push(store.email.value().to_string().into());
        params.push((store.delivery_payment as i32).into());
        params.push(
            store
                .record_lifetime
                .updated_at()
                .map(|t| t.format(&Rfc3339))
                .transpose()?
                .into(),
        );
        params.push(
            store
                .record_lifetime
                .deleted_at()
                .map(|t| t.format(&Rfc3339))
                .transpose()?
                .into(),
        );

        self.connection
            .conn(move |conn| {
                conn.execute(QUERY, params_from_iter(params))?;
                Ok(())
            })
            .await?;
        Ok(())
    }
}
