use crate::models::order::Order;
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
    pub(crate) order_id: Uuid,
    pub(crate) delivery_date: UtcDateTime,
    pub(crate) delivery_time: Time,
    pub(crate) delivery_address: String,
    pub(crate) client_name: String,
    pub(crate) courier_name: String,
}

impl Delivery {
    #[doc = "Конструктор для восстановления доставки из БД (все поля известны)"]
    pub fn create(
        order: &Order,
        delivery_date: UtcDateTime,
        delivery_time: Time,
        delivery_address: String,
        client_name: String,
        courier_name: String,
    ) -> Self {
        Self {
            order_id: order.id,
            delivery_date,
            delivery_time,
            delivery_address,
            client_name,
            courier_name,
        }
    }

    #[doc = "Конструктор новой доставки: дата и время берутся из момента создания"]
    pub fn create_new(
        order: &Order,
        delivery_address: String,
        client_name: String,
        courier_name: String,
    ) -> Self {
        let now: UtcDateTime = UtcDateTime::now();
        Self {
            order_id: order.id,
            delivery_date: now,
            delivery_time: now.time(),
            delivery_address,
            client_name,
            courier_name,
        }
    }
}
