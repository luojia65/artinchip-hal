//! Secure ID peripheral instance.
use super::register::RegisterBlock;
use core::marker::PhantomData;

pub struct Sid {
    reg: *const RegisterBlock,
    _private: PhantomData<()>,
}

impl Sid {
    /// Create a new SID instance.
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

impl Sid {
    /// Read efuse word at index `word_idx`.
    #[inline]
    pub fn efuse_read(&self, word_idx: usize) -> u32 {
        unsafe { &*self.reg }.buffer[word_idx].read()
    }
}
