#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FSM_PUL_CNTR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED25R {
    bits: u8,
}
impl RESERVED25R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CUR_EC_LEVELR {
    bits: u16,
}
impl CUR_EC_LEVELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED12R {
    bits: u8,
}
impl RESERVED12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PUL_CNTRR {
    bits: u16,
}
impl PUL_CNTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 25:31 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved25(&self) -> RESERVED25R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED25R { bits }
    }
    #[doc = "Bits 16:24 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn cur_ec_level(&self) -> CUR_EC_LEVELR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CUR_EC_LEVELR { bits }
    }
    #[doc = "Bits 12:15 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved12(&self) -> RESERVED12R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED12R { bits }
    }
    #[doc = "Bits 0:11 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn pul_cntr(&self) -> PUL_CNTRR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PUL_CNTRR { bits }
    }
}
