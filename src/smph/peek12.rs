#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PEEK12 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED1R {
    bits: u32,
}
impl RESERVED1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct STATR {
    bits: bool,
}
impl STATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 1:31 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved1(&self) -> RESERVED1R {
        let bits = {
            const MASK: u32 = 2147483647;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED1R { bits }
    }
    #[doc = "Bit 0 - Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[inline]
    pub fn stat(&self) -> STATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STATR { bits }
    }
}
