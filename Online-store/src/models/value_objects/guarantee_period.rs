/// Проверенный гарантийный срок товара в месяцах.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GuaranteePeriod(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuaranteePeriodError {
    Negative,
}

impl GuaranteePeriod {
    pub fn new(value: i32) -> Result<Self, GuaranteePeriodError> {
        if value < 0 {
            return Err(GuaranteePeriodError::Negative);
        }

        Ok(Self(value as u32))
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

impl_error!(GuaranteePeriodError {
    GuaranteePeriodError::Negative => "гарантийный срок не может быть отрицательным",
});
