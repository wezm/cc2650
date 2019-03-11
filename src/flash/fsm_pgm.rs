#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FSM_PGM {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED26R {
    bits: u8,
}
impl RESERVED26R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PGM_BANKR {
    bits: u8,
}
impl PGM_BANKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PGM_ADDRR {
    bits: u32,
}
impl PGM_ADDRR {
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
    #[doc = "Bits 26:31 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved26(&self) -> RESERVED26R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED26R { bits }
    }
    #[doc = "Bits 23:25 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn pgm_bank(&self) -> PGM_BANKR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PGM_BANKR { bits }
    }
    #[doc = "Bits 0:22 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn pgm_addr(&self) -> PGM_ADDRR {
        let bits = {
            const MASK: u32 = 8388607;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        PGM_ADDRR { bits }
    }
}
