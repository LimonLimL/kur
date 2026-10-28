use std::collections::HashMap;
use time::UtcDateTime;
use uuid::Uuid;

use crate::models::value_objects::{
    EntityId, EntityIdError, FormatOptions, GuaranteePeriod, GuaranteePeriodError, Name, NameError,
    Price, PriceError, RecordLifetime, RecordLifetimeError, impl_error,
};

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
    pub(crate) id: EntityId,
    pub(crate) name: Name,
    pub(crate) company: Name,
    pub(crate) model: Name,
    pub(crate) characteristics: HashMap<String, String>,
    pub(crate) price: Price,
    pub(crate) guarantee_period: GuaranteePeriod,
    pub(crate) image: String,
    pub(crate) record_lifetime: RecordLifetime,
}

/// Проверенные данные товара, которые можно создать или обновить.
pub struct ItemDetails {
    pub(crate) name: Name,
    pub(crate) company: Name,
    pub(crate) model: Name,
    pub(crate) characteristics: HashMap<String, String>,
    pub(crate) price: Price,
    pub(crate) guarantee_period: GuaranteePeriod,
    pub(crate) image: String,
}

#[derive(Debug)]
// Варианты повторяют имена ошибок значения, которые они оборачивают.
#[allow(clippy::enum_variant_names)]
pub enum ItemError {
    EntityIdError(EntityIdError),
    NameError(NameError),
    PriceError(PriceError),
    GuaranteePeriodError(GuaranteePeriodError),
    RecordLifetimeError(RecordLifetimeError),
}

impl From<EntityIdError> for ItemError {
    fn from(error: EntityIdError) -> Self {
        Self::EntityIdError(error)
    }
}

impl From<NameError> for ItemError {
    fn from(error: NameError) -> Self {
        Self::NameError(error)
    }
}

impl From<PriceError> for ItemError {
    fn from(error: PriceError) -> Self {
        Self::PriceError(error)
    }
}

impl From<GuaranteePeriodError> for ItemError {
    fn from(error: GuaranteePeriodError) -> Self {
        Self::GuaranteePeriodError(error)
    }
}

impl From<RecordLifetimeError> for ItemError {
    fn from(error: RecordLifetimeError) -> Self {
        Self::RecordLifetimeError(error)
    }
}

impl_error!(ItemError {
    ItemError::EntityIdError(error) => "некорректный идентификатор товара: {error}",
    ItemError::NameError(error) => "некорректное название, фирма или модель товара: {error}",
    ItemError::PriceError(error) => "некорректная цена товара: {error}",
    ItemError::GuaranteePeriodError(error) => "некорректный гарантийный срок: {error}",
    ItemError::RecordLifetimeError(error) => "некорректное время жизни товара: {error}",
});

impl ItemDetails {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: impl Into<String>,
        company: impl Into<String>,
        model: impl Into<String>,
        characteristics: HashMap<String, String>,
        price: f32,
        guarantee_period: i32,
        image: String,
    ) -> Result<Self, ItemError> {
        Ok(Self {
            name: Name::new_with_format_and_length_check(name, &[FormatOptions::Trim], 1, 255)?,
            company: Name::new_with_format_and_length_check(
                company,
                &[FormatOptions::Trim],
                1,
                128,
            )?,
            model: Name::new_with_format_and_length_check(model, &[FormatOptions::Trim], 1, 128)?,
            characteristics,
            price: Price::new(price)?,
            guarantee_period: GuaranteePeriod::new(guarantee_period)?,
            image,
        })
    }
}

impl Item {
    #[doc = "Конструктор для восстановления товара из БД (все поля известны)"]
    #[allow(clippy::too_many_arguments)]
    pub fn create(
        id: Uuid,
        name: impl Into<String>,
        company: impl Into<String>,
        model: impl Into<String>,
        characteristics: HashMap<String, String>,
        price: f32,
        guarantee_period: i32,
        image: String,
        created_at: UtcDateTime,
        updated_at: Option<UtcDateTime>,
        deleted_at: Option<UtcDateTime>,
    ) -> Result<Self, ItemError> {
        let details = ItemDetails::new(
            name,
            company,
            model,
            characteristics,
            price,
            guarantee_period,
            image,
        )?;

        Ok(Self {
            id: EntityId::create(id)?,
            name: details.name,
            company: details.company,
            model: details.model,
            characteristics: details.characteristics,
            price: details.price,
            guarantee_period: details.guarantee_period,
            image: details.image,
            record_lifetime: RecordLifetime::create(created_at, updated_at, deleted_at)?,
        })
    }

    #[doc = "Конструктор нового товара: идентификатор и дата создания создаются автоматически"]
    #[allow(clippy::too_many_arguments)]
    pub fn create_new(
        name: impl Into<String>,
        company: impl Into<String>,
        model: impl Into<String>,
        characteristics: HashMap<String, String>,
        price: f32,
        guarantee_period: i32,
        image: String,
    ) -> Result<Self, ItemError> {
        let created_at = UtcDateTime::now();

        Self::create(
            Uuid::new_v4(),
            name,
            company,
            model,
            characteristics,
            price,
            guarantee_period,
            image,
            created_at,
            None,
            None,
        )
    }

    /// Применяет к товару уже проверенные новые данные.
    pub fn update_details(&mut self, details: ItemDetails) {
        self.name = details.name;
        self.company = details.company;
        self.model = details.model;
        self.characteristics = details.characteristics;
        self.price = details.price;
        self.guarantee_period = details.guarantee_period;
        self.image = details.image;
        self.record_lifetime.touch();
    }
}
