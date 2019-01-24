#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LFAPRESC0 {
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
#[doc = "Possible values of the field `LETIMER0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LETIMER0R {
    #[doc = "LFACLKLETIMER0 = LFACLK"]
    DIV1,
    #[doc = "LFACLKLETIMER0 = LFACLK/2"]
    DIV2,
    #[doc = "LFACLKLETIMER0 = LFACLK/4"]
    DIV4,
    #[doc = "LFACLKLETIMER0 = LFACLK/8"]
    DIV8,
    #[doc = "LFACLKLETIMER0 = LFACLK/16"]
    DIV16,
    #[doc = "LFACLKLETIMER0 = LFACLK/32"]
    DIV32,
    #[doc = "LFACLKLETIMER0 = LFACLK/64"]
    DIV64,
    #[doc = "LFACLKLETIMER0 = LFACLK/128"]
    DIV128,
    #[doc = "LFACLKLETIMER0 = LFACLK/256"]
    DIV256,
    #[doc = "LFACLKLETIMER0 = LFACLK/512"]
    DIV512,
    #[doc = "LFACLKLETIMER0 = LFACLK/1024"]
    DIV1024,
    #[doc = "LFACLKLETIMER0 = LFACLK/2048"]
    DIV2048,
    #[doc = "LFACLKLETIMER0 = LFACLK/4096"]
    DIV4096,
    #[doc = "LFACLKLETIMER0 = LFACLK/8192"]
    DIV8192,
    #[doc = "LFACLKLETIMER0 = LFACLK/16384"]
    DIV16384,
    #[doc = "LFACLKLETIMER0 = LFACLK/32768"]
    DIV32768,
}
impl LETIMER0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LETIMER0R::DIV1 => 0,
            LETIMER0R::DIV2 => 1,
            LETIMER0R::DIV4 => 2,
            LETIMER0R::DIV8 => 3,
            LETIMER0R::DIV16 => 4,
            LETIMER0R::DIV32 => 5,
            LETIMER0R::DIV64 => 6,
            LETIMER0R::DIV128 => 7,
            LETIMER0R::DIV256 => 8,
            LETIMER0R::DIV512 => 9,
            LETIMER0R::DIV1024 => 10,
            LETIMER0R::DIV2048 => 11,
            LETIMER0R::DIV4096 => 12,
            LETIMER0R::DIV8192 => 13,
            LETIMER0R::DIV16384 => 14,
            LETIMER0R::DIV32768 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LETIMER0R {
        match value {
            0 => LETIMER0R::DIV1,
            1 => LETIMER0R::DIV2,
            2 => LETIMER0R::DIV4,
            3 => LETIMER0R::DIV8,
            4 => LETIMER0R::DIV16,
            5 => LETIMER0R::DIV32,
            6 => LETIMER0R::DIV64,
            7 => LETIMER0R::DIV128,
            8 => LETIMER0R::DIV256,
            9 => LETIMER0R::DIV512,
            10 => LETIMER0R::DIV1024,
            11 => LETIMER0R::DIV2048,
            12 => LETIMER0R::DIV4096,
            13 => LETIMER0R::DIV8192,
            14 => LETIMER0R::DIV16384,
            15 => LETIMER0R::DIV32768,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == LETIMER0R::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == LETIMER0R::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == LETIMER0R::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == LETIMER0R::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == LETIMER0R::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == LETIMER0R::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == LETIMER0R::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == LETIMER0R::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == LETIMER0R::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline]
    pub fn is_div512(&self) -> bool {
        *self == LETIMER0R::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline]
    pub fn is_div1024(&self) -> bool {
        *self == LETIMER0R::DIV1024
    }
    #[doc = "Checks if the value of the field is `DIV2048`"]
    #[inline]
    pub fn is_div2048(&self) -> bool {
        *self == LETIMER0R::DIV2048
    }
    #[doc = "Checks if the value of the field is `DIV4096`"]
    #[inline]
    pub fn is_div4096(&self) -> bool {
        *self == LETIMER0R::DIV4096
    }
    #[doc = "Checks if the value of the field is `DIV8192`"]
    #[inline]
    pub fn is_div8192(&self) -> bool {
        *self == LETIMER0R::DIV8192
    }
    #[doc = "Checks if the value of the field is `DIV16384`"]
    #[inline]
    pub fn is_div16384(&self) -> bool {
        *self == LETIMER0R::DIV16384
    }
    #[doc = "Checks if the value of the field is `DIV32768`"]
    #[inline]
    pub fn is_div32768(&self) -> bool {
        *self == LETIMER0R::DIV32768
    }
}
#[doc = "Values that can be written to the field `LETIMER0`"]
pub enum LETIMER0W {
    #[doc = "LFACLKLETIMER0 = LFACLK"]
    DIV1,
    #[doc = "LFACLKLETIMER0 = LFACLK/2"]
    DIV2,
    #[doc = "LFACLKLETIMER0 = LFACLK/4"]
    DIV4,
    #[doc = "LFACLKLETIMER0 = LFACLK/8"]
    DIV8,
    #[doc = "LFACLKLETIMER0 = LFACLK/16"]
    DIV16,
    #[doc = "LFACLKLETIMER0 = LFACLK/32"]
    DIV32,
    #[doc = "LFACLKLETIMER0 = LFACLK/64"]
    DIV64,
    #[doc = "LFACLKLETIMER0 = LFACLK/128"]
    DIV128,
    #[doc = "LFACLKLETIMER0 = LFACLK/256"]
    DIV256,
    #[doc = "LFACLKLETIMER0 = LFACLK/512"]
    DIV512,
    #[doc = "LFACLKLETIMER0 = LFACLK/1024"]
    DIV1024,
    #[doc = "LFACLKLETIMER0 = LFACLK/2048"]
    DIV2048,
    #[doc = "LFACLKLETIMER0 = LFACLK/4096"]
    DIV4096,
    #[doc = "LFACLKLETIMER0 = LFACLK/8192"]
    DIV8192,
    #[doc = "LFACLKLETIMER0 = LFACLK/16384"]
    DIV16384,
    #[doc = "LFACLKLETIMER0 = LFACLK/32768"]
    DIV32768,
}
impl LETIMER0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LETIMER0W::DIV1 => 0,
            LETIMER0W::DIV2 => 1,
            LETIMER0W::DIV4 => 2,
            LETIMER0W::DIV8 => 3,
            LETIMER0W::DIV16 => 4,
            LETIMER0W::DIV32 => 5,
            LETIMER0W::DIV64 => 6,
            LETIMER0W::DIV128 => 7,
            LETIMER0W::DIV256 => 8,
            LETIMER0W::DIV512 => 9,
            LETIMER0W::DIV1024 => 10,
            LETIMER0W::DIV2048 => 11,
            LETIMER0W::DIV4096 => 12,
            LETIMER0W::DIV8192 => 13,
            LETIMER0W::DIV16384 => 14,
            LETIMER0W::DIV32768 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LETIMER0W<'a> {
    w: &'a mut W,
}
impl<'a> _LETIMER0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LETIMER0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LFACLKLETIMER0 = LFACLK"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV1)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV2)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV4)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV8)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV16)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV32)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV64)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV128)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV256)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/512"]
    #[inline]
    pub fn div512(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV512)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/1024"]
    #[inline]
    pub fn div1024(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV1024)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/2048"]
    #[inline]
    pub fn div2048(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV2048)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/4096"]
    #[inline]
    pub fn div4096(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV4096)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/8192"]
    #[inline]
    pub fn div8192(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV8192)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/16384"]
    #[inline]
    pub fn div16384(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV16384)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/32768"]
    #[inline]
    pub fn div32768(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV32768)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:3 - Low Energy Timer 0 Prescaler"]
    #[inline]
    pub fn letimer0(&self) -> LETIMER0R {
        LETIMER0R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:3 - Low Energy Timer 0 Prescaler"]
    #[inline]
    pub fn letimer0(&mut self) -> _LETIMER0W {
        _LETIMER0W { w: self }
    }
}
