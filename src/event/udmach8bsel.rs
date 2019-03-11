#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::UDMACH8BSEL {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVR {
    #[doc = "DMA sofware trigger from AUX, triggered by AUX_EVCTL:DMASWREQ.START"]
    AUX_SW_DMABREQ,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EVR::AUX_SW_DMABREQ => 116,
            EVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EVR {
        match value {
            116 => EVR::AUX_SW_DMABREQ,
            i => EVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AUX_SW_DMABREQ`"]
    #[inline]
    pub fn is_aux_sw_dmabreq(&self) -> bool {
        *self == EVR::AUX_SW_DMABREQ
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
