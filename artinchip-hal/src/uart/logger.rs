//! UART0 logger driven by BlockingSerial — polling TX works even with interrupts disabled.

use core::fmt::Write as _;

use crate::{cmu::Cmu, uart::*};
use heapless::String;
use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};

static mut REG: *const RegisterBlock = core::ptr::null();
static mut INITIALIZED: bool = false;

struct Uart0Logger;

impl Log for Uart0Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        unsafe { INITIALIZED }
    }

    fn log(&self, record: &Record) {
        unsafe {
            if !INITIALIZED {
                return;
            }
        }

        let reg = unsafe { &*REG };
        let uart16550 = &reg.uart16550;

        let mut s: String<256> = String::new();
        write!(s, "[{}] {}", record.level(), record.args()).ok();

        for &byte in s.as_bytes() {
            while !uart16550.lsr().read().is_transmitter_fifo_empty() {
                core::hint::spin_loop();
            }
            uart16550.rbr_thr().tx_data(byte);
        }
        // Newline after each record
        while !uart16550.lsr().read().is_transmitter_fifo_empty() {
            core::hint::spin_loop();
        }
        uart16550.rbr_thr().tx_data(b'\n');

        self.flush();
    }

    fn flush(&self) {
        unsafe {
            if !INITIALIZED {
                return;
            }
        }
        let reg = unsafe { &*REG };
        while !reg.uart16550.lsr().read().is_transmitter_empty() {
            core::hint::spin_loop();
        }
    }
}

/// Initialize the global logger on UART0 and return a `BlockingSerial`.
///
/// The returned `BlockingSerial` can be used for subsequent blocking I/O
/// independently of the logger (theoretically it won't conflict with logging,
/// but using UART0 for general data transfer is not recommended).
pub fn uart_logger_init<TX, RX>(
    uart: Uart<0>,
    tx: TX,
    rx: RX,
    config: UartConfig,
    cmu: &mut Cmu,
) -> Result<BlockingSerial<'static, 0, TX, RX>, SetLoggerError>
where
    TX: UartPad<0> + Transmit<0>,
    RX: UartPad<0> + Receive<0>,
    Uart<0>: UartExt<'static, 0>,
{
    let reg_ptr: *const RegisterBlock = uart.register_block() as *const RegisterBlock;

    // `new_blocking` handles all hardware initialization.
    let serial = uart.new_blocking(tx, rx, config, cmu);

    unsafe {
        REG = reg_ptr;
        INITIALIZED = true;
    }

    log::set_logger(&Uart0Logger)?;
    log::set_max_level(LevelFilter::Trace);

    Ok(serial)
}
