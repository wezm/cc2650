#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ANA2_TRIM {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RCOSCHFCTRIMFRACT_ENR {
    bits: bool,
}
impl RCOSCHFCTRIMFRACT_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RCOSCHFCTRIMFRACTR {
    bits: u8,
}
impl RCOSCHFCTRIMFRACTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED0R {
    bits: bool,
}
impl RESERVED0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SET_RCOSC_HF_FINE_RESISTORR {
    bits: u8,
}
impl SET_RCOSC_HF_FINE_RESISTORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ATESTLF_UDIGLDO_IBIAS_TRIMR {
    bits: bool,
}
impl ATESTLF_UDIGLDO_IBIAS_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct NANOAMP_RES_TRIMR {
    bits: u8,
}
impl NANOAMP_RES_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED1R {
    bits: u8,
}
impl RESERVED1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DITHER_ENR {
    bits: bool,
}
impl DITHER_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DCDC_IPEAKR {
    bits: u8,
}
impl DCDC_IPEAKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DEAD_TIME_TRIMR {
    bits: u8,
}
impl DEAD_TIME_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCDC_LOW_EN_SELR {
    bits: u8,
}
impl DCDC_LOW_EN_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCDC_HIGH_EN_SELR {
    bits: u8,
}
impl DCDC_HIGH_EN_SELR {
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
    #[doc = "Bit 31 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcoschfctrimfract_en(&self) -> RCOSCHFCTRIMFRACT_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RCOSCHFCTRIMFRACT_ENR { bits }
    }
    #[doc = "Bits 26:30 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcoschfctrimfract(&self) -> RCOSCHFCTRIMFRACTR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RCOSCHFCTRIMFRACTR { bits }
    }
    #[doc = "Bit 25 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved0(&self) -> RESERVED0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED0R { bits }
    }
    #[doc = "Bits 23:24 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn set_rcosc_hf_fine_resistor(&self) -> SET_RCOSC_HF_FINE_RESISTORR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SET_RCOSC_HF_FINE_RESISTORR { bits }
    }
    #[doc = "Bit 22 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn atestlf_udigldo_ibias_trim(&self) -> ATESTLF_UDIGLDO_IBIAS_TRIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ATESTLF_UDIGLDO_IBIAS_TRIMR { bits }
    }
    #[doc = "Bits 16:21 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn nanoamp_res_trim(&self) -> NANOAMP_RES_TRIMR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NANOAMP_RES_TRIMR { bits }
    }
    #[doc = "Bits 12:15 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved1(&self) -> RESERVED1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED1R { bits }
    }
    #[doc = "Bit 11 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dither_en(&self) -> DITHER_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DITHER_ENR { bits }
    }
    #[doc = "Bits 8:10 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dcdc_ipeak(&self) -> DCDC_IPEAKR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCDC_IPEAKR { bits }
    }
    #[doc = "Bits 6:7 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dead_time_trim(&self) -> DEAD_TIME_TRIMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DEAD_TIME_TRIMR { bits }
    }
    #[doc = "Bits 3:5 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dcdc_low_en_sel(&self) -> DCDC_LOW_EN_SELR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCDC_LOW_EN_SELR { bits }
    }
    #[doc = "Bits 0:2 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dcdc_high_en_sel(&self) -> DCDC_HIGH_EN_SELR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCDC_HIGH_EN_SELR { bits }
    }
}
