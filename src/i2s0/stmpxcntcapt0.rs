#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STMPXCNTCAPT0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
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
    #[doc = "Bits 0:15 - The value of the samplestamp XOSC counter (STMPXCNT.CURR_VALUE) last time an event was pulsed (event source selected in \\[EVENT.I2SSTMPSEL0.EV\\] for channel 0). This number corresponds to the number of 24 MHz clock cycles since the last positive edge of the selected WCLK. The value is cleared when STMPCTL.STMP_EN = 0. Note: Due to buffering and synchronization, WCLK is delayed by a small number of BCLK periods and clk periods. Note: When calculating the fractional part of the sample stamp, STMPXPER may be less than this bit field."]
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
