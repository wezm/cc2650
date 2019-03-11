#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MCUBUSSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct DISCONNECTEDR {
    bits: bool,
}
impl DISCONNECTEDR {
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
pub struct DISCONNECT_ACKR {
    bits: bool,
}
impl DISCONNECT_ACKR {
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
    #[doc = "Bit 1 - Indicates whether the AUX domain and MCU domain buses are currently disconnected (1) or connected (0)."]
    #[inline]
    pub fn disconnected(&self) -> DISCONNECTEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISCONNECTEDR { bits }
    }
    #[doc = "Bit 0 - Acknowledges reception of the bus disconnection request, by matching the value of MCUBUSCTL.DISCONNECT_REQ. Note that if AON_WUC:AUXCTL.AUX_FORCE_ON = 1 a reconnect to the MCU domain bus will be made regardless of the state of MCUBUSCTL.DISCONNECT_REQ"]
    #[inline]
    pub fn disconnect_ack(&self) -> DISCONNECT_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISCONNECT_ACKR { bits }
    }
}
