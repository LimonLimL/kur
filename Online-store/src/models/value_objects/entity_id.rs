use uuid::{Uuid, Variant, Version};

/// Проверенный идентификатор сущности.
///
/// В отличие от «сырого» `Uuid`, значение этого типа гарантирует, что
/// идентификатор не пустой и имеет формат UUID v4.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityIdError {
    EmptyId,
    InvalidId,
}

impl EntityId {
    /// Создаёт новый идентификатор UUID v4.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Проверяет и оборачивает существующий идентификатор.
    pub fn create(id: Uuid) -> Result<Self, EntityIdError> {
        if id.is_nil() {
            return Err(EntityIdError::EmptyId);
        }

        if id.get_variant() != Variant::RFC4122 || id.get_version() != Some(Version::Random) {
            return Err(EntityIdError::InvalidId);
        }

        Ok(Self(id))
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self::new()
    }
}

impl_error!(EntityIdError {
    EntityIdError::EmptyId => "идентификатор не может быть пустым",
    EntityIdError::InvalidId => "идентификатор должен быть UUID версии 4",
});
