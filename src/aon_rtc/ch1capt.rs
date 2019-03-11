#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CH1CAPT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct SECR {
    bits: u16,
}
impl SECR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SUBSECR {
    bits: u16,
}
impl SUBSECR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 16:31 - Value of SEC.VALUE bits 15:0 at capture time."]
    #[inline]
    pub fn sec(&self) -> SECR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SECR { bits }
    }
    #[doc = "Bits 0:15 - Value of SUBSEC.VALUE bits 31:16 at capture time."]
    #[inline]
    pub fn subsec(&self) -> SUBSECR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SUBSECR { bits }
    }
}
