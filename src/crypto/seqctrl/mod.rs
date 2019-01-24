#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SEQCTRL {
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
pub struct LENGTHAR {
    bits: u16,
}
impl LENGTHAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `BLOCKSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLOCKSIZER {
    #[doc = "A block is 16 bytes long"]
    _16BYTES,
    #[doc = "A block is 32 bytes long"]
    _32BYTES,
    #[doc = "A block is 64 bytes long"]
    _64BYTES,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BLOCKSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BLOCKSIZER::_16BYTES => 0,
            BLOCKSIZER::_32BYTES => 1,
            BLOCKSIZER::_64BYTES => 2,
            BLOCKSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BLOCKSIZER {
        match value {
            0 => BLOCKSIZER::_16BYTES,
            1 => BLOCKSIZER::_32BYTES,
            2 => BLOCKSIZER::_64BYTES,
            i => BLOCKSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_16BYTES`"]
    #[inline]
    pub fn is_16bytes(&self) -> bool {
        *self == BLOCKSIZER::_16BYTES
    }
    #[doc = "Checks if the value of the field is `_32BYTES`"]
    #[inline]
    pub fn is_32bytes(&self) -> bool {
        *self == BLOCKSIZER::_32BYTES
    }
    #[doc = "Checks if the value of the field is `_64BYTES`"]
    #[inline]
    pub fn is_64bytes(&self) -> bool {
        *self == BLOCKSIZER::_64BYTES
    }
}
#[doc = r" Value of the field"]
pub struct DMA0SKIPR {
    bits: u8,
}
impl DMA0SKIPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DMA1SKIPR {
    bits: u8,
}
impl DMA1SKIPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DMA0PRESAR {
    bits: bool,
}
impl DMA0PRESAR {
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
pub struct DMA1PRESAR {
    bits: bool,
}
impl DMA1PRESAR {
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
pub struct HALTR {
    bits: bool,
}
impl HALTR {
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
#[doc = r" Proxy"]
pub struct _LENGTHAW<'a> {
    w: &'a mut W,
}
impl<'a> _LENGTHAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 16383;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BLOCKSIZE`"]
pub enum BLOCKSIZEW {
    #[doc = "A block is 16 bytes long"]
    _16BYTES,
    #[doc = "A block is 32 bytes long"]
    _32BYTES,
    #[doc = "A block is 64 bytes long"]
    _64BYTES,
}
impl BLOCKSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BLOCKSIZEW::_16BYTES => 0,
            BLOCKSIZEW::_32BYTES => 1,
            BLOCKSIZEW::_64BYTES => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLOCKSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _BLOCKSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLOCKSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "A block is 16 bytes long"]
    #[inline]
    pub fn _16bytes(self) -> &'a mut W {
        self.variant(BLOCKSIZEW::_16BYTES)
    }
    #[doc = "A block is 32 bytes long"]
    #[inline]
    pub fn _32bytes(self) -> &'a mut W {
        self.variant(BLOCKSIZEW::_32BYTES)
    }
    #[doc = "A block is 64 bytes long"]
    #[inline]
    pub fn _64bytes(self) -> &'a mut W {
        self.variant(BLOCKSIZEW::_64BYTES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMA0SKIPW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA0SKIPW<'a> {
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
pub struct _DMA1SKIPW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA1SKIPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMA0PRESAW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA0PRESAW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMA1PRESAW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA1PRESAW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HALTW<'a> {
    w: &'a mut W,
}
impl<'a> _HALTW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:13 - Buffer Length a in Bytes"]
    #[inline]
    pub fn lengtha(&self) -> LENGTHAR {
        let bits = {
            const MASK: u16 = 16383;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        LENGTHAR { bits }
    }
    #[doc = "Bits 20:21 - Size of Data Blocks"]
    #[inline]
    pub fn blocksize(&self) -> BLOCKSIZER {
        BLOCKSIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - DMA0 Skip"]
    #[inline]
    pub fn dma0skip(&self) -> DMA0SKIPR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DMA0SKIPR { bits }
    }
    #[doc = "Bits 26:27 - DMA1 Skip"]
    #[inline]
    pub fn dma1skip(&self) -> DMA1SKIPR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DMA1SKIPR { bits }
    }
    #[doc = "Bit 28 - DMA0 Preserve a"]
    #[inline]
    pub fn dma0presa(&self) -> DMA0PRESAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA0PRESAR { bits }
    }
    #[doc = "Bit 29 - DMA1 Preserve a"]
    #[inline]
    pub fn dma1presa(&self) -> DMA1PRESAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA1PRESAR { bits }
    }
    #[doc = "Bit 31 - Halt Sequence"]
    #[inline]
    pub fn halt(&self) -> HALTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HALTR { bits }
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
    #[doc = "Bits 0:13 - Buffer Length a in Bytes"]
    #[inline]
    pub fn lengtha(&mut self) -> _LENGTHAW {
        _LENGTHAW { w: self }
    }
    #[doc = "Bits 20:21 - Size of Data Blocks"]
    #[inline]
    pub fn blocksize(&mut self) -> _BLOCKSIZEW {
        _BLOCKSIZEW { w: self }
    }
    #[doc = "Bits 24:25 - DMA0 Skip"]
    #[inline]
    pub fn dma0skip(&mut self) -> _DMA0SKIPW {
        _DMA0SKIPW { w: self }
    }
    #[doc = "Bits 26:27 - DMA1 Skip"]
    #[inline]
    pub fn dma1skip(&mut self) -> _DMA1SKIPW {
        _DMA1SKIPW { w: self }
    }
    #[doc = "Bit 28 - DMA0 Preserve a"]
    #[inline]
    pub fn dma0presa(&mut self) -> _DMA0PRESAW {
        _DMA0PRESAW { w: self }
    }
    #[doc = "Bit 29 - DMA1 Preserve a"]
    #[inline]
    pub fn dma1presa(&mut self) -> _DMA1PRESAW {
        _DMA1PRESAW { w: self }
    }
    #[doc = "Bit 31 - Halt Sequence"]
    #[inline]
    pub fn halt(&mut self) -> _HALTW {
        _HALTW { w: self }
    }
}
