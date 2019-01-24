#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SINGLEACTR {
    bits: bool,
}
impl SINGLEACTR {
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
pub struct SCANACTR {
    bits: bool,
}
impl SCANACTR {
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
pub struct SINGLEREFWARMR {
    bits: bool,
}
impl SINGLEREFWARMR {
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
pub struct SCANREFWARMR {
    bits: bool,
}
impl SCANREFWARMR {
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
#[doc = "Possible values of the field `PROGERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROGERRR {
    #[doc = "undocumented"]
    BUSCONF,
    #[doc = "undocumented"]
    NEGSELCONF,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PROGERRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PROGERRR::BUSCONF => 1,
            PROGERRR::NEGSELCONF => 2,
            PROGERRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PROGERRR {
        match value {
            1 => PROGERRR::BUSCONF,
            2 => PROGERRR::NEGSELCONF,
            i => PROGERRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BUSCONF`"]
    #[inline]
    pub fn is_busconf(&self) -> bool {
        *self == PROGERRR::BUSCONF
    }
    #[doc = "Checks if the value of the field is `NEGSELCONF`"]
    #[inline]
    pub fn is_negselconf(&self) -> bool {
        *self == PROGERRR::NEGSELCONF
    }
}
#[doc = r" Value of the field"]
pub struct WARMR {
    bits: bool,
}
impl WARMR {
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
pub struct SINGLEDVR {
    bits: bool,
}
impl SINGLEDVR {
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
pub struct SCANDVR {
    bits: bool,
}
impl SCANDVR {
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
    #[doc = "Bit 0 - Single Channel Conversion Active"]
    #[inline]
    pub fn singleact(&self) -> SINGLEACTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SINGLEACTR { bits }
    }
    #[doc = "Bit 1 - Scan Conversion Active"]
    #[inline]
    pub fn scanact(&self) -> SCANACTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCANACTR { bits }
    }
    #[doc = "Bit 8 - Single Channel Reference Warmed Up"]
    #[inline]
    pub fn singlerefwarm(&self) -> SINGLEREFWARMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SINGLEREFWARMR { bits }
    }
    #[doc = "Bit 9 - Scan Reference Warmed Up"]
    #[inline]
    pub fn scanrefwarm(&self) -> SCANREFWARMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCANREFWARMR { bits }
    }
    #[doc = "Bits 10:11 - Programming Error Status"]
    #[inline]
    pub fn progerr(&self) -> PROGERRR {
        PROGERRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - ADC Warmed Up"]
    #[inline]
    pub fn warm(&self) -> WARMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WARMR { bits }
    }
    #[doc = "Bit 16 - Single Channel Data Valid"]
    #[inline]
    pub fn singledv(&self) -> SINGLEDVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SINGLEDVR { bits }
    }
    #[doc = "Bit 17 - Scan Data Valid"]
    #[inline]
    pub fn scandv(&self) -> SCANDVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCANDVR { bits }
    }
}
