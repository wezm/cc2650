#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SHDW_DIE_ID_1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct ID_63_32R {
    bits: u32,
}
impl ID_63_32R {
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
    #[doc = "Bits 0:31 - Shadow of DIE_ID_1 register in eFuse row number 4"]
    #[inline]
    pub fn id_63_32(&self) -> ID_63_32R {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        ID_63_32R { bits }
    }
}
