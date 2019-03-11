#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PWD_CURR_50C {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct DELTA_CACHE_REFR {
    bits: u8,
}
impl DELTA_CACHE_REFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DELTA_RFMEM_RETR {
    bits: u8,
}
impl DELTA_RFMEM_RETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DELTA_XOSC_LPMR {
    bits: u8,
}
impl DELTA_XOSC_LPMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BASELINER {
    bits: u8,
}
impl BASELINER {
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
    #[doc = "Bits 24:31 - Additional maximum current, in units of 1uA, with cache retention"]
    #[inline]
    pub fn delta_cache_ref(&self) -> DELTA_CACHE_REFR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DELTA_CACHE_REFR { bits }
    }
    #[doc = "Bits 16:23 - Additional maximum current, in 1uA units, with RF memory retention"]
    #[inline]
    pub fn delta_rfmem_ret(&self) -> DELTA_RFMEM_RETR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DELTA_RFMEM_RETR { bits }
    }
    #[doc = "Bits 8:15 - Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode"]
    #[inline]
    pub fn delta_xosc_lpm(&self) -> DELTA_XOSC_LPMR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DELTA_XOSC_LPMR { bits }
    }
    #[doc = "Bits 0:7 - Worst-case baseline maximum powerdown current, in units of 0.5uA"]
    #[inline]
    pub fn baseline(&self) -> BASELINER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BASELINER { bits }
    }
}
