#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MISC_OTP_DATA_1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct PEAK_DET_ITRIMR {
    bits: u8,
}
impl PEAK_DET_ITRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HP_BUF_ITRIMR {
    bits: u8,
}
impl HP_BUF_ITRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LP_BUF_ITRIMR {
    bits: u8,
}
impl LP_BUF_ITRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DBLR_LOOP_FILTER_RESET_VOLTAGER {
    bits: u8,
}
impl DBLR_LOOP_FILTER_RESET_VOLTAGER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPM_IBIAS_WAIT_CNTR {
    bits: u16,
}
impl HPM_IBIAS_WAIT_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LPM_IBIAS_WAIT_CNTR {
    bits: u8,
}
impl LPM_IBIAS_WAIT_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IDAC_STEPR {
    bits: u8,
}
impl IDAC_STEPR {
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
    pub fn peak_det_itrim(&self) -> PEAK_DET_ITRIMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PEAK_DET_ITRIMR { bits }
    }
    #[doc = "Bits 24:26 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hp_buf_itrim(&self) -> HP_BUF_ITRIMR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HP_BUF_ITRIMR { bits }
    }
    #[doc = "Bits 22:23 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn lp_buf_itrim(&self) -> LP_BUF_ITRIMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LP_BUF_ITRIMR { bits }
    }
    #[doc = "Bits 20:21 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dblr_loop_filter_reset_voltage(&self) -> DBLR_LOOP_FILTER_RESET_VOLTAGER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DBLR_LOOP_FILTER_RESET_VOLTAGER { bits }
    }
    #[doc = "Bits 10:19 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hpm_ibias_wait_cnt(&self) -> HPM_IBIAS_WAIT_CNTR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        HPM_IBIAS_WAIT_CNTR { bits }
    }
    #[doc = "Bits 4:9 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn lpm_ibias_wait_cnt(&self) -> LPM_IBIAS_WAIT_CNTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPM_IBIAS_WAIT_CNTR { bits }
    }
    #[doc = "Bits 0:3 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn idac_step(&self) -> IDAC_STEPR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IDAC_STEPR { bits }
    }
}
