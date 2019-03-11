#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AESAUTHLEN {
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
pub struct _LENW<'a> {
    w: &'a mut W,
}
impl<'a> _LENW<'a> {
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
    #[doc = "Bits 0:31 - Authentication data length in bytes for combined mode, CCM only. Supported AAD-lengths for CCM are from 0 to (216 - 28) bytes. Once processing with this context is started, this length decrements to zero. Writing this register triggers the engine to start using this context for CCM."]
    #[inline]
    pub fn len(&mut self) -> _LENW {
        _LENW { w: self }
    }
}
