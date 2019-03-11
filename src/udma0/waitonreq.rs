#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::WAITONREQ {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct CHNLSTATUSR {
    bits: u32,
}
impl CHNLSTATUSR {
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
    #[doc = "Bits 0:31 - Channel wait on request status: Bit \\[Ch\\] = 0: Once uDMA receives a single or burst request on channel Ch, this channel may come out of active state even if request is still present. Bit \\[Ch\\] = 1: Once uDMA receives a single or burst request on channel Ch, it keeps channel Ch in active state until the requests are deasserted. This handshake is necessary for channels where the requester is in an asynchronous domain or can run at slower clock speed than uDMA"]
    #[inline]
    pub fn chnlstatus(&self) -> CHNLSTATUSR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        CHNLSTATUSR { bits }
    }
}
