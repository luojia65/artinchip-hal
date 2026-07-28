//! QSPI error types.

/// QSPI bus error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QspiError {
    /// Timeout waiting for hardware.
    Timeout,
}

impl embedded_hal::spi::Error for QspiError {
    fn kind(&self) -> embedded_hal::spi::ErrorKind {
        embedded_hal::spi::ErrorKind::Other
    }
}
