#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CONFIG_IF_ADC {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct FF2ADJR {
    bits: u8,
}
impl FF2ADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FF3ADJR {
    bits: u8,
}
impl FF3ADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INT3ADJR {
    bits: u8,
}
impl INT3ADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FF1ADJR {
    bits: u8,
}
impl FF1ADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AAFCAPR {
    bits: u8,
}
impl AAFCAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INT2ADJR {
    bits: u8,
}
impl INT2ADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IFDIGLDO_TRIM_OUTPUTR {
    bits: u8,
}
impl IFDIGLDO_TRIM_OUTPUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IFANALDO_TRIM_OUTPUTR {
    bits: u8,
}
impl IFANALDO_TRIM_OUTPUTR {
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
    #[doc = "Bits 28:31 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ff2adj(&self) -> FF2ADJR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FF2ADJR { bits }
    }
    #[doc = "Bits 24:27 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ff3adj(&self) -> FF3ADJR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FF3ADJR { bits }
    }
    #[doc = "Bits 20:23 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn int3adj(&self) -> INT3ADJR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INT3ADJR { bits }
    }
    #[doc = "Bits 16:19 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ff1adj(&self) -> FF1ADJR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FF1ADJR { bits }
    }
    #[doc = "Bits 14:15 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn aafcap(&self) -> AAFCAPR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AAFCAPR { bits }
    }
    #[doc = "Bits 10:13 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn int2adj(&self) -> INT2ADJR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INT2ADJR { bits }
    }
    #[doc = "Bits 5:9 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ifdigldo_trim_output(&self) -> IFDIGLDO_TRIM_OUTPUTR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IFDIGLDO_TRIM_OUTPUTR { bits }
    }
    #[doc = "Bits 0:4 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ifanaldo_trim_output(&self) -> IFANALDO_TRIM_OUTPUTR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IFANALDO_TRIM_OUTPUTR { bits }
    }
}
