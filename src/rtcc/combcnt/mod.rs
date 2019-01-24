#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::COMBCNT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct PRECNTR {
    bits: u16,
}
impl PRECNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CNTLSBR {
    bits: u32,
}
impl CNTLSBR {
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
    #[doc = "Bits 0:14 - Pre-Counter Value"]
    #[inline]
    pub fn precnt(&self) -> PRECNTR {
        let bits = {
            const MASK: u16 = 32767;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PRECNTR { bits }
    }
    #[doc = "Bits 15:31 - Counter Value"]
    #[inline]
    pub fn cntlsb(&self) -> CNTLSBR {
        let bits = {
            const MASK: u32 = 131071;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        CNTLSBR { bits }
    }
}
