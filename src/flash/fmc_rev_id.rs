#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FMC_REV_ID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct MOD_VERSIONR {
    bits: u32,
}
impl MOD_VERSIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CONFIG_CRCR {
    bits: u16,
}
impl CONFIG_CRCR {
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
    #[doc = "Bits 12:31 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn mod_version(&self) -> MOD_VERSIONR {
        let bits = {
            const MASK: u32 = 1048575;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        MOD_VERSIONR { bits }
    }
    #[doc = "Bits 0:11 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn config_crc(&self) -> CONFIG_CRCR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CONFIG_CRCR { bits }
    }
}
