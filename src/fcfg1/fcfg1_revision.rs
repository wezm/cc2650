#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FCFG1_REVISION {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct REVR {
    bits: u32,
}
impl REVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - The revision number of the FCFG1 layout. This value will be read by application SW in order to determine which FCFG1 parameters that have valid values. This revision number must be incremented by 1 before any devices are to be produced if the FCFG1 layout has changed since the previous production of devices. Value migth change without warning."]
    #[inline]
    pub fn rev(&self) -> REVR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        REVR { bits }
    }
}
