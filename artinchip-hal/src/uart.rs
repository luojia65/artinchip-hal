//! Universal Asynchronous Receiver-Transmitter (UART).

mod blocking;
mod config;
mod instance;
#[cfg(feature = "uart-logger")]
mod logger;
#[cfg(feature = "clic-interrupts")]
mod non_blocking;
mod pad;
mod register;
mod uart_ext;

pub use blocking::*;
pub use config::*;
pub use instance::Uart;
#[cfg(feature = "uart-logger")]
pub use logger::*;
#[cfg(feature = "clic-interrupts")]
pub use non_blocking::*;
pub use pad::*;
pub use register::*;
pub use uart_ext::UartExt;
