#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SOC_ADC_OFFSET_INT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED24R {
    bits: u8,
}
impl RESERVED24R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SOC_ADC_REL_OFFSET_TEMP1R {
    bits: u8,
}
impl SOC_ADC_REL_OFFSET_TEMP1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED8R {
    bits: u8,
}
impl RESERVED8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SOC_ADC_ABS_OFFSET_TEMP1R {
    bits: u8,
}
impl SOC_ADC_ABS_OFFSET_TEMP1R {
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
    #[doc = "Bits 24:31 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved24(&self) -> RESERVED24R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED24R { bits }
    }
    #[doc = "Bits 16:23 - SOC_ADC offset in relative reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
    #[inline]
    pub fn soc_adc_rel_offset_temp1(&self) -> SOC_ADC_REL_OFFSET_TEMP1R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SOC_ADC_REL_OFFSET_TEMP1R { bits }
    }
    #[doc = "Bits 8:15 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved8(&self) -> RESERVED8R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED8R { bits }
    }
    #[doc = "Bits 0:7 - SOC_ADC offset in absolute reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
    #[inline]
    pub fn soc_adc_abs_offset_temp1(&self) -> SOC_ADC_ABS_OFFSET_TEMP1R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SOC_ADC_ABS_OFFSET_TEMP1R { bits }
    }
}
