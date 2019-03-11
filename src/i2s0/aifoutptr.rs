#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::AIFOUTPTR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct PTRR {
    bits: u32,
}
impl PTRR {
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
    #[doc = "Bits 0:31 - Value of the DMA output buffer pointer currently used by the DMA controller Incremented by 1 (byte) or 2 (word) for each AHB access."]
    #[inline]
    pub fn ptr(&self) -> PTRR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        PTRR { bits }
    }
}
