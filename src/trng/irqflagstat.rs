#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::IRQFLAGSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct NEED_CLOCKR {
    bits: bool,
}
impl NEED_CLOCKR {
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
pub struct RESERVED2R {
    bits: u32,
}
impl RESERVED2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SHUTDOWN_OVFR {
    bits: bool,
}
impl SHUTDOWN_OVFR {
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
pub struct RDYR {
    bits: bool,
}
impl RDYR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 31 - 1: Indicates that the TRNG is busy generating entropy or is in one of its test modes - clocks may not be turned off and the power supply voltage must be kept stable. 0: TRNG is idle and can be shut down"]
    #[inline]
    pub fn need_clock(&self) -> NEED_CLOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NEED_CLOCKR { bits }
    }
    #[doc = "Bits 2:30 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        let bits = {
            const MASK: u32 = 536870911;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED2R { bits }
    }
    #[doc = "Bit 1 - 1: The number of FROs shut down (i.e. the number of '1' bits in the ALARMSTOP register) has exceeded the threshold set by ALARMCNT.SHUTDOWN_THR Writing '1' to IRQFLAGCLR.SHUTDOWN_OVF clears this bit to '0' again."]
    #[inline]
    pub fn shutdown_ovf(&self) -> SHUTDOWN_OVFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SHUTDOWN_OVFR { bits }
    }
    #[doc = "Bit 0 - 1: Data are available in OUT0 and OUT1. Acknowledging this state by writing '1' to IRQFLAGCLR.RDY clears this bit to '0'. If a new number is already available in the internal register of the TRNG, the number is directly clocked into the result register. In this case the status bit is asserted again, after one clock cycle."]
    #[inline]
    pub fn rdy(&self) -> RDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RDYR { bits }
    }
}
