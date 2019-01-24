#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CH4_CFG {
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
#[doc = "Possible values of the field `ARBSLOTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBSLOTSR {
    #[doc = "One arbitration slot selected"]
    ONE,
    #[doc = "Two arbitration slots selected"]
    TWO,
    #[doc = "Four arbitration slots selected"]
    FOUR,
    #[doc = "Eight arbitration slots selected"]
    EIGHT,
}
impl ARBSLOTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ARBSLOTSR::ONE => 0,
            ARBSLOTSR::TWO => 1,
            ARBSLOTSR::FOUR => 2,
            ARBSLOTSR::EIGHT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ARBSLOTSR {
        match value {
            0 => ARBSLOTSR::ONE,
            1 => ARBSLOTSR::TWO,
            2 => ARBSLOTSR::FOUR,
            3 => ARBSLOTSR::EIGHT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == ARBSLOTSR::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline]
    pub fn is_two(&self) -> bool {
        *self == ARBSLOTSR::TWO
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline]
    pub fn is_four(&self) -> bool {
        *self == ARBSLOTSR::FOUR
    }
    #[doc = "Checks if the value of the field is `EIGHT`"]
    #[inline]
    pub fn is_eight(&self) -> bool {
        *self == ARBSLOTSR::EIGHT
    }
}
#[doc = r" Value of the field"]
pub struct SRCINCSIGNR {
    bits: bool,
}
impl SRCINCSIGNR {
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
pub struct DSTINCSIGNR {
    bits: bool,
}
impl DSTINCSIGNR {
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
#[doc = "Values that can be written to the field `ARBSLOTS`"]
pub enum ARBSLOTSW {
    #[doc = "One arbitration slot selected"]
    ONE,
    #[doc = "Two arbitration slots selected"]
    TWO,
    #[doc = "Four arbitration slots selected"]
    FOUR,
    #[doc = "Eight arbitration slots selected"]
    EIGHT,
}
impl ARBSLOTSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ARBSLOTSW::ONE => 0,
            ARBSLOTSW::TWO => 1,
            ARBSLOTSW::FOUR => 2,
            ARBSLOTSW::EIGHT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARBSLOTSW<'a> {
    w: &'a mut W,
}
impl<'a> _ARBSLOTSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARBSLOTSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "One arbitration slot selected"]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(ARBSLOTSW::ONE)
    }
    #[doc = "Two arbitration slots selected"]
    #[inline]
    pub fn two(self) -> &'a mut W {
        self.variant(ARBSLOTSW::TWO)
    }
    #[doc = "Four arbitration slots selected"]
    #[inline]
    pub fn four(self) -> &'a mut W {
        self.variant(ARBSLOTSW::FOUR)
    }
    #[doc = "Eight arbitration slots selected"]
    #[inline]
    pub fn eight(self) -> &'a mut W {
        self.variant(ARBSLOTSW::EIGHT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SRCINCSIGNW<'a> {
    w: &'a mut W,
}
impl<'a> _SRCINCSIGNW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DSTINCSIGNW<'a> {
    w: &'a mut W,
}
impl<'a> _DSTINCSIGNW<'a> {
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
        const OFFSET: u8 = 21;
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
    #[doc = "Bits 16:17 - Arbitration Slot Number Select"]
    #[inline]
    pub fn arbslots(&self) -> ARBSLOTSR {
        ARBSLOTSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Source Address Increment Sign"]
    #[inline]
    pub fn srcincsign(&self) -> SRCINCSIGNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SRCINCSIGNR { bits }
    }
    #[doc = "Bit 21 - Destination Address Increment Sign"]
    #[inline]
    pub fn dstincsign(&self) -> DSTINCSIGNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DSTINCSIGNR { bits }
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
    #[doc = "Bits 16:17 - Arbitration Slot Number Select"]
    #[inline]
    pub fn arbslots(&mut self) -> _ARBSLOTSW {
        _ARBSLOTSW { w: self }
    }
    #[doc = "Bit 20 - Source Address Increment Sign"]
    #[inline]
    pub fn srcincsign(&mut self) -> _SRCINCSIGNW {
        _SRCINCSIGNW { w: self }
    }
    #[doc = "Bit 21 - Destination Address Increment Sign"]
    #[inline]
    pub fn dstincsign(&mut self) -> _DSTINCSIGNW {
        _DSTINCSIGNW { w: self }
    }
}
