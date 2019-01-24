#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RXDOUBLEP {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RXDATAP0R {
    bits: u8,
}
impl RXDATAP0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RXDATAP1R {
    bits: u8,
}
impl RXDATAP1R {
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
    #[doc = "Bits 0:7 - RX Data 0 Peek"]
    #[inline]
    pub fn rxdatap0(&self) -> RXDATAP0R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RXDATAP0R { bits }
    }
    #[doc = "Bits 8:15 - RX Data 1 Peek"]
    #[inline]
    pub fn rxdatap1(&self) -> RXDATAP1R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RXDATAP1R { bits }
    }
}
