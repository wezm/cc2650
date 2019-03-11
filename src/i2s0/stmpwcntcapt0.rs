#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STMPWCNTCAPT0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED16R {
    bits: u16,
}
impl RESERVED16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CAPT_VALUER {
    bits: u16,
}
impl CAPT_VALUER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 16:31 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved16(&self) -> RESERVED16R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED16R { bits }
    }
    #[doc = "Bits 0:15 - The value of the samplestamp WCLK counter (STMPWCNT.CURR_VALUE) last time an event was pulsed (event source selected in EVENT:I2SSTMPSEL0.EV for channel 0). This number corresponds to the number of positive WCLK edges since the samplestamp generator was enabled (not taking modification through STMPWADD/STMPWSET into account). The value is cleared when STMPCTL.STMP_EN = 0."]
    #[inline]
    pub fn capt_value(&self) -> CAPT_VALUER {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CAPT_VALUER { bits }
    }
}
