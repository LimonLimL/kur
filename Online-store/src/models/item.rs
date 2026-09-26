use std::collections::HashMap;
use time::UtcDateTime;
use uuid::Uuid;

#[doc = "Товар:
        Поля сущности:
        - идентификатор
        - название товара
        - фирма
        - модель
        - технические характеристики
        - цена товара
        - гарантийный срок
        - изображение
        - дата создания
        - дата удаления
        - дата обновления"]
pub struct Item {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) company: String,
    pub(crate) model: String,
    pub(crate) characteristics: HashMap<String, String>,
    pub(crate) price: f32,
    pub(crate) guarantee_period: i32,
    pub(crate) image: String,

    pub(crate) created_at: UtcDateTime,
    pub(crate) updated_at: Option<UtcDateTime>,
    pub(crate) deleted_at: Option<UtcDateTime>,
}

impl Item {
    #[doc = "Конструктор для восстановления товара из БД (все поля известны)"]
    #[allow(clippy::too_many_arguments)]
    pub fn create(
        id: Uuid,
        name: String,
        company: String,
        model: String,
        characteristics: HashMap<String, String>,
        price: f32,
        guarantee_period: i32,
        image: String,
        created_at: UtcDateTime,
        updated_at: Option<UtcDateTime>,
        deleted_at: Option<UtcDateTime>,
    ) -> Self {
        Self {
            id,
            name,
            company,
            model,
            characteristics,
            price,
            guarantee_period,
            image,
            created_at,
            updated_at,
            deleted_at,
        }
    }

    #[doc = "Конструктор нового товара: идентификатор и дата создания создаются автоматически"]
    pub fn create_new(
        name: String,
        company: String,
        model: String,
        characteristics: HashMap<String, String>,
        price: f32,
        guarantee_period: i32,
        image: String,
    ) -> Self {
        let id: Uuid = Uuid::new_v4();
        let created_at: UtcDateTime = UtcDateTime::now();
        Self {
            id,
            name,
            company,
            model,
            characteristics,
            price,
            guarantee_period,
            image,
            created_at,
            updated_at: None,
            deleted_at: None,
        }
    }
}
