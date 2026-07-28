#![no_std]
#![no_main]

use artinchip_hal::prelude::*;
use artinchip_hal::uart::*;
use artinchip_rt::{Peripherals, pbp_entry, prelude::*};
use log::info;
use panic_halt as _;

#[pbp_entry]
fn pbp_main(boot_param: BootParam, _private_data: &[u8]) {
    check_startup(&boot_param);
    let mut p = Peripherals::take();
    let tx = p.gpioa.pa0.into_uart0_tx();
    let rx = p.gpioa.pa1.into_uart0_rx();
    let mut pa5 = p.gpioa.pa5.into_pull_up_input();

    let _uart0 = uart_logger_init(p.uart0, tx, rx, UartConfig::default(), &mut p.cmu).unwrap();

    info!("Welcome to pbp hello world example by artinchip-hal🦀!");
    loop {
        if pa5.is_low().unwrap_or(false) {
            info!("Button pressed!");
            while pa5.is_low().unwrap_or(false) {
                // wait for button to release
                core::hint::spin_loop();
            }
        }
    }
}
