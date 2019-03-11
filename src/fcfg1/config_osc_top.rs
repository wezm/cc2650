#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CONFIG_OSC_TOP {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct XOSC_HF_ROW_Q12R {
    bits: u8,
}
impl XOSC_HF_ROW_Q12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct XOSC_HF_COLUMN_Q12R {
    bits: u16,
}
impl XOSC_HF_COLUMN_Q12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RCOSCLF_CTUNE_TRIMR {
    bits: u8,
}
impl RCOSCLF_CTUNE_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RCOSCLF_RTUNE_TRIMR {
    bits: u8,
}
impl RCOSCLF_RTUNE_TRIMR {
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
    #[doc = "Bits 26:29 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn xosc_hf_row_q12(&self) -> XOSC_HF_ROW_Q12R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XOSC_HF_ROW_Q12R { bits }
    }
    #[doc = "Bits 10:25 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn xosc_hf_column_q12(&self) -> XOSC_HF_COLUMN_Q12R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        XOSC_HF_COLUMN_Q12R { bits }
    }
    #[doc = "Bits 2:9 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcosclf_ctune_trim(&self) -> RCOSCLF_CTUNE_TRIMR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RCOSCLF_CTUNE_TRIMR { bits }
    }
    #[doc = "Bits 0:1 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcosclf_rtune_trim(&self) -> RCOSCLF_RTUNE_TRIMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RCOSCLF_RTUNE_TRIMR { bits }
    }
}
