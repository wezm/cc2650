#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYNC {
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
pub struct RESERVED8R {
    bits: u32,
}
impl RESERVED8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `SYNC3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC3W {
    #[doc = "A timeout event for both Timer A and Timer B of GPT3 is triggered"]
    BOTH,
    #[doc = "A timeout event for Timer B of GPT3 is triggered"]
    TIMERB,
    #[doc = "A timeout event for Timer A of GPT3 is triggered"]
    TIMERA,
    #[doc = "No Sync. GPT3 is not affected. "]
    NOSYNC,
}
impl SYNC3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNC3W::BOTH => 3,
            SYNC3W::TIMERB => 2,
            SYNC3W::TIMERA => 1,
            SYNC3W::NOSYNC => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNC3W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNC3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNC3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPT3 is triggered"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(SYNC3W::BOTH)
    }
    #[doc = "A timeout event for Timer B of GPT3 is triggered"]
    #[inline]
    pub fn timerb(self) -> &'a mut W {
        self.variant(SYNC3W::TIMERB)
    }
    #[doc = "A timeout event for Timer A of GPT3 is triggered"]
    #[inline]
    pub fn timera(self) -> &'a mut W {
        self.variant(SYNC3W::TIMERA)
    }
    #[doc = "No Sync. GPT3 is not affected."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(SYNC3W::NOSYNC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYNC2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC2W {
    #[doc = "A timeout event for both Timer A and Timer B of GPT2 is triggered"]
    BOTH,
    #[doc = "A timeout event for Timer B of GPT2 is triggered"]
    TIMERB,
    #[doc = "A timeout event for Timer A of GPT2 is triggered"]
    TIMERA,
    #[doc = "No Sync. GPT2 is not affected. "]
    NOSYNC,
}
impl SYNC2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNC2W::BOTH => 3,
            SYNC2W::TIMERB => 2,
            SYNC2W::TIMERA => 1,
            SYNC2W::NOSYNC => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNC2W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNC2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNC2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPT2 is triggered"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(SYNC2W::BOTH)
    }
    #[doc = "A timeout event for Timer B of GPT2 is triggered"]
    #[inline]
    pub fn timerb(self) -> &'a mut W {
        self.variant(SYNC2W::TIMERB)
    }
    #[doc = "A timeout event for Timer A of GPT2 is triggered"]
    #[inline]
    pub fn timera(self) -> &'a mut W {
        self.variant(SYNC2W::TIMERA)
    }
    #[doc = "No Sync. GPT2 is not affected."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(SYNC2W::NOSYNC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYNC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC1W {
    #[doc = "A timeout event for both Timer A and Timer B of GPT1 is triggered"]
    BOTH,
    #[doc = "A timeout event for Timer B of GPT1 is triggered"]
    TIMERB,
    #[doc = "A timeout event for Timer A of GPT1 is triggered"]
    TIMERA,
    #[doc = "No Sync. GPT1 is not affected. "]
    NOSYNC,
}
impl SYNC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNC1W::BOTH => 3,
            SYNC1W::TIMERB => 2,
            SYNC1W::TIMERA => 1,
            SYNC1W::NOSYNC => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNC1W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNC1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNC1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPT1 is triggered"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(SYNC1W::BOTH)
    }
    #[doc = "A timeout event for Timer B of GPT1 is triggered"]
    #[inline]
    pub fn timerb(self) -> &'a mut W {
        self.variant(SYNC1W::TIMERB)
    }
    #[doc = "A timeout event for Timer A of GPT1 is triggered"]
    #[inline]
    pub fn timera(self) -> &'a mut W {
        self.variant(SYNC1W::TIMERA)
    }
    #[doc = "No Sync. GPT1 is not affected."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(SYNC1W::NOSYNC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYNC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC0W {
    #[doc = "A timeout event for both Timer A and Timer B of GPT0 is triggered"]
    BOTH,
    #[doc = "A timeout event for Timer B of GPT0 is triggered"]
    TIMERB,
    #[doc = "A timeout event for Timer A of GPT0 is triggered"]
    TIMERA,
    #[doc = "No Sync. GPT0 is not affected. "]
    NOSYNC,
}
impl SYNC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNC0W::BOTH => 3,
            SYNC0W::TIMERB => 2,
            SYNC0W::TIMERA => 1,
            SYNC0W::NOSYNC => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNC0W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNC0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPT0 is triggered"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(SYNC0W::BOTH)
    }
    #[doc = "A timeout event for Timer B of GPT0 is triggered"]
    #[inline]
    pub fn timerb(self) -> &'a mut W {
        self.variant(SYNC0W::TIMERB)
    }
    #[doc = "A timeout event for Timer A of GPT0 is triggered"]
    #[inline]
    pub fn timera(self) -> &'a mut W {
        self.variant(SYNC0W::TIMERA)
    }
    #[doc = "No Sync. GPT0 is not affected."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(SYNC0W::NOSYNC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 8:31 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved8(&self) -> RESERVED8R {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED8R { bits }
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
    #[doc = "Bits 6:7 - Synchronize GPT Timer 3."]
    #[inline]
    pub fn sync3(&mut self) -> _SYNC3W {
        _SYNC3W { w: self }
    }
    #[doc = "Bits 4:5 - Synchronize GPT Timer 2."]
    #[inline]
    pub fn sync2(&mut self) -> _SYNC2W {
        _SYNC2W { w: self }
    }
    #[doc = "Bits 2:3 - Synchronize GPT Timer 1"]
    #[inline]
    pub fn sync1(&mut self) -> _SYNC1W {
        _SYNC1W { w: self }
    }
    #[doc = "Bits 0:1 - Synchronize GPT Timer 0"]
    #[inline]
    pub fn sync0(&mut self) -> _SYNC0W {
        _SYNC0W { w: self }
    }
}
