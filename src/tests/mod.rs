#[macro_use]
pub mod macros;

mod protocol;
mod setup;

pub use self::protocol::Protocol;
pub use self::setup::IocTestSpawner;
