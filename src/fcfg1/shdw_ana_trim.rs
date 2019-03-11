#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SHDW_ANA_TRIM {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct BOD_BANDGAP_TRIM_CNFR {
    bits: u8,
}
impl BOD_BANDGAP_TRIM_CNFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VDDR_ENABLE_PG1R {
    bits: bool,
}
impl VDDR_ENABLE_PG1R {
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
pub struct VDDR_OK_HYSR {
    bits: bool,
}
impl VDDR_OK_HYSR {
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
pub struct IPTAT_TRIMR {
    bits: u8,
}
impl IPTAT_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VDDR_TRIMR {
    bits: u8,
}
impl VDDR_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIMBOD_INTMODER {
    bits: u8,
}
impl TRIMBOD_INTMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIMBOD_EXTMODER {
    bits: u8,
}
impl TRIMBOD_EXTMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIMTEMPR {
    bits: u8,
}
impl TRIMTEMPR {
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
    #[doc = "Bits 25:26 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn bod_bandgap_trim_cnf(&self) -> BOD_BANDGAP_TRIM_CNFR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BOD_BANDGAP_TRIM_CNFR { bits }
    }
    #[doc = "Bit 24 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vddr_enable_pg1(&self) -> VDDR_ENABLE_PG1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VDDR_ENABLE_PG1R { bits }
    }
    #[doc = "Bit 23 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vddr_ok_hys(&self) -> VDDR_OK_HYSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VDDR_OK_HYSR { bits }
    }
    #[doc = "Bits 21:22 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn iptat_trim(&self) -> IPTAT_TRIMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IPTAT_TRIMR { bits }
    }
    #[doc = "Bits 16:20 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vddr_trim(&self) -> VDDR_TRIMR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VDDR_TRIMR { bits }
    }
    #[doc = "Bits 11:15 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn trimbod_intmode(&self) -> TRIMBOD_INTMODER {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIMBOD_INTMODER { bits }
    }
    #[doc = "Bits 6:10 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn trimbod_extmode(&self) -> TRIMBOD_EXTMODER {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIMBOD_EXTMODER { bits }
    }
    #[doc = "Bits 0:5 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn trimtemp(&self) -> TRIMTEMPR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIMTEMPR { bits }
    }
}
