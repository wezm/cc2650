#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MISC_CONF_1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct DEVICE_MINOR_REVR {
    bits: u8,
}
impl DEVICE_MINOR_REVR {
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
    #[doc = "Bits 0:7 - HW minor revision number (a value of 0xFF shall be treated equally to 0x00). Any test of this field by SW should be implemented as a 'greater or equal' comparison as signed integer. Value may change without warning."]
    #[inline]
    pub fn device_minor_rev(&self) -> DEVICE_MINOR_REVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DEVICE_MINOR_REVR { bits }
    }
}
