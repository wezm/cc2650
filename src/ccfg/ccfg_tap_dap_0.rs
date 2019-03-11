#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CCFG_TAP_DAP_0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct CPU_DAP_ENABLER {
    bits: u8,
}
impl CPU_DAP_ENABLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRCM_TAP_ENABLER {
    bits: u8,
}
impl PRCM_TAP_ENABLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TEST_TAP_ENABLER {
    bits: u8,
}
impl TEST_TAP_ENABLER {
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
    #[doc = "Bits 16:23 - Enable CPU DAP. 0xC5: Main CPU DAP access is enabled during power-up/system-reset by ROM boot FW. Any other value: Main CPU DAP access will remain disabled out of power-up/system-reset."]
    #[inline]
    pub fn cpu_dap_enable(&self) -> CPU_DAP_ENABLER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CPU_DAP_ENABLER { bits }
    }
    #[doc = "Bits 8:15 - Enable PRCM TAP. 0xC5: PRCM TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PRCM TAP access will remain disabled out of power-up/system-reset."]
    #[inline]
    pub fn prcm_tap_enable(&self) -> PRCM_TAP_ENABLER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRCM_TAP_ENABLER { bits }
    }
    #[doc = "Bits 0:7 - Enable Test TAP. 0xC5: TEST TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: TEST TAP access will remain disabled out of power-up/system-reset."]
    #[inline]
    pub fn test_tap_enable(&self) -> TEST_TAP_ENABLER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TEST_TAP_ENABLER { bits }
    }
}
