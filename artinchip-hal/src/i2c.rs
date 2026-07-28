//! Inter-Integrated Circuit (I2C).

mod blocking;
mod config;
mod error;
mod i2c_ext;
mod instance;
mod pad;
mod register;

pub use blocking::*;
pub use config::*;
pub use error::*;
pub use i2c_ext::I2cExt;
pub use instance::I2c;
pub use pad::*;
pub use register::*;
