#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EM4CTRL {
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
pub struct EM4STATER {
    bits: bool,
}
impl EM4STATER {
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
pub struct RETAINLFRCOR {
    bits: bool,
}
impl RETAINLFRCOR {
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
pub struct RETAINLFXOR {
    bits: bool,
}
impl RETAINLFXOR {
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
pub struct RETAINULFRCOR {
    bits: bool,
}
impl RETAINULFRCOR {
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
#[doc = "Possible values of the field `EM4IORETMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM4IORETMODER {
    #[doc = "No Retention: Pads enter reset state when entering EM4"]
    DISABLE,
    #[doc = "Retention through EM4: Pads enter reset state when exiting EM4"]
    EM4EXIT,
    #[doc = "Retention through EM4 and Wakeup: software writes UNLATCH register to remove retention"]
    SWUNLATCH,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EM4IORETMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EM4IORETMODER::DISABLE => 0,
            EM4IORETMODER::EM4EXIT => 1,
            EM4IORETMODER::SWUNLATCH => 2,
            EM4IORETMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EM4IORETMODER {
        match value {
            0 => EM4IORETMODER::DISABLE,
            1 => EM4IORETMODER::EM4EXIT,
            2 => EM4IORETMODER::SWUNLATCH,
            i => EM4IORETMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == EM4IORETMODER::DISABLE
    }
    #[doc = "Checks if the value of the field is `EM4EXIT`"]
    #[inline]
    pub fn is_em4exit(&self) -> bool {
        *self == EM4IORETMODER::EM4EXIT
    }
    #[doc = "Checks if the value of the field is `SWUNLATCH`"]
    #[inline]
    pub fn is_swunlatch(&self) -> bool {
        *self == EM4IORETMODER::SWUNLATCH
    }
}
#[doc = r" Proxy"]
pub struct _EM4STATEW<'a> {
    w: &'a mut W,
}
impl<'a> _EM4STATEW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RETAINLFRCOW<'a> {
    w: &'a mut W,
}
impl<'a> _RETAINLFRCOW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RETAINLFXOW<'a> {
    w: &'a mut W,
}
impl<'a> _RETAINLFXOW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RETAINULFRCOW<'a> {
    w: &'a mut W,
}
impl<'a> _RETAINULFRCOW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EM4IORETMODE`"]
pub enum EM4IORETMODEW {
    #[doc = "No Retention: Pads enter reset state when entering EM4"]
    DISABLE,
    #[doc = "Retention through EM4: Pads enter reset state when exiting EM4"]
    EM4EXIT,
    #[doc = "Retention through EM4 and Wakeup: software writes UNLATCH register to remove retention"]
    SWUNLATCH,
}
impl EM4IORETMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EM4IORETMODEW::DISABLE => 0,
            EM4IORETMODEW::EM4EXIT => 1,
            EM4IORETMODEW::SWUNLATCH => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EM4IORETMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _EM4IORETMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EM4IORETMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No Retention: Pads enter reset state when entering EM4"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(EM4IORETMODEW::DISABLE)
    }
    #[doc = "Retention through EM4: Pads enter reset state when exiting EM4"]
    #[inline]
    pub fn em4exit(self) -> &'a mut W {
        self.variant(EM4IORETMODEW::EM4EXIT)
    }
    #[doc = "Retention through EM4 and Wakeup: software writes UNLATCH register to remove retention"]
    #[inline]
    pub fn swunlatch(self) -> &'a mut W {
        self.variant(EM4IORETMODEW::SWUNLATCH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EM4ENTRYW<'a> {
    w: &'a mut W,
}
impl<'a> _EM4ENTRYW<'a> {
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
    #[doc = "Bit 0 - Energy Mode 4 State"]
    #[inline]
    pub fn em4state(&self) -> EM4STATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EM4STATER { bits }
    }
    #[doc = "Bit 1 - LFRCO Retain During EM4"]
    #[inline]
    pub fn retainlfrco(&self) -> RETAINLFRCOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RETAINLFRCOR { bits }
    }
    #[doc = "Bit 2 - LFXO Retain During EM4"]
    #[inline]
    pub fn retainlfxo(&self) -> RETAINLFXOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RETAINLFXOR { bits }
    }
    #[doc = "Bit 3 - ULFRCO Retain During EM4S"]
    #[inline]
    pub fn retainulfrco(&self) -> RETAINULFRCOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RETAINULFRCOR { bits }
    }
    #[doc = "Bits 4:5 - EM4 IO Retention Disable"]
    #[inline]
    pub fn em4ioretmode(&self) -> EM4IORETMODER {
        EM4IORETMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Energy Mode 4 State"]
    #[inline]
    pub fn em4state(&mut self) -> _EM4STATEW {
        _EM4STATEW { w: self }
    }
    #[doc = "Bit 1 - LFRCO Retain During EM4"]
    #[inline]
    pub fn retainlfrco(&mut self) -> _RETAINLFRCOW {
        _RETAINLFRCOW { w: self }
    }
    #[doc = "Bit 2 - LFXO Retain During EM4"]
    #[inline]
    pub fn retainlfxo(&mut self) -> _RETAINLFXOW {
        _RETAINLFXOW { w: self }
    }
    #[doc = "Bit 3 - ULFRCO Retain During EM4S"]
    #[inline]
    pub fn retainulfrco(&mut self) -> _RETAINULFRCOW {
        _RETAINULFRCOW { w: self }
    }
    #[doc = "Bits 4:5 - EM4 IO Retention Disable"]
    #[inline]
    pub fn em4ioretmode(&mut self) -> _EM4IORETMODEW {
        _EM4IORETMODEW { w: self }
    }
    #[doc = "Bits 16:17 - Energy Mode 4 Entry"]
    #[inline]
    pub fn em4entry(&mut self) -> _EM4ENTRYW {
        _EM4ENTRYW { w: self }
    }
}
