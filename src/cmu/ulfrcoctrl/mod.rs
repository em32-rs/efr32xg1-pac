#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ULFRCOCTRL {
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
#[doc = r" Value of the field"]
pub struct TUNINGR {
    bits: u8,
}
impl TUNINGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "ULFRCO = 1 kHz"]
    _1KHZ,
    #[doc = "ULFRCO = 2 kHz"]
    _2KHZ,
    #[doc = "ULFRCO = 4 kHz"]
    _4KHZ,
    #[doc = "ULFRCO = 32 kHz"]
    _32KHZ,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::_1KHZ => 0,
            MODER::_2KHZ => 1,
            MODER::_4KHZ => 2,
            MODER::_32KHZ => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::_1KHZ,
            1 => MODER::_2KHZ,
            2 => MODER::_4KHZ,
            3 => MODER::_32KHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1KHZ`"]
    #[inline]
    pub fn is_1khz(&self) -> bool {
        *self == MODER::_1KHZ
    }
    #[doc = "Checks if the value of the field is `_2KHZ`"]
    #[inline]
    pub fn is_2khz(&self) -> bool {
        *self == MODER::_2KHZ
    }
    #[doc = "Checks if the value of the field is `_4KHZ`"]
    #[inline]
    pub fn is_4khz(&self) -> bool {
        *self == MODER::_4KHZ
    }
    #[doc = "Checks if the value of the field is `_32KHZ`"]
    #[inline]
    pub fn is_32khz(&self) -> bool {
        *self == MODER::_32KHZ
    }
}
#[doc = r" Value of the field"]
pub struct RESTRIMR {
    bits: u8,
}
impl RESTRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _TUNINGW<'a> {
    w: &'a mut W,
}
impl<'a> _TUNINGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "ULFRCO = 1 kHz"]
    _1KHZ,
    #[doc = "ULFRCO = 2 kHz"]
    _2KHZ,
    #[doc = "ULFRCO = 4 kHz"]
    _4KHZ,
    #[doc = "ULFRCO = 32 kHz"]
    _32KHZ,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::_1KHZ => 0,
            MODEW::_2KHZ => 1,
            MODEW::_4KHZ => 2,
            MODEW::_32KHZ => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ULFRCO = 1 kHz"]
    #[inline]
    pub fn _1khz(self) -> &'a mut W {
        self.variant(MODEW::_1KHZ)
    }
    #[doc = "ULFRCO = 2 kHz"]
    #[inline]
    pub fn _2khz(self) -> &'a mut W {
        self.variant(MODEW::_2KHZ)
    }
    #[doc = "ULFRCO = 4 kHz"]
    #[inline]
    pub fn _4khz(self) -> &'a mut W {
        self.variant(MODEW::_4KHZ)
    }
    #[doc = "ULFRCO = 32 kHz"]
    #[inline]
    pub fn _32khz(self) -> &'a mut W {
        self.variant(MODEW::_32KHZ)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESTRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _RESTRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:5 - ULFRCO TUNING Value"]
    #[inline]
    pub fn tuning(&self) -> TUNINGR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TUNINGR { bits }
    }
    #[doc = "Bits 10:11 - ULFRCO Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - ULFRCO Resistor Trim Value (for Resistor in Bias Circuit; NOT for USE as FREQUENCY CALIBRATION)"]
    #[inline]
    pub fn restrim(&self) -> RESTRIMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESTRIMR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 131104 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - ULFRCO TUNING Value"]
    #[inline]
    pub fn tuning(&mut self) -> _TUNINGW {
        _TUNINGW { w: self }
    }
    #[doc = "Bits 10:11 - ULFRCO Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bits 16:17 - ULFRCO Resistor Trim Value (for Resistor in Bias Circuit; NOT for USE as FREQUENCY CALIBRATION)"]
    #[inline]
    pub fn restrim(&mut self) -> _RESTRIMW {
        _RESTRIMW { w: self }
    }
}
