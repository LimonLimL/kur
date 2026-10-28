use std::sync::LazyLock;

use regex::Regex;

/// Максимальная длина электронного адреса.
pub const MAX_EMAIL_LENGTH: usize = 255;

static EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\A[A-Za-z0-9._%+\-]+@[A-Za-z0-9.\-]+\.[A-Za-z]{2,}\z")
        .expect("Шаблон электронной почты должен быть корректным")
});

/// Проверенный электронный адрес интернет-магазина.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmailError {
    EmptyEmail,
    InvalidFormat,
    EmailTooLong,
}

impl Email {
    pub fn new(value: impl Into<String>) -> Result<Self, EmailError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(EmailError::EmptyEmail);
        }

        if value.chars().count() > MAX_EMAIL_LENGTH {
            return Err(EmailError::EmailTooLong);
        }

        if !EMAIL_REGEX.is_match(&value) {
            return Err(EmailError::InvalidFormat);
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl_error!(EmailError {
    EmailError::EmptyEmail => "электронный адрес не может быть пустым",
    EmailError::InvalidFormat => "электронный адрес имеет некорректный формат",
    EmailError::EmailTooLong => "электронный адрес слишком длинный",
});
