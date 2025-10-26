#![no_std]
#![no_main]

use artinchip_hal::gpio::{self, OutputClear, OutputSet};
use core::arch::naked_asm;
use panic_halt as _;

extern "C" fn pbp_main(_boot_param: u32, _priv_addr: *const (), _priv_len: u32) {
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
        // pa5 clear
        unsafe {
            gpioa
                .output_clear
                .write(OutputClear::default().clear_output(5))
        };
    }
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start() {
    const STACK_SIZE: usize = 1024; // 1 KiB

    #[unsafe(link_section = ".bss.uninit")]
    static mut STACK: [u8; STACK_SIZE] = [0u8; STACK_SIZE];

    naked_asm!(
        "   la      t0, sbss
            la      t1, ebss
        1:  bgeu    t0, t1, 2f
            sw      zero, 0(t0)
            addi    t0, t0, 4
            j       1b",
        "2:",
        "   la sp, {stack} + {stack_size}",
        "   j  {main}",
        stack_size = const STACK_SIZE,
        stack      =   sym STACK,
        main       =   sym pbp_main,
    )
}

#[repr(C)]
pub struct PbpHeader {
    magic: u32,
    checksum: u32,
}

#[unsafe(link_section = ".head.pbp")]
#[used]
pub static PBP_HEADER: PbpHeader = PbpHeader {
    magic: 0x20504250,
    checksum: 0x0,
};
