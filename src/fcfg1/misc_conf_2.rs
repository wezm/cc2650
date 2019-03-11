#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MISC_CONF_2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct HPOSC_COMP_P3R {
    bits: u8,
}
impl HPOSC_COMP_P3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_comp_p3(&self) -> HPOSC_COMP_P3R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HPOSC_COMP_P3R { bits }
    }
}
