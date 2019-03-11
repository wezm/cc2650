#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RFCSEL1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVR {
    #[doc = "GPT0B compare event. Configured by GPT0:TBMR.TCACT"]
    GPT0B_CMP,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EVR::GPT0B_CMP => 62,
            EVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EVR {
        match value {
            62 => EVR::GPT0B_CMP,
            i => EVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GPT0B_CMP`"]
    #[inline]
    pub fn is_gpt0b_cmp(&self) -> bool {
        *self == EVR::GPT0B_CMP
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - Read only selection value"]
    #[inline]
    pub fn ev(&self) -> EVR {
        EVR::_from({
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
