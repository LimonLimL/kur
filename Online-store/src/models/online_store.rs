use time::UtcDateTime;
use uuid::Uuid;

#[doc = "Интернет-магазин:
        Поля сущности:
        - идентификатор
        - электронный адрес
        - оплата доставки
        - дата создания
        - дата удаления
        - дата обновления"]
pub struct OnlineStore {
    pub(crate) id: Uuid,
    pub(crate) email: String,
    pub(crate) delivery_payment: bool,
    pub(crate) created_at: UtcDateTime,
    pub(crate) updated_at: Option<UtcDateTime>,
    pub(crate) deleted_at: Option<UtcDateTime>,
}

impl OnlineStore {
    #[doc = "Конструктор для восстановления магазина из БД (все поля известны)"]
    pub fn create(
        id: Uuid,
        email: String,
        delivery_payment: bool,
        created_at: UtcDateTime,
        updated_at: Option<UtcDateTime>,
        deleted_at: Option<UtcDateTime>,
    ) -> Self {
        Self {
            id,
            email,
            delivery_payment,
            created_at,
            updated_at,
            deleted_at,
        }
    }

    #[doc = "Конструктор нового магазина: идентификатор и дата создания создаются автоматически"]
    pub fn create_new(email: String, delivery_payment: bool) -> Self {
        let id: Uuid = Uuid::new_v4();
        let created_at: UtcDateTime = UtcDateTime::now();
        Self {
            id,
            email,
            delivery_payment,
            created_at,
            updated_at: None,
            deleted_at: None,
        }
    }
}
