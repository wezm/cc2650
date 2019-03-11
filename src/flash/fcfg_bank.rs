#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FCFG_BANK {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct EE_BANK_WIDTHR {
    bits: u16,
}
impl EE_BANK_WIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EE_NUM_BANKR {
    bits: u8,
}
impl EE_NUM_BANKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAIN_BANK_WIDTHR {
    bits: u16,
}
impl MAIN_BANK_WIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAIN_NUM_BANKR {
    bits: u8,
}
impl MAIN_NUM_BANKR {
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
    #[doc = "Bits 20:31 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ee_bank_width(&self) -> EE_BANK_WIDTHR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        EE_BANK_WIDTHR { bits }
    }
    #[doc = "Bits 16:19 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ee_num_bank(&self) -> EE_NUM_BANKR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EE_NUM_BANKR { bits }
    }
    #[doc = "Bits 4:15 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn main_bank_width(&self) -> MAIN_BANK_WIDTHR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MAIN_BANK_WIDTHR { bits }
    }
    #[doc = "Bits 0:3 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn main_num_bank(&self) -> MAIN_NUM_BANKR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAIN_NUM_BANKR { bits }
    }
}
