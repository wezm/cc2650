#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CONFIG_MISC_ADC {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RSSITRIMCOMPLETE_NR {
    bits: bool,
}
impl RSSITRIMCOMPLETE_NR {
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
    #[doc = "Bit 17 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rssitrimcomplete_n(&self) -> RSSITRIMCOMPLETE_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSSITRIMCOMPLETE_NR { bits }
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
