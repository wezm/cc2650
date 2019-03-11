#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STAT2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct ADC_DCBIASR {
    bits: u8,
}
impl ADC_DCBIASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPM_RAMP1_THMETR {
    bits: bool,
}
impl HPM_RAMP1_THMETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct HPM_RAMP2_THMETR {
    bits: bool,
}
impl HPM_RAMP2_THMETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct HPM_RAMP3_THMETR {
    bits: bool,
}
impl HPM_RAMP3_THMETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED16R {
    bits: u8,
}
impl RESERVED16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RAMPSTATER {
    bits: u8,
}
impl RAMPSTATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED4R {
    bits: u8,
}
impl RESERVED4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AMPCOMP_REQR {
    bits: bool,
}
impl AMPCOMP_REQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct XOSC_HF_AMPGOODR {
    bits: bool,
}
impl XOSC_HF_AMPGOODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct XOSC_HF_FREQGOODR {
    bits: bool,
}
impl XOSC_HF_FREQGOODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct XOSC_HF_RF_FREQGOODR {
    bits: bool,
}
impl XOSC_HF_RF_FREQGOODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 26:31 - DC Bias read by RADC during SAR mode The value is an unsigned integer. It is used for debug only."]
    #[inline]
    pub fn adc_dcbias(&self) -> ADC_DCBIASR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADC_DCBIASR { bits }
    }
    #[doc = "Bit 25 - Indication of threshold is met for hpm_ramp1"]
    #[inline]
    pub fn hpm_ramp1_thmet(&self) -> HPM_RAMP1_THMETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPM_RAMP1_THMETR { bits }
    }
    #[doc = "Bit 24 - Indication of threshold is met for hpm_ramp2"]
    #[inline]
    pub fn hpm_ramp2_thmet(&self) -> HPM_RAMP2_THMETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPM_RAMP2_THMETR { bits }
    }
    #[doc = "Bit 23 - Indication of threshold is met for hpm_ramp3"]
    #[inline]
    pub fn hpm_ramp3_thmet(&self) -> HPM_RAMP3_THMETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPM_RAMP3_THMETR { bits }
    }
    #[doc = "Bits 16:22 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved16(&self) -> RESERVED16R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED16R { bits }
    }
    #[doc = "Bits 12:15 - xosc_hf amplitude compensation FSM This is identical to STAT1.RAMPSTATE. See that description for encoding."]
    #[inline]
    pub fn rampstate(&self) -> RAMPSTATER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RAMPSTATER { bits }
    }
    #[doc = "Bits 4:11 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved4(&self) -> RESERVED4R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED4R { bits }
    }
    #[doc = "Bit 3 - ampcomp_req"]
    #[inline]
    pub fn ampcomp_req(&self) -> AMPCOMP_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AMPCOMP_REQR { bits }
    }
    #[doc = "Bit 2 - amplitude of xosc_hf is within the required threshold (set by DDI). Not used for anything just for debug/status"]
    #[inline]
    pub fn xosc_hf_ampgood(&self) -> XOSC_HF_AMPGOODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XOSC_HF_AMPGOODR { bits }
    }
    #[doc = "Bit 1 - frequency of xosc_hf is good to use for the digital clocks"]
    #[inline]
    pub fn xosc_hf_freqgood(&self) -> XOSC_HF_FREQGOODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XOSC_HF_FREQGOODR { bits }
    }
    #[doc = "Bit 0 - frequency of xosc_hf is within +/- 20 ppm and xosc_hf is good for radio operations. Used for SW to start synthesizer."]
    #[inline]
    pub fn xosc_hf_rf_freqgood(&self) -> XOSC_HF_RF_FREQGOODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XOSC_HF_RF_FREQGOODR { bits }
    }
}
