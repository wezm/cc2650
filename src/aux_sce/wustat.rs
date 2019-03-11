#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::WUSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED18R {
    bits: u16,
}
impl RESERVED18R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXC_VECTORR {
    bits: u8,
}
impl EXC_VECTORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED9R {
    bits: u8,
}
impl RESERVED9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WU_SIGNALR {
    bits: bool,
}
impl WU_SIGNALR {
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
pub struct EV_SIGNALSR {
    bits: u8,
}
impl EV_SIGNALSR {
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
    #[doc = "Bits 18:31 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved18(&self) -> RESERVED18R {
        let bits = {
            const MASK: u16 = 16383;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED18R { bits }
    }
    #[doc = "Bits 16:17 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn exc_vector(&self) -> EXC_VECTORR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXC_VECTORR { bits }
    }
    #[doc = "Bits 9:15 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved9(&self) -> RESERVED9R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED9R { bits }
    }
    #[doc = "Bit 8 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn wu_signal(&self) -> WU_SIGNALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WU_SIGNALR { bits }
    }
    #[doc = "Bits 0:7 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ev_signals(&self) -> EV_SIGNALSR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EV_SIGNALSR { bits }
    }
}
