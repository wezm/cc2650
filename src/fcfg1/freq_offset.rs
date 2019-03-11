#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FREQ_OFFSET {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct HPOSC_COMP_P0R {
    bits: u16,
}
impl HPOSC_COMP_P0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPOSC_COMP_P1R {
    bits: u8,
}
impl HPOSC_COMP_P1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPOSC_COMP_P2R {
    bits: u8,
}
impl HPOSC_COMP_P2R {
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
    #[doc = "Bits 16:31 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_comp_p0(&self) -> HPOSC_COMP_P0R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        HPOSC_COMP_P0R { bits }
    }
    #[doc = "Bits 8:15 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_comp_p1(&self) -> HPOSC_COMP_P1R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HPOSC_COMP_P1R { bits }
    }
    #[doc = "Bits 0:7 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_comp_p2(&self) -> HPOSC_COMP_P2R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HPOSC_COMP_P2R { bits }
    }
}
