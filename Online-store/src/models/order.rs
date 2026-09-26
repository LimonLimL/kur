use crate::models::item::Item;
use crate::models::online_store::OnlineStore;
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
    pub(crate) id: Uuid,
    pub(crate) store_id: Uuid,
    pub(crate) item_id: Uuid,
    pub(crate) order_date: UtcDateTime,
    pub(crate) order_time: Time,
    pub(crate) quantity: i32,
    pub(crate) client_name: String,
    pub(crate) client_phone: String,
    pub(crate) confirmation: bool,
}

impl Order {
    #[doc = "Конструктор для восстановления заказа из БД (все поля известны)"]
    #[allow(clippy::too_many_arguments)]
    pub fn create(
        id: Uuid,
        store: &OnlineStore,
        item: &Item,
        order_date: UtcDateTime,
        order_time: Time,
        quantity: i32,
        client_name: String,
        client_phone: String,
        confirmation: bool,
    ) -> Self {
        Self {
            id,
            store_id: store.id,
            item_id: item.id,
            order_date,
            order_time,
            quantity,
            client_name,
            client_phone,
            confirmation,
        }
    }

    #[doc = "Конструктор нового заказа: идентификатор, дата и время создаются автоматически,
            заказ еще не подтвержден клиентом"]
    pub fn create_new(
        store: &OnlineStore,
        item: &Item,
        quantity: i32,
        client_name: String,
        client_phone: String,
    ) -> Self {
        let id: Uuid = Uuid::new_v4();
        let now: UtcDateTime = UtcDateTime::now();
        Self {
            id,
            store_id: store.id,
            item_id: item.id,
            order_date: now,
            order_time: now.time(),
            quantity,
            client_name,
            client_phone,
            confirmation: false,
        }
    }

    #[doc = "Подтверждение заказа клиентом по телефону"]
    pub fn confirm(&mut self) {
        self.confirmation = true;
    }
}
