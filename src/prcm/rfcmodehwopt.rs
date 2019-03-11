#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RFCMODEHWOPT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED8R {
    bits: u32,
}
impl RESERVED8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `AVAIL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAILR {
    #[doc = "Mode 7 permitted"]
    MODE7,
    #[doc = "Mode 6 permitted"]
    MODE6,
    #[doc = "Mode 5 permitted"]
    MODE5,
    #[doc = "Mode 4 permitted"]
    MODE4,
    #[doc = "Mode 3 permitted"]
    MODE3,
    #[doc = "Mode 2 permitted"]
    MODE2,
    #[doc = "Mode 1 permitted"]
    MODE1,
    #[doc = "Mode 0 permitted"]
    MODE0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AVAILR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AVAILR::MODE7 => 128,
            AVAILR::MODE6 => 64,
            AVAILR::MODE5 => 32,
            AVAILR::MODE4 => 16,
            AVAILR::MODE3 => 8,
            AVAILR::MODE2 => 4,
            AVAILR::MODE1 => 2,
            AVAILR::MODE0 => 1,
            AVAILR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AVAILR {
        match value {
            128 => AVAILR::MODE7,
            64 => AVAILR::MODE6,
            32 => AVAILR::MODE5,
            16 => AVAILR::MODE4,
            8 => AVAILR::MODE3,
            4 => AVAILR::MODE2,
            2 => AVAILR::MODE1,
            1 => AVAILR::MODE0,
            i => AVAILR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MODE7`"]
    #[inline]
    pub fn is_mode7(&self) -> bool {
        *self == AVAILR::MODE7
    }
    #[doc = "Checks if the value of the field is `MODE6`"]
    #[inline]
    pub fn is_mode6(&self) -> bool {
        *self == AVAILR::MODE6
    }
    #[doc = "Checks if the value of the field is `MODE5`"]
    #[inline]
    pub fn is_mode5(&self) -> bool {
        *self == AVAILR::MODE5
    }
    #[doc = "Checks if the value of the field is `MODE4`"]
    #[inline]
    pub fn is_mode4(&self) -> bool {
        *self == AVAILR::MODE4
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline]
    pub fn is_mode3(&self) -> bool {
        *self == AVAILR::MODE3
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline]
    pub fn is_mode2(&self) -> bool {
        *self == AVAILR::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline]
    pub fn is_mode1(&self) -> bool {
        *self == AVAILR::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline]
    pub fn is_mode0(&self) -> bool {
        *self == AVAILR::MODE0
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 8:31 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved8(&self) -> RESERVED8R {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED8R { bits }
    }
    #[doc = "Bits 0:7 - Permitted RFC modes. More than one mode can be permitted."]
    #[inline]
    pub fn avail(&self) -> AVAILR {
        AVAILR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
