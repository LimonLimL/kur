use crate::models::order::Order;
use crate::models::value_objects::{
    Address, AddressError, EntityId, EntityIdError, FormatOptions, Name, NameError, impl_error,
};
use time::{Time, UtcDateTime};
use uuid::Uuid;

#[doc = "Доставка:
        Поля сущности:
        - идентификатор (код заказа, доставка 1:1 связана с заказом)
        - дата доставки
        - время доставки
        - адрес доставки
        - Ф.И.О. клиента
        - Ф.И.О. курьера"]
pub struct Delivery {
    pub(crate) order_id: EntityId,
    pub(crate) delivery_date: UtcDateTime,
    pub(crate) delivery_time: Time,
    pub(crate) delivery_address: Address,
    pub(crate) client_name: Name,
    pub(crate) courier_name: Name,
}

#[derive(Debug)]
// Варианты повторяют имена ошибок значения, которые они оборачивают.
#[allow(clippy::enum_variant_names)]
pub enum DeliveryError {
    EntityIdError(EntityIdError),
    AddressError(AddressError),
    NameError(NameError),
}

impl From<EntityIdError> for DeliveryError {
    fn from(error: EntityIdError) -> Self {
        Self::EntityIdError(error)
    }
}

impl From<AddressError> for DeliveryError {
    fn from(error: AddressError) -> Self {
        Self::AddressError(error)
    }
}

impl From<NameError> for DeliveryError {
    fn from(error: NameError) -> Self {
        Self::NameError(error)
    }
}

impl_error!(DeliveryError {
    DeliveryError::EntityIdError(error) => "некорректный идентификатор заказа в доставке: {error}",
    DeliveryError::AddressError(error) => "некорректный адрес доставки: {error}",
    DeliveryError::NameError(error) => "некорректное Ф.И.О. в доставке: {error}",
});

impl Delivery {
    #[doc = "Конструктор для восстановления доставки из БД (все поля известны)"]
    pub fn create(
        order_id: Uuid,
        delivery_date: UtcDateTime,
        delivery_time: Time,
        delivery_address: impl Into<String>,
        client_name: impl Into<String>,
        courier_name: impl Into<String>,
    ) -> Result<Self, DeliveryError> {
        Ok(Self {
            order_id: EntityId::create(order_id)?,
            delivery_date,
            delivery_time,
            delivery_address: Address::new(delivery_address)?,
            client_name: Name::new_with_format_and_length_check(
                client_name,
                &[FormatOptions::Trim, FormatOptions::FirstLetterToUpper],
                1,
                255,
            )?,
            courier_name: Name::new_with_format_and_length_check(
                courier_name,
                &[FormatOptions::Trim, FormatOptions::FirstLetterToUpper],
                1,
                255,
            )?,
        })
    }

    #[doc = "Конструктор новой доставки: дата и время берутся из момента создания"]
    pub fn create_new(
        order: &Order,
        delivery_address: impl Into<String>,
        client_name: impl Into<String>,
        courier_name: impl Into<String>,
    ) -> Result<Self, DeliveryError> {
        let now = UtcDateTime::now();

        Self::create(
            order.id.value(),
            now,
            now.time(),
            delivery_address,
            client_name,
            courier_name,
        )
    }
}
