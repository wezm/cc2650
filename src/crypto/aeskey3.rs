#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AESKEY3 {
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
pub struct _KEY3W<'a> {
    w: &'a mut W,
}
impl<'a> _KEY3W<'a> {
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
    #[doc = "Bits 0:31 - AESKEY3.* bits 31+x:0+x or AESKEY2.* bits 159+x:128+x, where x = 0, 32, 64, 96 ordered from the LSW entry of this 4-deep register arrary. The interpretation of this field depends on the crypto operation mode."]
    #[inline]
    pub fn key3(&mut self) -> _KEY3W {
        _KEY3W { w: self }
    }
}
