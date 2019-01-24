#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CH6_CTRL {
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
#[doc = "Possible values of the field `STRUCTTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRUCTTYPER {
    #[doc = "DMA transfer structure type selected."]
    TRANSFER,
    #[doc = "Synchronization structure type selected."]
    SYNCHRONIZE,
    #[doc = "Write immediate value structure type selected."]
    WRITE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STRUCTTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STRUCTTYPER::TRANSFER => 0,
            STRUCTTYPER::SYNCHRONIZE => 1,
            STRUCTTYPER::WRITE => 2,
            STRUCTTYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STRUCTTYPER {
        match value {
            0 => STRUCTTYPER::TRANSFER,
            1 => STRUCTTYPER::SYNCHRONIZE,
            2 => STRUCTTYPER::WRITE,
            i => STRUCTTYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TRANSFER`"]
    #[inline]
    pub fn is_transfer(&self) -> bool {
        *self == STRUCTTYPER::TRANSFER
    }
    #[doc = "Checks if the value of the field is `SYNCHRONIZE`"]
    #[inline]
    pub fn is_synchronize(&self) -> bool {
        *self == STRUCTTYPER::SYNCHRONIZE
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline]
    pub fn is_write(&self) -> bool {
        *self == STRUCTTYPER::WRITE
    }
}
#[doc = r" Value of the field"]
pub struct XFERCNTR {
    bits: u16,
}
impl XFERCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BYTESWAPR {
    bits: bool,
}
impl BYTESWAPR {
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
#[doc = "Possible values of the field `BLOCKSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLOCKSIZER {
    #[doc = "One unit transfer per arbitration"]
    UNIT1,
    #[doc = "Two unit transfers per arbitration"]
    UNIT2,
    #[doc = "Three unit transfers per arbitration"]
    UNIT3,
    #[doc = "Four unit transfers per arbitration"]
    UNIT4,
    #[doc = "Six unit transfers per arbitration"]
    UNIT6,
    #[doc = "Eight unit transfers per arbitration"]
    UNIT8,
    #[doc = "Sixteen unit transfers per arbitration"]
    UNIT16,
    #[doc = "32 unit transfers per arbitration"]
    UNIT32,
    #[doc = "64 unit transfers per arbitration"]
    UNIT64,
    #[doc = "128 unit transfers per arbitration"]
    UNIT128,
    #[doc = "256 unit transfers per arbitration"]
    UNIT256,
    #[doc = "512 unit transfers per arbitration"]
    UNIT512,
    #[doc = "1024 unit transfers per arbitration"]
    UNIT1024,
    #[doc = "Transfer all units as specified by the XFRCNT field"]
    ALL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BLOCKSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BLOCKSIZER::UNIT1 => 0,
            BLOCKSIZER::UNIT2 => 1,
            BLOCKSIZER::UNIT3 => 2,
            BLOCKSIZER::UNIT4 => 3,
            BLOCKSIZER::UNIT6 => 4,
            BLOCKSIZER::UNIT8 => 5,
            BLOCKSIZER::UNIT16 => 7,
            BLOCKSIZER::UNIT32 => 9,
            BLOCKSIZER::UNIT64 => 10,
            BLOCKSIZER::UNIT128 => 11,
            BLOCKSIZER::UNIT256 => 12,
            BLOCKSIZER::UNIT512 => 13,
            BLOCKSIZER::UNIT1024 => 14,
            BLOCKSIZER::ALL => 15,
            BLOCKSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BLOCKSIZER {
        match value {
            0 => BLOCKSIZER::UNIT1,
            1 => BLOCKSIZER::UNIT2,
            2 => BLOCKSIZER::UNIT3,
            3 => BLOCKSIZER::UNIT4,
            4 => BLOCKSIZER::UNIT6,
            5 => BLOCKSIZER::UNIT8,
            7 => BLOCKSIZER::UNIT16,
            9 => BLOCKSIZER::UNIT32,
            10 => BLOCKSIZER::UNIT64,
            11 => BLOCKSIZER::UNIT128,
            12 => BLOCKSIZER::UNIT256,
            13 => BLOCKSIZER::UNIT512,
            14 => BLOCKSIZER::UNIT1024,
            15 => BLOCKSIZER::ALL,
            i => BLOCKSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UNIT1`"]
    #[inline]
    pub fn is_unit1(&self) -> bool {
        *self == BLOCKSIZER::UNIT1
    }
    #[doc = "Checks if the value of the field is `UNIT2`"]
    #[inline]
    pub fn is_unit2(&self) -> bool {
        *self == BLOCKSIZER::UNIT2
    }
    #[doc = "Checks if the value of the field is `UNIT3`"]
    #[inline]
    pub fn is_unit3(&self) -> bool {
        *self == BLOCKSIZER::UNIT3
    }
    #[doc = "Checks if the value of the field is `UNIT4`"]
    #[inline]
    pub fn is_unit4(&self) -> bool {
        *self == BLOCKSIZER::UNIT4
    }
    #[doc = "Checks if the value of the field is `UNIT6`"]
    #[inline]
    pub fn is_unit6(&self) -> bool {
        *self == BLOCKSIZER::UNIT6
    }
    #[doc = "Checks if the value of the field is `UNIT8`"]
    #[inline]
    pub fn is_unit8(&self) -> bool {
        *self == BLOCKSIZER::UNIT8
    }
    #[doc = "Checks if the value of the field is `UNIT16`"]
    #[inline]
    pub fn is_unit16(&self) -> bool {
        *self == BLOCKSIZER::UNIT16
    }
    #[doc = "Checks if the value of the field is `UNIT32`"]
    #[inline]
    pub fn is_unit32(&self) -> bool {
        *self == BLOCKSIZER::UNIT32
    }
    #[doc = "Checks if the value of the field is `UNIT64`"]
    #[inline]
    pub fn is_unit64(&self) -> bool {
        *self == BLOCKSIZER::UNIT64
    }
    #[doc = "Checks if the value of the field is `UNIT128`"]
    #[inline]
    pub fn is_unit128(&self) -> bool {
        *self == BLOCKSIZER::UNIT128
    }
    #[doc = "Checks if the value of the field is `UNIT256`"]
    #[inline]
    pub fn is_unit256(&self) -> bool {
        *self == BLOCKSIZER::UNIT256
    }
    #[doc = "Checks if the value of the field is `UNIT512`"]
    #[inline]
    pub fn is_unit512(&self) -> bool {
        *self == BLOCKSIZER::UNIT512
    }
    #[doc = "Checks if the value of the field is `UNIT1024`"]
    #[inline]
    pub fn is_unit1024(&self) -> bool {
        *self == BLOCKSIZER::UNIT1024
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline]
    pub fn is_all(&self) -> bool {
        *self == BLOCKSIZER::ALL
    }
}
#[doc = r" Value of the field"]
pub struct DONEIFSENR {
    bits: bool,
}
impl DONEIFSENR {
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
pub struct REQMODER {
    bits: bool,
}
impl REQMODER {
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
pub struct DECLOOPCNTR {
    bits: bool,
}
impl DECLOOPCNTR {
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
pub struct IGNORESREQR {
    bits: bool,
}
impl IGNORESREQR {
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
#[doc = "Possible values of the field `SRCINC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCINCR {
    #[doc = "Increment source address by one unit data size after each read"]
    ONE,
    #[doc = "Increment source address by two unit data sizes after each read"]
    TWO,
    #[doc = "Increment source address by four unit data sizes after each read"]
    FOUR,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    NONE,
}
impl SRCINCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRCINCR::ONE => 0,
            SRCINCR::TWO => 1,
            SRCINCR::FOUR => 2,
            SRCINCR::NONE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRCINCR {
        match value {
            0 => SRCINCR::ONE,
            1 => SRCINCR::TWO,
            2 => SRCINCR::FOUR,
            3 => SRCINCR::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == SRCINCR::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline]
    pub fn is_two(&self) -> bool {
        *self == SRCINCR::TWO
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline]
    pub fn is_four(&self) -> bool {
        *self == SRCINCR::FOUR
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SRCINCR::NONE
    }
}
#[doc = "Possible values of the field `SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIZER {
    #[doc = "Each unit transfer is a byte"]
    BYTE,
    #[doc = "Each unit transfer is a half-word"]
    HALFWORD,
    #[doc = "Each unit transfer is a word"]
    WORD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SIZER::BYTE => 0,
            SIZER::HALFWORD => 1,
            SIZER::WORD => 2,
            SIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SIZER {
        match value {
            0 => SIZER::BYTE,
            1 => SIZER::HALFWORD,
            2 => SIZER::WORD,
            i => SIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline]
    pub fn is_byte(&self) -> bool {
        *self == SIZER::BYTE
    }
    #[doc = "Checks if the value of the field is `HALFWORD`"]
    #[inline]
    pub fn is_halfword(&self) -> bool {
        *self == SIZER::HALFWORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline]
    pub fn is_word(&self) -> bool {
        *self == SIZER::WORD
    }
}
#[doc = "Possible values of the field `DSTINC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSTINCR {
    #[doc = "Increment destination address by one unit data size after each write"]
    ONE,
    #[doc = "Increment destination address by two unit data sizes after each write"]
    TWO,
    #[doc = "Increment destination address by four unit data sizes after each write"]
    FOUR,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    NONE,
}
impl DSTINCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DSTINCR::ONE => 0,
            DSTINCR::TWO => 1,
            DSTINCR::FOUR => 2,
            DSTINCR::NONE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DSTINCR {
        match value {
            0 => DSTINCR::ONE,
            1 => DSTINCR::TWO,
            2 => DSTINCR::FOUR,
            3 => DSTINCR::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == DSTINCR::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline]
    pub fn is_two(&self) -> bool {
        *self == DSTINCR::TWO
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline]
    pub fn is_four(&self) -> bool {
        *self == DSTINCR::FOUR
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == DSTINCR::NONE
    }
}
#[doc = r" Value of the field"]
pub struct SRCMODER {
    bits: bool,
}
impl SRCMODER {
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
pub struct DSTMODER {
    bits: bool,
}
impl DSTMODER {
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
pub struct _STRUCTREQW<'a> {
    w: &'a mut W,
}
impl<'a> _STRUCTREQW<'a> {
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
#[doc = r" Proxy"]
pub struct _XFERCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _XFERCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 2047;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BYTESWAPW<'a> {
    w: &'a mut W,
}
impl<'a> _BYTESWAPW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BLOCKSIZE`"]
pub enum BLOCKSIZEW {
    #[doc = "One unit transfer per arbitration"]
    UNIT1,
    #[doc = "Two unit transfers per arbitration"]
    UNIT2,
    #[doc = "Three unit transfers per arbitration"]
    UNIT3,
    #[doc = "Four unit transfers per arbitration"]
    UNIT4,
    #[doc = "Six unit transfers per arbitration"]
    UNIT6,
    #[doc = "Eight unit transfers per arbitration"]
    UNIT8,
    #[doc = "Sixteen unit transfers per arbitration"]
    UNIT16,
    #[doc = "32 unit transfers per arbitration"]
    UNIT32,
    #[doc = "64 unit transfers per arbitration"]
    UNIT64,
    #[doc = "128 unit transfers per arbitration"]
    UNIT128,
    #[doc = "256 unit transfers per arbitration"]
    UNIT256,
    #[doc = "512 unit transfers per arbitration"]
    UNIT512,
    #[doc = "1024 unit transfers per arbitration"]
    UNIT1024,
    #[doc = "Transfer all units as specified by the XFRCNT field"]
    ALL,
}
impl BLOCKSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BLOCKSIZEW::UNIT1 => 0,
            BLOCKSIZEW::UNIT2 => 1,
            BLOCKSIZEW::UNIT3 => 2,
            BLOCKSIZEW::UNIT4 => 3,
            BLOCKSIZEW::UNIT6 => 4,
            BLOCKSIZEW::UNIT8 => 5,
            BLOCKSIZEW::UNIT16 => 7,
            BLOCKSIZEW::UNIT32 => 9,
            BLOCKSIZEW::UNIT64 => 10,
            BLOCKSIZEW::UNIT128 => 11,
            BLOCKSIZEW::UNIT256 => 12,
            BLOCKSIZEW::UNIT512 => 13,
            BLOCKSIZEW::UNIT1024 => 14,
            BLOCKSIZEW::ALL => 15,
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
    #[doc = "One unit transfer per arbitration"]
    #[inline]
    pub fn unit1(self) -> &'a mut W {
        self.variant(BLOCKSIZEW::UNIT1)
    }
    #[doc = "Two unit transfers per arbitration"]
    #[inline]
    pub fn unit2(self) -> &'a mut W {
        self.variant(BLOCKSIZEW::UNIT2)
    }
    #[doc = "Three unit transfers per arbitration"]
    #[inline]
    pub fn unit3(self) -> &'a mut W {
        self.variant(BLOCKSIZEW::UNIT3)
    }
    #[doc = "Four unit transfers per arbitration"]
    #[inline]
    pub fn unit4(self) -> &'a mut W {
        self.variant(BLOCKSIZEW::UNIT4)
    }
    #[doc = "Six unit transfers per arbitration"]
    #[inline]
    pub fn unit6(self) -> &'a mut W {
        self.variant(BLOCKSIZEW::UNIT6)
    }
    #[doc = "Eight unit transfers per arbitration"]
    #[inline]
    pub fn unit8(self) -> &'a mut W {
        self.variant(BLOCKSIZEW::UNIT8)
    }
    #[doc = "Sixteen unit transfers per arbitration"]
    #[inline]
    pub fn unit16(self) -> &'a mut W {
        self.variant(BLOCKSIZEW::UNIT16)
    }
    #[doc = "32 unit transfers per arbitration"]
    #[inline]
    pub fn unit32(self) -> &'a mut W {
        self.variant(BLOCKSIZEW::UNIT32)
    }
    #[doc = "64 unit transfers per arbitration"]
    #[inline]
    pub fn unit64(self) -> &'a mut W {
        self.variant(BLOCKSIZEW::UNIT64)
    }
    #[doc = "128 unit transfers per arbitration"]
    #[inline]
    pub fn unit128(self) -> &'a mut W {
        self.variant(BLOCKSIZEW::UNIT128)
    }
    #[doc = "256 unit transfers per arbitration"]
    #[inline]
    pub fn unit256(self) -> &'a mut W {
        self.variant(BLOCKSIZEW::UNIT256)
    }
    #[doc = "512 unit transfers per arbitration"]
    #[inline]
    pub fn unit512(self) -> &'a mut W {
        self.variant(BLOCKSIZEW::UNIT512)
    }
    #[doc = "1024 unit transfers per arbitration"]
    #[inline]
    pub fn unit1024(self) -> &'a mut W {
        self.variant(BLOCKSIZEW::UNIT1024)
    }
    #[doc = "Transfer all units as specified by the XFRCNT field"]
    #[inline]
    pub fn all(self) -> &'a mut W {
        self.variant(BLOCKSIZEW::ALL)
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
#[doc = r" Proxy"]
pub struct _DONEIFSENW<'a> {
    w: &'a mut W,
}
impl<'a> _DONEIFSENW<'a> {
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
pub struct _REQMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _REQMODEW<'a> {
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
#[doc = r" Proxy"]
pub struct _DECLOOPCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _DECLOOPCNTW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IGNORESREQW<'a> {
    w: &'a mut W,
}
impl<'a> _IGNORESREQW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRCINC`"]
pub enum SRCINCW {
    #[doc = "Increment source address by one unit data size after each read"]
    ONE,
    #[doc = "Increment source address by two unit data sizes after each read"]
    TWO,
    #[doc = "Increment source address by four unit data sizes after each read"]
    FOUR,
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    NONE,
}
impl SRCINCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRCINCW::ONE => 0,
            SRCINCW::TWO => 1,
            SRCINCW::FOUR => 2,
            SRCINCW::NONE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRCINCW<'a> {
    w: &'a mut W,
}
impl<'a> _SRCINCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRCINCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Increment source address by one unit data size after each read"]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(SRCINCW::ONE)
    }
    #[doc = "Increment source address by two unit data sizes after each read"]
    #[inline]
    pub fn two(self) -> &'a mut W {
        self.variant(SRCINCW::TWO)
    }
    #[doc = "Increment source address by four unit data sizes after each read"]
    #[inline]
    pub fn four(self) -> &'a mut W {
        self.variant(SRCINCW::FOUR)
    }
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SRCINCW::NONE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SIZE`"]
pub enum SIZEW {
    #[doc = "Each unit transfer is a byte"]
    BYTE,
    #[doc = "Each unit transfer is a half-word"]
    HALFWORD,
    #[doc = "Each unit transfer is a word"]
    WORD,
}
impl SIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SIZEW::BYTE => 0,
            SIZEW::HALFWORD => 1,
            SIZEW::WORD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _SIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Each unit transfer is a byte"]
    #[inline]
    pub fn byte(self) -> &'a mut W {
        self.variant(SIZEW::BYTE)
    }
    #[doc = "Each unit transfer is a half-word"]
    #[inline]
    pub fn halfword(self) -> &'a mut W {
        self.variant(SIZEW::HALFWORD)
    }
    #[doc = "Each unit transfer is a word"]
    #[inline]
    pub fn word(self) -> &'a mut W {
        self.variant(SIZEW::WORD)
    }
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
#[doc = "Values that can be written to the field `DSTINC`"]
pub enum DSTINCW {
    #[doc = "Increment destination address by one unit data size after each write"]
    ONE,
    #[doc = "Increment destination address by two unit data sizes after each write"]
    TWO,
    #[doc = "Increment destination address by four unit data sizes after each write"]
    FOUR,
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    NONE,
}
impl DSTINCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DSTINCW::ONE => 0,
            DSTINCW::TWO => 1,
            DSTINCW::FOUR => 2,
            DSTINCW::NONE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSTINCW<'a> {
    w: &'a mut W,
}
impl<'a> _DSTINCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSTINCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Increment destination address by one unit data size after each write"]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(DSTINCW::ONE)
    }
    #[doc = "Increment destination address by two unit data sizes after each write"]
    #[inline]
    pub fn two(self) -> &'a mut W {
        self.variant(DSTINCW::TWO)
    }
    #[doc = "Increment destination address by four unit data sizes after each write"]
    #[inline]
    pub fn four(self) -> &'a mut W {
        self.variant(DSTINCW::FOUR)
    }
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(DSTINCW::NONE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:1 - DMA Structure Type"]
    #[inline]
    pub fn structtype(&self) -> STRUCTTYPER {
        STRUCTTYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:14 - DMA Unit Data Transfer Count"]
    #[inline]
    pub fn xfercnt(&self) -> XFERCNTR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        XFERCNTR { bits }
    }
    #[doc = "Bit 15 - Endian Byte Swap"]
    #[inline]
    pub fn byteswap(&self) -> BYTESWAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BYTESWAPR { bits }
    }
    #[doc = "Bits 16:19 - Block Transfer Size"]
    #[inline]
    pub fn blocksize(&self) -> BLOCKSIZER {
        BLOCKSIZER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - DMA Operation Done Interrupt Flag Set Enable"]
    #[inline]
    pub fn doneifsen(&self) -> DONEIFSENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DONEIFSENR { bits }
    }
    #[doc = "Bit 21 - DMA Request Transfer Mode Select"]
    #[inline]
    pub fn reqmode(&self) -> REQMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        REQMODER { bits }
    }
    #[doc = "Bit 22 - Decrement Loop Count"]
    #[inline]
    pub fn decloopcnt(&self) -> DECLOOPCNTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DECLOOPCNTR { bits }
    }
    #[doc = "Bit 23 - Ignore Sreq"]
    #[inline]
    pub fn ignoresreq(&self) -> IGNORESREQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IGNORESREQR { bits }
    }
    #[doc = "Bits 24:25 - Source Address Increment Size"]
    #[inline]
    pub fn srcinc(&self) -> SRCINCR {
        SRCINCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Unit Data Transfer Size"]
    #[inline]
    pub fn size(&self) -> SIZER {
        SIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Destination Address Increment Size"]
    #[inline]
    pub fn dstinc(&self) -> DSTINCR {
        DSTINCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 30 - Source Addressing Mode"]
    #[inline]
    pub fn srcmode(&self) -> SRCMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SRCMODER { bits }
    }
    #[doc = "Bit 31 - Destination Addressing Mode"]
    #[inline]
    pub fn dstmode(&self) -> DSTMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DSTMODER { bits }
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
    #[doc = "Bit 3 - Structure DMA Transfer Request"]
    #[inline]
    pub fn structreq(&mut self) -> _STRUCTREQW {
        _STRUCTREQW { w: self }
    }
    #[doc = "Bits 4:14 - DMA Unit Data Transfer Count"]
    #[inline]
    pub fn xfercnt(&mut self) -> _XFERCNTW {
        _XFERCNTW { w: self }
    }
    #[doc = "Bit 15 - Endian Byte Swap"]
    #[inline]
    pub fn byteswap(&mut self) -> _BYTESWAPW {
        _BYTESWAPW { w: self }
    }
    #[doc = "Bits 16:19 - Block Transfer Size"]
    #[inline]
    pub fn blocksize(&mut self) -> _BLOCKSIZEW {
        _BLOCKSIZEW { w: self }
    }
    #[doc = "Bit 20 - DMA Operation Done Interrupt Flag Set Enable"]
    #[inline]
    pub fn doneifsen(&mut self) -> _DONEIFSENW {
        _DONEIFSENW { w: self }
    }
    #[doc = "Bit 21 - DMA Request Transfer Mode Select"]
    #[inline]
    pub fn reqmode(&mut self) -> _REQMODEW {
        _REQMODEW { w: self }
    }
    #[doc = "Bit 22 - Decrement Loop Count"]
    #[inline]
    pub fn decloopcnt(&mut self) -> _DECLOOPCNTW {
        _DECLOOPCNTW { w: self }
    }
    #[doc = "Bit 23 - Ignore Sreq"]
    #[inline]
    pub fn ignoresreq(&mut self) -> _IGNORESREQW {
        _IGNORESREQW { w: self }
    }
    #[doc = "Bits 24:25 - Source Address Increment Size"]
    #[inline]
    pub fn srcinc(&mut self) -> _SRCINCW {
        _SRCINCW { w: self }
    }
    #[doc = "Bits 26:27 - Unit Data Transfer Size"]
    #[inline]
    pub fn size(&mut self) -> _SIZEW {
        _SIZEW { w: self }
    }
    #[doc = "Bits 28:29 - Destination Address Increment Size"]
    #[inline]
    pub fn dstinc(&mut self) -> _DSTINCW {
        _DSTINCW { w: self }
    }
}
