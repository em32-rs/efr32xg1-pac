#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SCANDATAXP {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DATAPR {
    bits: u16,
}
impl DATAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SCANINPUTIDPEEKR {
    bits: u8,
}
impl SCANINPUTIDPEEKR {
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
    #[doc = "Bits 0:15 - Scan Conversion Result Data Peek"]
    #[inline]
    pub fn datap(&self) -> DATAPR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DATAPR { bits }
    }
    #[doc = "Bits 16:20 - Scan Conversion Data Source Peek"]
    #[inline]
    pub fn scaninputidpeek(&self) -> SCANINPUTIDPEEKR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SCANINPUTIDPEEKR { bits }
    }
}
