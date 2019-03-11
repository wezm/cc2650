#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RCOSC_HF_TEMPCOMP {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct FINE_RESISTORR {
    bits: u8,
}
impl FINE_RESISTORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTRIMR {
    bits: u8,
}
impl CTRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTRIMFRACT_QUADR {
    bits: u8,
}
impl CTRIMFRACT_QUADR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTRIMFRACT_SLOPER {
    bits: u8,
}
impl CTRIMFRACT_SLOPER {
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
    #[doc = "Bits 24:31 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn fine_resistor(&self) -> FINE_RESISTORR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FINE_RESISTORR { bits }
    }
    #[doc = "Bits 16:23 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ctrim(&self) -> CTRIMR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTRIMR { bits }
    }
    #[doc = "Bits 8:15 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ctrimfract_quad(&self) -> CTRIMFRACT_QUADR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTRIMFRACT_QUADR { bits }
    }
    #[doc = "Bits 0:7 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ctrimfract_slope(&self) -> CTRIMFRACT_SLOPER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTRIMFRACT_SLOPER { bits }
    }
}
