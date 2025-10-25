#![no_std]
#![no_main]

use artinchip_hal::{cmu, gpio};
use core::arch::naked_asm;
use panic_halt as _;

extern "C" fn pbp_main(_boot_param: u32, _priv_addr: *const (), _priv_len: u32) {
    let cmu = unsafe { &*(0x18020000 as *const cmu::RegisterBlock) };
    unsafe {
        // 1.2GHz / (24 + 1) = 48MHz
        cmu.clock_uart0
            .modify(|v| v.enable_module_clk().set_module_clk_div(24))
    };

    let gpio = unsafe { &*(0x18700000 as *const gpio::RegisterBlock) };
    let gpioa = &gpio.groups[0];
    unsafe {
        // pa0 = 0x325
        gpioa.pin_config[0].modify(|v| {
            v.set_pin_pull(gpio::PinPull::PullUp)
                .set_drive_strength(gpio::PinDriveStrength::Level2)
                .set_pin_func(5)
        });
        // pa1 = 0x325
        gpioa.pin_config[1].modify(|v| {
            v.set_pin_pull(gpio::PinPull::PullUp)
                .set_drive_strength(gpio::PinDriveStrength::Level2)
                .set_pin_func(5)
        });
    }

    let uart0 = unsafe { &*(0x18710000 as *const uart16550::Uart16550<u32>) };
    // 48MHz / 417 ≈ 115200 Bd * (1 - 0.08%)
    uart0.write_divisor(417);
    // 115200 8N1
    let lcr = uart0.lcr().read();
    uart0.lcr().write(
        lcr.set_char_len(uart16550::CharLen::EIGHT)
            .set_one_stop_bit(true)
            .set_parity(uart16550::PARITY::NONE),
    );
    uart0.write(b"Hello World from Rust Artinchip HAL!\n");
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
