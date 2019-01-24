#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HFXOTIMEOUTCTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `STARTUPTIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTUPTIMEOUTR {
    #[doc = "Timeout period of 2 cycles"]
    _2CYCLES,
    #[doc = "Timeout period of 4 cycles"]
    _4CYCLES,
    #[doc = "Timeout period of 16 cycles"]
    _16CYCLES,
    #[doc = "Timeout period of 32 cycles"]
    _32CYCLES,
    #[doc = "Timeout period of 256 cycles"]
    _256CYCLES,
    #[doc = "Timeout period of 1024 cycles"]
    _1KCYCLES,
    #[doc = "Timeout period of 2048 cycles"]
    _2KCYCLES,
    #[doc = "Timeout period of 4096 cycles"]
    _4KCYCLES,
    #[doc = "Timeout period of 8192 cycles"]
    _8KCYCLES,
    #[doc = "Timeout period of 16384 cycles"]
    _16KCYCLES,
    #[doc = "Timeout period of 32768 cycles"]
    _32KCYCLES,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STARTUPTIMEOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STARTUPTIMEOUTR::_2CYCLES => 0,
            STARTUPTIMEOUTR::_4CYCLES => 1,
            STARTUPTIMEOUTR::_16CYCLES => 2,
            STARTUPTIMEOUTR::_32CYCLES => 3,
            STARTUPTIMEOUTR::_256CYCLES => 4,
            STARTUPTIMEOUTR::_1KCYCLES => 5,
            STARTUPTIMEOUTR::_2KCYCLES => 6,
            STARTUPTIMEOUTR::_4KCYCLES => 7,
            STARTUPTIMEOUTR::_8KCYCLES => 8,
            STARTUPTIMEOUTR::_16KCYCLES => 9,
            STARTUPTIMEOUTR::_32KCYCLES => 10,
            STARTUPTIMEOUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STARTUPTIMEOUTR {
        match value {
            0 => STARTUPTIMEOUTR::_2CYCLES,
            1 => STARTUPTIMEOUTR::_4CYCLES,
            2 => STARTUPTIMEOUTR::_16CYCLES,
            3 => STARTUPTIMEOUTR::_32CYCLES,
            4 => STARTUPTIMEOUTR::_256CYCLES,
            5 => STARTUPTIMEOUTR::_1KCYCLES,
            6 => STARTUPTIMEOUTR::_2KCYCLES,
            7 => STARTUPTIMEOUTR::_4KCYCLES,
            8 => STARTUPTIMEOUTR::_8KCYCLES,
            9 => STARTUPTIMEOUTR::_16KCYCLES,
            10 => STARTUPTIMEOUTR::_32KCYCLES,
            i => STARTUPTIMEOUTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline]
    pub fn is_2cycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline]
    pub fn is_4cycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline]
    pub fn is_16cycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline]
    pub fn is_32cycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline]
    pub fn is_256cycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_256CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline]
    pub fn is_1kcycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_2KCYCLES`"]
    #[inline]
    pub fn is_2kcycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_2KCYCLES
    }
    #[doc = "Checks if the value of the field is `_4KCYCLES`"]
    #[inline]
    pub fn is_4kcycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_4KCYCLES
    }
    #[doc = "Checks if the value of the field is `_8KCYCLES`"]
    #[inline]
    pub fn is_8kcycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_8KCYCLES
    }
    #[doc = "Checks if the value of the field is `_16KCYCLES`"]
    #[inline]
    pub fn is_16kcycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_16KCYCLES
    }
    #[doc = "Checks if the value of the field is `_32KCYCLES`"]
    #[inline]
    pub fn is_32kcycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_32KCYCLES
    }
}
#[doc = "Possible values of the field `STEADYTIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STEADYTIMEOUTR {
    #[doc = "Timeout period of 2 cycles"]
    _2CYCLES,
    #[doc = "Timeout period of 4 cycles"]
    _4CYCLES,
    #[doc = "Timeout period of 16 cycles"]
    _16CYCLES,
    #[doc = "Timeout period of 32 cycles"]
    _32CYCLES,
    #[doc = "Timeout period of 256 cycles"]
    _256CYCLES,
    #[doc = "Timeout period of 1024 cycles"]
    _1KCYCLES,
    #[doc = "Timeout period of 2048 cycles"]
    _2KCYCLES,
    #[doc = "Timeout period of 4096 cycles"]
    _4KCYCLES,
    #[doc = "Timeout period of 8192 cycles"]
    _8KCYCLES,
    #[doc = "Timeout period of 16384 cycles"]
    _16KCYCLES,
    #[doc = "Timeout period of 32768 cycles"]
    _32KCYCLES,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STEADYTIMEOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STEADYTIMEOUTR::_2CYCLES => 0,
            STEADYTIMEOUTR::_4CYCLES => 1,
            STEADYTIMEOUTR::_16CYCLES => 2,
            STEADYTIMEOUTR::_32CYCLES => 3,
            STEADYTIMEOUTR::_256CYCLES => 4,
            STEADYTIMEOUTR::_1KCYCLES => 5,
            STEADYTIMEOUTR::_2KCYCLES => 6,
            STEADYTIMEOUTR::_4KCYCLES => 7,
            STEADYTIMEOUTR::_8KCYCLES => 8,
            STEADYTIMEOUTR::_16KCYCLES => 9,
            STEADYTIMEOUTR::_32KCYCLES => 10,
            STEADYTIMEOUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STEADYTIMEOUTR {
        match value {
            0 => STEADYTIMEOUTR::_2CYCLES,
            1 => STEADYTIMEOUTR::_4CYCLES,
            2 => STEADYTIMEOUTR::_16CYCLES,
            3 => STEADYTIMEOUTR::_32CYCLES,
            4 => STEADYTIMEOUTR::_256CYCLES,
            5 => STEADYTIMEOUTR::_1KCYCLES,
            6 => STEADYTIMEOUTR::_2KCYCLES,
            7 => STEADYTIMEOUTR::_4KCYCLES,
            8 => STEADYTIMEOUTR::_8KCYCLES,
            9 => STEADYTIMEOUTR::_16KCYCLES,
            10 => STEADYTIMEOUTR::_32KCYCLES,
            i => STEADYTIMEOUTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline]
    pub fn is_2cycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline]
    pub fn is_4cycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline]
    pub fn is_16cycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline]
    pub fn is_32cycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline]
    pub fn is_256cycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_256CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline]
    pub fn is_1kcycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_2KCYCLES`"]
    #[inline]
    pub fn is_2kcycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_2KCYCLES
    }
    #[doc = "Checks if the value of the field is `_4KCYCLES`"]
    #[inline]
    pub fn is_4kcycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_4KCYCLES
    }
    #[doc = "Checks if the value of the field is `_8KCYCLES`"]
    #[inline]
    pub fn is_8kcycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_8KCYCLES
    }
    #[doc = "Checks if the value of the field is `_16KCYCLES`"]
    #[inline]
    pub fn is_16kcycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_16KCYCLES
    }
    #[doc = "Checks if the value of the field is `_32KCYCLES`"]
    #[inline]
    pub fn is_32kcycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_32KCYCLES
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED2R {
    bits: u8,
}
impl RESERVED2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PEAKDETTIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEAKDETTIMEOUTR {
    #[doc = "Timeout period of 2 cycles"]
    _2CYCLES,
    #[doc = "Timeout period of 4 cycles"]
    _4CYCLES,
    #[doc = "Timeout period of 16 cycles"]
    _16CYCLES,
    #[doc = "Timeout period of 32 cycles"]
    _32CYCLES,
    #[doc = "Timeout period of 256 cycles"]
    _256CYCLES,
    #[doc = "Timeout period of 1024 cycles"]
    _1KCYCLES,
    #[doc = "Timeout period of 2048 cycles"]
    _2KCYCLES,
    #[doc = "Timeout period of 4096 cycles"]
    _4KCYCLES,
    #[doc = "Timeout period of 8192 cycles"]
    _8KCYCLES,
    #[doc = "Timeout period of 16384 cycles"]
    _16KCYCLES,
    #[doc = "Timeout period of 32768 cycles"]
    _32KCYCLES,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PEAKDETTIMEOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PEAKDETTIMEOUTR::_2CYCLES => 0,
            PEAKDETTIMEOUTR::_4CYCLES => 1,
            PEAKDETTIMEOUTR::_16CYCLES => 2,
            PEAKDETTIMEOUTR::_32CYCLES => 3,
            PEAKDETTIMEOUTR::_256CYCLES => 4,
            PEAKDETTIMEOUTR::_1KCYCLES => 5,
            PEAKDETTIMEOUTR::_2KCYCLES => 6,
            PEAKDETTIMEOUTR::_4KCYCLES => 7,
            PEAKDETTIMEOUTR::_8KCYCLES => 8,
            PEAKDETTIMEOUTR::_16KCYCLES => 9,
            PEAKDETTIMEOUTR::_32KCYCLES => 10,
            PEAKDETTIMEOUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PEAKDETTIMEOUTR {
        match value {
            0 => PEAKDETTIMEOUTR::_2CYCLES,
            1 => PEAKDETTIMEOUTR::_4CYCLES,
            2 => PEAKDETTIMEOUTR::_16CYCLES,
            3 => PEAKDETTIMEOUTR::_32CYCLES,
            4 => PEAKDETTIMEOUTR::_256CYCLES,
            5 => PEAKDETTIMEOUTR::_1KCYCLES,
            6 => PEAKDETTIMEOUTR::_2KCYCLES,
            7 => PEAKDETTIMEOUTR::_4KCYCLES,
            8 => PEAKDETTIMEOUTR::_8KCYCLES,
            9 => PEAKDETTIMEOUTR::_16KCYCLES,
            10 => PEAKDETTIMEOUTR::_32KCYCLES,
            i => PEAKDETTIMEOUTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline]
    pub fn is_2cycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline]
    pub fn is_4cycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline]
    pub fn is_16cycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline]
    pub fn is_32cycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline]
    pub fn is_256cycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_256CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline]
    pub fn is_1kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_2KCYCLES`"]
    #[inline]
    pub fn is_2kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_2KCYCLES
    }
    #[doc = "Checks if the value of the field is `_4KCYCLES`"]
    #[inline]
    pub fn is_4kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_4KCYCLES
    }
    #[doc = "Checks if the value of the field is `_8KCYCLES`"]
    #[inline]
    pub fn is_8kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_8KCYCLES
    }
    #[doc = "Checks if the value of the field is `_16KCYCLES`"]
    #[inline]
    pub fn is_16kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_16KCYCLES
    }
    #[doc = "Checks if the value of the field is `_32KCYCLES`"]
    #[inline]
    pub fn is_32kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_32KCYCLES
    }
}
#[doc = "Possible values of the field `SHUNTOPTTIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHUNTOPTTIMEOUTR {
    #[doc = "Timeout period of 2 cycles"]
    _2CYCLES,
    #[doc = "Timeout period of 4 cycles"]
    _4CYCLES,
    #[doc = "Timeout period of 16 cycles"]
    _16CYCLES,
    #[doc = "Timeout period of 32 cycles"]
    _32CYCLES,
    #[doc = "Timeout period of 256 cycles"]
    _256CYCLES,
    #[doc = "Timeout period of 1024 cycles"]
    _1KCYCLES,
    #[doc = "Timeout period of 2048 cycles"]
    _2KCYCLES,
    #[doc = "Timeout period of 4096 cycles"]
    _4KCYCLES,
    #[doc = "Timeout period of 8192 cycles"]
    _8KCYCLES,
    #[doc = "Timeout period of 16384 cycles"]
    _16KCYCLES,
    #[doc = "Timeout period of 32768 cycles"]
    _32KCYCLES,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SHUNTOPTTIMEOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SHUNTOPTTIMEOUTR::_2CYCLES => 0,
            SHUNTOPTTIMEOUTR::_4CYCLES => 1,
            SHUNTOPTTIMEOUTR::_16CYCLES => 2,
            SHUNTOPTTIMEOUTR::_32CYCLES => 3,
            SHUNTOPTTIMEOUTR::_256CYCLES => 4,
            SHUNTOPTTIMEOUTR::_1KCYCLES => 5,
            SHUNTOPTTIMEOUTR::_2KCYCLES => 6,
            SHUNTOPTTIMEOUTR::_4KCYCLES => 7,
            SHUNTOPTTIMEOUTR::_8KCYCLES => 8,
            SHUNTOPTTIMEOUTR::_16KCYCLES => 9,
            SHUNTOPTTIMEOUTR::_32KCYCLES => 10,
            SHUNTOPTTIMEOUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SHUNTOPTTIMEOUTR {
        match value {
            0 => SHUNTOPTTIMEOUTR::_2CYCLES,
            1 => SHUNTOPTTIMEOUTR::_4CYCLES,
            2 => SHUNTOPTTIMEOUTR::_16CYCLES,
            3 => SHUNTOPTTIMEOUTR::_32CYCLES,
            4 => SHUNTOPTTIMEOUTR::_256CYCLES,
            5 => SHUNTOPTTIMEOUTR::_1KCYCLES,
            6 => SHUNTOPTTIMEOUTR::_2KCYCLES,
            7 => SHUNTOPTTIMEOUTR::_4KCYCLES,
            8 => SHUNTOPTTIMEOUTR::_8KCYCLES,
            9 => SHUNTOPTTIMEOUTR::_16KCYCLES,
            10 => SHUNTOPTTIMEOUTR::_32KCYCLES,
            i => SHUNTOPTTIMEOUTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline]
    pub fn is_2cycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUTR::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline]
    pub fn is_4cycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUTR::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline]
    pub fn is_16cycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUTR::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline]
    pub fn is_32cycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUTR::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline]
    pub fn is_256cycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUTR::_256CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline]
    pub fn is_1kcycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUTR::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_2KCYCLES`"]
    #[inline]
    pub fn is_2kcycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUTR::_2KCYCLES
    }
    #[doc = "Checks if the value of the field is `_4KCYCLES`"]
    #[inline]
    pub fn is_4kcycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUTR::_4KCYCLES
    }
    #[doc = "Checks if the value of the field is `_8KCYCLES`"]
    #[inline]
    pub fn is_8kcycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUTR::_8KCYCLES
    }
    #[doc = "Checks if the value of the field is `_16KCYCLES`"]
    #[inline]
    pub fn is_16kcycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUTR::_16KCYCLES
    }
    #[doc = "Checks if the value of the field is `_32KCYCLES`"]
    #[inline]
    pub fn is_32kcycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUTR::_32KCYCLES
    }
}
#[doc = "Values that can be written to the field `STARTUPTIMEOUT`"]
pub enum STARTUPTIMEOUTW {
    #[doc = "Timeout period of 2 cycles"]
    _2CYCLES,
    #[doc = "Timeout period of 4 cycles"]
    _4CYCLES,
    #[doc = "Timeout period of 16 cycles"]
    _16CYCLES,
    #[doc = "Timeout period of 32 cycles"]
    _32CYCLES,
    #[doc = "Timeout period of 256 cycles"]
    _256CYCLES,
    #[doc = "Timeout period of 1024 cycles"]
    _1KCYCLES,
    #[doc = "Timeout period of 2048 cycles"]
    _2KCYCLES,
    #[doc = "Timeout period of 4096 cycles"]
    _4KCYCLES,
    #[doc = "Timeout period of 8192 cycles"]
    _8KCYCLES,
    #[doc = "Timeout period of 16384 cycles"]
    _16KCYCLES,
    #[doc = "Timeout period of 32768 cycles"]
    _32KCYCLES,
}
impl STARTUPTIMEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STARTUPTIMEOUTW::_2CYCLES => 0,
            STARTUPTIMEOUTW::_4CYCLES => 1,
            STARTUPTIMEOUTW::_16CYCLES => 2,
            STARTUPTIMEOUTW::_32CYCLES => 3,
            STARTUPTIMEOUTW::_256CYCLES => 4,
            STARTUPTIMEOUTW::_1KCYCLES => 5,
            STARTUPTIMEOUTW::_2KCYCLES => 6,
            STARTUPTIMEOUTW::_4KCYCLES => 7,
            STARTUPTIMEOUTW::_8KCYCLES => 8,
            STARTUPTIMEOUTW::_16KCYCLES => 9,
            STARTUPTIMEOUTW::_32KCYCLES => 10,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STARTUPTIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTUPTIMEOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTUPTIMEOUTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_2CYCLES)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_4CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_32CYCLES)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_256CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_1KCYCLES)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline]
    pub fn _2kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_2KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline]
    pub fn _4kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_4KCYCLES)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline]
    pub fn _8kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_8KCYCLES)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline]
    pub fn _16kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_16KCYCLES)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline]
    pub fn _32kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_32KCYCLES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STEADYTIMEOUT`"]
pub enum STEADYTIMEOUTW {
    #[doc = "Timeout period of 2 cycles"]
    _2CYCLES,
    #[doc = "Timeout period of 4 cycles"]
    _4CYCLES,
    #[doc = "Timeout period of 16 cycles"]
    _16CYCLES,
    #[doc = "Timeout period of 32 cycles"]
    _32CYCLES,
    #[doc = "Timeout period of 256 cycles"]
    _256CYCLES,
    #[doc = "Timeout period of 1024 cycles"]
    _1KCYCLES,
    #[doc = "Timeout period of 2048 cycles"]
    _2KCYCLES,
    #[doc = "Timeout period of 4096 cycles"]
    _4KCYCLES,
    #[doc = "Timeout period of 8192 cycles"]
    _8KCYCLES,
    #[doc = "Timeout period of 16384 cycles"]
    _16KCYCLES,
    #[doc = "Timeout period of 32768 cycles"]
    _32KCYCLES,
}
impl STEADYTIMEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STEADYTIMEOUTW::_2CYCLES => 0,
            STEADYTIMEOUTW::_4CYCLES => 1,
            STEADYTIMEOUTW::_16CYCLES => 2,
            STEADYTIMEOUTW::_32CYCLES => 3,
            STEADYTIMEOUTW::_256CYCLES => 4,
            STEADYTIMEOUTW::_1KCYCLES => 5,
            STEADYTIMEOUTW::_2KCYCLES => 6,
            STEADYTIMEOUTW::_4KCYCLES => 7,
            STEADYTIMEOUTW::_8KCYCLES => 8,
            STEADYTIMEOUTW::_16KCYCLES => 9,
            STEADYTIMEOUTW::_32KCYCLES => 10,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STEADYTIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _STEADYTIMEOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STEADYTIMEOUTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_2CYCLES)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_4CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_32CYCLES)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_256CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_1KCYCLES)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline]
    pub fn _2kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_2KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline]
    pub fn _4kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_4KCYCLES)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline]
    pub fn _8kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_8KCYCLES)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline]
    pub fn _16kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_16KCYCLES)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline]
    pub fn _32kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_32KCYCLES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED2W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PEAKDETTIMEOUT`"]
pub enum PEAKDETTIMEOUTW {
    #[doc = "Timeout period of 2 cycles"]
    _2CYCLES,
    #[doc = "Timeout period of 4 cycles"]
    _4CYCLES,
    #[doc = "Timeout period of 16 cycles"]
    _16CYCLES,
    #[doc = "Timeout period of 32 cycles"]
    _32CYCLES,
    #[doc = "Timeout period of 256 cycles"]
    _256CYCLES,
    #[doc = "Timeout period of 1024 cycles"]
    _1KCYCLES,
    #[doc = "Timeout period of 2048 cycles"]
    _2KCYCLES,
    #[doc = "Timeout period of 4096 cycles"]
    _4KCYCLES,
    #[doc = "Timeout period of 8192 cycles"]
    _8KCYCLES,
    #[doc = "Timeout period of 16384 cycles"]
    _16KCYCLES,
    #[doc = "Timeout period of 32768 cycles"]
    _32KCYCLES,
}
impl PEAKDETTIMEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PEAKDETTIMEOUTW::_2CYCLES => 0,
            PEAKDETTIMEOUTW::_4CYCLES => 1,
            PEAKDETTIMEOUTW::_16CYCLES => 2,
            PEAKDETTIMEOUTW::_32CYCLES => 3,
            PEAKDETTIMEOUTW::_256CYCLES => 4,
            PEAKDETTIMEOUTW::_1KCYCLES => 5,
            PEAKDETTIMEOUTW::_2KCYCLES => 6,
            PEAKDETTIMEOUTW::_4KCYCLES => 7,
            PEAKDETTIMEOUTW::_8KCYCLES => 8,
            PEAKDETTIMEOUTW::_16KCYCLES => 9,
            PEAKDETTIMEOUTW::_32KCYCLES => 10,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEAKDETTIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _PEAKDETTIMEOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEAKDETTIMEOUTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_2CYCLES)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_4CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_32CYCLES)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_256CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_1KCYCLES)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline]
    pub fn _2kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_2KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline]
    pub fn _4kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_4KCYCLES)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline]
    pub fn _8kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_8KCYCLES)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline]
    pub fn _16kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_16KCYCLES)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline]
    pub fn _32kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_32KCYCLES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SHUNTOPTTIMEOUT`"]
pub enum SHUNTOPTTIMEOUTW {
    #[doc = "Timeout period of 2 cycles"]
    _2CYCLES,
    #[doc = "Timeout period of 4 cycles"]
    _4CYCLES,
    #[doc = "Timeout period of 16 cycles"]
    _16CYCLES,
    #[doc = "Timeout period of 32 cycles"]
    _32CYCLES,
    #[doc = "Timeout period of 256 cycles"]
    _256CYCLES,
    #[doc = "Timeout period of 1024 cycles"]
    _1KCYCLES,
    #[doc = "Timeout period of 2048 cycles"]
    _2KCYCLES,
    #[doc = "Timeout period of 4096 cycles"]
    _4KCYCLES,
    #[doc = "Timeout period of 8192 cycles"]
    _8KCYCLES,
    #[doc = "Timeout period of 16384 cycles"]
    _16KCYCLES,
    #[doc = "Timeout period of 32768 cycles"]
    _32KCYCLES,
}
impl SHUNTOPTTIMEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SHUNTOPTTIMEOUTW::_2CYCLES => 0,
            SHUNTOPTTIMEOUTW::_4CYCLES => 1,
            SHUNTOPTTIMEOUTW::_16CYCLES => 2,
            SHUNTOPTTIMEOUTW::_32CYCLES => 3,
            SHUNTOPTTIMEOUTW::_256CYCLES => 4,
            SHUNTOPTTIMEOUTW::_1KCYCLES => 5,
            SHUNTOPTTIMEOUTW::_2KCYCLES => 6,
            SHUNTOPTTIMEOUTW::_4KCYCLES => 7,
            SHUNTOPTTIMEOUTW::_8KCYCLES => 8,
            SHUNTOPTTIMEOUTW::_16KCYCLES => 9,
            SHUNTOPTTIMEOUTW::_32KCYCLES => 10,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SHUNTOPTTIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _SHUNTOPTTIMEOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SHUNTOPTTIMEOUTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUTW::_2CYCLES)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUTW::_4CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUTW::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUTW::_32CYCLES)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUTW::_256CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUTW::_1KCYCLES)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline]
    pub fn _2kcycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUTW::_2KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline]
    pub fn _4kcycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUTW::_4KCYCLES)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline]
    pub fn _8kcycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUTW::_8KCYCLES)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline]
    pub fn _16kcycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUTW::_16KCYCLES)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline]
    pub fn _32kcycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUTW::_32KCYCLES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Wait Duration in HFXO Startup Enable Wait State"]
    #[inline]
    pub fn startuptimeout(&self) -> STARTUPTIMEOUTR {
        STARTUPTIMEOUTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Wait Duration in HFXO Startup Steady Wait State"]
    #[inline]
    pub fn steadytimeout(&self) -> STEADYTIMEOUTR {
        STEADYTIMEOUTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Wait Duration in HFXO Warm Startup Steady Wait State"]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED2R { bits }
    }
    #[doc = "Bits 12:15 - Wait Duration in HFXO Peak Detection Wait State"]
    #[inline]
    pub fn peakdettimeout(&self) -> PEAKDETTIMEOUTR {
        PEAKDETTIMEOUTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Wait Duration in HFXO Shunt Current Optimization Wait State"]
    #[inline]
    pub fn shuntopttimeout(&self) -> SHUNTOPTTIMEOUTR {
        SHUNTOPTTIMEOUTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 157287 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Wait Duration in HFXO Startup Enable Wait State"]
    #[inline]
    pub fn startuptimeout(&mut self) -> _STARTUPTIMEOUTW {
        _STARTUPTIMEOUTW { w: self }
    }
    #[doc = "Bits 4:7 - Wait Duration in HFXO Startup Steady Wait State"]
    #[inline]
    pub fn steadytimeout(&mut self) -> _STEADYTIMEOUTW {
        _STEADYTIMEOUTW { w: self }
    }
    #[doc = "Bits 8:11 - Wait Duration in HFXO Warm Startup Steady Wait State"]
    #[inline]
    pub fn reserved2(&mut self) -> _RESERVED2W {
        _RESERVED2W { w: self }
    }
    #[doc = "Bits 12:15 - Wait Duration in HFXO Peak Detection Wait State"]
    #[inline]
    pub fn peakdettimeout(&mut self) -> _PEAKDETTIMEOUTW {
        _PEAKDETTIMEOUTW { w: self }
    }
    #[doc = "Bits 16:19 - Wait Duration in HFXO Shunt Current Optimization Wait State"]
    #[inline]
    pub fn shuntopttimeout(&mut self) -> _SHUNTOPTTIMEOUTW {
        _SHUNTOPTTIMEOUTW { w: self }
    }
}
