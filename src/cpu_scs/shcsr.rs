#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SHCSR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
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
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED19R {
    bits: u16,
}
impl RESERVED19R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `USGFAULTENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USGFAULTENAR {
    #[doc = "Exception enabled"]
    EN,
    #[doc = "Exception disabled"]
    DIS,
}
impl USGFAULTENAR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            USGFAULTENAR::EN => true,
            USGFAULTENAR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USGFAULTENAR {
        match value {
            true => USGFAULTENAR::EN,
            false => USGFAULTENAR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == USGFAULTENAR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == USGFAULTENAR::DIS
    }
}
#[doc = "Possible values of the field `BUSFAULTENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSFAULTENAR {
    #[doc = "Exception enabled"]
    EN,
    #[doc = "Exception disabled"]
    DIS,
}
impl BUSFAULTENAR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            BUSFAULTENAR::EN => true,
            BUSFAULTENAR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUSFAULTENAR {
        match value {
            true => BUSFAULTENAR::EN,
            false => BUSFAULTENAR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == BUSFAULTENAR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == BUSFAULTENAR::DIS
    }
}
#[doc = "Possible values of the field `MEMFAULTENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMFAULTENAR {
    #[doc = "Exception enabled"]
    EN,
    #[doc = "Exception disabled"]
    DIS,
}
impl MEMFAULTENAR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MEMFAULTENAR::EN => true,
            MEMFAULTENAR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MEMFAULTENAR {
        match value {
            true => MEMFAULTENAR::EN,
            false => MEMFAULTENAR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == MEMFAULTENAR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == MEMFAULTENAR::DIS
    }
}
#[doc = "Possible values of the field `SVCALLPENDED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCALLPENDEDR {
    #[doc = "Exception is pending."]
    PENDING,
    #[doc = "Exception is not active"]
    NOTPENDING,
}
impl SVCALLPENDEDR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SVCALLPENDEDR::PENDING => true,
            SVCALLPENDEDR::NOTPENDING => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SVCALLPENDEDR {
        match value {
            true => SVCALLPENDEDR::PENDING,
            false => SVCALLPENDEDR::NOTPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == SVCALLPENDEDR::PENDING
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline]
    pub fn is_notpending(&self) -> bool {
        *self == SVCALLPENDEDR::NOTPENDING
    }
}
#[doc = "Possible values of the field `BUSFAULTPENDED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSFAULTPENDEDR {
    #[doc = "Exception is pending."]
    PENDING,
    #[doc = "Exception is not active"]
    NOTPENDING,
}
impl BUSFAULTPENDEDR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            BUSFAULTPENDEDR::PENDING => true,
            BUSFAULTPENDEDR::NOTPENDING => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUSFAULTPENDEDR {
        match value {
            true => BUSFAULTPENDEDR::PENDING,
            false => BUSFAULTPENDEDR::NOTPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == BUSFAULTPENDEDR::PENDING
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline]
    pub fn is_notpending(&self) -> bool {
        *self == BUSFAULTPENDEDR::NOTPENDING
    }
}
#[doc = "Possible values of the field `MEMFAULTPENDED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMFAULTPENDEDR {
    #[doc = "Exception is pending."]
    PENDING,
    #[doc = "Exception is not active"]
    NOTPENDING,
}
impl MEMFAULTPENDEDR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MEMFAULTPENDEDR::PENDING => true,
            MEMFAULTPENDEDR::NOTPENDING => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MEMFAULTPENDEDR {
        match value {
            true => MEMFAULTPENDEDR::PENDING,
            false => MEMFAULTPENDEDR::NOTPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == MEMFAULTPENDEDR::PENDING
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline]
    pub fn is_notpending(&self) -> bool {
        *self == MEMFAULTPENDEDR::NOTPENDING
    }
}
#[doc = "Possible values of the field `USGFAULTPENDED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USGFAULTPENDEDR {
    #[doc = "Exception is pending."]
    PENDING,
    #[doc = "Exception is not active"]
    NOTPENDING,
}
impl USGFAULTPENDEDR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            USGFAULTPENDEDR::PENDING => true,
            USGFAULTPENDEDR::NOTPENDING => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USGFAULTPENDEDR {
        match value {
            true => USGFAULTPENDEDR::PENDING,
            false => USGFAULTPENDEDR::NOTPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == USGFAULTPENDEDR::PENDING
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline]
    pub fn is_notpending(&self) -> bool {
        *self == USGFAULTPENDEDR::NOTPENDING
    }
}
#[doc = "Possible values of the field `SYSTICKACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSTICKACTR {
    #[doc = "Exception is active"]
    ACTIVE,
    #[doc = "Exception is not active"]
    NOTACTIVE,
}
impl SYSTICKACTR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SYSTICKACTR::ACTIVE => true,
            SYSTICKACTR::NOTACTIVE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYSTICKACTR {
        match value {
            true => SYSTICKACTR::ACTIVE,
            false => SYSTICKACTR::NOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == SYSTICKACTR::ACTIVE
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline]
    pub fn is_notactive(&self) -> bool {
        *self == SYSTICKACTR::NOTACTIVE
    }
}
#[doc = r" Value of the field"]
pub struct PENDSVACTR {
    bits: bool,
}
impl PENDSVACTR {
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
pub struct RESERVED9R {
    bits: bool,
}
impl RESERVED9R {
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
#[doc = "Possible values of the field `MONITORACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONITORACTR {
    #[doc = "Exception is active"]
    ACTIVE,
    #[doc = "Exception is not active"]
    NOTACTIVE,
}
impl MONITORACTR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MONITORACTR::ACTIVE => true,
            MONITORACTR::NOTACTIVE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MONITORACTR {
        match value {
            true => MONITORACTR::ACTIVE,
            false => MONITORACTR::NOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == MONITORACTR::ACTIVE
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline]
    pub fn is_notactive(&self) -> bool {
        *self == MONITORACTR::NOTACTIVE
    }
}
#[doc = "Possible values of the field `SVCALLACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCALLACTR {
    #[doc = "Exception is active"]
    ACTIVE,
    #[doc = "Exception is not active"]
    NOTACTIVE,
}
impl SVCALLACTR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SVCALLACTR::ACTIVE => true,
            SVCALLACTR::NOTACTIVE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SVCALLACTR {
        match value {
            true => SVCALLACTR::ACTIVE,
            false => SVCALLACTR::NOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == SVCALLACTR::ACTIVE
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline]
    pub fn is_notactive(&self) -> bool {
        *self == SVCALLACTR::NOTACTIVE
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED4R {
    bits: u8,
}
impl RESERVED4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `USGFAULTACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USGFAULTACTR {
    #[doc = "Exception is active"]
    ACTIVE,
    #[doc = "Exception is not active"]
    NOTACTIVE,
}
impl USGFAULTACTR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            USGFAULTACTR::ACTIVE => true,
            USGFAULTACTR::NOTACTIVE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USGFAULTACTR {
        match value {
            true => USGFAULTACTR::ACTIVE,
            false => USGFAULTACTR::NOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == USGFAULTACTR::ACTIVE
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline]
    pub fn is_notactive(&self) -> bool {
        *self == USGFAULTACTR::NOTACTIVE
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED2R {
    bits: bool,
}
impl RESERVED2R {
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
#[doc = "Possible values of the field `BUSFAULTACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSFAULTACTR {
    #[doc = "Exception is active"]
    ACTIVE,
    #[doc = "Exception is not active"]
    NOTACTIVE,
}
impl BUSFAULTACTR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            BUSFAULTACTR::ACTIVE => true,
            BUSFAULTACTR::NOTACTIVE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUSFAULTACTR {
        match value {
            true => BUSFAULTACTR::ACTIVE,
            false => BUSFAULTACTR::NOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == BUSFAULTACTR::ACTIVE
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline]
    pub fn is_notactive(&self) -> bool {
        *self == BUSFAULTACTR::NOTACTIVE
    }
}
#[doc = "Possible values of the field `MEMFAULTACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMFAULTACTR {
    #[doc = "Exception is active"]
    ACTIVE,
    #[doc = "Exception is not active"]
    NOTACTIVE,
}
impl MEMFAULTACTR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MEMFAULTACTR::ACTIVE => true,
            MEMFAULTACTR::NOTACTIVE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MEMFAULTACTR {
        match value {
            true => MEMFAULTACTR::ACTIVE,
            false => MEMFAULTACTR::NOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == MEMFAULTACTR::ACTIVE
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline]
    pub fn is_notactive(&self) -> bool {
        *self == MEMFAULTACTR::NOTACTIVE
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED19W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED19W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 8191;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USGFAULTENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USGFAULTENAW {
    #[doc = "Exception enabled"]
    EN,
    #[doc = "Exception disabled"]
    DIS,
}
impl USGFAULTENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USGFAULTENAW::EN => true,
            USGFAULTENAW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USGFAULTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _USGFAULTENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USGFAULTENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exception enabled"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(USGFAULTENAW::EN)
    }
    #[doc = "Exception disabled"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(USGFAULTENAW::DIS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUSFAULTENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSFAULTENAW {
    #[doc = "Exception enabled"]
    EN,
    #[doc = "Exception disabled"]
    DIS,
}
impl BUSFAULTENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUSFAULTENAW::EN => true,
            BUSFAULTENAW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUSFAULTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _BUSFAULTENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUSFAULTENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exception enabled"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(BUSFAULTENAW::EN)
    }
    #[doc = "Exception disabled"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(BUSFAULTENAW::DIS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MEMFAULTENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMFAULTENAW {
    #[doc = "Exception enabled"]
    EN,
    #[doc = "Exception disabled"]
    DIS,
}
impl MEMFAULTENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MEMFAULTENAW::EN => true,
            MEMFAULTENAW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MEMFAULTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _MEMFAULTENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MEMFAULTENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exception enabled"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(MEMFAULTENAW::EN)
    }
    #[doc = "Exception disabled"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(MEMFAULTENAW::DIS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 19:31 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved19(&self) -> RESERVED19R {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED19R { bits }
    }
    #[doc = "Bit 18 - Usage fault system handler enable"]
    #[inline]
    pub fn usgfaultena(&self) -> USGFAULTENAR {
        USGFAULTENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Bus fault system handler enable"]
    #[inline]
    pub fn busfaultena(&self) -> BUSFAULTENAR {
        BUSFAULTENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - MemManage fault system handler enable"]
    #[inline]
    pub fn memfaultena(&self) -> MEMFAULTENAR {
        MEMFAULTENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - SVCall pending"]
    #[inline]
    pub fn svcallpended(&self) -> SVCALLPENDEDR {
        SVCALLPENDEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - BusFault pending"]
    #[inline]
    pub fn busfaultpended(&self) -> BUSFAULTPENDEDR {
        BUSFAULTPENDEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - MemManage exception pending"]
    #[inline]
    pub fn memfaultpended(&self) -> MEMFAULTPENDEDR {
        MEMFAULTPENDEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Usage fault pending"]
    #[inline]
    pub fn usgfaultpended(&self) -> USGFAULTPENDEDR {
        USGFAULTPENDEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - SysTick active flag. 0x0: Not active 0x1: Active"]
    #[inline]
    pub fn systickact(&self) -> SYSTICKACTR {
        SYSTICKACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - PendSV active 0x0: Not active 0x1: Active"]
    #[inline]
    pub fn pendsvact(&self) -> PENDSVACTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PENDSVACTR { bits }
    }
    #[doc = "Bit 9 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved9(&self) -> RESERVED9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED9R { bits }
    }
    #[doc = "Bit 8 - Debug monitor active"]
    #[inline]
    pub fn monitoract(&self) -> MONITORACTR {
        MONITORACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - SVCall active"]
    #[inline]
    pub fn svcallact(&self) -> SVCALLACTR {
        SVCALLACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:6 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved4(&self) -> RESERVED4R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED4R { bits }
    }
    #[doc = "Bit 3 - UsageFault exception active"]
    #[inline]
    pub fn usgfaultact(&self) -> USGFAULTACTR {
        USGFAULTACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED2R { bits }
    }
    #[doc = "Bit 1 - BusFault exception active"]
    #[inline]
    pub fn busfaultact(&self) -> BUSFAULTACTR {
        BUSFAULTACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - MemManage exception active"]
    #[inline]
    pub fn memfaultact(&self) -> MEMFAULTACTR {
        MEMFAULTACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bits 19:31 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved19(&mut self) -> _RESERVED19W {
        _RESERVED19W { w: self }
    }
    #[doc = "Bit 18 - Usage fault system handler enable"]
    #[inline]
    pub fn usgfaultena(&mut self) -> _USGFAULTENAW {
        _USGFAULTENAW { w: self }
    }
    #[doc = "Bit 17 - Bus fault system handler enable"]
    #[inline]
    pub fn busfaultena(&mut self) -> _BUSFAULTENAW {
        _BUSFAULTENAW { w: self }
    }
    #[doc = "Bit 16 - MemManage fault system handler enable"]
    #[inline]
    pub fn memfaultena(&mut self) -> _MEMFAULTENAW {
        _MEMFAULTENAW { w: self }
    }
}
