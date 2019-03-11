#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MODE_CONF_1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct ALT_DCDC_VMINR {
    bits: u8,
}
impl ALT_DCDC_VMINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ALT_DCDC_DITHER_ENR {
    bits: bool,
}
impl ALT_DCDC_DITHER_ENR {
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
pub struct ALT_DCDC_IPEAKR {
    bits: u8,
}
impl ALT_DCDC_IPEAKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DELTA_IBIAS_INITR {
    bits: u8,
}
impl DELTA_IBIAS_INITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DELTA_IBIAS_OFFSETR {
    bits: u8,
}
impl DELTA_IBIAS_OFFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct XOSC_MAX_STARTR {
    bits: u8,
}
impl XOSC_MAX_STARTR {
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
    #[doc = "Bits 20:23 - Minimum voltage for when DC/DC should be used if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Voltage = (28 + ALT_DCDC_VMIN) / 16. 0: 1.75V 1: 1.8125V ... 14: 2.625V 15: 2.6875V NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline]
    pub fn alt_dcdc_vmin(&self) -> ALT_DCDC_VMINR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ALT_DCDC_VMINR { bits }
    }
    #[doc = "Bit 19 - Enable DC/DC dithering if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). 0: Dither disable 1: Dither enable"]
    #[inline]
    pub fn alt_dcdc_dither_en(&self) -> ALT_DCDC_DITHER_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALT_DCDC_DITHER_ENR { bits }
    }
    #[doc = "Bits 16:18 - Inductor peak current if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Assuming 10uH external inductor! Peak current = 31 + ( 4 * ALT_DCDC_IPEAK ) : 0: 31mA (min) ... 4: 47mA ... 7: 59mA (max)"]
    #[inline]
    pub fn alt_dcdc_ipeak(&self) -> ALT_DCDC_IPEAKR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ALT_DCDC_IPEAKR { bits }
    }
    #[doc = "Bits 12:15 - Signed delta value for IBIAS_INIT. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_INIT"]
    #[inline]
    pub fn delta_ibias_init(&self) -> DELTA_IBIAS_INITR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DELTA_IBIAS_INITR { bits }
    }
    #[doc = "Bits 8:11 - Signed delta value for IBIAS_OFFSET. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_OFFSET"]
    #[inline]
    pub fn delta_ibias_offset(&self) -> DELTA_IBIAS_OFFSETR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DELTA_IBIAS_OFFSETR { bits }
    }
    #[doc = "Bits 0:7 - Unsigned value of maximum XOSC startup time (worst case) in units of 100us. Value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0."]
    #[inline]
    pub fn xosc_max_start(&self) -> XOSC_MAX_STARTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XOSC_MAX_STARTR { bits }
    }
}
