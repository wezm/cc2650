#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FCFG_BNK_TYPE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct B7_TYPER {
    bits: u8,
}
impl B7_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct B6_TYPER {
    bits: u8,
}
impl B6_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct B5_TYPER {
    bits: u8,
}
impl B5_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct B4_TYPER {
    bits: u8,
}
impl B4_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct B3_TYPER {
    bits: u8,
}
impl B3_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct B2_TYPER {
    bits: u8,
}
impl B2_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct B1_TYPER {
    bits: u8,
}
impl B1_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct B0_TYPER {
    bits: u8,
}
impl B0_TYPER {
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
    #[doc = "Bits 28:31 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b7_type(&self) -> B7_TYPER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        B7_TYPER { bits }
    }
    #[doc = "Bits 24:27 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b6_type(&self) -> B6_TYPER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        B6_TYPER { bits }
    }
    #[doc = "Bits 20:23 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b5_type(&self) -> B5_TYPER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        B5_TYPER { bits }
    }
    #[doc = "Bits 16:19 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b4_type(&self) -> B4_TYPER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        B4_TYPER { bits }
    }
    #[doc = "Bits 12:15 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b3_type(&self) -> B3_TYPER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        B3_TYPER { bits }
    }
    #[doc = "Bits 8:11 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b2_type(&self) -> B2_TYPER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        B2_TYPER { bits }
    }
    #[doc = "Bits 4:7 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b1_type(&self) -> B1_TYPER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        B1_TYPER { bits }
    }
    #[doc = "Bits 0:3 - Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b0_type(&self) -> B0_TYPER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        B0_TYPER { bits }
    }
}
