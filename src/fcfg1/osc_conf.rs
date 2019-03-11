#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::OSC_CONF {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED1R {
    bits: u8,
}
impl RESERVED1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADC_SH_VBUF_ENR {
    bits: bool,
}
impl ADC_SH_VBUF_ENR {
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
pub struct ADC_SH_MODE_ENR {
    bits: bool,
}
impl ADC_SH_MODE_ENR {
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
pub struct ATESTLF_RCOSCLF_IBIAS_TRIMR {
    bits: bool,
}
impl ATESTLF_RCOSCLF_IBIAS_TRIMR {
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
pub struct XOSCLF_REGULATOR_TRIMR {
    bits: u8,
}
impl XOSCLF_REGULATOR_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct XOSCLF_CMIRRWR_RATIOR {
    bits: u8,
}
impl XOSCLF_CMIRRWR_RATIOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct XOSC_HF_FAST_STARTR {
    bits: u8,
}
impl XOSC_HF_FAST_STARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct XOSC_OPTIONR {
    bits: bool,
}
impl XOSC_OPTIONR {
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
pub struct HPOSC_OPTIONR {
    bits: bool,
}
impl HPOSC_OPTIONR {
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
pub struct HPOSC_BIAS_HOLD_MODE_ENR {
    bits: bool,
}
impl HPOSC_BIAS_HOLD_MODE_ENR {
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
pub struct HPOSC_CURRMIRR_RATIOR {
    bits: u8,
}
impl HPOSC_CURRMIRR_RATIOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPOSC_BIAS_RES_SETR {
    bits: u8,
}
impl HPOSC_BIAS_RES_SETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPOSC_FILTER_ENR {
    bits: bool,
}
impl HPOSC_FILTER_ENR {
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
pub struct HPOSC_BIAS_RECHARGE_DELAYR {
    bits: u8,
}
impl HPOSC_BIAS_RECHARGE_DELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED2R {
    bits: u8,
}
impl RESERVED2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPOSC_SERIES_CAPR {
    bits: u8,
}
impl HPOSC_SERIES_CAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPOSC_DIV3_BYPASSR {
    bits: bool,
}
impl HPOSC_DIV3_BYPASSR {
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
    #[doc = "Bits 30:31 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved1(&self) -> RESERVED1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED1R { bits }
    }
    #[doc = "Bit 29 - Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_VBUF_EN."]
    #[inline]
    pub fn adc_sh_vbuf_en(&self) -> ADC_SH_VBUF_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADC_SH_VBUF_ENR { bits }
    }
    #[doc = "Bit 28 - Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_MODE_EN."]
    #[inline]
    pub fn adc_sh_mode_en(&self) -> ADC_SH_MODE_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADC_SH_MODE_ENR { bits }
    }
    #[doc = "Bit 27 - Trim value for DDI_0_OSC:ATESTCTL.ATESTLF_RCOSCLF_IBIAS_TRIM."]
    #[inline]
    pub fn atestlf_rcosclf_ibias_trim(&self) -> ATESTLF_RCOSCLF_IBIAS_TRIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ATESTLF_RCOSCLF_IBIAS_TRIMR { bits }
    }
    #[doc = "Bits 25:26 - Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_REGULATOR_TRIM."]
    #[inline]
    pub fn xosclf_regulator_trim(&self) -> XOSCLF_REGULATOR_TRIMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XOSCLF_REGULATOR_TRIMR { bits }
    }
    #[doc = "Bits 21:24 - Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_CMIRRWR_RATIO."]
    #[inline]
    pub fn xosclf_cmirrwr_ratio(&self) -> XOSCLF_CMIRRWR_RATIOR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XOSCLF_CMIRRWR_RATIOR { bits }
    }
    #[doc = "Bits 19:20 - Trim value for DDI_0_OSC:CTL1.XOSC_HF_FAST_START."]
    #[inline]
    pub fn xosc_hf_fast_start(&self) -> XOSC_HF_FAST_STARTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XOSC_HF_FAST_STARTR { bits }
    }
    #[doc = "Bit 18 - 0: XOSC_HF unavailable (may not be bonded out) 1: XOSC_HF available (default)"]
    #[inline]
    pub fn xosc_option(&self) -> XOSC_OPTIONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XOSC_OPTIONR { bits }
    }
    #[doc = "Bit 17 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_option(&self) -> HPOSC_OPTIONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPOSC_OPTIONR { bits }
    }
    #[doc = "Bit 16 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_bias_hold_mode_en(&self) -> HPOSC_BIAS_HOLD_MODE_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPOSC_BIAS_HOLD_MODE_ENR { bits }
    }
    #[doc = "Bits 12:15 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_currmirr_ratio(&self) -> HPOSC_CURRMIRR_RATIOR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HPOSC_CURRMIRR_RATIOR { bits }
    }
    #[doc = "Bits 8:11 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_bias_res_set(&self) -> HPOSC_BIAS_RES_SETR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HPOSC_BIAS_RES_SETR { bits }
    }
    #[doc = "Bit 7 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_filter_en(&self) -> HPOSC_FILTER_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPOSC_FILTER_ENR { bits }
    }
    #[doc = "Bits 5:6 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_bias_recharge_delay(&self) -> HPOSC_BIAS_RECHARGE_DELAYR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HPOSC_BIAS_RECHARGE_DELAYR { bits }
    }
    #[doc = "Bits 3:4 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED2R { bits }
    }
    #[doc = "Bits 1:2 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_series_cap(&self) -> HPOSC_SERIES_CAPR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HPOSC_SERIES_CAPR { bits }
    }
    #[doc = "Bit 0 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_div3_bypass(&self) -> HPOSC_DIV3_BYPASSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPOSC_DIV3_BYPASSR { bits }
    }
}
