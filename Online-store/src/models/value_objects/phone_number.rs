use std::sync::LazyLock;

use regex::Regex;

/// Номер телефона по правилу приложения: `7` и ровно десять цифр.
static PHONE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\A7[0-9]{10}\z").expect("Шаблон номера телефона должен быть корректным")
});

/// Проверенный контактный номер телефона.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhoneNumber(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhoneNumberError {
    EmptyNumber,
    InvalidFormat,
}

impl PhoneNumber {
    pub fn new(value: impl Into<String>) -> Result<Self, PhoneNumberError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(PhoneNumberError::EmptyNumber);
        }

        if !PHONE_REGEX.is_match(&value) {
            return Err(PhoneNumberError::InvalidFormat);
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl_error!(PhoneNumberError {
    PhoneNumberError::EmptyNumber => "номер телефона не может быть пустым",
    PhoneNumberError::InvalidFormat => "номер телефона должен начинаться с 7 и содержать ровно 10 цифр",
});
