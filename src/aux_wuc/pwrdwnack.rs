#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PWRDWNACK {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct ACKR {
    bits: bool,
}
impl ACKR {
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
    #[doc = "Bit 0 - Power down acknowledgment. Indicates whether the power down request given by PWRDWNREQ.REQ is captured by the AON domain or not 0: AUX can assume that the system is in active mode 1: The request for power down is acknowledged and the AUX must act like the system is in power down mode and power supply is limited The system CPU cannot use this bit since the bus bridge between MCU domain and AUX domain is always disconnected when this bit is set. For AUX_SCE use only"]
    #[inline]
    pub fn ack(&self) -> ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACKR { bits }
    }
}
