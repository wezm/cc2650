#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::EFUSEPINS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED16R {
    bits: u16,
}
impl RESERVED16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EFC_SELF_TEST_DONER {
    bits: bool,
}
impl EFC_SELF_TEST_DONER {
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
pub struct EFC_SELF_TEST_ERRORR {
    bits: bool,
}
impl EFC_SELF_TEST_ERRORR {
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
pub struct SYS_ECC_SELF_TEST_ENR {
    bits: bool,
}
impl SYS_ECC_SELF_TEST_ENR {
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
pub struct EFC_INSTRUCTION_INFOR {
    bits: bool,
}
impl EFC_INSTRUCTION_INFOR {
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
pub struct EFC_INSTRUCTION_ERRORR {
    bits: bool,
}
impl EFC_INSTRUCTION_ERRORR {
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
pub struct EFC_AUTOLOAD_ERRORR {
    bits: bool,
}
impl EFC_AUTOLOAD_ERRORR {
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
pub struct SYS_ECC_OVERRIDE_ENR {
    bits: bool,
}
impl SYS_ECC_OVERRIDE_ENR {
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
pub struct EFC_READYR {
    bits: bool,
}
impl EFC_READYR {
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
pub struct EFC_FCLRZR {
    bits: bool,
}
impl EFC_FCLRZR {
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
pub struct SYS_DIEID_AUTOLOAD_ENR {
    bits: bool,
}
impl SYS_DIEID_AUTOLOAD_ENR {
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
pub struct SYS_REPAIR_ENR {
    bits: u8,
}
impl SYS_REPAIR_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_WS_READ_STATESR {
    bits: u8,
}
impl SYS_WS_READ_STATESR {
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
    #[doc = "Bits 16:31 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved16(&self) -> RESERVED16R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED16R { bits }
    }
    #[doc = "Bit 15 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efc_self_test_done(&self) -> EFC_SELF_TEST_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EFC_SELF_TEST_DONER { bits }
    }
    #[doc = "Bit 14 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efc_self_test_error(&self) -> EFC_SELF_TEST_ERRORR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EFC_SELF_TEST_ERRORR { bits }
    }
    #[doc = "Bit 13 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sys_ecc_self_test_en(&self) -> SYS_ECC_SELF_TEST_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_ECC_SELF_TEST_ENR { bits }
    }
    #[doc = "Bit 12 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efc_instruction_info(&self) -> EFC_INSTRUCTION_INFOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EFC_INSTRUCTION_INFOR { bits }
    }
    #[doc = "Bit 11 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efc_instruction_error(&self) -> EFC_INSTRUCTION_ERRORR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EFC_INSTRUCTION_ERRORR { bits }
    }
    #[doc = "Bit 10 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efc_autoload_error(&self) -> EFC_AUTOLOAD_ERRORR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EFC_AUTOLOAD_ERRORR { bits }
    }
    #[doc = "Bit 9 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sys_ecc_override_en(&self) -> SYS_ECC_OVERRIDE_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_ECC_OVERRIDE_ENR { bits }
    }
    #[doc = "Bit 8 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efc_ready(&self) -> EFC_READYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EFC_READYR { bits }
    }
    #[doc = "Bit 7 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efc_fclrz(&self) -> EFC_FCLRZR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EFC_FCLRZR { bits }
    }
    #[doc = "Bit 6 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sys_dieid_autoload_en(&self) -> SYS_DIEID_AUTOLOAD_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_DIEID_AUTOLOAD_ENR { bits }
    }
    #[doc = "Bits 4:5 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sys_repair_en(&self) -> SYS_REPAIR_ENR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_REPAIR_ENR { bits }
    }
    #[doc = "Bits 0:3 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sys_ws_read_states(&self) -> SYS_WS_READ_STATESR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_WS_READ_STATESR { bits }
    }
}
