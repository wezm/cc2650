#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SOC_ADC_REF_TRIM_AND_OFFSET_EXT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED6R {
    bits: u32,
}
impl RESERVED6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SOC_ADC_REF_VOLTAGE_TRIM_TEMP1R {
    bits: u8,
}
impl SOC_ADC_REF_VOLTAGE_TRIM_TEMP1R {
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
    #[doc = "Bits 6:31 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved6(&self) -> RESERVED6R {
        let bits = {
            const MASK: u32 = 67108863;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED6R { bits }
    }
    #[doc = "Bits 0:5 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn soc_adc_ref_voltage_trim_temp1(&self) -> SOC_ADC_REF_VOLTAGE_TRIM_TEMP1R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SOC_ADC_REF_VOLTAGE_TRIM_TEMP1R { bits }
    }
}
