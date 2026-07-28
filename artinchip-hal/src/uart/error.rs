//! UART error types.

use core::fmt;

/// UART bus error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UartError {
    /// Timeout waiting for hardware.
    Timeout,
}

impl fmt::Display for UartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Timeout => write!(f, "UART timeout"),
        }
    }
}

impl core::error::Error for UartError {}

impl embedded_io::Error for UartError {
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}
