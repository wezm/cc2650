#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FCOR_ERR_CNT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct COR_ERR_CNTR {
    bits: u32,
}
impl COR_ERR_CNTR {
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
    #[doc = "Bits 0:31 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn cor_err_cnt(&self) -> COR_ERR_CNTR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        COR_ERR_CNTR { bits }
    }
}
