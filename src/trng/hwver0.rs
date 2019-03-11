#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HWVER0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED28R {
    bits: u8,
}
impl RESERVED28R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HW_MAJOR_VERR {
    bits: u8,
}
impl HW_MAJOR_VERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HW_MINOR_VERR {
    bits: u8,
}
impl HW_MINOR_VERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HW_PATCH_LVLR {
    bits: u8,
}
impl HW_PATCH_LVLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EIP_NUM_COMPLR {
    bits: u8,
}
impl EIP_NUM_COMPLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EIP_NUMR {
    bits: u8,
}
impl EIP_NUMR {
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
    #[doc = "Bits 28:31 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved28(&self) -> RESERVED28R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED28R { bits }
    }
    #[doc = "Bits 24:27 - 4 bits binary encoding of the major hardware revision number."]
    #[inline]
    pub fn hw_major_ver(&self) -> HW_MAJOR_VERR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HW_MAJOR_VERR { bits }
    }
    #[doc = "Bits 20:23 - 4 bits binary encoding of the minor hardware revision number."]
    #[inline]
    pub fn hw_minor_ver(&self) -> HW_MINOR_VERR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HW_MINOR_VERR { bits }
    }
    #[doc = "Bits 16:19 - 4 bits binary encoding of the hardware patch level, initial release will carry value zero."]
    #[inline]
    pub fn hw_patch_lvl(&self) -> HW_PATCH_LVLR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HW_PATCH_LVLR { bits }
    }
    #[doc = "Bits 8:15 - Bit-by-bit logic complement of bits \\[7:0\\]. This TRNG gives 0xB4."]
    #[inline]
    pub fn eip_num_compl(&self) -> EIP_NUM_COMPLR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EIP_NUM_COMPLR { bits }
    }
    #[doc = "Bits 0:7 - 8 bits binary encoding of the module number. This TRNG gives 0x4B."]
    #[inline]
    pub fn eip_num(&self) -> EIP_NUMR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EIP_NUMR { bits }
    }
}
