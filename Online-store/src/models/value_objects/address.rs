/// Максимальная длина адреса доставки.
pub const MAX_ADDRESS_LENGTH: usize = 512;

/// Проверенный адрес доставки.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressError {
    AddressIsEmpty,
    AddressTooLong,
}

impl Address {
    pub fn new(value: impl Into<String>) -> Result<Self, AddressError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(AddressError::AddressIsEmpty);
        }

        if value.chars().count() > MAX_ADDRESS_LENGTH {
            return Err(AddressError::AddressTooLong);
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl_error!(AddressError {
    AddressError::AddressIsEmpty => "адрес доставки не может быть пустым",
    AddressError::AddressTooLong => "адрес доставки слишком длинный",
});
