#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::UDMACH7BSEL {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVR {
    #[doc = "DMA burst request event from AUX, configured by AUX_EVCTL:DMACTL"]
    AUX_DMABREQ,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EVR::AUX_DMABREQ => 118,
            EVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EVR {
        match value {
            118 => EVR::AUX_DMABREQ,
            i => EVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AUX_DMABREQ`"]
    #[inline]
    pub fn is_aux_dmabreq(&self) -> bool {
        *self == EVR::AUX_DMABREQ
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
