#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::BL_CONFIG {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct BOOTLOADER_ENABLER {
    bits: u8,
}
impl BOOTLOADER_ENABLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BL_LEVELR {
    bits: bool,
}
impl BL_LEVELR {
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
pub struct BL_PIN_NUMBERR {
    bits: u8,
}
impl BL_PIN_NUMBERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BL_ENABLER {
    bits: u8,
}
impl BL_ENABLER {
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
    #[doc = "Bits 24:31 - Bootloader enable. Boot loader can be accessed if IMAGE_VALID_CONF.IMAGE_VALID is non-zero or BL_ENABLE is enabled (and conditions for boot loader backdoor are met). 0xC5: Boot loader is enabled. Any other value: Boot loader is disabled."]
    #[inline]
    pub fn bootloader_enable(&self) -> BOOTLOADER_ENABLER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BOOTLOADER_ENABLER { bits }
    }
    #[doc = "Bit 16 - Sets the active level of the selected DIO number BL_PIN_NUMBER if boot loader backdoor is enabled by the BL_ENABLE field. 0: Active low. 1: Active high."]
    #[inline]
    pub fn bl_level(&self) -> BL_LEVELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BL_LEVELR { bits }
    }
    #[doc = "Bits 8:15 - DIO number that is level checked if the boot loader backdoor is enabled by the BL_ENABLE field."]
    #[inline]
    pub fn bl_pin_number(&self) -> BL_PIN_NUMBERR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BL_PIN_NUMBERR { bits }
    }
    #[doc = "Bits 0:7 - Enables the boot loader backdoor. 0xC5: Boot loader backdoor is enabled. Any other value: Boot loader backdoor is disabled. NOTE! Boot loader must be enabled (see BOOTLOADER_ENABLE) if boot loader backdoor is enabled."]
    #[inline]
    pub fn bl_enable(&self) -> BL_ENABLER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BL_ENABLER { bits }
    }
}
