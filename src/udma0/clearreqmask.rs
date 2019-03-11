#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLEARREQMASK {
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
pub struct _CHNLSW<'a> {
    w: &'a mut W,
}
impl<'a> _CHNLSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
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
    #[doc = "Bits 0:31 - Set the appropriate bit to enable DMA request for the channel. Write as: Bit \\[Ch\\] = 0: No effect. Use the SETREQMASK.CHNLS to disable channel C from generating requests. Bit \\[Ch\\] = 1: Enables channel \\[C\\] to generate DMA requests. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline]
    pub fn chnls(&mut self) -> _CHNLSW {
        _CHNLSW { w: self }
    }
}
