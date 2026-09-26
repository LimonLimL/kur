#[doc = "Интернет-магазин:
        Поля сущности:
        - идентификатор
        - электронный адрес
        - оплата доставки
        - дата создания
        - дата удаления
        - дата обновления"]
pub struct OnlineStore {
    // - код товара (UUID)
    pub(crate) id: String,
    // - электронный адрес (String)
    pub(crate) email: String,
    // - оплата доставки (bool)
    pub(crate) delivery: bool,

    pub(crate) created_at: String,
    pub(crate) updated_at: Option<String>,
    pub(crate) deleted_at: Option<String>,
}

#[doc = "Товар:
        Поля сущности:
        - идентификатор
        - название товара
        - цена товара
        - дата создания
        - дата удаления
        - дата обновления"]
pub struct Item {
    // - код товара (UUID)
    pub(crate) id: String,
    // - Наименование товара (String)
    pub(crate) name: String,
    // - фирма (String)
    pub(crate) company: String,
    // - модель (String)
    pub(crate) model: String,
    // - технические характеристики (String)
    pub(crate) characteristics: String,
    // - Цена товара (float)
    pub(crate) price: f32,
    // - гарантийный срок (int)
    pub(crate) guarantee_period: i32,
    // - изображение (String)
    pub(crate) image: String,

    pub(crate) created_at: String,
    pub(crate) updated_at: Option<String>,
    pub(crate) deleted_at: Option<String>,
}

#[doc = "Заказы:
        Поля сущности:
        - идентификатор
        - идентификатор магазина
        - идентификатор товара
        - дата заказа
        - время заказа
        - количество
        - Ф.И.О. клиента
        - контактный телефон
        - подтверждение заказа"]
pub struct order {
    // - код заказа (UUID)
    pub(crate) id: String,
    // - идентификатор магазина (UUID)
    pub(crate) store_id: String,
    // - идентификатор товара (UUID)
    pub(crate) item_id: String,
    // - дата заказа (String)
    pub(crate) order_date: String,
    // - время заказа (String)
    pub(crate) order_time: String,
    // - количество (int)
    pub(crate) quantity: i32,
    // - Ф.И.О. клиента (String)
    pub(crate) client_name: String,
    // - контактный телефон (String)
    pub(crate) client_phone: String,
    // - подтверждение заказа (bool)
    pub(crate) confirmation: bool,
}

#[doc = "Доставка
        Поля сущности:
        - идентификатор
        - дата доставки
        - время доставки
        - адрес доставки
        - Ф.И.О. клиента
        - Ф.И.О. курьера"]

pub struct Delivery {
    // - код доставки (UUID)
    pub(crate) id: String,
    // - дата доставки (String)
    pub(crate) delivery_date: String,
    // - время доставки (String)
    pub(crate) delivery_time: String,
    // - адрес доставки (String)
    pub(crate) delivery_address: String,
    // - Ф.И.О. клиента (String)
    pub(crate) client_name: String,
    // - Ф.И.О. курьера (String)
    pub(crate) courier_name: String,
}
