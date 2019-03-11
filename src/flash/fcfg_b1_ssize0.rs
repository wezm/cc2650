#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FCFG_B1_SSIZE0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct B1_SECT_SIZER {
    bits: u32,
}
impl B1_SECT_SIZER {
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
    #[doc = "Bits 0:31 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b1_sect_size(&self) -> B1_SECT_SIZER {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        B1_SECT_SIZER { bits }
    }
}
