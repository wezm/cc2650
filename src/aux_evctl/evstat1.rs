#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::EVSTAT1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED16R {
    bits: u16,
}
impl RESERVED16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADC_IRQR {
    bits: bool,
}
impl ADC_IRQR {
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
pub struct MCU_EVR {
    bits: bool,
}
impl MCU_EVR {
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
pub struct ACLK_REFR {
    bits: bool,
}
impl ACLK_REFR {
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
pub struct AUXIO15R {
    bits: bool,
}
impl AUXIO15R {
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
pub struct AUXIO14R {
    bits: bool,
}
impl AUXIO14R {
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
pub struct AUXIO13R {
    bits: bool,
}
impl AUXIO13R {
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
pub struct AUXIO12R {
    bits: bool,
}
impl AUXIO12R {
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
pub struct AUXIO11R {
    bits: bool,
}
impl AUXIO11R {
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
pub struct AUXIO10R {
    bits: bool,
}
impl AUXIO10R {
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
pub struct AUXIO9R {
    bits: bool,
}
impl AUXIO9R {
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
pub struct AUXIO8R {
    bits: bool,
}
impl AUXIO8R {
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
pub struct AUXIO7R {
    bits: bool,
}
impl AUXIO7R {
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
pub struct AUXIO6R {
    bits: bool,
}
impl AUXIO6R {
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
pub struct AUXIO5R {
    bits: bool,
}
impl AUXIO5R {
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
pub struct AUXIO4R {
    bits: bool,
}
impl AUXIO4R {
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
pub struct AUXIO3R {
    bits: bool,
}
impl AUXIO3R {
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
    #[doc = "Bits 16:31 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved16(&self) -> RESERVED16R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED16R { bits }
    }
    #[doc = "Bit 15 - The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
    #[inline]
    pub fn adc_irq(&self) -> ADC_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADC_IRQR { bits }
    }
    #[doc = "Bit 14 - Event from EVENT configured by EVENT:AUXSEL0."]
    #[inline]
    pub fn mcu_ev(&self) -> MCU_EVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MCU_EVR { bits }
    }
    #[doc = "Bit 13 - TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_WUC:REFCLKCTL.REQ."]
    #[inline]
    pub fn aclk_ref(&self) -> ACLK_REFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACLK_REFR { bits }
    }
    #[doc = "Bit 12 - AUXIO15 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 7."]
    #[inline]
    pub fn auxio15(&self) -> AUXIO15R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO15R { bits }
    }
    #[doc = "Bit 11 - AUXIO14 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 6."]
    #[inline]
    pub fn auxio14(&self) -> AUXIO14R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO14R { bits }
    }
    #[doc = "Bit 10 - AUXIO13 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 5."]
    #[inline]
    pub fn auxio13(&self) -> AUXIO13R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO13R { bits }
    }
    #[doc = "Bit 9 - AUXIO12 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 4."]
    #[inline]
    pub fn auxio12(&self) -> AUXIO12R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO12R { bits }
    }
    #[doc = "Bit 8 - AUXIO11 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 3."]
    #[inline]
    pub fn auxio11(&self) -> AUXIO11R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO11R { bits }
    }
    #[doc = "Bit 7 - AUXIO10 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 2."]
    #[inline]
    pub fn auxio10(&self) -> AUXIO10R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO10R { bits }
    }
    #[doc = "Bit 6 - AUXIO9 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 1."]
    #[inline]
    pub fn auxio9(&self) -> AUXIO9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO9R { bits }
    }
    #[doc = "Bit 5 - AUXIO8 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 0."]
    #[inline]
    pub fn auxio8(&self) -> AUXIO8R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO8R { bits }
    }
    #[doc = "Bit 4 - AUXIO7 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 7."]
    #[inline]
    pub fn auxio7(&self) -> AUXIO7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO7R { bits }
    }
    #[doc = "Bit 3 - AUXIO6 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 6."]
    #[inline]
    pub fn auxio6(&self) -> AUXIO6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO6R { bits }
    }
    #[doc = "Bit 2 - AUXIO5 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 5."]
    #[inline]
    pub fn auxio5(&self) -> AUXIO5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO5R { bits }
    }
    #[doc = "Bit 1 - AUXIO4 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 4."]
    #[inline]
    pub fn auxio4(&self) -> AUXIO4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO4R { bits }
    }
    #[doc = "Bit 0 - AUXIO3 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 3."]
    #[inline]
    pub fn auxio3(&self) -> AUXIO3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO3R { bits }
    }
}
