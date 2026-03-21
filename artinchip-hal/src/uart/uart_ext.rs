//! UART extension traits.

use super::blocking::BlockingSerial;
use super::config::UartConfig;
use super::pad::{IntoReceive, IntoTransmit, Receive, Transmit};
use crate::cmu::Cmu;

pub trait UartExt<'a, const I: u8> {
    /// Greats a blocking UART interface with the specified pads.
    fn new_blocking<TX, RX>(
        self,
        tx: impl IntoTransmit<'static, I, TX>,
        rx: impl IntoReceive<'static, I, RX>,
        config: UartConfig,
        cmu: &mut Cmu,
    ) -> BlockingSerial<'a, I, (TX, RX)>
    where
        TX: Transmit<I>,
        RX: Receive<I>;
}
