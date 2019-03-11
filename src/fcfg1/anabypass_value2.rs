#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ANABYPASS_VALUE2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct XOSC_HF_IBIASTHERMR {
    bits: u16,
}
impl XOSC_HF_IBIASTHERMR {
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
    #[doc = "Bits 0:13 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn xosc_hf_ibiastherm(&self) -> XOSC_HF_IBIASTHERMR {
        let bits = {
            const MASK: u16 = 16383;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        XOSC_HF_IBIASTHERMR { bits }
    }
}