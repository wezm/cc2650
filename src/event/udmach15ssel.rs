#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::UDMACH15SSEL {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVR {
    #[doc = "Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting"]
    AON_RTC_COMB,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            EVR::AON_RTC_COMB => 7,
            EVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> EVR {
        match value {
            7 => EVR::AON_RTC_COMB,
            i => EVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AON_RTC_COMB`"]
    #[inline]
    pub fn is_aon_rtc_comb(&self) -> bool {
        *self == EVR::AON_RTC_COMB
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn ev(&self) -> EVR {
        EVR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
