#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::I2SBCLKDIV {
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
pub struct RESERVED10R {
    bits: u32,
}
impl RESERVED10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BDIVR {
    bits: u16,
}
impl BDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _BDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _BDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
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
    #[doc = "Bits 10:31 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved10(&self) -> RESERVED10R {
        let bits = {
            const MASK: u32 = 4194303;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED10R { bits }
    }
    #[doc = "Bits 0:9 - An unsigned factor of the division ratio used to generate I2S BCLK \\[2-1024\\]: BCLK = MCUCLK/BDIV\\[Hz\\] MCUCLK is 48MHz in normal mode. For powerdown mode the frequency is defined by AON_WUC:MCUCLK.PWR_DWN_SRC A value of 0 is interpreted as 1024. A value of 1 is invalid. If BDIV is odd and I2SCLKCTL.SMPL_ON_POSEDGE = 0, the low phase of the clock is one MCUCLK period longer than the high phase. If BDIV is odd and I2SCLKCTL.SMPL_ON_POSEDGE = 1 , the high phase of the clock is one MCUCLK period longer than the low phase. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline]
    pub fn bdiv(&self) -> BDIVR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        BDIVR { bits }
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
    #[doc = "Bits 0:9 - An unsigned factor of the division ratio used to generate I2S BCLK \\[2-1024\\]: BCLK = MCUCLK/BDIV\\[Hz\\] MCUCLK is 48MHz in normal mode. For powerdown mode the frequency is defined by AON_WUC:MCUCLK.PWR_DWN_SRC A value of 0 is interpreted as 1024. A value of 1 is invalid. If BDIV is odd and I2SCLKCTL.SMPL_ON_POSEDGE = 0, the low phase of the clock is one MCUCLK period longer than the high phase. If BDIV is odd and I2SCLKCTL.SMPL_ON_POSEDGE = 1 , the high phase of the clock is one MCUCLK period longer than the low phase. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline]
    pub fn bdiv(&mut self) -> _BDIVW {
        _BDIVW { w: self }
    }
}
