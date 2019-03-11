#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CAP_TRIM {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct FLUX_CAP_0P28_TRIMR {
    bits: u16,
}
impl FLUX_CAP_0P28_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FLUX_CAP_0P4_TRIMR {
    bits: u16,
}
impl FLUX_CAP_0P4_TRIMR {
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
    #[doc = "Bits 16:31 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn flux_cap_0p28_trim(&self) -> FLUX_CAP_0P28_TRIMR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FLUX_CAP_0P28_TRIMR { bits }
    }
    #[doc = "Bits 0:15 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn flux_cap_0p4_trim(&self) -> FLUX_CAP_0P4_TRIMR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FLUX_CAP_0P4_TRIMR { bits }
    }
}
