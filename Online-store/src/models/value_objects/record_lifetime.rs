use std::sync::LazyLock;

use time::UtcDateTime;
use time::format_description::well_known::Rfc3339;

static MIN_VALID_DATETIME: LazyLock<UtcDateTime> = LazyLock::new(|| {
    UtcDateTime::parse("2000-01-01T00:00:00Z", &Rfc3339)
        .expect("Минимальная допустимая дата должна быть корректной")
});

static MAX_VALID_DATETIME: LazyLock<UtcDateTime> = LazyLock::new(|| {
    UtcDateTime::parse("2060-01-01T00:00:00Z", &Rfc3339)
        .expect("Максимальная допустимая дата должна быть корректной")
});

/// Время жизни записи: создание, обновление и удаление.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordLifetime {
    created_at: UtcDateTime,
    updated_at: Option<UtcDateTime>,
    deleted_at: Option<UtcDateTime>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordLifetimeError {
    UpdatedBeforeCreated,
    DeletedBeforeCreated,
    DateTimeOutOfRange,
}

impl RecordLifetime {
    pub fn create(
        created_at: UtcDateTime,
        updated_at: Option<UtcDateTime>,
        deleted_at: Option<UtcDateTime>,
    ) -> Result<Self, RecordLifetimeError> {
        if let Some(updated_at) = updated_at
            && updated_at < created_at
        {
            return Err(RecordLifetimeError::UpdatedBeforeCreated);
        }

        if let Some(deleted_at) = deleted_at
            && deleted_at < created_at
        {
            return Err(RecordLifetimeError::DeletedBeforeCreated);
        }

        if !Self::is_in_range(created_at) {
            return Err(RecordLifetimeError::DateTimeOutOfRange);
        }

        if let Some(updated_at) = updated_at
            && !Self::is_in_range(updated_at)
        {
            return Err(RecordLifetimeError::DateTimeOutOfRange);
        }

        if let Some(deleted_at) = deleted_at
            && !Self::is_in_range(deleted_at)
        {
            return Err(RecordLifetimeError::DateTimeOutOfRange);
        }

        Ok(Self {
            created_at,
            updated_at,
            deleted_at,
        })
    }

    /// Отмечает запись как обновлённую в текущий момент.
    pub fn touch(&mut self) {
        self.updated_at = Some(UtcDateTime::now());
    }

    pub fn created_at(&self) -> UtcDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<UtcDateTime> {
        self.updated_at
    }

    pub fn deleted_at(&self) -> Option<UtcDateTime> {
        self.deleted_at
    }

    fn is_in_range(value: UtcDateTime) -> bool {
        *MIN_VALID_DATETIME <= value && value <= *MAX_VALID_DATETIME
    }
}

impl_error!(RecordLifetimeError {
    RecordLifetimeError::UpdatedBeforeCreated => "дата обновления не может быть раньше даты создания",
    RecordLifetimeError::DeletedBeforeCreated => "дата удаления не может быть раньше даты создания",
    RecordLifetimeError::DateTimeOutOfRange => "дата выходит за допустимый диапазон (2000-2060 годы)",
});
