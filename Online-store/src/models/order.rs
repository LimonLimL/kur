use crate::models::item::Item;
use crate::models::online_store::OnlineStore;
use crate::models::value_objects::{
    EntityId, EntityIdError, FormatOptions, ItemQuantity, ItemQuantityError, Name, NameError,
    PhoneNumber, PhoneNumberError, impl_error,
};
use time::{Time, UtcDateTime};
use uuid::Uuid;

#[doc = "Заказ:
        Поля сущности:
        - идентификатор заказа
        - идентификатор интернет-магазина
        - идентификатор товара
        - дата заказа
        - время заказа
        - количество
        - Ф.И.О. клиента
        - контактный телефон
        - подтверждение заказа"]
pub struct Order {
    pub(crate) id: EntityId,
    pub(crate) store_id: EntityId,
    pub(crate) item_id: EntityId,
    pub(crate) order_date: UtcDateTime,
    pub(crate) order_time: Time,
    pub(crate) quantity: ItemQuantity,
    pub(crate) client_name: Name,
    pub(crate) client_phone: PhoneNumber,
    pub(crate) confirmation: bool,
}

#[derive(Debug)]
// Варианты повторяют имена ошибок значения, которые они оборачивают.
#[allow(clippy::enum_variant_names)]
pub enum OrderError {
    EntityIdError(EntityIdError),
    ItemQuantityError(ItemQuantityError),
    NameError(NameError),
    PhoneNumberError(PhoneNumberError),
}

impl From<EntityIdError> for OrderError {
    fn from(error: EntityIdError) -> Self {
        Self::EntityIdError(error)
    }
}

impl From<ItemQuantityError> for OrderError {
    fn from(error: ItemQuantityError) -> Self {
        Self::ItemQuantityError(error)
    }
}

impl From<NameError> for OrderError {
    fn from(error: NameError) -> Self {
        Self::NameError(error)
    }
}

impl From<PhoneNumberError> for OrderError {
    fn from(error: PhoneNumberError) -> Self {
        Self::PhoneNumberError(error)
    }
}

impl_error!(OrderError {
    OrderError::EntityIdError(error) => "некорректный идентификатор в заказе: {error}",
    OrderError::ItemQuantityError(error) => "некорректное количество товара в заказе: {error}",
    OrderError::NameError(error) => "некорректное Ф.И.О. клиента: {error}",
    OrderError::PhoneNumberError(error) => "некорректный телефон клиента: {error}",
});

impl Order {
    #[doc = "Конструктор для восстановления заказа из БД (все поля известны)"]
    #[allow(clippy::too_many_arguments)]
    pub fn create(
        id: Uuid,
        store_id: Uuid,
        item_id: Uuid,
        order_date: UtcDateTime,
        order_time: Time,
        quantity: i32,
        client_name: impl Into<String>,
        client_phone: impl Into<String>,
        confirmation: bool,
    ) -> Result<Self, OrderError> {
        Ok(Self {
            id: EntityId::create(id)?,
            store_id: EntityId::create(store_id)?,
            item_id: EntityId::create(item_id)?,
            order_date,
            order_time,
            quantity: ItemQuantity::from_i32(quantity)?,
            client_name: Name::new_with_format_and_length_check(
                client_name,
                &[FormatOptions::Trim, FormatOptions::FirstLetterToUpper],
                1,
                255,
            )?,
            client_phone: PhoneNumber::new(client_phone)?,
            confirmation,
        })
    }

    #[doc = "Конструктор нового заказа: идентификатор, дата и время создаются автоматически,
            заказ еще не подтвержден клиентом"]
    pub fn create_new(
        store: &OnlineStore,
        item: &Item,
        quantity: i32,
        client_name: impl Into<String>,
        client_phone: impl Into<String>,
    ) -> Result<Self, OrderError> {
        let now = UtcDateTime::now();

        Self::create(
            Uuid::new_v4(),
            store.id.value(),
            item.id.value(),
            now,
            now.time(),
            quantity,
            client_name,
            client_phone,
            false,
        )
    }

    #[doc = "Подтверждение заказа клиентом по телефону"]
    pub fn confirm(&mut self) {
        self.confirmation = true;
    }
}
