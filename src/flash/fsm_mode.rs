#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FSM_MODE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED20R {
    bits: u16,
}
impl RESERVED20R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RDV_SUBMODER {
    bits: u8,
}
impl RDV_SUBMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PGM_SUBMODER {
    bits: u8,
}
impl PGM_SUBMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ERA_SUBMODER {
    bits: u8,
}
impl ERA_SUBMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SUBMODER {
    bits: u8,
}
impl SUBMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SAV_PGM_CMDR {
    bits: u8,
}
impl SAV_PGM_CMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SAV_ERA_MODER {
    bits: u8,
}
impl SAV_ERA_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MODER {
    bits: u8,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CMDR {
    bits: u8,
}
impl CMDR {
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
    pub fn reserved20(&self) -> RESERVED20R {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED20R { bits }
    }
    #[doc = "Bits 18:19 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rdv_submode(&self) -> RDV_SUBMODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RDV_SUBMODER { bits }
    }
    #[doc = "Bits 16:17 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn pgm_submode(&self) -> PGM_SUBMODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PGM_SUBMODER { bits }
    }
    #[doc = "Bits 14:15 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn era_submode(&self) -> ERA_SUBMODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ERA_SUBMODER { bits }
    }
    #[doc = "Bits 12:13 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn submode(&self) -> SUBMODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SUBMODER { bits }
    }
    #[doc = "Bits 9:11 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sav_pgm_cmd(&self) -> SAV_PGM_CMDR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SAV_PGM_CMDR { bits }
    }
    #[doc = "Bits 6:8 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sav_era_mode(&self) -> SAV_ERA_MODER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SAV_ERA_MODER { bits }
    }
    #[doc = "Bits 3:5 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn mode(&self) -> MODER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MODER { bits }
    }
    #[doc = "Bits 0:2 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn cmd(&self) -> CMDR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMDR { bits }
    }
}
