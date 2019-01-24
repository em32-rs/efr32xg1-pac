#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LFRCOCTRL {
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
    bits: u16,
}
impl TUNINGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ENVREFR {
    bits: bool,
}
impl ENVREFR {
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
pub struct ENCHOPR {
    bits: bool,
}
impl ENCHOPR {
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
pub struct ENDEMR {
    bits: bool,
}
impl ENDEMR {
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
#[doc = "Possible values of the field `TIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUTR {
    #[doc = "Timeout period of 2 cycles"]
    _2CYCLES,
    #[doc = "Timeout period of 16 cycles"]
    _16CYCLES,
    #[doc = "Timeout period of 32 cycles"]
    _32CYCLES,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TIMEOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMEOUTR::_2CYCLES => 0,
            TIMEOUTR::_16CYCLES => 1,
            TIMEOUTR::_32CYCLES => 2,
            TIMEOUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMEOUTR {
        match value {
            0 => TIMEOUTR::_2CYCLES,
            1 => TIMEOUTR::_16CYCLES,
            2 => TIMEOUTR::_32CYCLES,
            i => TIMEOUTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline]
    pub fn is_2cycles(&self) -> bool {
        *self == TIMEOUTR::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline]
    pub fn is_16cycles(&self) -> bool {
        *self == TIMEOUTR::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline]
    pub fn is_32cycles(&self) -> bool {
        *self == TIMEOUTR::_32CYCLES
    }
}
#[doc = r" Value of the field"]
pub struct GMCCURTUNER {
    bits: u8,
}
impl GMCCURTUNER {
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
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENVREFW<'a> {
    w: &'a mut W,
}
impl<'a> _ENVREFW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENCHOPW<'a> {
    w: &'a mut W,
}
impl<'a> _ENCHOPW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENDEMW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDEMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIMEOUT`"]
pub enum TIMEOUTW {
    #[doc = "Timeout period of 2 cycles"]
    _2CYCLES,
    #[doc = "Timeout period of 16 cycles"]
    _16CYCLES,
    #[doc = "Timeout period of 32 cycles"]
    _32CYCLES,
}
impl TIMEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMEOUTW::_2CYCLES => 0,
            TIMEOUTW::_16CYCLES => 1,
            TIMEOUTW::_32CYCLES => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMEOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMEOUTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(TIMEOUTW::_2CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(TIMEOUTW::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(TIMEOUTW::_32CYCLES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GMCCURTUNEW<'a> {
    w: &'a mut W,
}
impl<'a> _GMCCURTUNEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:8 - LFRCO Tuning Value"]
    #[inline]
    pub fn tuning(&self) -> TUNINGR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TUNINGR { bits }
    }
    #[doc = "Bit 16 - Enable Duty Cycling of Vref"]
    #[inline]
    pub fn envref(&self) -> ENVREFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENVREFR { bits }
    }
    #[doc = "Bit 17 - Enable Comparator Chopping"]
    #[inline]
    pub fn enchop(&self) -> ENCHOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENCHOPR { bits }
    }
    #[doc = "Bit 18 - Enable Dynamic Element Matching"]
    #[inline]
    pub fn endem(&self) -> ENDEMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENDEMR { bits }
    }
    #[doc = "Bits 24:25 - LFRCO Timeout"]
    #[inline]
    pub fn timeout(&self) -> TIMEOUTR {
        TIMEOUTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Tuning of Gmc Current"]
    #[inline]
    pub fn gmccurtune(&self) -> GMCCURTUNER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GMCCURTUNER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2164654336 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:8 - LFRCO Tuning Value"]
    #[inline]
    pub fn tuning(&mut self) -> _TUNINGW {
        _TUNINGW { w: self }
    }
    #[doc = "Bit 16 - Enable Duty Cycling of Vref"]
    #[inline]
    pub fn envref(&mut self) -> _ENVREFW {
        _ENVREFW { w: self }
    }
    #[doc = "Bit 17 - Enable Comparator Chopping"]
    #[inline]
    pub fn enchop(&mut self) -> _ENCHOPW {
        _ENCHOPW { w: self }
    }
    #[doc = "Bit 18 - Enable Dynamic Element Matching"]
    #[inline]
    pub fn endem(&mut self) -> _ENDEMW {
        _ENDEMW { w: self }
    }
    #[doc = "Bits 24:25 - LFRCO Timeout"]
    #[inline]
    pub fn timeout(&mut self) -> _TIMEOUTW {
        _TIMEOUTW { w: self }
    }
    #[doc = "Bits 28:31 - Tuning of Gmc Current"]
    #[inline]
    pub fn gmccurtune(&mut self) -> _GMCCURTUNEW {
        _GMCCURTUNEW { w: self }
    }
}
