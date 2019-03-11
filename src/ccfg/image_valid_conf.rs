#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::IMAGE_VALID_CONF {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct IMAGE_VALIDR {
    bits: u32,
}
impl IMAGE_VALIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - This field must have a value of 0x00000000 in order for enabling the boot sequence to transfer control to a flash image. A non-zero value forces the boot sequence to call the boot loader. For CC2640R2: This field must have the address value of the start of the flash vector table in order for enabling the boot sequence to transfer control to a flash image. Any illegal vector table start address value forces the boot sequence to call the boot loader. Note that if any other legal vector table start address value than 0x0 is selected the PRCM:WARMRESET.WR_TO_PINRESET must be set to 1."]
    #[inline]
    pub fn image_valid(&self) -> IMAGE_VALIDR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        IMAGE_VALIDR { bits }
    }
}
