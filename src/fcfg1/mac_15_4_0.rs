#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MAC_15_4_0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct ADDR_0_31R {
    bits: u32,
}
impl ADDR_0_31R {
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
    #[doc = "Bits 0:31 - The first 32-bits of the 64-bit MAC 15.4 address"]
    #[inline]
    pub fn addr_0_31(&self) -> ADDR_0_31R {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        ADDR_0_31R { bits }
    }
}
