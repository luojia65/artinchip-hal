//! Artinchip D13x chip series.

use core::marker::PhantomData;

/// Artinchip D13x peripheral ownership structures available on BootROM.
pub struct Peripherals {
    /// General Purpose Input/Output peripheral group A.
    pub gpioa: GpioAPads,
    // TODO all other peripherals.
}

impl Peripherals {
    #[doc(hidden)]
    #[inline]
    pub const fn __new() -> Self {
        Self {
            gpioa: GpioAPads::__new(),
        }
    }

    /// Take initialized peripherals.
    ///
    /// TODO ensure must called once.
    #[inline]
    pub const fn take() -> Self {
        Self::__new()
    }
}

// TODO macro defined GPIOA, etc.

/// General Purpose Input/Output peripheral group A.
pub struct GPIOA {
    _private: (),
}

impl GPIOA {
    #[inline]
    pub const fn ptr() -> *const artinchip_hal::gpio::GpioGroup {
        &unsafe { &*(0x18700000 as *const artinchip_hal::gpio::RegisterBlock) }.groups[0]
            as *const _
    }
}

impl core::ops::Deref for GPIOA {
    type Target = artinchip_hal::gpio::GpioGroup;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &unsafe { &*(0x18700000 as *const artinchip_hal::gpio::RegisterBlock) }.groups[0]
    }
}

// TODO GPIOB, GPIOC, GPIOD, GPIOE.

/// GPIOA pads.
pub struct GpioAPads {
    /// GPIO pad PA0.
    pub pa0: GpioPad<GPIOA, 0>,
    /// GPIO pad PA1.
    pub pa1: GpioPad<GPIOA, 1>,
    /// GPIO pad PA2.
    pub pa2: GpioPad<GPIOA, 2>,
    /// GPIO pad PA3.
    pub pa3: GpioPad<GPIOA, 3>,
    /// GPIO pad PA4.
    pub pa4: GpioPad<GPIOA, 4>,
    /// GPIO pad PA5.
    pub pa5: GpioPad<GPIOA, 5>,
    // TODO PA6, .., PA31.
}

impl GpioAPads {
    #[inline]
    const fn __new() -> Self {
        Self {
            pa0: GpioPad {
                _private: PhantomData,
            },
            pa1: GpioPad {
                _private: PhantomData,
            },
            pa2: GpioPad {
                _private: PhantomData,
            },
            pa3: GpioPad {
                _private: PhantomData,
            },
            pa4: GpioPad {
                _private: PhantomData,
            },
            pa5: GpioPad {
                _private: PhantomData,
            },
            // TODO PA6, .., PA31.
        }
    }
}

/// GPIO pad with statically known GPIO number.
pub struct GpioPad<T, const N: u8> {
    _private: PhantomData<T>,
}

// TODO macro defined GpioPad<GPIOx>.

impl<'a, const N: u8> artinchip_hal::gpio::PadExt<'a> for &'a mut GpioPad<GPIOA, N> {
    #[inline]
    fn into_output(self) -> artinchip_hal::gpio::Output<'a> {
        unsafe { artinchip_hal::gpio::Output::__new(N, &*GPIOA::ptr()) }
    }
}

impl<const N: u8> artinchip_hal::gpio::PadExt<'static> for GpioPad<GPIOA, N> {
    #[inline]
    fn into_output(self) -> artinchip_hal::gpio::Output<'static> {
        unsafe { artinchip_hal::gpio::Output::__new(N, &*GPIOA::ptr()) }
    }
}

// TODO other ownership modules.
