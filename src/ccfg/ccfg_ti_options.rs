#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CCFG_TI_OPTIONS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct TI_FA_ENABLER {
    bits: u8,
}
impl TI_FA_ENABLER {
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
    #[doc = "Bits 0:7 - TI Failure Analysis. 0xC5: Enable the functionality of unlocking the TI FA (TI Failure Analysis) option with the unlock code. All other values: Disable the functionality of unlocking the TI FA option with the unlock code."]
    #[inline]
    pub fn ti_fa_enable(&self) -> TI_FA_ENABLER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TI_FA_ENABLER { bits }
    }
}
