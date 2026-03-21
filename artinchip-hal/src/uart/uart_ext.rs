//! UART extension traits.

use super::blocking::BlockingSerial;
use super::config::UartConfig;
use super::pad::{Receive, Transmit};
use crate::cmu::Cmu;

pub trait UartExt<'a, const I: u8> {
    /// Greats a blocking UART interface with the specified pads.
    fn new_blocking<TX, RX>(
        self,
        tx: TX,
        rx: RX,
        config: UartConfig,
        cmu: &mut Cmu,
    ) -> BlockingSerial<'a, I, TX, RX>
    where
        TX: Transmit<I>,
        RX: Receive<I>;
}
