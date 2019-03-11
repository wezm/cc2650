#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FLASH_VHV_E {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct VHV_E_STARTR {
    bits: u16,
}
impl VHV_E_STARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VHV_E_STEP_HIGHTR {
    bits: u16,
}
impl VHV_E_STEP_HIGHTR {
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
    pub fn vhv_e_start(&self) -> VHV_E_STARTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        VHV_E_STARTR { bits }
    }
    #[doc = "Bits 0:15 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vhv_e_step_hight(&self) -> VHV_E_STEP_HIGHTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        VHV_E_STEP_HIGHTR { bits }
    }
}
