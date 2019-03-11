#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::USER_ID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct PG_REVR {
    bits: u8,
}
impl PG_REVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VERR {
    bits: u8,
}
impl VERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED23R {
    bits: u8,
}
impl RESERVED23R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SEQUENCER {
    bits: u8,
}
impl SEQUENCER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PKGR {
    bits: u8,
}
impl PKGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PROTOCOLR {
    bits: u8,
}
impl PROTOCOLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED0R {
    bits: u16,
}
impl RESERVED0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 28:31 - Field used to distinguish revisions of the device."]
    #[inline]
    pub fn pg_rev(&self) -> PG_REVR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PG_REVR { bits }
    }
    #[doc = "Bits 26:27 - Version number. 0x0: Bits \\[25:12\\] of this register has the stated meaning. Any other setting indicate a different encoding of these bits."]
    #[inline]
    pub fn ver(&self) -> VERR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VERR { bits }
    }
    #[doc = "Bits 23:25 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved23(&self) -> RESERVED23R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED23R { bits }
    }
    #[doc = "Bits 19:22 - Sequence. Used to differentiate between marketing/orderable product where other fields of USER_ID is the same (temp range, flash size, voltage range etc)"]
    #[inline]
    pub fn sequence(&self) -> SEQUENCER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SEQUENCER { bits }
    }
    #[doc = "Bits 16:18 - Package type. 0x0: 4x4mm QFN (RHB) package 0x1: 5x5mm QFN (RSM) package 0x2: 7x7mm QFN (RGZ) package 0x3: Wafer sale package (naked die) 0x4: 2.7x2.7mm WCSP (YFV) 0x5: 7x7mm QFN package with Wettable Flanks Other values are reserved for future use. Packages available for a specific device are shown in the device datasheet."]
    #[inline]
    pub fn pkg(&self) -> PKGR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PKGR { bits }
    }
    #[doc = "Bits 12:15 - Protocols supported. 0x1: BLE 0x2: RF4CE 0x4: Zigbee/6lowpan 0x8: Proprietary More than one protocol can be supported on same device - values above are then combined."]
    #[inline]
    pub fn protocol(&self) -> PROTOCOLR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PROTOCOLR { bits }
    }
    #[doc = "Bits 0:11 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved0(&self) -> RESERVED0R {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED0R { bits }
    }
}
