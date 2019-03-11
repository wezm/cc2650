#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::AMPCOMP_TH1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED1R {
    bits: u8,
}
impl RESERVED1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPMRAMP3_LTHR {
    bits: u8,
}
impl HPMRAMP3_LTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED0R {
    bits: u8,
}
impl RESERVED0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPMRAMP3_HTHR {
    bits: u8,
}
impl HPMRAMP3_HTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IBIASCAP_LPTOHP_OL_CNTR {
    bits: u8,
}
impl IBIASCAP_LPTOHP_OL_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPMRAMP1_THR {
    bits: u8,
}
impl HPMRAMP1_THR {
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
    pub fn reserved1(&self) -> RESERVED1R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED1R { bits }
    }
    #[doc = "Bits 18:23 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hpmramp3_lth(&self) -> HPMRAMP3_LTHR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HPMRAMP3_LTHR { bits }
    }
    #[doc = "Bits 16:17 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved0(&self) -> RESERVED0R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED0R { bits }
    }
    #[doc = "Bits 10:15 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hpmramp3_hth(&self) -> HPMRAMP3_HTHR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HPMRAMP3_HTHR { bits }
    }
    #[doc = "Bits 6:9 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ibiascap_lptohp_ol_cnt(&self) -> IBIASCAP_LPTOHP_OL_CNTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IBIASCAP_LPTOHP_OL_CNTR { bits }
    }
    #[doc = "Bits 0:5 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hpmramp1_th(&self) -> HPMRAMP1_THR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HPMRAMP1_THR { bits }
    }
}
