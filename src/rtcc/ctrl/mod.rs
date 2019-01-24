#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
pub struct ENABLER {
    bits: bool,
}
impl ENABLER {
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
pub struct DEBUGRUNR {
    bits: bool,
}
impl DEBUGRUNR {
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
pub struct PRECCV0TOPR {
    bits: bool,
}
impl PRECCV0TOPR {
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
pub struct CCV1TOPR {
    bits: bool,
}
impl CCV1TOPR {
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
#[doc = "Possible values of the field `CNTPRESC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTPRESCR {
    #[doc = "CLKCNT = LFECLKRTCC/1"]
    DIV1,
    #[doc = "CLKCNT = LFECLKRTCC/2"]
    DIV2,
    #[doc = "CLKCNT = LFECLKRTCC/4"]
    DIV4,
    #[doc = "CLKCNT = LFECLKRTCC/8"]
    DIV8,
    #[doc = "CLKCNT = LFECLKRTCC/16"]
    DIV16,
    #[doc = "CLKCNT = LFECLKRTCC/32"]
    DIV32,
    #[doc = "CLKCNT = LFECLKRTCC/64"]
    DIV64,
    #[doc = "CLKCNT = LFECLKRTCC/128"]
    DIV128,
    #[doc = "CLKCNT = LFECLKRTCC/256"]
    DIV256,
    #[doc = "CLKCNT = LFECLKRTCC/512"]
    DIV512,
    #[doc = "CLKCNT = LFECLKRTCC/1024"]
    DIV1024,
    #[doc = "CLKCNT = LFECLKRTCC/2048"]
    DIV2048,
    #[doc = "CLKCNT = LFECLKRTCC/4096"]
    DIV4096,
    #[doc = "CLKCNT = LFECLKRTCC/8192"]
    DIV8192,
    #[doc = "CLKCNT = LFECLKRTCC/16384"]
    DIV16384,
    #[doc = "CLKCNT = LFECLKRTCC/32768"]
    DIV32768,
}
impl CNTPRESCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CNTPRESCR::DIV1 => 0,
            CNTPRESCR::DIV2 => 1,
            CNTPRESCR::DIV4 => 2,
            CNTPRESCR::DIV8 => 3,
            CNTPRESCR::DIV16 => 4,
            CNTPRESCR::DIV32 => 5,
            CNTPRESCR::DIV64 => 6,
            CNTPRESCR::DIV128 => 7,
            CNTPRESCR::DIV256 => 8,
            CNTPRESCR::DIV512 => 9,
            CNTPRESCR::DIV1024 => 10,
            CNTPRESCR::DIV2048 => 11,
            CNTPRESCR::DIV4096 => 12,
            CNTPRESCR::DIV8192 => 13,
            CNTPRESCR::DIV16384 => 14,
            CNTPRESCR::DIV32768 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CNTPRESCR {
        match value {
            0 => CNTPRESCR::DIV1,
            1 => CNTPRESCR::DIV2,
            2 => CNTPRESCR::DIV4,
            3 => CNTPRESCR::DIV8,
            4 => CNTPRESCR::DIV16,
            5 => CNTPRESCR::DIV32,
            6 => CNTPRESCR::DIV64,
            7 => CNTPRESCR::DIV128,
            8 => CNTPRESCR::DIV256,
            9 => CNTPRESCR::DIV512,
            10 => CNTPRESCR::DIV1024,
            11 => CNTPRESCR::DIV2048,
            12 => CNTPRESCR::DIV4096,
            13 => CNTPRESCR::DIV8192,
            14 => CNTPRESCR::DIV16384,
            15 => CNTPRESCR::DIV32768,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == CNTPRESCR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == CNTPRESCR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == CNTPRESCR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == CNTPRESCR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == CNTPRESCR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == CNTPRESCR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == CNTPRESCR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == CNTPRESCR::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == CNTPRESCR::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline]
    pub fn is_div512(&self) -> bool {
        *self == CNTPRESCR::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline]
    pub fn is_div1024(&self) -> bool {
        *self == CNTPRESCR::DIV1024
    }
    #[doc = "Checks if the value of the field is `DIV2048`"]
    #[inline]
    pub fn is_div2048(&self) -> bool {
        *self == CNTPRESCR::DIV2048
    }
    #[doc = "Checks if the value of the field is `DIV4096`"]
    #[inline]
    pub fn is_div4096(&self) -> bool {
        *self == CNTPRESCR::DIV4096
    }
    #[doc = "Checks if the value of the field is `DIV8192`"]
    #[inline]
    pub fn is_div8192(&self) -> bool {
        *self == CNTPRESCR::DIV8192
    }
    #[doc = "Checks if the value of the field is `DIV16384`"]
    #[inline]
    pub fn is_div16384(&self) -> bool {
        *self == CNTPRESCR::DIV16384
    }
    #[doc = "Checks if the value of the field is `DIV32768`"]
    #[inline]
    pub fn is_div32768(&self) -> bool {
        *self == CNTPRESCR::DIV32768
    }
}
#[doc = r" Value of the field"]
pub struct CNTTICKR {
    bits: bool,
}
impl CNTTICKR {
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
pub struct OSCFDETENR {
    bits: bool,
}
impl OSCFDETENR {
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
pub struct CNTMODER {
    bits: bool,
}
impl CNTMODER {
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
pub struct LYEARCORRDISR {
    bits: bool,
}
impl LYEARCORRDISR {
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
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
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
pub struct _DEBUGRUNW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBUGRUNW<'a> {
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
pub struct _PRECCV0TOPW<'a> {
    w: &'a mut W,
}
impl<'a> _PRECCV0TOPW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CCV1TOPW<'a> {
    w: &'a mut W,
}
impl<'a> _CCV1TOPW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CNTPRESC`"]
pub enum CNTPRESCW {
    #[doc = "CLKCNT = LFECLKRTCC/1"]
    DIV1,
    #[doc = "CLKCNT = LFECLKRTCC/2"]
    DIV2,
    #[doc = "CLKCNT = LFECLKRTCC/4"]
    DIV4,
    #[doc = "CLKCNT = LFECLKRTCC/8"]
    DIV8,
    #[doc = "CLKCNT = LFECLKRTCC/16"]
    DIV16,
    #[doc = "CLKCNT = LFECLKRTCC/32"]
    DIV32,
    #[doc = "CLKCNT = LFECLKRTCC/64"]
    DIV64,
    #[doc = "CLKCNT = LFECLKRTCC/128"]
    DIV128,
    #[doc = "CLKCNT = LFECLKRTCC/256"]
    DIV256,
    #[doc = "CLKCNT = LFECLKRTCC/512"]
    DIV512,
    #[doc = "CLKCNT = LFECLKRTCC/1024"]
    DIV1024,
    #[doc = "CLKCNT = LFECLKRTCC/2048"]
    DIV2048,
    #[doc = "CLKCNT = LFECLKRTCC/4096"]
    DIV4096,
    #[doc = "CLKCNT = LFECLKRTCC/8192"]
    DIV8192,
    #[doc = "CLKCNT = LFECLKRTCC/16384"]
    DIV16384,
    #[doc = "CLKCNT = LFECLKRTCC/32768"]
    DIV32768,
}
impl CNTPRESCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CNTPRESCW::DIV1 => 0,
            CNTPRESCW::DIV2 => 1,
            CNTPRESCW::DIV4 => 2,
            CNTPRESCW::DIV8 => 3,
            CNTPRESCW::DIV16 => 4,
            CNTPRESCW::DIV32 => 5,
            CNTPRESCW::DIV64 => 6,
            CNTPRESCW::DIV128 => 7,
            CNTPRESCW::DIV256 => 8,
            CNTPRESCW::DIV512 => 9,
            CNTPRESCW::DIV1024 => 10,
            CNTPRESCW::DIV2048 => 11,
            CNTPRESCW::DIV4096 => 12,
            CNTPRESCW::DIV8192 => 13,
            CNTPRESCW::DIV16384 => 14,
            CNTPRESCW::DIV32768 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNTPRESCW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTPRESCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNTPRESCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CLKCNT = LFECLKRTCC/1"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(CNTPRESCW::DIV1)
    }
    #[doc = "CLKCNT = LFECLKRTCC/2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(CNTPRESCW::DIV2)
    }
    #[doc = "CLKCNT = LFECLKRTCC/4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(CNTPRESCW::DIV4)
    }
    #[doc = "CLKCNT = LFECLKRTCC/8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(CNTPRESCW::DIV8)
    }
    #[doc = "CLKCNT = LFECLKRTCC/16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(CNTPRESCW::DIV16)
    }
    #[doc = "CLKCNT = LFECLKRTCC/32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(CNTPRESCW::DIV32)
    }
    #[doc = "CLKCNT = LFECLKRTCC/64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(CNTPRESCW::DIV64)
    }
    #[doc = "CLKCNT = LFECLKRTCC/128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(CNTPRESCW::DIV128)
    }
    #[doc = "CLKCNT = LFECLKRTCC/256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(CNTPRESCW::DIV256)
    }
    #[doc = "CLKCNT = LFECLKRTCC/512"]
    #[inline]
    pub fn div512(self) -> &'a mut W {
        self.variant(CNTPRESCW::DIV512)
    }
    #[doc = "CLKCNT = LFECLKRTCC/1024"]
    #[inline]
    pub fn div1024(self) -> &'a mut W {
        self.variant(CNTPRESCW::DIV1024)
    }
    #[doc = "CLKCNT = LFECLKRTCC/2048"]
    #[inline]
    pub fn div2048(self) -> &'a mut W {
        self.variant(CNTPRESCW::DIV2048)
    }
    #[doc = "CLKCNT = LFECLKRTCC/4096"]
    #[inline]
    pub fn div4096(self) -> &'a mut W {
        self.variant(CNTPRESCW::DIV4096)
    }
    #[doc = "CLKCNT = LFECLKRTCC/8192"]
    #[inline]
    pub fn div8192(self) -> &'a mut W {
        self.variant(CNTPRESCW::DIV8192)
    }
    #[doc = "CLKCNT = LFECLKRTCC/16384"]
    #[inline]
    pub fn div16384(self) -> &'a mut W {
        self.variant(CNTPRESCW::DIV16384)
    }
    #[doc = "CLKCNT = LFECLKRTCC/32768"]
    #[inline]
    pub fn div32768(self) -> &'a mut W {
        self.variant(CNTPRESCW::DIV32768)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CNTTICKW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTTICKW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OSCFDETENW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCFDETENW<'a> {
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
#[doc = r" Proxy"]
pub struct _CNTMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTMODEW<'a> {
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
pub struct _LYEARCORRDISW<'a> {
    w: &'a mut W,
}
impl<'a> _LYEARCORRDISW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - RTCC Enable"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLER { bits }
    }
    #[doc = "Bit 2 - Debug Mode Run Enable"]
    #[inline]
    pub fn debugrun(&self) -> DEBUGRUNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DEBUGRUNR { bits }
    }
    #[doc = "Bit 4 - Pre-counter CCV0 Top Value Enable"]
    #[inline]
    pub fn preccv0top(&self) -> PRECCV0TOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRECCV0TOPR { bits }
    }
    #[doc = "Bit 5 - CCV1 Top Value Enable"]
    #[inline]
    pub fn ccv1top(&self) -> CCV1TOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CCV1TOPR { bits }
    }
    #[doc = "Bits 8:11 - Counter Prescaler Value"]
    #[inline]
    pub fn cntpresc(&self) -> CNTPRESCR {
        CNTPRESCR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Counter Prescaler Mode"]
    #[inline]
    pub fn cnttick(&self) -> CNTTICKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CNTTICKR { bits }
    }
    #[doc = "Bit 15 - Oscillator Failure Detection Enable"]
    #[inline]
    pub fn oscfdeten(&self) -> OSCFDETENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OSCFDETENR { bits }
    }
    #[doc = "Bit 16 - Main Counter Mode"]
    #[inline]
    pub fn cntmode(&self) -> CNTMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CNTMODER { bits }
    }
    #[doc = "Bit 17 - Leap Year Correction Disabled"]
    #[inline]
    pub fn lyearcorrdis(&self) -> LYEARCORRDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LYEARCORRDISR { bits }
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
    #[doc = "Bit 0 - RTCC Enable"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bit 2 - Debug Mode Run Enable"]
    #[inline]
    pub fn debugrun(&mut self) -> _DEBUGRUNW {
        _DEBUGRUNW { w: self }
    }
    #[doc = "Bit 4 - Pre-counter CCV0 Top Value Enable"]
    #[inline]
    pub fn preccv0top(&mut self) -> _PRECCV0TOPW {
        _PRECCV0TOPW { w: self }
    }
    #[doc = "Bit 5 - CCV1 Top Value Enable"]
    #[inline]
    pub fn ccv1top(&mut self) -> _CCV1TOPW {
        _CCV1TOPW { w: self }
    }
    #[doc = "Bits 8:11 - Counter Prescaler Value"]
    #[inline]
    pub fn cntpresc(&mut self) -> _CNTPRESCW {
        _CNTPRESCW { w: self }
    }
    #[doc = "Bit 12 - Counter Prescaler Mode"]
    #[inline]
    pub fn cnttick(&mut self) -> _CNTTICKW {
        _CNTTICKW { w: self }
    }
    #[doc = "Bit 15 - Oscillator Failure Detection Enable"]
    #[inline]
    pub fn oscfdeten(&mut self) -> _OSCFDETENW {
        _OSCFDETENW { w: self }
    }
    #[doc = "Bit 16 - Main Counter Mode"]
    #[inline]
    pub fn cntmode(&mut self) -> _CNTMODEW {
        _CNTMODEW { w: self }
    }
    #[doc = "Bit 17 - Leap Year Correction Disabled"]
    #[inline]
    pub fn lyearcorrdis(&mut self) -> _LYEARCORRDISW {
        _LYEARCORRDISW { w: self }
    }
}
