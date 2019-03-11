#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::WUEVFLAGS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct AON_RTC_CH2R {
    bits: bool,
}
impl AON_RTC_CH2R {
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
pub struct AON_SWR {
    bits: bool,
}
impl AON_SWR {
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
pub struct AON_PROG_WUR {
    bits: bool,
}
impl AON_PROG_WUR {
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
    #[doc = "Bit 2 - Indicates pending event from AON_RTC_CH2 compare. Note that this flag will be set whenever the AON_RTC_CH2 event happens, but that does not mean that this event is a wake-up event. To make the AON_RTC_CH2 a wake-up event for the AUX domain configure it as a wake-up event in AON_EVENT:AUXWUSEL.WU0_EV, AON_EVENT:AUXWUSEL.WU1_EV or AON_EVENT:AUXWUSEL.WU2_EV."]
    #[inline]
    pub fn aon_rtc_ch2(&self) -> AON_RTC_CH2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AON_RTC_CH2R { bits }
    }
    #[doc = "Bit 1 - Indicates pending event triggered by system CPU writing a 1 to AON_WUC:AUXCTL.SWEV."]
    #[inline]
    pub fn aon_sw(&self) -> AON_SWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AON_SWR { bits }
    }
    #[doc = "Bit 0 - Indicates pending event triggered by the sources selected in AON_EVENT:AUXWUSEL.WU0_EV, AON_EVENT:AUXWUSEL.WU1_EV and AON_EVENT:AUXWUSEL.WU2_EV."]
    #[inline]
    pub fn aon_prog_wu(&self) -> AON_PROG_WUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AON_PROG_WUR { bits }
    }
}
