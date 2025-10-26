#![no_std]
#![no_main]

use artinchip_hal::gpio::{self, OutputClear, OutputSet};
use artinchip_rt::pbp_entry;
use panic_halt as _;

#[pbp_entry]
fn pbp_main(_boot_param: u32, _private_data: &[u8]) {
    let gpio = unsafe { &*(0x18700000 as *const gpio::RegisterBlock) };
    let gpioa = &gpio.groups[0];
    unsafe {
        // pa5 = function 1
        gpioa.pin_config[5].modify(|v| {
            v.set_pin_pull(gpio::PinPull::Disabled)
                .enable_general_output()
                .disable_general_input()
                .set_drive_strength(gpio::PinDriveStrength::Level3)
                .set_pin_func(1)
        });
    }

    loop {
        // pa5 set
        unsafe { gpioa.output_set.write(OutputSet::default().set_output(5)) };
        for _ in 0..=100_000 {
            unsafe { core::arch::asm!("nop") };
        }
        // pa5 clear
        unsafe {
            gpioa
                .output_clear
                .write(OutputClear::default().clear_output(5))
        };
        for _ in 0..=100_000 {
            unsafe { core::arch::asm!("nop") };
        }
    }
}
