#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SHDW_OSC_BIAS_LDO_TRIM {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct SET_RCOSC_HF_COARSE_RESISTORR {
    bits: u8,
}
impl SET_RCOSC_HF_COARSE_RESISTORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIMMAGR {
    bits: u8,
}
impl TRIMMAGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIMIREFR {
    bits: u8,
}
impl TRIMIREFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ITRIM_DIG_LDOR {
    bits: u8,
}
impl ITRIM_DIG_LDOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VTRIM_DIGR {
    bits: u8,
}
impl VTRIM_DIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VTRIM_COARSER {
    bits: u8,
}
impl VTRIM_COARSER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RCOSCHF_CTRIMR {
    bits: u8,
}
impl RCOSCHF_CTRIMR {
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
    #[doc = "Bits 27:28 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn set_rcosc_hf_coarse_resistor(&self) -> SET_RCOSC_HF_COARSE_RESISTORR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SET_RCOSC_HF_COARSE_RESISTORR { bits }
    }
    #[doc = "Bits 23:26 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn trimmag(&self) -> TRIMMAGR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIMMAGR { bits }
    }
    #[doc = "Bits 18:22 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn trimiref(&self) -> TRIMIREFR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIMIREFR { bits }
    }
    #[doc = "Bits 16:17 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn itrim_dig_ldo(&self) -> ITRIM_DIG_LDOR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ITRIM_DIG_LDOR { bits }
    }
    #[doc = "Bits 12:15 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vtrim_dig(&self) -> VTRIM_DIGR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VTRIM_DIGR { bits }
    }
    #[doc = "Bits 8:11 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vtrim_coarse(&self) -> VTRIM_COARSER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VTRIM_COARSER { bits }
    }
    #[doc = "Bits 0:7 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcoschf_ctrim(&self) -> RCOSCHF_CTRIMR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RCOSCHF_CTRIMR { bits }
    }
}
