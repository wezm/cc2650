#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RIS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct TXRISR {
    bits: bool,
}
impl TXRISR {
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
pub struct RXRISR {
    bits: bool,
}
impl RXRISR {
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
pub struct RTRISR {
    bits: bool,
}
impl RTRISR {
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
pub struct RORRISR {
    bits: bool,
}
impl RORRISR {
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
    #[doc = "Bit 3 - Raw transmit FIFO interrupt status: The transmit interrupt is asserted when there are four or fewer valid entries in the transmit FIFO. The transmit interrupt is not qualified with the SSI enable signal. Therefore one of the following ways can be used: - data can be written to the transmit FIFO prior to enabling the SSI and the interrupts. - SSI and interrupts can be enabled so that data can be written to the transmit FIFO by an interrupt service routine."]
    #[inline]
    pub fn txris(&self) -> TXRISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXRISR { bits }
    }
    #[doc = "Bit 2 - Raw interrupt state of receive FIFO interrupt: The receive interrupt is asserted when there are four or more valid entries in the receive FIFO."]
    #[inline]
    pub fn rxris(&self) -> RXRISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXRISR { bits }
    }
    #[doc = "Bit 1 - Raw interrupt state of receive timeout interrupt: The receive timeout interrupt is asserted when the receive FIFO is not empty and SSI has remained idle for a fixed 32 bit period. This mechanism can be used to notify the user that data is still present in the receive FIFO and requires servicing. This interrupt is deasserted if the receive FIFO becomes empty by subsequent reads, or if new data is received on RXD. It can also be cleared by writing to ICR.RTIC."]
    #[inline]
    pub fn rtris(&self) -> RTRISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RTRISR { bits }
    }
    #[doc = "Bit 0 - Raw interrupt state of receive overrun interrupt: The receive overrun interrupt is asserted when the FIFO is already full and an additional data frame is received, causing an overrun of the FIFO. Data is over-written in the receive shift register, but not the FIFO so the FIFO contents stay valid. It can also be cleared by writing to ICR.RORIC."]
    #[inline]
    pub fn rorris(&self) -> RORRISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RORRISR { bits }
    }
}
