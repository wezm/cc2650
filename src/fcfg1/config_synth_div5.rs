#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CONFIG_SYNTH_DIV5 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RFC_MDM_DEMIQMC0R {
    bits: u16,
}
impl RFC_MDM_DEMIQMC0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LDOVCO_TRIM_OUTPUTR {
    bits: u8,
}
impl LDOVCO_TRIM_OUTPUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SLDO_TRIM_OUTPUTR {
    bits: u8,
}
impl SLDO_TRIM_OUTPUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 12:27 - Trim value for RF Core. Value is read by RF Core ROM FW during RF Core initialization."]
    #[inline]
    pub fn rfc_mdm_demiqmc0(&self) -> RFC_MDM_DEMIQMC0R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RFC_MDM_DEMIQMC0R { bits }
    }
    #[doc = "Bits 6:11 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ldovco_trim_output(&self) -> LDOVCO_TRIM_OUTPUTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LDOVCO_TRIM_OUTPUTR { bits }
    }
    #[doc = "Bits 0:5 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sldo_trim_output(&self) -> SLDO_TRIM_OUTPUTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SLDO_TRIM_OUTPUTR { bits }
    }
}
