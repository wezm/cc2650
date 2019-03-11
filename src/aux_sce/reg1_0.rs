#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::REG1_0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct REG1R {
    bits: u16,
}
impl REG1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct REG0R {
    bits: u16,
}
impl REG0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 16:31 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reg1(&self) -> REG1R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        REG1R { bits }
    }
    #[doc = "Bits 0:15 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reg0(&self) -> REG0R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        REG0R { bits }
    }
}
