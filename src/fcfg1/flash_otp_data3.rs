#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FLASH_OTP_DATA3 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct EC_STEP_SIZER {
    bits: u16,
}
impl EC_STEP_SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DO_PRECONDR {
    bits: bool,
}
impl DO_PRECONDR {
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
pub struct MAX_EC_LEVELR {
    bits: u8,
}
impl MAX_EC_LEVELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIM_1P7R {
    bits: u8,
}
impl TRIM_1P7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FLASH_SIZER {
    bits: u8,
}
impl FLASH_SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WAIT_SYSCODER {
    bits: u8,
}
impl WAIT_SYSCODER {
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
    #[doc = "Bits 23:31 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ec_step_size(&self) -> EC_STEP_SIZER {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        EC_STEP_SIZER { bits }
    }
    #[doc = "Bit 22 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn do_precond(&self) -> DO_PRECONDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DO_PRECONDR { bits }
    }
    #[doc = "Bits 18:21 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn max_ec_level(&self) -> MAX_EC_LEVELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAX_EC_LEVELR { bits }
    }
    #[doc = "Bits 16:17 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn trim_1p7(&self) -> TRIM_1P7R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIM_1P7R { bits }
    }
    #[doc = "Bits 8:15 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn flash_size(&self) -> FLASH_SIZER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FLASH_SIZER { bits }
    }
    #[doc = "Bits 0:7 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn wait_syscode(&self) -> WAIT_SYSCODER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WAIT_SYSCODER { bits }
    }
}
