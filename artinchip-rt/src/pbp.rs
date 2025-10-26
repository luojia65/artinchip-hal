//! Pre-Boot Program runtime.
use core::arch::naked_asm;

/// Pre-Boot Program header structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct PbpHeader {
    /// Magic number, should be ASCII "PBP ".
    pub magic: u32,
    /// PBP checksum.
    pub checksum: u32,
}

/// Static-linked Pre-Boot Program header.
#[unsafe(link_section = ".head.pbp")]
#[used]
pub static PBP_HEADER: PbpHeader = PbpHeader {
    magic: 0x20504250,
    checksum: 0x0, // <- Real checksum filled by PBP tools.
};

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
        "   la      sp, {stack} + {stack_size}",
        "   j       {main}",
        stack_size = const STACK_SIZE,
        stack      =   sym STACK,
        main       =   sym pbp_main,
    )
}

unsafe extern "C" {
    fn pbp_main(boot_param: u32, priv_addr: *const (), priv_len: u32);
}
