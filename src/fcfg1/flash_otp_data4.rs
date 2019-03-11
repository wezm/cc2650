#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FLASH_OTP_DATA4 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct STANDBY_MODE_SEL_INT_WRTR {
    bits: bool,
}
impl STANDBY_MODE_SEL_INT_WRTR {
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
pub struct STANDBY_PW_SEL_INT_WRTR {
    bits: u8,
}
impl STANDBY_PW_SEL_INT_WRTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIS_STANDBY_INT_WRTR {
    bits: bool,
}
impl DIS_STANDBY_INT_WRTR {
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
pub struct DIS_IDLE_INT_WRTR {
    bits: bool,
}
impl DIS_IDLE_INT_WRTR {
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
pub struct VIN_AT_X_INT_WRTR {
    bits: u8,
}
impl VIN_AT_X_INT_WRTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct STANDBY_MODE_SEL_EXT_WRTR {
    bits: bool,
}
impl STANDBY_MODE_SEL_EXT_WRTR {
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
pub struct STANDBY_PW_SEL_EXT_WRTR {
    bits: u8,
}
impl STANDBY_PW_SEL_EXT_WRTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIS_STANDBY_EXT_WRTR {
    bits: bool,
}
impl DIS_STANDBY_EXT_WRTR {
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
pub struct DIS_IDLE_EXT_WRTR {
    bits: bool,
}
impl DIS_IDLE_EXT_WRTR {
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
pub struct VIN_AT_X_EXT_WRTR {
    bits: u8,
}
impl VIN_AT_X_EXT_WRTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct STANDBY_MODE_SEL_INT_RDR {
    bits: bool,
}
impl STANDBY_MODE_SEL_INT_RDR {
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
pub struct STANDBY_PW_SEL_INT_RDR {
    bits: u8,
}
impl STANDBY_PW_SEL_INT_RDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIS_STANDBY_INT_RDR {
    bits: bool,
}
impl DIS_STANDBY_INT_RDR {
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
pub struct DIS_IDLE_INT_RDR {
    bits: bool,
}
impl DIS_IDLE_INT_RDR {
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
pub struct VIN_AT_X_INT_RDR {
    bits: u8,
}
impl VIN_AT_X_INT_RDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct STANDBY_MODE_SEL_EXT_RDR {
    bits: bool,
}
impl STANDBY_MODE_SEL_EXT_RDR {
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
pub struct STANDBY_PW_SEL_EXT_RDR {
    bits: u8,
}
impl STANDBY_PW_SEL_EXT_RDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIS_STANDBY_EXT_RDR {
    bits: bool,
}
impl DIS_STANDBY_EXT_RDR {
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
pub struct DIS_IDLE_EXT_RDR {
    bits: bool,
}
impl DIS_IDLE_EXT_RDR {
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
pub struct VIN_AT_X_EXT_RDR {
    bits: u8,
}
impl VIN_AT_X_EXT_RDR {
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
    #[doc = "Bit 31 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn standby_mode_sel_int_wrt(&self) -> STANDBY_MODE_SEL_INT_WRTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STANDBY_MODE_SEL_INT_WRTR { bits }
    }
    #[doc = "Bits 29:30 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn standby_pw_sel_int_wrt(&self) -> STANDBY_PW_SEL_INT_WRTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STANDBY_PW_SEL_INT_WRTR { bits }
    }
    #[doc = "Bit 28 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dis_standby_int_wrt(&self) -> DIS_STANDBY_INT_WRTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIS_STANDBY_INT_WRTR { bits }
    }
    #[doc = "Bit 27 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dis_idle_int_wrt(&self) -> DIS_IDLE_INT_WRTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIS_IDLE_INT_WRTR { bits }
    }
    #[doc = "Bits 24:26 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vin_at_x_int_wrt(&self) -> VIN_AT_X_INT_WRTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VIN_AT_X_INT_WRTR { bits }
    }
    #[doc = "Bit 23 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn standby_mode_sel_ext_wrt(&self) -> STANDBY_MODE_SEL_EXT_WRTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STANDBY_MODE_SEL_EXT_WRTR { bits }
    }
    #[doc = "Bits 21:22 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn standby_pw_sel_ext_wrt(&self) -> STANDBY_PW_SEL_EXT_WRTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STANDBY_PW_SEL_EXT_WRTR { bits }
    }
    #[doc = "Bit 20 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dis_standby_ext_wrt(&self) -> DIS_STANDBY_EXT_WRTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIS_STANDBY_EXT_WRTR { bits }
    }
    #[doc = "Bit 19 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dis_idle_ext_wrt(&self) -> DIS_IDLE_EXT_WRTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIS_IDLE_EXT_WRTR { bits }
    }
    #[doc = "Bits 16:18 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vin_at_x_ext_wrt(&self) -> VIN_AT_X_EXT_WRTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VIN_AT_X_EXT_WRTR { bits }
    }
    #[doc = "Bit 15 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn standby_mode_sel_int_rd(&self) -> STANDBY_MODE_SEL_INT_RDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STANDBY_MODE_SEL_INT_RDR { bits }
    }
    #[doc = "Bits 13:14 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn standby_pw_sel_int_rd(&self) -> STANDBY_PW_SEL_INT_RDR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STANDBY_PW_SEL_INT_RDR { bits }
    }
    #[doc = "Bit 12 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dis_standby_int_rd(&self) -> DIS_STANDBY_INT_RDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIS_STANDBY_INT_RDR { bits }
    }
    #[doc = "Bit 11 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dis_idle_int_rd(&self) -> DIS_IDLE_INT_RDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIS_IDLE_INT_RDR { bits }
    }
    #[doc = "Bits 8:10 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vin_at_x_int_rd(&self) -> VIN_AT_X_INT_RDR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VIN_AT_X_INT_RDR { bits }
    }
    #[doc = "Bit 7 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn standby_mode_sel_ext_rd(&self) -> STANDBY_MODE_SEL_EXT_RDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STANDBY_MODE_SEL_EXT_RDR { bits }
    }
    #[doc = "Bits 5:6 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn standby_pw_sel_ext_rd(&self) -> STANDBY_PW_SEL_EXT_RDR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STANDBY_PW_SEL_EXT_RDR { bits }
    }
    #[doc = "Bit 4 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dis_standby_ext_rd(&self) -> DIS_STANDBY_EXT_RDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIS_STANDBY_EXT_RDR { bits }
    }
    #[doc = "Bit 3 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dis_idle_ext_rd(&self) -> DIS_IDLE_EXT_RDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIS_IDLE_EXT_RDR { bits }
    }
    #[doc = "Bits 0:2 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vin_at_x_ext_rd(&self) -> VIN_AT_X_EXT_RDR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VIN_AT_X_EXT_RDR { bits }
    }
}
