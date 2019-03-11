#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FSM_ADDR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED31R {
    bits: bool,
}
impl RESERVED31R {
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
pub struct BANKR {
    bits: u8,
}
impl BANKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CUR_ADDRR {
    bits: u32,
}
impl CUR_ADDRR {
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
    #[doc = "Bit 31 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved31(&self) -> RESERVED31R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED31R { bits }
    }
    #[doc = "Bits 28:30 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn bank(&self) -> BANKR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BANKR { bits }
    }
    #[doc = "Bits 0:27 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn cur_addr(&self) -> CUR_ADDRR {
        let bits = {
            const MASK: u32 = 268435455;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        CUR_ADDRR { bits }
    }
}
