//! UART instance.

use super::blocking::BlockingSerial;
use super::config::UartConfig;
use super::pad::IntoUartPads;
use super::register::RegisterBlock;
use super::uart_ext::UartExt;
use crate::cmu::Cmu;
use core::marker::PhantomData;

/// UART with statically known instance number.
pub struct Uart<const I: u8> {
    reg: *const RegisterBlock,
    _private: PhantomData<()>,
}

impl<const I: u8> Uart<I> {
    /// Create a new UART instance.
    pub const fn __new(reg: *const RegisterBlock) -> Self {
        Self {
            reg,
            _private: PhantomData,
        }
    }

    /// Get a reference to the register block.
    pub const fn register_block(&self) -> &'static RegisterBlock {
        unsafe { &*self.reg }
    }
}

impl<const I: u8> UartExt<'static, I> for Uart<I> {
    #[inline]
    fn new_blocking<PADS>(
        self,
        pads: impl IntoUartPads<'static, I, PADS>,
        config: UartConfig,
        cmu: &mut Cmu,
    ) -> BlockingSerial<'static, I, PADS> {
        BlockingSerial::new(self.register_block(), pads, config, cmu)
    }
}
