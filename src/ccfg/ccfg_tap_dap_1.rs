#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CCFG_TAP_DAP_1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct PBIST2_TAP_ENABLER {
    bits: u8,
}
impl PBIST2_TAP_ENABLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PBIST1_TAP_ENABLER {
    bits: u8,
}
impl PBIST1_TAP_ENABLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WUC_TAP_ENABLER {
    bits: u8,
}
impl WUC_TAP_ENABLER {
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
    #[doc = "Bits 16:23 - Enable PBIST2 TAP. 0xC5: PBIST2 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST2 TAP access will remain disabled out of power-up/system-reset."]
    #[inline]
    pub fn pbist2_tap_enable(&self) -> PBIST2_TAP_ENABLER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PBIST2_TAP_ENABLER { bits }
    }
    #[doc = "Bits 8:15 - Enable PBIST1 TAP. 0xC5: PBIST1 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST1 TAP access will remain disabled out of power-up/system-reset."]
    #[inline]
    pub fn pbist1_tap_enable(&self) -> PBIST1_TAP_ENABLER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PBIST1_TAP_ENABLER { bits }
    }
    #[doc = "Bits 0:7 - Enable WUC TAP 0xC5: WUC TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: WUC TAP access will remain disabled out of power-up/system-reset."]
    #[inline]
    pub fn wuc_tap_enable(&self) -> WUC_TAP_ENABLER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WUC_TAP_ENABLER { bits }
    }
}
