#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DSTATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `DATA0ZERO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA0ZEROR {
    #[doc = "In DATA0 bits 0 to 31 are all zero."]
    ZERO0TO31,
    #[doc = "In DATA0 bits 32 to 63 are all zero."]
    ZERO32TO63,
    #[doc = "In DATA0 bits 64 to 95 are all zero."]
    ZERO64TO95,
    #[doc = "In DATA0 bits 96 to 127 are all zero."]
    ZERO96TO127,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DATA0ZEROR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DATA0ZEROR::ZERO0TO31 => 1,
            DATA0ZEROR::ZERO32TO63 => 2,
            DATA0ZEROR::ZERO64TO95 => 4,
            DATA0ZEROR::ZERO96TO127 => 8,
            DATA0ZEROR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DATA0ZEROR {
        match value {
            1 => DATA0ZEROR::ZERO0TO31,
            2 => DATA0ZEROR::ZERO32TO63,
            4 => DATA0ZEROR::ZERO64TO95,
            8 => DATA0ZEROR::ZERO96TO127,
            i => DATA0ZEROR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO0TO31`"]
    #[inline]
    pub fn is_zero0to31(&self) -> bool {
        *self == DATA0ZEROR::ZERO0TO31
    }
    #[doc = "Checks if the value of the field is `ZERO32TO63`"]
    #[inline]
    pub fn is_zero32to63(&self) -> bool {
        *self == DATA0ZEROR::ZERO32TO63
    }
    #[doc = "Checks if the value of the field is `ZERO64TO95`"]
    #[inline]
    pub fn is_zero64to95(&self) -> bool {
        *self == DATA0ZEROR::ZERO64TO95
    }
    #[doc = "Checks if the value of the field is `ZERO96TO127`"]
    #[inline]
    pub fn is_zero96to127(&self) -> bool {
        *self == DATA0ZEROR::ZERO96TO127
    }
}
#[doc = r" Value of the field"]
pub struct DDATA0LSBSR {
    bits: u8,
}
impl DDATA0LSBSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DDATA0MSBSR {
    bits: u8,
}
impl DDATA0MSBSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DDATA1MSBR {
    bits: bool,
}
impl DDATA1MSBR {
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
pub struct CARRYR {
    bits: bool,
}
impl CARRYR {
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
    #[doc = "Bits 0:3 - Data 0 Zero"]
    #[inline]
    pub fn data0zero(&self) -> DATA0ZEROR {
        DATA0ZEROR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - LSBs in DDATA0"]
    #[inline]
    pub fn ddata0lsbs(&self) -> DDATA0LSBSR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DDATA0LSBSR { bits }
    }
    #[doc = "Bits 16:19 - MSB in DDATA0"]
    #[inline]
    pub fn ddata0msbs(&self) -> DDATA0MSBSR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DDATA0MSBSR { bits }
    }
    #[doc = "Bit 20 - MSB in DDATA1"]
    #[inline]
    pub fn ddata1msb(&self) -> DDATA1MSBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DDATA1MSBR { bits }
    }
    #[doc = "Bit 24 - Carry From Arithmetic Operation"]
    #[inline]
    pub fn carry(&self) -> CARRYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CARRYR { bits }
    }
}
