#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IRQCLR {
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
}
#[doc = r" Proxy"]
pub struct _RESERVED6W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED6W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 67108863;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AIF_DMA_INW<'a> {
    w: &'a mut W,
}
impl<'a> _AIF_DMA_INW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AIF_DMA_OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _AIF_DMA_OUTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WCLK_TIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _WCLK_TIMEOUTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BUS_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _BUS_ERRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WCLK_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _WCLK_ERRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PTR_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _PTR_ERRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[doc = "Bits 6:31 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved6(&mut self) -> _RESERVED6W {
        _RESERVED6W { w: self }
    }
    #[doc = "Bit 5 - 1: Clears the interrupt of IRQFLAGS.AIF_DMA_IN (unless a set criteria was given at the same time in which the clear will be ignored)"]
    #[inline]
    pub fn aif_dma_in(&mut self) -> _AIF_DMA_INW {
        _AIF_DMA_INW { w: self }
    }
    #[doc = "Bit 4 - 1: Clears the interrupt of IRQFLAGS.AIF_DMA_OUT (unless a set criteria was given at the same time in which the clear will be ignored)"]
    #[inline]
    pub fn aif_dma_out(&mut self) -> _AIF_DMA_OUTW {
        _AIF_DMA_OUTW { w: self }
    }
    #[doc = "Bit 3 - 1: Clears the interrupt of IRQFLAGS.WCLK_TIMEOUT (unless a set criteria was given at the same time in which the clear will be ignored)"]
    #[inline]
    pub fn wclk_timeout(&mut self) -> _WCLK_TIMEOUTW {
        _WCLK_TIMEOUTW { w: self }
    }
    #[doc = "Bit 2 - 1: Clears the interrupt of IRQFLAGS.BUS_ERR (unless a set criteria was given at the same time in which the clear will be ignored)"]
    #[inline]
    pub fn bus_err(&mut self) -> _BUS_ERRW {
        _BUS_ERRW { w: self }
    }
    #[doc = "Bit 1 - 1: Clears the interrupt of IRQFLAGS.WCLK_ERR (unless a set criteria was given at the same time in which the clear will be ignored)"]
    #[inline]
    pub fn wclk_err(&mut self) -> _WCLK_ERRW {
        _WCLK_ERRW { w: self }
    }
    #[doc = "Bit 0 - 1: Clears the interrupt of IRQFLAGS.PTR_ERR (unless a set criteria was given at the same time in which the clear will be ignored)"]
    #[inline]
    pub fn ptr_err(&mut self) -> _PTR_ERRW {
        _PTR_ERRW { w: self }
    }
}
