#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MAC_15_4_1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct ADDR_32_63R {
    bits: u32,
}
impl ADDR_32_63R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - The last 32-bits of the 64-bit MAC 15.4 address"]
    #[inline]
    pub fn addr_32_63(&self) -> ADDR_32_63R {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        ADDR_32_63R { bits }
    }
}
