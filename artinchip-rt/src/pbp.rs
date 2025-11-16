//! Pre-Boot Program runtime.
use core::arch::naked_asm;

/// Pre-Boot Program header structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct PbpHeader {
    /// Magic number, should be ASCII "PBP ".
    pub magic: [u8; 4],
    /// PBP checksum.
    pub checksum: u32,
}

/// Static-linked Pre-Boot Program header.
#[unsafe(link_section = ".head.pbp")]
#[used]
pub static PBP_HEADER: PbpHeader = PbpHeader {
    magic: *b"PBP ",
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
        // 1. Disable interrupt
        "   csrw    mie, zero",
        // 2. Hart specific initialization
        // Enable T-Head instruction sets (THEADISAEE) and
        // misaligned access (MM) in `mxstatus` register.
        "   li      t1, 0x408000
            csrs    0x7c0, t1",
        // TODO SPUSHEN and SPSWAPEN in `mexstatus` once we have trap handler
        // Enable T-Head caches (BTB=1, BPE=1, RS=1, WA=1, WB=1, DE=1, IE=1)
        // in `mhcr` register;
        // enable T-Head hint operations (PREF_N=3, AMR=1, D_PLD=1)
        // in `mhint` register.
        "   li      t0, 0x103f
            csrw    0x7c1, t0
            li      t1, 0x600c
            csrw    0x7c5, t1",
        // 3. Initialize float point unit
        "   li      t0, 0x4000
            li      t1, 0x2000
            csrc    mstatus, t0
            csrs    mstatus, t1
            csrw    fcsr, zero",
        // 4. Clear `.bss` section
        "   la      t0, sbss
            la      t1, ebss
        2:  bgeu    t0, t1, 3f
            sw      zero, 0(t0)
            addi    t0, t0, 4
            j       2b",
        "3:",
        // 5. Prepare programming language stack
        "   la      sp, {stack} + {stack_size}",
        // 6. Start Rust main function
        "   j       {main}",
        // 7. Platform halt (by loop-wfi) if main function returns
        // Set T-Head wfi behavior to deep-sleep, disable interrupt then
        // loop-wfi. Clears LPMD=0 and WFEEN=0 in `mexstatus`.
        "   li      t0, 0x1c
            csrc    0x7e1, t0
            csrci   mstatus, 0x8
        2:  wfi
            j       2b",
        stack_size = const STACK_SIZE,
        stack      =   sym STACK,
        main       =   sym pbp_main,
    )
}

unsafe extern "C" {
    fn pbp_main(boot_param: u32, priv_addr: *const u8, priv_len: u32);
}
