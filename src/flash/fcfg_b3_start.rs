#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FCFG_B3_START {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct B3_MAX_SECTORR {
    bits: u8,
}
impl B3_MAX_SECTORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct B3_MUX_FACTORR {
    bits: u8,
}
impl B3_MUX_FACTORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct B3_START_ADDRR {
    bits: u32,
}
impl B3_START_ADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
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
    pub fn b3_max_sector(&self) -> B3_MAX_SECTORR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        B3_MAX_SECTORR { bits }
    }
    #[doc = "Bits 24:27 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b3_mux_factor(&self) -> B3_MUX_FACTORR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        B3_MUX_FACTORR { bits }
    }
    #[doc = "Bits 0:23 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b3_start_addr(&self) -> B3_START_ADDRR {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        B3_START_ADDRR { bits }
    }
}
