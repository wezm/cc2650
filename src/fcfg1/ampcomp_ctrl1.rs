#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::AMPCOMP_CTRL1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED1R {
    bits: bool,
}
impl RESERVED1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct AMPCOMP_REQ_MODER {
    bits: bool,
}
impl AMPCOMP_REQ_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
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
pub struct IBIAS_OFFSETR {
    bits: u8,
}
impl IBIAS_OFFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IBIAS_INITR {
    bits: u8,
}
impl IBIAS_INITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LPM_IBIAS_WAIT_CNT_FINALR {
    bits: u8,
}
impl LPM_IBIAS_WAIT_CNT_FINALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CAP_STEPR {
    bits: u8,
}
impl CAP_STEPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IBIASCAP_HPTOLP_OL_CNTR {
    bits: u8,
}
impl IBIASCAP_HPTOLP_OL_CNTR {
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
    #[doc = "Bit 31 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved1(&self) -> RESERVED1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED1R { bits }
    }
    #[doc = "Bit 30 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ampcomp_req_mode(&self) -> AMPCOMP_REQ_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AMPCOMP_REQ_MODER { bits }
    }
    #[doc = "Bits 24:29 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved0(&self) -> RESERVED0R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED0R { bits }
    }
    #[doc = "Bits 20:23 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ibias_offset(&self) -> IBIAS_OFFSETR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IBIAS_OFFSETR { bits }
    }
    #[doc = "Bits 16:19 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ibias_init(&self) -> IBIAS_INITR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IBIAS_INITR { bits }
    }
    #[doc = "Bits 8:15 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn lpm_ibias_wait_cnt_final(&self) -> LPM_IBIAS_WAIT_CNT_FINALR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPM_IBIAS_WAIT_CNT_FINALR { bits }
    }
    #[doc = "Bits 4:7 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn cap_step(&self) -> CAP_STEPR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CAP_STEPR { bits }
    }
    #[doc = "Bits 0:3 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ibiascap_hptolp_ol_cnt(&self) -> IBIASCAP_HPTOLP_OL_CNTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IBIASCAP_HPTOLP_OL_CNTR { bits }
    }
}
