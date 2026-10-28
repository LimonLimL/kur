use time::UtcDateTime;
use uuid::Uuid;

use crate::models::value_objects::{
    Email, EmailError, EntityId, EntityIdError, RecordLifetime, RecordLifetimeError, impl_error,
};

#[doc = "Интернет-магазин:
        Поля сущности:
        - идентификатор
        - электронный адрес
        - оплата доставки
        - дата создания
        - дата удаления
        - дата обновления"]
pub struct OnlineStore {
    pub(crate) id: EntityId,
    pub(crate) email: Email,
    pub(crate) delivery_payment: bool,
    pub(crate) record_lifetime: RecordLifetime,
}

#[derive(Debug)]
// Варианты повторяют имена ошибок значения, которые они оборачивают.
#[allow(clippy::enum_variant_names)]
pub enum OnlineStoreError {
    EntityIdError(EntityIdError),
    EmailError(EmailError),
    RecordLifetimeError(RecordLifetimeError),
}

impl From<EntityIdError> for OnlineStoreError {
    fn from(error: EntityIdError) -> Self {
        Self::EntityIdError(error)
    }
}

impl From<EmailError> for OnlineStoreError {
    fn from(error: EmailError) -> Self {
        Self::EmailError(error)
    }
}

impl From<RecordLifetimeError> for OnlineStoreError {
    fn from(error: RecordLifetimeError) -> Self {
        Self::RecordLifetimeError(error)
    }
}

impl_error!(OnlineStoreError {
    OnlineStoreError::EntityIdError(error) => "некорректный идентификатор магазина: {error}",
    OnlineStoreError::EmailError(error) => "некорректный электронный адрес магазина: {error}",
    OnlineStoreError::RecordLifetimeError(error) => "некорректное время жизни записи магазина: {error}",
});

impl OnlineStore {
    #[doc = "Конструктор для восстановления магазина из БД (все поля известны)"]
    pub fn create(
        id: Uuid,
        email: impl Into<String>,
        delivery_payment: bool,
        created_at: UtcDateTime,
        updated_at: Option<UtcDateTime>,
        deleted_at: Option<UtcDateTime>,
    ) -> Result<Self, OnlineStoreError> {
        Ok(Self {
            id: EntityId::create(id)?,
            email: Email::new(email)?,
            delivery_payment,
            record_lifetime: RecordLifetime::create(created_at, updated_at, deleted_at)?,
        })
    }

    #[doc = "Конструктор нового магазина: идентификатор и дата создания создаются автоматически"]
    pub fn create_new(
        email: impl Into<String>,
        delivery_payment: bool,
    ) -> Result<Self, OnlineStoreError> {
        let created_at = UtcDateTime::now();

        Self::create(
            Uuid::new_v4(),
            email,
            delivery_payment,
            created_at,
            None,
            None,
        )
    }
}
