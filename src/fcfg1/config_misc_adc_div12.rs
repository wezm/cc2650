#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CONFIG_MISC_ADC_DIV12 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RSSI_OFFSETR {
    bits: u8,
}
impl RSSI_OFFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct QUANTCTLTHRESR {
    bits: u8,
}
impl QUANTCTLTHRESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DACTRIMR {
    bits: u8,
}
impl DACTRIMR {
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
    #[doc = "Bits 9:16 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rssi_offset(&self) -> RSSI_OFFSETR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSSI_OFFSETR { bits }
    }
    #[doc = "Bits 6:8 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn quantctlthres(&self) -> QUANTCTLTHRESR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        QUANTCTLTHRESR { bits }
    }
    #[doc = "Bits 0:5 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dactrim(&self) -> DACTRIMR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DACTRIMR { bits }
    }
}
