#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TRIGCNTLOAD {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
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
pub struct CNTR {
    bits: u16,
}
impl CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[doc = "Bits 0:15 - Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. To measure frequency of an event source: - Set start event equal to stop event. - Set CNT to number of periods to measure. Both 0 and 1 values measures a single event source period. To measure pulse width of an event source: - Set start event source equal to stop event source. - Select different polarity for start and stop event. - Set CNT to 0. To measure time from the start event to the Nth stop event when N > 1: - Select different start and stop event source. - Set CNT to (N-1). See the Technical Reference Manual for event timing requirements. When AUX_TDC:TRIGCNTCFG.EN is 1, CNT is loaded into TRIGCNT.CNT at the start of the measurement."]
    #[inline]
    pub fn cnt(&self) -> CNTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CNTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. To measure frequency of an event source: - Set start event equal to stop event. - Set CNT to number of periods to measure. Both 0 and 1 values measures a single event source period. To measure pulse width of an event source: - Set start event source equal to stop event source. - Select different polarity for start and stop event. - Set CNT to 0. To measure time from the start event to the Nth stop event when N > 1: - Select different start and stop event source. - Set CNT to (N-1). See the Technical Reference Manual for event timing requirements. When AUX_TDC:TRIGCNTCFG.EN is 1, CNT is loaded into TRIGCNT.CNT at the start of the measurement."]
    #[inline]
    pub fn cnt(&mut self) -> _CNTW {
        _CNTW { w: self }
    }
}
