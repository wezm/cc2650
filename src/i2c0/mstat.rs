#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED7R {
    bits: u32,
}
impl RESERVED7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BUSBSYR {
    bits: bool,
}
impl BUSBSYR {
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
pub struct IDLER {
    bits: bool,
}
impl IDLER {
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
pub struct ARBLSTR {
    bits: bool,
}
impl ARBLSTR {
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
pub struct DATACK_NR {
    bits: bool,
}
impl DATACK_NR {
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
pub struct ADRACK_NR {
    bits: bool,
}
impl ADRACK_NR {
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
pub struct ERRR {
    bits: bool,
}
impl ERRR {
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
pub struct BUSYR {
    bits: bool,
}
impl BUSYR {
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
    #[doc = "Bits 7:31 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved7(&self) -> RESERVED7R {
        let bits = {
            const MASK: u32 = 33554431;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED7R { bits }
    }
    #[doc = "Bit 6 - Bus busy 0: The I2C bus is idle. 1: The I2C bus is busy. The bit changes based on the MCTRL.START and MCTRL.STOP conditions."]
    #[inline]
    pub fn busbsy(&self) -> BUSBSYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BUSBSYR { bits }
    }
    #[doc = "Bit 5 - I2C idle 0: The I2C controller is not idle. 1: The I2C controller is idle."]
    #[inline]
    pub fn idle(&self) -> IDLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IDLER { bits }
    }
    #[doc = "Bit 4 - Arbitration lost 0: The I2C controller won arbitration. 1: The I2C controller lost arbitration."]
    #[inline]
    pub fn arblst(&self) -> ARBLSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ARBLSTR { bits }
    }
    #[doc = "Bit 3 - Data Was Not Acknowledge 0: The transmitted data was acknowledged. 1: The transmitted data was not acknowledged."]
    #[inline]
    pub fn datack_n(&self) -> DATACK_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DATACK_NR { bits }
    }
    #[doc = "Bit 2 - Address Was Not Acknowledge 0: The transmitted address was acknowledged. 1: The transmitted address was not acknowledged."]
    #[inline]
    pub fn adrack_n(&self) -> ADRACK_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADRACK_NR { bits }
    }
    #[doc = "Bit 1 - Error 0: No error was detected on the last operation. 1: An error occurred on the last operation."]
    #[inline]
    pub fn err(&self) -> ERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ERRR { bits }
    }
    #[doc = "Bit 0 - I2C busy 0: The controller is idle. 1: The controller is busy. When this bit-field is set, the other status bits are not valid. Note: The I2C controller requires four SYSBUS clock cycles to assert the BUSY status after I2C master operation has been initiated through MCTRL register. Hence after programming MCTRL register, application is requested to wait for four SYSBUS clock cycles before issuing a controller status inquiry through MSTAT register. Any prior inquiry would result in wrong status being reported."]
    #[inline]
    pub fn busy(&self) -> BUSYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BUSYR { bits }
    }
}
