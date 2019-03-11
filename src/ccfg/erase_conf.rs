#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ERASE_CONF {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED2R {
    bits: u32,
}
impl RESERVED2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CHIP_ERASE_DIS_NR {
    bits: bool,
}
impl CHIP_ERASE_DIS_NR {
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
pub struct RESERVED1R {
    bits: u8,
}
impl RESERVED1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BANK_ERASE_DIS_NR {
    bits: bool,
}
impl BANK_ERASE_DIS_NR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 9:31 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        let bits = {
            const MASK: u32 = 8388607;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED2R { bits }
    }
    #[doc = "Bit 8 - Chip erase. This bit controls if a chip erase requested through the JTAG WUC TAP will be ignored in a following boot caused by a reset of the MCU VD. A successful chip erase operation will force the content of the flash main bank back to the state as it was when delivered by TI. 0: Disable. Any chip erase request detected during boot will be ignored. 1: Enable. Any chip erase request detected during boot will be performed by the boot FW."]
    #[inline]
    pub fn chip_erase_dis_n(&self) -> CHIP_ERASE_DIS_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHIP_ERASE_DIS_NR { bits }
    }
    #[doc = "Bits 1:7 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved1(&self) -> RESERVED1R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED1R { bits }
    }
    #[doc = "Bit 0 - Bank erase. This bit controls if the ROM serial boot loader will accept a received Bank Erase command (COMMAND_BANK_ERASE). A successful Bank Erase operation will erase all main bank sectors not protected by write protect configuration bits in CCFG. 0: Disable the boot loader bank erase function. 1: Enable the boot loader bank erase function."]
    #[inline]
    pub fn bank_erase_dis_n(&self) -> BANK_ERASE_DIS_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BANK_ERASE_DIS_NR { bits }
    }
}
