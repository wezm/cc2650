#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FLASH_C_E_P_R {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RVSUR {
    bits: u8,
}
impl RVSUR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PV_ACCESSR {
    bits: u8,
}
impl PV_ACCESSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct A_EXEZ_SETUPR {
    bits: u8,
}
impl A_EXEZ_SETUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CVSUR {
    bits: u16,
}
impl CVSUR {
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
    #[doc = "Bits 24:31 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rvsu(&self) -> RVSUR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RVSUR { bits }
    }
    #[doc = "Bits 16:23 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn pv_access(&self) -> PV_ACCESSR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PV_ACCESSR { bits }
    }
    #[doc = "Bits 12:15 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn a_exez_setup(&self) -> A_EXEZ_SETUPR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        A_EXEZ_SETUPR { bits }
    }
    #[doc = "Bits 0:11 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn cvsu(&self) -> CVSUR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CVSUR { bits }
    }
}
