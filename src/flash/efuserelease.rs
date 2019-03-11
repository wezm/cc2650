#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::EFUSERELEASE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct ODPYEARR {
    bits: u8,
}
impl ODPYEARR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ODPMONTHR {
    bits: u8,
}
impl ODPMONTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ODPDAYR {
    bits: u8,
}
impl ODPDAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EFUSEYEARR {
    bits: u8,
}
impl EFUSEYEARR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EFUSEMONTHR {
    bits: u8,
}
impl EFUSEMONTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EFUSEDAYR {
    bits: u8,
}
impl EFUSEDAYR {
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
    #[doc = "Bits 25:31 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn odpyear(&self) -> ODPYEARR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ODPYEARR { bits }
    }
    #[doc = "Bits 21:24 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn odpmonth(&self) -> ODPMONTHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ODPMONTHR { bits }
    }
    #[doc = "Bits 16:20 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn odpday(&self) -> ODPDAYR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ODPDAYR { bits }
    }
    #[doc = "Bits 9:15 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efuseyear(&self) -> EFUSEYEARR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EFUSEYEARR { bits }
    }
    #[doc = "Bits 5:8 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efusemonth(&self) -> EFUSEMONTHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EFUSEMONTHR { bits }
    }
    #[doc = "Bits 0:4 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efuseday(&self) -> EFUSEDAYR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EFUSEDAYR { bits }
    }
}
