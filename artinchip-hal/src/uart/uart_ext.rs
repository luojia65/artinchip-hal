//! UART extension traits.

use super::blocking::BlockingSerial;
use super::config::UartConfig;
use super::pad::IntoUartPads;
use crate::cmu::Cmu;

pub trait UartExt<'a, const I: u8> {
    /// Greats a blocking UART interface with the specified pads.
    fn new_blocking<PADS>(
        self,
        pads: impl IntoUartPads<'static, I, PADS>,
        config: UartConfig,
        cmu: &mut Cmu,
    ) -> BlockingSerial<'static, I, PADS>;
}
