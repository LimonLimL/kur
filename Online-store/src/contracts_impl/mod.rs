// Адаптеры магазинов, заказов и доставок будут подключены к HTTP-обработчикам
// в следующих этапах, поэтому их неиспользование пока не считается ошибкой.
#![allow(dead_code)]
// Список параметров собирается строго по порядку плейсхолдеров SQL-запроса,
// поэтому последовательные `push` здесь читаемее, чем литерал `vec![]`.
#![allow(clippy::vec_init_then_push)]

pub mod delivery_sqlite_storage;
pub mod item_sqlite_storage;
pub mod managed_connection_pool;
pub mod online_store_sqlite_storage;
pub mod order_sqlite_storage;

/// Преобразует ошибку валидации сущности в ошибку чтения SQLite.
pub(crate) fn sqlite_validation_error(
    error: impl std::error::Error + Send + Sync + 'static,
) -> rusqlite::Error {
    rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(error))
}
