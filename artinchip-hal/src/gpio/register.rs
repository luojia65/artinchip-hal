//! GPIO register blocks and registers.

use volatile_register::{RO, RW, WO};

/// General Purpose Input Output register block.
#[repr(C)]
pub struct RegisterBlock {
    /// GPIO group PA, PB until PO.
    pub groups_0: [GpioGroup; 15],
    _reserved0: [u8; 252],
    /// GPIO version register.
    #[doc(alias = "VERSION")]
    pub version: RO<u32>,
    /// GPIO group PQ and PR.
    pub groups_1: [GpioGroup; 2],
}

/// GPIO group register block.
#[repr(C)]
pub struct GpioGroup {
    /// Input state register.
    #[doc(alias = "GEN_IN_STA")]
    pub input_state: RO<u32>,
    /// Output cofiguration register.
    #[doc(alias = "GEN_OUT_CFG")]
    pub output_config: RW<u32>,
    /// Interrupt enable register.
    #[doc(alias = "GEN_IRQ_EN")]
    pub interrupt_enable: RW<u32>,
    /// Interrupt state register.
    #[doc(alias = "GEN_IRQ_STA")]
    pub interrupt_state: RW<u32>,
    /// Clear output register.
    #[doc(alias = "GEN_OUT_CLR")]
    pub output_clear: WO<u32>,
    /// Set output register.
    #[doc(alias = "GEN_OUT_SET")]
    pub output_set: WO<u32>,
    /// Toggle output register.
    #[doc(alias = "GEN_OUT_TOG")]
    pub output_toggle: WO<u32>,
    _reserved0: [u8; 100],
    /// Pin configuration register.
    #[doc(alias = "PIN_CFG")]
    pub pin_config: [RW<u32>; 32],
}

// TODO: GPIO register structures.

#[cfg(test)]
mod tests {
    use crate::gpio::register::GpioGroup;
    use core::mem::{offset_of, size_of};

    use super::RegisterBlock;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, groups_0), 0x0);
        assert_eq!(offset_of!(RegisterBlock, version), 0xffc);
        assert_eq!(offset_of!(RegisterBlock, groups_1), 0x1000);
    }

    #[test]
    fn struct_gpio_group_offset() {
        assert_eq!(size_of::<GpioGroup>(), 0x100);
        assert_eq!(offset_of!(GpioGroup, input_state), 0x0);
        assert_eq!(offset_of!(GpioGroup, output_config), 0x4);
        assert_eq!(offset_of!(GpioGroup, interrupt_enable), 0x8);
        assert_eq!(offset_of!(GpioGroup, interrupt_state), 0xc);
        assert_eq!(offset_of!(GpioGroup, output_clear), 0x10);
        assert_eq!(offset_of!(GpioGroup, output_set), 0x14);
        assert_eq!(offset_of!(GpioGroup, output_toggle), 0x18);
        assert_eq!(offset_of!(GpioGroup, pin_config), 0x80);
    }

    // TODO: GPIO register structure tests.
}
