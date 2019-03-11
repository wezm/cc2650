#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CONFIG_RF_FRONTEND_DIV15 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct IFAMP_IBR {
    bits: u8,
}
impl IFAMP_IBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_IBR {
    bits: u8,
}
impl LNA_IBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IFAMP_TRIMR {
    bits: u8,
}
impl IFAMP_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTL_PA0_TRIMR {
    bits: u8,
}
impl CTL_PA0_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RFLDO_TRIM_OUTPUTR {
    bits: u8,
}
impl RFLDO_TRIM_OUTPUTR {
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
    pub fn ifamp_ib(&self) -> IFAMP_IBR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IFAMP_IBR { bits }
    }
    #[doc = "Bits 24:27 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn lna_ib(&self) -> LNA_IBR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNA_IBR { bits }
    }
    #[doc = "Bits 19:23 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ifamp_trim(&self) -> IFAMP_TRIMR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IFAMP_TRIMR { bits }
    }
    #[doc = "Bits 14:18 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ctl_pa0_trim(&self) -> CTL_PA0_TRIMR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTL_PA0_TRIMR { bits }
    }
    #[doc = "Bits 0:6 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rfldo_trim_output(&self) -> RFLDO_TRIM_OUTPUTR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RFLDO_TRIM_OUTPUTR { bits }
    }
}
