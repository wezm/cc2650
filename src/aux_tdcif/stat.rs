#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED8R {
    bits: u32,
}
impl RESERVED8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SATR {
    bits: bool,
}
impl SATR {
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
pub struct DONER {
    bits: bool,
}
impl DONER {
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
#[doc = "Possible values of the field `STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATER {
    #[doc = "Current state is TDC_FORCESTOP.\nYou wrote ABORT to CTL.CMD to abort the TDC measurement."]
    FORCE_STOP,
    #[doc = "Current state is TDC_WAIT_STARTFALL. \nThe fast-counter circuit waits for a falling edge on the start event."]
    START_FALL,
    #[doc = "Current state is TDC_STATE_WAIT_CLRCNT_DONE. \nThe state machine waits for fast-counter circuit to finish reset."]
    WAIT_CLR_CNT_DONE,
    #[doc = "Current state is TDC_STATE_POR. \nThis is the reset state."]
    POR,
    #[doc = "Current state is TDC_STATE_GETRESULTS.\nThe state machine copies the counter value from the fast-counter circuit."]
    GET_RESULT,
    #[doc = "Current state is TDC_STATE_WAIT_STOPCNTDOWN.\nThe fast-counter circuit looks for the stop condition. It will ignore a number of stop events configured in TRIGCNTLOAD.CNT."]
    WAIT_STOP_CNTDWN,
    #[doc = "Current state is TDC_STATE_WAIT_STOP.\nThe state machine waits for the fast-counter circuit to stop."]
    WAIT_STOP,
    #[doc = "Current state is TDC_STATE_CLRCNT. The fast-counter circuit is reset."]
    CLR_CNT,
    #[doc = "Current state is TDC_STATE_IDLE. \nThis is the default state after reset and abortion. State will change when you write CTL.CMD to either RUN_SYNC_START or RUN."]
    IDLE,
    #[doc = "Current state is TDC_STATE_WAIT_STARTSTOPCNTEN.\nThe fast-counter circuit looks for the start condition. The state machine waits for the fast-counter to increment."]
    WAIT_START_STOP_CNT_EN,
    #[doc = "Current state is TDC_STATE_WAIT_START. \nThe fast-counter circuit looks for the start condition. The state machine waits for the fast-counter to increment."]
    WAIT_START,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STATER::FORCE_STOP => 46,
            STATER::START_FALL => 30,
            STATER::WAIT_CLR_CNT_DONE => 22,
            STATER::POR => 15,
            STATER::GET_RESULT => 14,
            STATER::WAIT_STOP_CNTDWN => 12,
            STATER::WAIT_STOP => 8,
            STATER::CLR_CNT => 7,
            STATER::IDLE => 6,
            STATER::WAIT_START_STOP_CNT_EN => 4,
            STATER::WAIT_START => 0,
            STATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STATER {
        match value {
            46 => STATER::FORCE_STOP,
            30 => STATER::START_FALL,
            22 => STATER::WAIT_CLR_CNT_DONE,
            15 => STATER::POR,
            14 => STATER::GET_RESULT,
            12 => STATER::WAIT_STOP_CNTDWN,
            8 => STATER::WAIT_STOP,
            7 => STATER::CLR_CNT,
            6 => STATER::IDLE,
            4 => STATER::WAIT_START_STOP_CNT_EN,
            0 => STATER::WAIT_START,
            i => STATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FORCE_STOP`"]
    #[inline]
    pub fn is_force_stop(&self) -> bool {
        *self == STATER::FORCE_STOP
    }
    #[doc = "Checks if the value of the field is `START_FALL`"]
    #[inline]
    pub fn is_start_fall(&self) -> bool {
        *self == STATER::START_FALL
    }
    #[doc = "Checks if the value of the field is `WAIT_CLR_CNT_DONE`"]
    #[inline]
    pub fn is_wait_clr_cnt_done(&self) -> bool {
        *self == STATER::WAIT_CLR_CNT_DONE
    }
    #[doc = "Checks if the value of the field is `POR`"]
    #[inline]
    pub fn is_por(&self) -> bool {
        *self == STATER::POR
    }
    #[doc = "Checks if the value of the field is `GET_RESULT`"]
    #[inline]
    pub fn is_get_result(&self) -> bool {
        *self == STATER::GET_RESULT
    }
    #[doc = "Checks if the value of the field is `WAIT_STOP_CNTDWN`"]
    #[inline]
    pub fn is_wait_stop_cntdwn(&self) -> bool {
        *self == STATER::WAIT_STOP_CNTDWN
    }
    #[doc = "Checks if the value of the field is `WAIT_STOP`"]
    #[inline]
    pub fn is_wait_stop(&self) -> bool {
        *self == STATER::WAIT_STOP
    }
    #[doc = "Checks if the value of the field is `CLR_CNT`"]
    #[inline]
    pub fn is_clr_cnt(&self) -> bool {
        *self == STATER::CLR_CNT
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline]
    pub fn is_idle(&self) -> bool {
        *self == STATER::IDLE
    }
    #[doc = "Checks if the value of the field is `WAIT_START_STOP_CNT_EN`"]
    #[inline]
    pub fn is_wait_start_stop_cnt_en(&self) -> bool {
        *self == STATER::WAIT_START_STOP_CNT_EN
    }
    #[doc = "Checks if the value of the field is `WAIT_START`"]
    #[inline]
    pub fn is_wait_start(&self) -> bool {
        *self == STATER::WAIT_START
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 8:31 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved8(&self) -> RESERVED8R {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED8R { bits }
    }
    #[doc = "Bit 7 - TDC measurement saturation flag. 0: Conversion has not saturated. 1: Conversion stopped due to saturation. This field is cleared when a new measurement is started or when CLR_RESULT is written to CTL.CMD."]
    #[inline]
    pub fn sat(&self) -> SATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SATR { bits }
    }
    #[doc = "Bit 6 - TDC measurement complete flag. 0: TDC measurement has not yet completed. 1: TDC measurement has completed. This field clears when a new TDC measurement starts or when you write CLR_RESULT to CTL.CMD."]
    #[inline]
    pub fn done(&self) -> DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DONER { bits }
    }
    #[doc = "Bits 0:5 - TDC state machine status."]
    #[inline]
    pub fn state(&self) -> STATER {
        STATER::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
