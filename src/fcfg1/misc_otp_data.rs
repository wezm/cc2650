#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MISC_OTP_DATA {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RCOSC_HF_ITUNER {
    bits: u8,
}
impl RCOSC_HF_ITUNER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RCOSC_HF_CRIMR {
    bits: u8,
}
impl RCOSC_HF_CRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PER_MR {
    bits: u8,
}
impl PER_MR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PER_ER {
    bits: u8,
}
impl PER_ER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PO_TAIL_RES_TRIMR {
    bits: u8,
}
impl PO_TAIL_RES_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TEST_PROGRAM_REVR {
    bits: u8,
}
impl TEST_PROGRAM_REVR {
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
    #[doc = "Bits 28:31 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcosc_hf_itune(&self) -> RCOSC_HF_ITUNER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RCOSC_HF_ITUNER { bits }
    }
    #[doc = "Bits 20:27 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcosc_hf_crim(&self) -> RCOSC_HF_CRIMR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RCOSC_HF_CRIMR { bits }
    }
    #[doc = "Bits 15:19 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn per_m(&self) -> PER_MR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PER_MR { bits }
    }
    #[doc = "Bits 12:14 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn per_e(&self) -> PER_ER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PER_ER { bits }
    }
    #[doc = "Bits 8:11 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn po_tail_res_trim(&self) -> PO_TAIL_RES_TRIMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PO_TAIL_RES_TRIMR { bits }
    }
    #[doc = "Bits 0:7 - The revision of the test program used in the production process when FCFG1 was programmed. Value migth change without warning."]
    #[inline]
    pub fn test_program_rev(&self) -> TEST_PROGRAM_REVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TEST_PROGRAM_REVR { bits }
    }
}
