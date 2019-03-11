#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CPUIRQSEL28 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVR {
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS"]
    AUX_COMB,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EVR::AUX_COMB => 11,
            EVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EVR {
        match value {
            11 => EVR::AUX_COMB,
            i => EVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AUX_COMB`"]
    #[inline]
    pub fn is_aux_comb(&self) -> bool {
        *self == EVR::AUX_COMB
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
