#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::EXT_LF_CLK {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct DIOR {
    bits: u8,
}
impl DIOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RTC_INCREMENTR {
    bits: u32,
}
impl RTC_INCREMENTR {
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
    #[doc = "Bits 24:31 - Unsigned integer, selecting the DIO to supply external 32kHz clock as SCLK_LF when MODE_CONF.SCLK_LF_OPTION is set to EXTERNAL. The selected DIO will be marked as reserved by the pin driver (TI-RTOS environment) and hence not selectable for other usage."]
    #[inline]
    pub fn dio(&self) -> DIOR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIOR { bits }
    }
    #[doc = "Bits 0:23 - Unsigned integer, defining the input frequency of the external clock and is written to AON_RTC:SUBSECINC.VALUEINC. Defined as follows: EXT_LF_CLK.RTC_INCREMENT = 2^38/InputClockFrequency in Hertz (e.g.: RTC_INCREMENT=0x800000 for InputClockFrequency=32768 Hz)"]
    #[inline]
    pub fn rtc_increment(&self) -> RTC_INCREMENTR {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RTC_INCREMENTR { bits }
    }
}
