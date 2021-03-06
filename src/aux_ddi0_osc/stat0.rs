#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STAT0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct SPARE31R {
    bits: bool,
}
impl SPARE31R {
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
#[doc = "Possible values of the field `SCLK_LF_SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLK_LF_SRCR {
    #[doc = "Low frequency XOSC"]
    XOSCLF,
    #[doc = "Low frequency RCOSC"]
    RCOSCLF,
    #[doc = "Low frequency clock derived from High Frequency XOSC"]
    XOSCHFDLF,
    #[doc = "Low frequency clock derived from High Frequency RCOSC"]
    RCOSCHFDLF,
}
impl SCLK_LF_SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SCLK_LF_SRCR::XOSCLF => 3,
            SCLK_LF_SRCR::RCOSCLF => 2,
            SCLK_LF_SRCR::XOSCHFDLF => 1,
            SCLK_LF_SRCR::RCOSCHFDLF => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SCLK_LF_SRCR {
        match value {
            3 => SCLK_LF_SRCR::XOSCLF,
            2 => SCLK_LF_SRCR::RCOSCLF,
            1 => SCLK_LF_SRCR::XOSCHFDLF,
            0 => SCLK_LF_SRCR::RCOSCHFDLF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XOSCLF`"]
    #[inline]
    pub fn is_xosclf(&self) -> bool {
        *self == SCLK_LF_SRCR::XOSCLF
    }
    #[doc = "Checks if the value of the field is `RCOSCLF`"]
    #[inline]
    pub fn is_rcosclf(&self) -> bool {
        *self == SCLK_LF_SRCR::RCOSCLF
    }
    #[doc = "Checks if the value of the field is `XOSCHFDLF`"]
    #[inline]
    pub fn is_xoschfdlf(&self) -> bool {
        *self == SCLK_LF_SRCR::XOSCHFDLF
    }
    #[doc = "Checks if the value of the field is `RCOSCHFDLF`"]
    #[inline]
    pub fn is_rcoschfdlf(&self) -> bool {
        *self == SCLK_LF_SRCR::RCOSCHFDLF
    }
}
#[doc = "Possible values of the field `SCLK_HF_SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLK_HF_SRCR {
    #[doc = "High frequency XOSC"]
    XOSC,
    #[doc = "High frequency RCOSC clock"]
    RCOSC,
}
impl SCLK_HF_SRCR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SCLK_HF_SRCR::XOSC => true,
            SCLK_HF_SRCR::RCOSC => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCLK_HF_SRCR {
        match value {
            true => SCLK_HF_SRCR::XOSC,
            false => SCLK_HF_SRCR::RCOSC,
        }
    }
    #[doc = "Checks if the value of the field is `XOSC`"]
    #[inline]
    pub fn is_xosc(&self) -> bool {
        *self == SCLK_HF_SRCR::XOSC
    }
    #[doc = "Checks if the value of the field is `RCOSC`"]
    #[inline]
    pub fn is_rcosc(&self) -> bool {
        *self == SCLK_HF_SRCR::RCOSC
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED23R {
    bits: u8,
}
impl RESERVED23R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RCOSC_HF_ENR {
    bits: bool,
}
impl RCOSC_HF_ENR {
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
pub struct RCOSC_LF_ENR {
    bits: bool,
}
impl RCOSC_LF_ENR {
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
pub struct XOSC_LF_ENR {
    bits: bool,
}
impl XOSC_LF_ENR {
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
pub struct CLK_DCDC_RDYR {
    bits: bool,
}
impl CLK_DCDC_RDYR {
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
pub struct CLK_DCDC_RDY_ACKR {
    bits: bool,
}
impl CLK_DCDC_RDY_ACKR {
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
pub struct SCLK_HF_LOSSR {
    bits: bool,
}
impl SCLK_HF_LOSSR {
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
pub struct SCLK_LF_LOSSR {
    bits: bool,
}
impl SCLK_LF_LOSSR {
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
pub struct XOSC_HF_ENR {
    bits: bool,
}
impl XOSC_HF_ENR {
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
pub struct RESERVED14R {
    bits: bool,
}
impl RESERVED14R {
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
pub struct XB_48M_CLK_ENR {
    bits: bool,
}
impl XB_48M_CLK_ENR {
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
pub struct RESERVED12R {
    bits: bool,
}
impl RESERVED12R {
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
pub struct XOSC_HF_LP_BUF_ENR {
    bits: bool,
}
impl XOSC_HF_LP_BUF_ENR {
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
pub struct XOSC_HF_HP_BUF_ENR {
    bits: bool,
}
impl XOSC_HF_HP_BUF_ENR {
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
pub struct RESERVED9R {
    bits: bool,
}
impl RESERVED9R {
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
pub struct ADC_THMETR {
    bits: bool,
}
impl ADC_THMETR {
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
pub struct ADC_DATA_READYR {
    bits: bool,
}
impl ADC_DATA_READYR {
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
pub struct ADC_DATAR {
    bits: u8,
}
impl ADC_DATAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PENDINGSCLKHFSWITCHINGR {
    bits: bool,
}
impl PENDINGSCLKHFSWITCHINGR {
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
    #[doc = "Bit 31 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn spare31(&self) -> SPARE31R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SPARE31R { bits }
    }
    #[doc = "Bits 29:30 - Indicates source for the sclk_lf"]
    #[inline]
    pub fn sclk_lf_src(&self) -> SCLK_LF_SRCR {
        SCLK_LF_SRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Indicates source for the sclk_hf"]
    #[inline]
    pub fn sclk_hf_src(&self) -> SCLK_HF_SRCR {
        SCLK_HF_SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 23:27 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved23(&self) -> RESERVED23R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED23R { bits }
    }
    #[doc = "Bit 22 - RCOSC_HF_EN"]
    #[inline]
    pub fn rcosc_hf_en(&self) -> RCOSC_HF_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RCOSC_HF_ENR { bits }
    }
    #[doc = "Bit 21 - RCOSC_LF_EN"]
    #[inline]
    pub fn rcosc_lf_en(&self) -> RCOSC_LF_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RCOSC_LF_ENR { bits }
    }
    #[doc = "Bit 20 - XOSC_LF_EN"]
    #[inline]
    pub fn xosc_lf_en(&self) -> XOSC_LF_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XOSC_LF_ENR { bits }
    }
    #[doc = "Bit 19 - CLK_DCDC_RDY"]
    #[inline]
    pub fn clk_dcdc_rdy(&self) -> CLK_DCDC_RDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLK_DCDC_RDYR { bits }
    }
    #[doc = "Bit 18 - CLK_DCDC_RDY_ACK"]
    #[inline]
    pub fn clk_dcdc_rdy_ack(&self) -> CLK_DCDC_RDY_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLK_DCDC_RDY_ACKR { bits }
    }
    #[doc = "Bit 17 - Indicates sclk_hf is lost"]
    #[inline]
    pub fn sclk_hf_loss(&self) -> SCLK_HF_LOSSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCLK_HF_LOSSR { bits }
    }
    #[doc = "Bit 16 - Indicates sclk_lf is lost"]
    #[inline]
    pub fn sclk_lf_loss(&self) -> SCLK_LF_LOSSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCLK_LF_LOSSR { bits }
    }
    #[doc = "Bit 15 - Indicates that XOSC_HF is enabled."]
    #[inline]
    pub fn xosc_hf_en(&self) -> XOSC_HF_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XOSC_HF_ENR { bits }
    }
    #[doc = "Bit 14 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved14(&self) -> RESERVED14R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED14R { bits }
    }
    #[doc = "Bit 13 - Indicates that the 48MHz clock from the DOUBLER is enabled. It will be enabled if 24 or 48 MHz crystal is used (enabled in doubler bypass for the 48MHz crystal)."]
    #[inline]
    pub fn xb_48m_clk_en(&self) -> XB_48M_CLK_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XB_48M_CLK_ENR { bits }
    }
    #[doc = "Bit 12 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved12(&self) -> RESERVED12R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED12R { bits }
    }
    #[doc = "Bit 11 - XOSC_HF_LP_BUF_EN"]
    #[inline]
    pub fn xosc_hf_lp_buf_en(&self) -> XOSC_HF_LP_BUF_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XOSC_HF_LP_BUF_ENR { bits }
    }
    #[doc = "Bit 10 - XOSC_HF_HP_BUF_EN"]
    #[inline]
    pub fn xosc_hf_hp_buf_en(&self) -> XOSC_HF_HP_BUF_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XOSC_HF_HP_BUF_ENR { bits }
    }
    #[doc = "Bit 9 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved9(&self) -> RESERVED9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED9R { bits }
    }
    #[doc = "Bit 8 - ADC_THMET"]
    #[inline]
    pub fn adc_thmet(&self) -> ADC_THMETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADC_THMETR { bits }
    }
    #[doc = "Bit 7 - indicates when adc_data is ready."]
    #[inline]
    pub fn adc_data_ready(&self) -> ADC_DATA_READYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADC_DATA_READYR { bits }
    }
    #[doc = "Bits 1:6 - adc_data"]
    #[inline]
    pub fn adc_data(&self) -> ADC_DATAR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADC_DATAR { bits }
    }
    #[doc = "Bit 0 - Indicates when sclk_hf is ready to be switched"]
    #[inline]
    pub fn pendingsclkhfswitching(&self) -> PENDINGSCLKHFSWITCHINGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PENDINGSCLKHFSWITCHINGR { bits }
    }
}
