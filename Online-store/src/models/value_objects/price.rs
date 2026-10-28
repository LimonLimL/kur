/// Проверенная цена товара.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Price(f32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriceError {
    PriceIsNegative,
    PriceIsNotFinite,
}

impl Price {
    pub fn new(value: f32) -> Result<Self, PriceError> {
        if !value.is_finite() {
            return Err(PriceError::PriceIsNotFinite);
        }

        if value < 0.0 {
            return Err(PriceError::PriceIsNegative);
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

impl_error!(PriceError {
    PriceError::PriceIsNegative => "цена не может быть отрицательной",
    PriceError::PriceIsNotFinite => "цена должна быть конечным числом",
});
