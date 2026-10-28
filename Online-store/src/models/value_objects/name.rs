/// Максимальная длина имени, названия или модели.
pub const MAX_NAME_LENGTH: usize = 255;

/// Проверенное непустое имя (название товара, фирма, модель, Ф.И.О.).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Варианты названы по правилам методички: с именем проверяемого значения.
#[allow(clippy::enum_variant_names)]
pub enum NameError {
    NameIsEmpty,
    NameTooLong,
    NameExceedsMaxLength,
    NameLessThanMinLength,
}

/// Дополнительное форматирование значения перед проверкой.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatOptions {
    Trim,
    FirstLetterToUpper,
}

impl Name {
    /// Создаёт непустое имя.
    pub fn new(value: impl Into<String>) -> Result<Self, NameError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(NameError::NameIsEmpty);
        }

        if value.chars().count() > MAX_NAME_LENGTH {
            return Err(NameError::NameTooLong);
        }

        Ok(Self(value))
    }

    /// Создаёт имя с применением форматирования.
    pub fn new_with_format(
        value: impl Into<String>,
        options: &[FormatOptions],
    ) -> Result<Self, NameError> {
        let formatted_value = Self::apply_format(value.into(), options);
        Self::new(formatted_value)
    }

    /// Создаёт имя с проверкой длины.
    pub fn new_with_length_check(
        value: impl Into<String>,
        min_length: usize,
        max_length: usize,
    ) -> Result<Self, NameError> {
        let value = value.into();
        let length = value.chars().count();

        if length < min_length {
            return Err(NameError::NameLessThanMinLength);
        }

        if length > max_length {
            return Err(NameError::NameExceedsMaxLength);
        }

        Self::new(value)
    }

    /// Создаёт имя с форматированием и проверкой длины.
    pub fn new_with_format_and_length_check(
        value: impl Into<String>,
        options: &[FormatOptions],
        min_length: usize,
        max_length: usize,
    ) -> Result<Self, NameError> {
        let formatted_value = Self::apply_format(value.into(), options);
        Self::new_with_length_check(formatted_value, min_length, max_length)
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    fn apply_format(mut value: String, options: &[FormatOptions]) -> String {
        for option in options {
            match option {
                FormatOptions::Trim => value = Self::trim_value(&value),
                FormatOptions::FirstLetterToUpper => {
                    value = Self::first_letter_to_upper(&value);
                }
            }
        }

        value
    }

    fn trim_value(value: &str) -> String {
        value.trim().to_string()
    }

    fn first_letter_to_upper(value: &str) -> String {
        let mut chars = value.chars();

        match chars.next() {
            Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
            None => String::new(),
        }
    }
}

impl_error!(NameError {
    NameError::NameIsEmpty => "имя не может быть пустым",
    NameError::NameTooLong => "имя слишком длинное",
    NameError::NameExceedsMaxLength => "имя превышает максимально допустимую длину",
    NameError::NameLessThanMinLength => "имя короче минимально допустимой длины",
});
