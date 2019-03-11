#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STAT1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `RAMPSTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMPSTATER {
    #[doc = "FAST_START_SETTLE"]
    FAST_START_SETTLE,
    #[doc = "FAST_START"]
    FAST_START,
    #[doc = "DUMMY_TO_INIT_1"]
    DUMMY_TO_INIT_1,
    #[doc = "IDAC_DECREMENT_WITH_MEASURE"]
    IDAC_DEC_W_MEASURE,
    #[doc = "IBIAS_INCREMENT"]
    IBIAS_INC,
    #[doc = "LPM_UPDATE"]
    LPM_UPDATE,
    #[doc = "IBIAS_DECREMENT_WITH_MEASURE"]
    IBIAS_DEC_W_MEASURE,
    #[doc = "IBIAS_CAP_UPDATE"]
    IBIAS_CAP_UPDATE,
    #[doc = "IDAC_INCREMENT"]
    IDAC_INCREMENT,
    #[doc = "HPM_UPDATE"]
    HPM_UPDATE,
    #[doc = "HPM_RAMP3"]
    HPM_RAMP3,
    #[doc = "HPM_RAMP2"]
    HPM_RAMP2,
    #[doc = "HPM_RAMP1"]
    HPM_RAMP1,
    #[doc = "INITIALIZATION"]
    INITIALIZATION,
    #[doc = "RESET"]
    RESET,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RAMPSTATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RAMPSTATER::FAST_START_SETTLE => 14,
            RAMPSTATER::FAST_START => 13,
            RAMPSTATER::DUMMY_TO_INIT_1 => 12,
            RAMPSTATER::IDAC_DEC_W_MEASURE => 11,
            RAMPSTATER::IBIAS_INC => 10,
            RAMPSTATER::LPM_UPDATE => 9,
            RAMPSTATER::IBIAS_DEC_W_MEASURE => 8,
            RAMPSTATER::IBIAS_CAP_UPDATE => 7,
            RAMPSTATER::IDAC_INCREMENT => 6,
            RAMPSTATER::HPM_UPDATE => 5,
            RAMPSTATER::HPM_RAMP3 => 4,
            RAMPSTATER::HPM_RAMP2 => 3,
            RAMPSTATER::HPM_RAMP1 => 2,
            RAMPSTATER::INITIALIZATION => 1,
            RAMPSTATER::RESET => 0,
            RAMPSTATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RAMPSTATER {
        match value {
            14 => RAMPSTATER::FAST_START_SETTLE,
            13 => RAMPSTATER::FAST_START,
            12 => RAMPSTATER::DUMMY_TO_INIT_1,
            11 => RAMPSTATER::IDAC_DEC_W_MEASURE,
            10 => RAMPSTATER::IBIAS_INC,
            9 => RAMPSTATER::LPM_UPDATE,
            8 => RAMPSTATER::IBIAS_DEC_W_MEASURE,
            7 => RAMPSTATER::IBIAS_CAP_UPDATE,
            6 => RAMPSTATER::IDAC_INCREMENT,
            5 => RAMPSTATER::HPM_UPDATE,
            4 => RAMPSTATER::HPM_RAMP3,
            3 => RAMPSTATER::HPM_RAMP2,
            2 => RAMPSTATER::HPM_RAMP1,
            1 => RAMPSTATER::INITIALIZATION,
            0 => RAMPSTATER::RESET,
            i => RAMPSTATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FAST_START_SETTLE`"]
    #[inline]
    pub fn is_fast_start_settle(&self) -> bool {
        *self == RAMPSTATER::FAST_START_SETTLE
    }
    #[doc = "Checks if the value of the field is `FAST_START`"]
    #[inline]
    pub fn is_fast_start(&self) -> bool {
        *self == RAMPSTATER::FAST_START
    }
    #[doc = "Checks if the value of the field is `DUMMY_TO_INIT_1`"]
    #[inline]
    pub fn is_dummy_to_init_1(&self) -> bool {
        *self == RAMPSTATER::DUMMY_TO_INIT_1
    }
    #[doc = "Checks if the value of the field is `IDAC_DEC_W_MEASURE`"]
    #[inline]
    pub fn is_idac_dec_w_measure(&self) -> bool {
        *self == RAMPSTATER::IDAC_DEC_W_MEASURE
    }
    #[doc = "Checks if the value of the field is `IBIAS_INC`"]
    #[inline]
    pub fn is_ibias_inc(&self) -> bool {
        *self == RAMPSTATER::IBIAS_INC
    }
    #[doc = "Checks if the value of the field is `LPM_UPDATE`"]
    #[inline]
    pub fn is_lpm_update(&self) -> bool {
        *self == RAMPSTATER::LPM_UPDATE
    }
    #[doc = "Checks if the value of the field is `IBIAS_DEC_W_MEASURE`"]
    #[inline]
    pub fn is_ibias_dec_w_measure(&self) -> bool {
        *self == RAMPSTATER::IBIAS_DEC_W_MEASURE
    }
    #[doc = "Checks if the value of the field is `IBIAS_CAP_UPDATE`"]
    #[inline]
    pub fn is_ibias_cap_update(&self) -> bool {
        *self == RAMPSTATER::IBIAS_CAP_UPDATE
    }
    #[doc = "Checks if the value of the field is `IDAC_INCREMENT`"]
    #[inline]
    pub fn is_idac_increment(&self) -> bool {
        *self == RAMPSTATER::IDAC_INCREMENT
    }
    #[doc = "Checks if the value of the field is `HPM_UPDATE`"]
    #[inline]
    pub fn is_hpm_update(&self) -> bool {
        *self == RAMPSTATER::HPM_UPDATE
    }
    #[doc = "Checks if the value of the field is `HPM_RAMP3`"]
    #[inline]
    pub fn is_hpm_ramp3(&self) -> bool {
        *self == RAMPSTATER::HPM_RAMP3
    }
    #[doc = "Checks if the value of the field is `HPM_RAMP2`"]
    #[inline]
    pub fn is_hpm_ramp2(&self) -> bool {
        *self == RAMPSTATER::HPM_RAMP2
    }
    #[doc = "Checks if the value of the field is `HPM_RAMP1`"]
    #[inline]
    pub fn is_hpm_ramp1(&self) -> bool {
        *self == RAMPSTATER::HPM_RAMP1
    }
    #[doc = "Checks if the value of the field is `INITIALIZATION`"]
    #[inline]
    pub fn is_initialization(&self) -> bool {
        *self == RAMPSTATER::INITIALIZATION
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == RAMPSTATER::RESET
    }
}
#[doc = r" Value of the field"]
pub struct HPM_UPDATE_AMPR {
    bits: u8,
}
impl HPM_UPDATE_AMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LPM_UPDATE_AMPR {
    bits: u8,
}
impl LPM_UPDATE_AMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FORCE_RCOSC_HFR {
    bits: bool,
}
impl FORCE_RCOSC_HFR {
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
pub struct SCLK_HF_ENR {
    bits: bool,
}
impl SCLK_HF_ENR {
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
pub struct SCLK_MF_ENR {
    bits: bool,
}
impl SCLK_MF_ENR {
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
pub struct ACLK_ADC_ENR {
    bits: bool,
}
impl ACLK_ADC_ENR {
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
pub struct ACLK_TDC_ENR {
    bits: bool,
}
impl ACLK_TDC_ENR {
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
pub struct ACLK_REF_ENR {
    bits: bool,
}
impl ACLK_REF_ENR {
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
pub struct CLK_CHP_ENR {
    bits: bool,
}
impl CLK_CHP_ENR {
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
pub struct CLK_DCDC_ENR {
    bits: bool,
}
impl CLK_DCDC_ENR {
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
pub struct SCLK_HF_GOODR {
    bits: bool,
}
impl SCLK_HF_GOODR {
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
pub struct SCLK_MF_GOODR {
    bits: bool,
}
impl SCLK_MF_GOODR {
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
pub struct SCLK_LF_GOODR {
    bits: bool,
}
impl SCLK_LF_GOODR {
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
pub struct ACLK_ADC_GOODR {
    bits: bool,
}
impl ACLK_ADC_GOODR {
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
pub struct ACLK_TDC_GOODR {
    bits: bool,
}
impl ACLK_TDC_GOODR {
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
pub struct ACLK_REF_GOODR {
    bits: bool,
}
impl ACLK_REF_GOODR {
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
pub struct CLK_CHP_GOODR {
    bits: bool,
}
impl CLK_CHP_GOODR {
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
pub struct CLK_DCDC_GOODR {
    bits: bool,
}
impl CLK_DCDC_GOODR {
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
    #[doc = "Bits 28:31 - AMPCOMP FSM State"]
    #[inline]
    pub fn rampstate(&self) -> RAMPSTATER {
        RAMPSTATER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:27 - OSC amplitude during HPM_UPDATE state. When amplitude compensation of XOSC_HF is enabled in high performance mode, this value is the amplitude of the crystal oscillations measured by the on-chip oscillator ADC, divided by 15 mV. For example, a value of 0x20 would indicate that the amplitude of the crystal is approximately 480 mV. To enable amplitude compensation, AON_WUC OSCCFG must be set to a non-zero value."]
    #[inline]
    pub fn hpm_update_amp(&self) -> HPM_UPDATE_AMPR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HPM_UPDATE_AMPR { bits }
    }
    #[doc = "Bits 16:21 - OSC amplitude during LPM_UPDATE state When amplitude compensation of XOSC_HF is enabled in low power mode, this value is the amplitude of the crystal oscillations measured by the on-chip oscillator ADC, divided by 15 mV. For example, a value of 0x20 would indicate that the amplitude of the crystal is approximately 480 mV. To enable amplitude compensation, AON_WUC OSCCFG must be set to a non-zero value."]
    #[inline]
    pub fn lpm_update_amp(&self) -> LPM_UPDATE_AMPR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPM_UPDATE_AMPR { bits }
    }
    #[doc = "Bit 15 - force_rcosc_hf"]
    #[inline]
    pub fn force_rcosc_hf(&self) -> FORCE_RCOSC_HFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FORCE_RCOSC_HFR { bits }
    }
    #[doc = "Bit 14 - SCLK_HF_EN"]
    #[inline]
    pub fn sclk_hf_en(&self) -> SCLK_HF_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCLK_HF_ENR { bits }
    }
    #[doc = "Bit 13 - SCLK_MF_EN"]
    #[inline]
    pub fn sclk_mf_en(&self) -> SCLK_MF_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCLK_MF_ENR { bits }
    }
    #[doc = "Bit 12 - ACLK_ADC_EN"]
    #[inline]
    pub fn aclk_adc_en(&self) -> ACLK_ADC_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACLK_ADC_ENR { bits }
    }
    #[doc = "Bit 11 - ACLK_TDC_EN"]
    #[inline]
    pub fn aclk_tdc_en(&self) -> ACLK_TDC_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACLK_TDC_ENR { bits }
    }
    #[doc = "Bit 10 - ACLK_REF_EN"]
    #[inline]
    pub fn aclk_ref_en(&self) -> ACLK_REF_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACLK_REF_ENR { bits }
    }
    #[doc = "Bit 9 - CLK_CHP_EN"]
    #[inline]
    pub fn clk_chp_en(&self) -> CLK_CHP_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLK_CHP_ENR { bits }
    }
    #[doc = "Bit 8 - CLK_DCDC_EN"]
    #[inline]
    pub fn clk_dcdc_en(&self) -> CLK_DCDC_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLK_DCDC_ENR { bits }
    }
    #[doc = "Bit 7 - SCLK_HF_GOOD"]
    #[inline]
    pub fn sclk_hf_good(&self) -> SCLK_HF_GOODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCLK_HF_GOODR { bits }
    }
    #[doc = "Bit 6 - SCLK_MF_GOOD"]
    #[inline]
    pub fn sclk_mf_good(&self) -> SCLK_MF_GOODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCLK_MF_GOODR { bits }
    }
    #[doc = "Bit 5 - SCLK_LF_GOOD"]
    #[inline]
    pub fn sclk_lf_good(&self) -> SCLK_LF_GOODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCLK_LF_GOODR { bits }
    }
    #[doc = "Bit 4 - ACLK_ADC_GOOD"]
    #[inline]
    pub fn aclk_adc_good(&self) -> ACLK_ADC_GOODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACLK_ADC_GOODR { bits }
    }
    #[doc = "Bit 3 - ACLK_TDC_GOOD"]
    #[inline]
    pub fn aclk_tdc_good(&self) -> ACLK_TDC_GOODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACLK_TDC_GOODR { bits }
    }
    #[doc = "Bit 2 - ACLK_REF_GOOD"]
    #[inline]
    pub fn aclk_ref_good(&self) -> ACLK_REF_GOODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACLK_REF_GOODR { bits }
    }
    #[doc = "Bit 1 - CLK_CHP_GOOD"]
    #[inline]
    pub fn clk_chp_good(&self) -> CLK_CHP_GOODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLK_CHP_GOODR { bits }
    }
    #[doc = "Bit 0 - CLK_DCDC_GOOD"]
    #[inline]
    pub fn clk_dcdc_good(&self) -> CLK_DCDC_GOODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLK_DCDC_GOODR { bits }
    }
}
