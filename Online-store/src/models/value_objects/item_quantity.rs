/// Проверенное количество товара в заказе.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemQuantity(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemQuantityError {
    QuantityIsZero,
    QuantityIsNegative,
}

impl ItemQuantity {
    pub fn new(value: u32) -> Result<Self, ItemQuantityError> {
        if value == 0 {
            return Err(ItemQuantityError::QuantityIsZero);
        }

        Ok(Self(value))
    }

    /// Проверяет значение, полученное из базы данных или HTTP-слоя.
    pub fn from_i32(value: i32) -> Result<Self, ItemQuantityError> {
        if value < 0 {
            return Err(ItemQuantityError::QuantityIsNegative);
        }

        Self::new(value as u32)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

impl_error!(ItemQuantityError {
    ItemQuantityError::QuantityIsZero => "количество товара не может быть нулевым",
    ItemQuantityError::QuantityIsNegative => "количество товара не может быть отрицательным",
});
