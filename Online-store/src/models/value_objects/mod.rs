//! Объекты-значения предметной области.
//!
//! Каждый объект-значение инкапсулирует проверку корректности своего
//! значения. Сущности получают уже проверенные данные и не дублируют
//! одну и ту же валидацию в разных конструкторах.

macro_rules! impl_error {
    ($error:ty { $($pattern:pat => $message:expr),+ $(,)? }) => {
        impl std::fmt::Display for $error {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $($pattern => write!(formatter, $message)),+
                }
            }
        }

        impl std::error::Error for $error {}
    };
}

pub(crate) use impl_error;

mod address;
mod email;
mod entity_id;
mod guarantee_period;
mod item_quantity;
mod name;
mod phone_number;
mod price;
mod record_lifetime;

pub use address::{Address, AddressError};
pub use email::{Email, EmailError};
pub use entity_id::{EntityId, EntityIdError};
pub use guarantee_period::{GuaranteePeriod, GuaranteePeriodError};
pub use item_quantity::{ItemQuantity, ItemQuantityError};
pub use name::{FormatOptions, Name, NameError};
pub use phone_number::{PhoneNumber, PhoneNumberError};
pub use price::{Price, PriceError};
pub use record_lifetime::{RecordLifetime, RecordLifetimeError};
