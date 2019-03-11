#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::OUT0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct VALUE_31_0R {
    bits: u32,
}
impl VALUE_31_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - LSW of 64- bit random value. New value ready when IRQFLAGSTAT.RDY = 1."]
    #[inline]
    pub fn value_31_0(&self) -> VALUE_31_0R {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        VALUE_31_0R { bits }
    }
}
