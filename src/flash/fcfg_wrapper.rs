#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FCFG_WRAPPER {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct FAMILY_TYPER {
    bits: u8,
}
impl FAMILY_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED21R {
    bits: u8,
}
impl RESERVED21R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MEM_MAPR {
    bits: bool,
}
impl MEM_MAPR {
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
pub struct CPU2R {
    bits: u8,
}
impl CPU2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EE_IN_MAINR {
    bits: u8,
}
impl EE_IN_MAINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ROMR {
    bits: bool,
}
impl ROMR {
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
pub struct IFLUSHR {
    bits: bool,
}
impl IFLUSHR {
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
pub struct SIL3R {
    bits: bool,
}
impl SIL3R {
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
pub struct ECCAR {
    bits: bool,
}
impl ECCAR {
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
pub struct AUTO_SUSPR {
    bits: u8,
}
impl AUTO_SUSPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UERRR {
    bits: u8,
}
impl UERRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CPU_TYPE1R {
    bits: u8,
}
impl CPU_TYPE1R {
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
    #[doc = "Bits 24:31 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn family_type(&self) -> FAMILY_TYPER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FAMILY_TYPER { bits }
    }
    #[doc = "Bits 21:23 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved21(&self) -> RESERVED21R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED21R { bits }
    }
    #[doc = "Bit 20 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn mem_map(&self) -> MEM_MAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MEM_MAPR { bits }
    }
    #[doc = "Bits 16:19 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn cpu2(&self) -> CPU2R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CPU2R { bits }
    }
    #[doc = "Bits 12:15 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ee_in_main(&self) -> EE_IN_MAINR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EE_IN_MAINR { bits }
    }
    #[doc = "Bit 11 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rom(&self) -> ROMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ROMR { bits }
    }
    #[doc = "Bit 10 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn iflush(&self) -> IFLUSHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IFLUSHR { bits }
    }
    #[doc = "Bit 9 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sil3(&self) -> SIL3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SIL3R { bits }
    }
    #[doc = "Bit 8 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ecca(&self) -> ECCAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ECCAR { bits }
    }
    #[doc = "Bits 6:7 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auto_susp(&self) -> AUTO_SUSPR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AUTO_SUSPR { bits }
    }
    #[doc = "Bits 4:5 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn uerr(&self) -> UERRR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UERRR { bits }
    }
    #[doc = "Bits 0:3 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn cpu_type1(&self) -> CPU_TYPE1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CPU_TYPE1R { bits }
    }
}
