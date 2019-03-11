#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ICEPICK_DEVICE_ID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct PG_REVR {
    bits: u8,
}
impl PG_REVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WAFER_IDR {
    bits: u16,
}
impl WAFER_IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MANUFACTURER_IDR {
    bits: u16,
}
impl MANUFACTURER_IDR {
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
    #[doc = "Bits 28:31 - Field used to distinguish revisions of the device."]
    #[inline]
    pub fn pg_rev(&self) -> PG_REVR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PG_REVR { bits }
    }
    #[doc = "Bits 12:27 - Field used to identify silicon die."]
    #[inline]
    pub fn wafer_id(&self) -> WAFER_IDR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        WAFER_IDR { bits }
    }
    #[doc = "Bits 0:11 - Manufacturer code. 0x02F: Texas Instruments"]
    #[inline]
    pub fn manufacturer_id(&self) -> MANUFACTURER_IDR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MANUFACTURER_IDR { bits }
    }
}
