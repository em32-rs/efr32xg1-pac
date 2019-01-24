#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EXTIPINSELH {
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
#[doc = "Possible values of the field `EXTIPINSEL8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPINSEL8R {
    #[doc = "Pin 8"]
    PIN8,
    #[doc = "Pin 9"]
    PIN9,
    #[doc = "Pin 10"]
    PIN10,
    #[doc = "Pin 11"]
    PIN11,
}
impl EXTIPINSEL8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPINSEL8R::PIN8 => 0,
            EXTIPINSEL8R::PIN9 => 1,
            EXTIPINSEL8R::PIN10 => 2,
            EXTIPINSEL8R::PIN11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPINSEL8R {
        match value {
            0 => EXTIPINSEL8R::PIN8,
            1 => EXTIPINSEL8R::PIN9,
            2 => EXTIPINSEL8R::PIN10,
            3 => EXTIPINSEL8R::PIN11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN8`"]
    #[inline]
    pub fn is_pin8(&self) -> bool {
        *self == EXTIPINSEL8R::PIN8
    }
    #[doc = "Checks if the value of the field is `PIN9`"]
    #[inline]
    pub fn is_pin9(&self) -> bool {
        *self == EXTIPINSEL8R::PIN9
    }
    #[doc = "Checks if the value of the field is `PIN10`"]
    #[inline]
    pub fn is_pin10(&self) -> bool {
        *self == EXTIPINSEL8R::PIN10
    }
    #[doc = "Checks if the value of the field is `PIN11`"]
    #[inline]
    pub fn is_pin11(&self) -> bool {
        *self == EXTIPINSEL8R::PIN11
    }
}
#[doc = "Possible values of the field `EXTIPINSEL9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPINSEL9R {
    #[doc = "Pin 8"]
    PIN8,
    #[doc = "Pin 9"]
    PIN9,
    #[doc = "Pin 10"]
    PIN10,
    #[doc = "Pin 11"]
    PIN11,
}
impl EXTIPINSEL9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPINSEL9R::PIN8 => 0,
            EXTIPINSEL9R::PIN9 => 1,
            EXTIPINSEL9R::PIN10 => 2,
            EXTIPINSEL9R::PIN11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPINSEL9R {
        match value {
            0 => EXTIPINSEL9R::PIN8,
            1 => EXTIPINSEL9R::PIN9,
            2 => EXTIPINSEL9R::PIN10,
            3 => EXTIPINSEL9R::PIN11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN8`"]
    #[inline]
    pub fn is_pin8(&self) -> bool {
        *self == EXTIPINSEL9R::PIN8
    }
    #[doc = "Checks if the value of the field is `PIN9`"]
    #[inline]
    pub fn is_pin9(&self) -> bool {
        *self == EXTIPINSEL9R::PIN9
    }
    #[doc = "Checks if the value of the field is `PIN10`"]
    #[inline]
    pub fn is_pin10(&self) -> bool {
        *self == EXTIPINSEL9R::PIN10
    }
    #[doc = "Checks if the value of the field is `PIN11`"]
    #[inline]
    pub fn is_pin11(&self) -> bool {
        *self == EXTIPINSEL9R::PIN11
    }
}
#[doc = "Possible values of the field `EXTIPINSEL10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPINSEL10R {
    #[doc = "Pin 8"]
    PIN8,
    #[doc = "Pin 9"]
    PIN9,
    #[doc = "Pin 10"]
    PIN10,
    #[doc = "Pin 11"]
    PIN11,
}
impl EXTIPINSEL10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPINSEL10R::PIN8 => 0,
            EXTIPINSEL10R::PIN9 => 1,
            EXTIPINSEL10R::PIN10 => 2,
            EXTIPINSEL10R::PIN11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPINSEL10R {
        match value {
            0 => EXTIPINSEL10R::PIN8,
            1 => EXTIPINSEL10R::PIN9,
            2 => EXTIPINSEL10R::PIN10,
            3 => EXTIPINSEL10R::PIN11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN8`"]
    #[inline]
    pub fn is_pin8(&self) -> bool {
        *self == EXTIPINSEL10R::PIN8
    }
    #[doc = "Checks if the value of the field is `PIN9`"]
    #[inline]
    pub fn is_pin9(&self) -> bool {
        *self == EXTIPINSEL10R::PIN9
    }
    #[doc = "Checks if the value of the field is `PIN10`"]
    #[inline]
    pub fn is_pin10(&self) -> bool {
        *self == EXTIPINSEL10R::PIN10
    }
    #[doc = "Checks if the value of the field is `PIN11`"]
    #[inline]
    pub fn is_pin11(&self) -> bool {
        *self == EXTIPINSEL10R::PIN11
    }
}
#[doc = "Possible values of the field `EXTIPINSEL11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPINSEL11R {
    #[doc = "Pin 8"]
    PIN8,
    #[doc = "Pin 9"]
    PIN9,
    #[doc = "Pin 10"]
    PIN10,
    #[doc = "Pin 11"]
    PIN11,
}
impl EXTIPINSEL11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPINSEL11R::PIN8 => 0,
            EXTIPINSEL11R::PIN9 => 1,
            EXTIPINSEL11R::PIN10 => 2,
            EXTIPINSEL11R::PIN11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPINSEL11R {
        match value {
            0 => EXTIPINSEL11R::PIN8,
            1 => EXTIPINSEL11R::PIN9,
            2 => EXTIPINSEL11R::PIN10,
            3 => EXTIPINSEL11R::PIN11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN8`"]
    #[inline]
    pub fn is_pin8(&self) -> bool {
        *self == EXTIPINSEL11R::PIN8
    }
    #[doc = "Checks if the value of the field is `PIN9`"]
    #[inline]
    pub fn is_pin9(&self) -> bool {
        *self == EXTIPINSEL11R::PIN9
    }
    #[doc = "Checks if the value of the field is `PIN10`"]
    #[inline]
    pub fn is_pin10(&self) -> bool {
        *self == EXTIPINSEL11R::PIN10
    }
    #[doc = "Checks if the value of the field is `PIN11`"]
    #[inline]
    pub fn is_pin11(&self) -> bool {
        *self == EXTIPINSEL11R::PIN11
    }
}
#[doc = "Possible values of the field `EXTIPINSEL12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPINSEL12R {
    #[doc = "Pin 12"]
    PIN12,
    #[doc = "Pin 13"]
    PIN13,
    #[doc = "Pin 14"]
    PIN14,
    #[doc = "Pin 15"]
    PIN15,
}
impl EXTIPINSEL12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPINSEL12R::PIN12 => 0,
            EXTIPINSEL12R::PIN13 => 1,
            EXTIPINSEL12R::PIN14 => 2,
            EXTIPINSEL12R::PIN15 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPINSEL12R {
        match value {
            0 => EXTIPINSEL12R::PIN12,
            1 => EXTIPINSEL12R::PIN13,
            2 => EXTIPINSEL12R::PIN14,
            3 => EXTIPINSEL12R::PIN15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN12`"]
    #[inline]
    pub fn is_pin12(&self) -> bool {
        *self == EXTIPINSEL12R::PIN12
    }
    #[doc = "Checks if the value of the field is `PIN13`"]
    #[inline]
    pub fn is_pin13(&self) -> bool {
        *self == EXTIPINSEL12R::PIN13
    }
    #[doc = "Checks if the value of the field is `PIN14`"]
    #[inline]
    pub fn is_pin14(&self) -> bool {
        *self == EXTIPINSEL12R::PIN14
    }
    #[doc = "Checks if the value of the field is `PIN15`"]
    #[inline]
    pub fn is_pin15(&self) -> bool {
        *self == EXTIPINSEL12R::PIN15
    }
}
#[doc = "Possible values of the field `EXTIPINSEL13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPINSEL13R {
    #[doc = "Pin 12"]
    PIN12,
    #[doc = "Pin 13"]
    PIN13,
    #[doc = "Pin 14"]
    PIN14,
    #[doc = "Pin 15"]
    PIN15,
}
impl EXTIPINSEL13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPINSEL13R::PIN12 => 0,
            EXTIPINSEL13R::PIN13 => 1,
            EXTIPINSEL13R::PIN14 => 2,
            EXTIPINSEL13R::PIN15 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPINSEL13R {
        match value {
            0 => EXTIPINSEL13R::PIN12,
            1 => EXTIPINSEL13R::PIN13,
            2 => EXTIPINSEL13R::PIN14,
            3 => EXTIPINSEL13R::PIN15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN12`"]
    #[inline]
    pub fn is_pin12(&self) -> bool {
        *self == EXTIPINSEL13R::PIN12
    }
    #[doc = "Checks if the value of the field is `PIN13`"]
    #[inline]
    pub fn is_pin13(&self) -> bool {
        *self == EXTIPINSEL13R::PIN13
    }
    #[doc = "Checks if the value of the field is `PIN14`"]
    #[inline]
    pub fn is_pin14(&self) -> bool {
        *self == EXTIPINSEL13R::PIN14
    }
    #[doc = "Checks if the value of the field is `PIN15`"]
    #[inline]
    pub fn is_pin15(&self) -> bool {
        *self == EXTIPINSEL13R::PIN15
    }
}
#[doc = "Possible values of the field `EXTIPINSEL14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPINSEL14R {
    #[doc = "Pin 12"]
    PIN12,
    #[doc = "Pin 13"]
    PIN13,
    #[doc = "Pin 14"]
    PIN14,
    #[doc = "Pin 15"]
    PIN15,
}
impl EXTIPINSEL14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPINSEL14R::PIN12 => 0,
            EXTIPINSEL14R::PIN13 => 1,
            EXTIPINSEL14R::PIN14 => 2,
            EXTIPINSEL14R::PIN15 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPINSEL14R {
        match value {
            0 => EXTIPINSEL14R::PIN12,
            1 => EXTIPINSEL14R::PIN13,
            2 => EXTIPINSEL14R::PIN14,
            3 => EXTIPINSEL14R::PIN15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN12`"]
    #[inline]
    pub fn is_pin12(&self) -> bool {
        *self == EXTIPINSEL14R::PIN12
    }
    #[doc = "Checks if the value of the field is `PIN13`"]
    #[inline]
    pub fn is_pin13(&self) -> bool {
        *self == EXTIPINSEL14R::PIN13
    }
    #[doc = "Checks if the value of the field is `PIN14`"]
    #[inline]
    pub fn is_pin14(&self) -> bool {
        *self == EXTIPINSEL14R::PIN14
    }
    #[doc = "Checks if the value of the field is `PIN15`"]
    #[inline]
    pub fn is_pin15(&self) -> bool {
        *self == EXTIPINSEL14R::PIN15
    }
}
#[doc = "Possible values of the field `EXTIPINSEL15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPINSEL15R {
    #[doc = "Pin 12"]
    PIN12,
    #[doc = "Pin 13"]
    PIN13,
    #[doc = "Pin 14"]
    PIN14,
    #[doc = "Pin 15"]
    PIN15,
}
impl EXTIPINSEL15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPINSEL15R::PIN12 => 0,
            EXTIPINSEL15R::PIN13 => 1,
            EXTIPINSEL15R::PIN14 => 2,
            EXTIPINSEL15R::PIN15 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPINSEL15R {
        match value {
            0 => EXTIPINSEL15R::PIN12,
            1 => EXTIPINSEL15R::PIN13,
            2 => EXTIPINSEL15R::PIN14,
            3 => EXTIPINSEL15R::PIN15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN12`"]
    #[inline]
    pub fn is_pin12(&self) -> bool {
        *self == EXTIPINSEL15R::PIN12
    }
    #[doc = "Checks if the value of the field is `PIN13`"]
    #[inline]
    pub fn is_pin13(&self) -> bool {
        *self == EXTIPINSEL15R::PIN13
    }
    #[doc = "Checks if the value of the field is `PIN14`"]
    #[inline]
    pub fn is_pin14(&self) -> bool {
        *self == EXTIPINSEL15R::PIN14
    }
    #[doc = "Checks if the value of the field is `PIN15`"]
    #[inline]
    pub fn is_pin15(&self) -> bool {
        *self == EXTIPINSEL15R::PIN15
    }
}
#[doc = "Values that can be written to the field `EXTIPINSEL8`"]
pub enum EXTIPINSEL8W {
    #[doc = "Pin 8"]
    PIN8,
    #[doc = "Pin 9"]
    PIN9,
    #[doc = "Pin 10"]
    PIN10,
    #[doc = "Pin 11"]
    PIN11,
}
impl EXTIPINSEL8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPINSEL8W::PIN8 => 0,
            EXTIPINSEL8W::PIN9 => 1,
            EXTIPINSEL8W::PIN10 => 2,
            EXTIPINSEL8W::PIN11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPINSEL8W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPINSEL8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPINSEL8W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pin 8"]
    #[inline]
    pub fn pin8(self) -> &'a mut W {
        self.variant(EXTIPINSEL8W::PIN8)
    }
    #[doc = "Pin 9"]
    #[inline]
    pub fn pin9(self) -> &'a mut W {
        self.variant(EXTIPINSEL8W::PIN9)
    }
    #[doc = "Pin 10"]
    #[inline]
    pub fn pin10(self) -> &'a mut W {
        self.variant(EXTIPINSEL8W::PIN10)
    }
    #[doc = "Pin 11"]
    #[inline]
    pub fn pin11(self) -> &'a mut W {
        self.variant(EXTIPINSEL8W::PIN11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTIPINSEL9`"]
pub enum EXTIPINSEL9W {
    #[doc = "Pin 8"]
    PIN8,
    #[doc = "Pin 9"]
    PIN9,
    #[doc = "Pin 10"]
    PIN10,
    #[doc = "Pin 11"]
    PIN11,
}
impl EXTIPINSEL9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPINSEL9W::PIN8 => 0,
            EXTIPINSEL9W::PIN9 => 1,
            EXTIPINSEL9W::PIN10 => 2,
            EXTIPINSEL9W::PIN11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPINSEL9W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPINSEL9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPINSEL9W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pin 8"]
    #[inline]
    pub fn pin8(self) -> &'a mut W {
        self.variant(EXTIPINSEL9W::PIN8)
    }
    #[doc = "Pin 9"]
    #[inline]
    pub fn pin9(self) -> &'a mut W {
        self.variant(EXTIPINSEL9W::PIN9)
    }
    #[doc = "Pin 10"]
    #[inline]
    pub fn pin10(self) -> &'a mut W {
        self.variant(EXTIPINSEL9W::PIN10)
    }
    #[doc = "Pin 11"]
    #[inline]
    pub fn pin11(self) -> &'a mut W {
        self.variant(EXTIPINSEL9W::PIN11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTIPINSEL10`"]
pub enum EXTIPINSEL10W {
    #[doc = "Pin 8"]
    PIN8,
    #[doc = "Pin 9"]
    PIN9,
    #[doc = "Pin 10"]
    PIN10,
    #[doc = "Pin 11"]
    PIN11,
}
impl EXTIPINSEL10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPINSEL10W::PIN8 => 0,
            EXTIPINSEL10W::PIN9 => 1,
            EXTIPINSEL10W::PIN10 => 2,
            EXTIPINSEL10W::PIN11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPINSEL10W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPINSEL10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPINSEL10W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pin 8"]
    #[inline]
    pub fn pin8(self) -> &'a mut W {
        self.variant(EXTIPINSEL10W::PIN8)
    }
    #[doc = "Pin 9"]
    #[inline]
    pub fn pin9(self) -> &'a mut W {
        self.variant(EXTIPINSEL10W::PIN9)
    }
    #[doc = "Pin 10"]
    #[inline]
    pub fn pin10(self) -> &'a mut W {
        self.variant(EXTIPINSEL10W::PIN10)
    }
    #[doc = "Pin 11"]
    #[inline]
    pub fn pin11(self) -> &'a mut W {
        self.variant(EXTIPINSEL10W::PIN11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTIPINSEL11`"]
pub enum EXTIPINSEL11W {
    #[doc = "Pin 8"]
    PIN8,
    #[doc = "Pin 9"]
    PIN9,
    #[doc = "Pin 10"]
    PIN10,
    #[doc = "Pin 11"]
    PIN11,
}
impl EXTIPINSEL11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPINSEL11W::PIN8 => 0,
            EXTIPINSEL11W::PIN9 => 1,
            EXTIPINSEL11W::PIN10 => 2,
            EXTIPINSEL11W::PIN11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPINSEL11W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPINSEL11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPINSEL11W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pin 8"]
    #[inline]
    pub fn pin8(self) -> &'a mut W {
        self.variant(EXTIPINSEL11W::PIN8)
    }
    #[doc = "Pin 9"]
    #[inline]
    pub fn pin9(self) -> &'a mut W {
        self.variant(EXTIPINSEL11W::PIN9)
    }
    #[doc = "Pin 10"]
    #[inline]
    pub fn pin10(self) -> &'a mut W {
        self.variant(EXTIPINSEL11W::PIN10)
    }
    #[doc = "Pin 11"]
    #[inline]
    pub fn pin11(self) -> &'a mut W {
        self.variant(EXTIPINSEL11W::PIN11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTIPINSEL12`"]
pub enum EXTIPINSEL12W {
    #[doc = "Pin 12"]
    PIN12,
    #[doc = "Pin 13"]
    PIN13,
    #[doc = "Pin 14"]
    PIN14,
    #[doc = "Pin 15"]
    PIN15,
}
impl EXTIPINSEL12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPINSEL12W::PIN12 => 0,
            EXTIPINSEL12W::PIN13 => 1,
            EXTIPINSEL12W::PIN14 => 2,
            EXTIPINSEL12W::PIN15 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPINSEL12W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPINSEL12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPINSEL12W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pin 12"]
    #[inline]
    pub fn pin12(self) -> &'a mut W {
        self.variant(EXTIPINSEL12W::PIN12)
    }
    #[doc = "Pin 13"]
    #[inline]
    pub fn pin13(self) -> &'a mut W {
        self.variant(EXTIPINSEL12W::PIN13)
    }
    #[doc = "Pin 14"]
    #[inline]
    pub fn pin14(self) -> &'a mut W {
        self.variant(EXTIPINSEL12W::PIN14)
    }
    #[doc = "Pin 15"]
    #[inline]
    pub fn pin15(self) -> &'a mut W {
        self.variant(EXTIPINSEL12W::PIN15)
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
#[doc = "Values that can be written to the field `EXTIPINSEL13`"]
pub enum EXTIPINSEL13W {
    #[doc = "Pin 12"]
    PIN12,
    #[doc = "Pin 13"]
    PIN13,
    #[doc = "Pin 14"]
    PIN14,
    #[doc = "Pin 15"]
    PIN15,
}
impl EXTIPINSEL13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPINSEL13W::PIN12 => 0,
            EXTIPINSEL13W::PIN13 => 1,
            EXTIPINSEL13W::PIN14 => 2,
            EXTIPINSEL13W::PIN15 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPINSEL13W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPINSEL13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPINSEL13W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pin 12"]
    #[inline]
    pub fn pin12(self) -> &'a mut W {
        self.variant(EXTIPINSEL13W::PIN12)
    }
    #[doc = "Pin 13"]
    #[inline]
    pub fn pin13(self) -> &'a mut W {
        self.variant(EXTIPINSEL13W::PIN13)
    }
    #[doc = "Pin 14"]
    #[inline]
    pub fn pin14(self) -> &'a mut W {
        self.variant(EXTIPINSEL13W::PIN14)
    }
    #[doc = "Pin 15"]
    #[inline]
    pub fn pin15(self) -> &'a mut W {
        self.variant(EXTIPINSEL13W::PIN15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTIPINSEL14`"]
pub enum EXTIPINSEL14W {
    #[doc = "Pin 12"]
    PIN12,
    #[doc = "Pin 13"]
    PIN13,
    #[doc = "Pin 14"]
    PIN14,
    #[doc = "Pin 15"]
    PIN15,
}
impl EXTIPINSEL14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPINSEL14W::PIN12 => 0,
            EXTIPINSEL14W::PIN13 => 1,
            EXTIPINSEL14W::PIN14 => 2,
            EXTIPINSEL14W::PIN15 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPINSEL14W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPINSEL14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPINSEL14W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pin 12"]
    #[inline]
    pub fn pin12(self) -> &'a mut W {
        self.variant(EXTIPINSEL14W::PIN12)
    }
    #[doc = "Pin 13"]
    #[inline]
    pub fn pin13(self) -> &'a mut W {
        self.variant(EXTIPINSEL14W::PIN13)
    }
    #[doc = "Pin 14"]
    #[inline]
    pub fn pin14(self) -> &'a mut W {
        self.variant(EXTIPINSEL14W::PIN14)
    }
    #[doc = "Pin 15"]
    #[inline]
    pub fn pin15(self) -> &'a mut W {
        self.variant(EXTIPINSEL14W::PIN15)
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
#[doc = "Values that can be written to the field `EXTIPINSEL15`"]
pub enum EXTIPINSEL15W {
    #[doc = "Pin 12"]
    PIN12,
    #[doc = "Pin 13"]
    PIN13,
    #[doc = "Pin 14"]
    PIN14,
    #[doc = "Pin 15"]
    PIN15,
}
impl EXTIPINSEL15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPINSEL15W::PIN12 => 0,
            EXTIPINSEL15W::PIN13 => 1,
            EXTIPINSEL15W::PIN14 => 2,
            EXTIPINSEL15W::PIN15 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPINSEL15W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPINSEL15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPINSEL15W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pin 12"]
    #[inline]
    pub fn pin12(self) -> &'a mut W {
        self.variant(EXTIPINSEL15W::PIN12)
    }
    #[doc = "Pin 13"]
    #[inline]
    pub fn pin13(self) -> &'a mut W {
        self.variant(EXTIPINSEL15W::PIN13)
    }
    #[doc = "Pin 14"]
    #[inline]
    pub fn pin14(self) -> &'a mut W {
        self.variant(EXTIPINSEL15W::PIN14)
    }
    #[doc = "Pin 15"]
    #[inline]
    pub fn pin15(self) -> &'a mut W {
        self.variant(EXTIPINSEL15W::PIN15)
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
    #[doc = "Bits 0:1 - External Interrupt 8 Pin Select"]
    #[inline]
    pub fn extipinsel8(&self) -> EXTIPINSEL8R {
        EXTIPINSEL8R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - External Interrupt 9 Pin Select"]
    #[inline]
    pub fn extipinsel9(&self) -> EXTIPINSEL9R {
        EXTIPINSEL9R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - External Interrupt 10 Pin Select"]
    #[inline]
    pub fn extipinsel10(&self) -> EXTIPINSEL10R {
        EXTIPINSEL10R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - External Interrupt 11 Pin Select"]
    #[inline]
    pub fn extipinsel11(&self) -> EXTIPINSEL11R {
        EXTIPINSEL11R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - External Interrupt 12 Pin Select"]
    #[inline]
    pub fn extipinsel12(&self) -> EXTIPINSEL12R {
        EXTIPINSEL12R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - External Interrupt 13 Pin Select"]
    #[inline]
    pub fn extipinsel13(&self) -> EXTIPINSEL13R {
        EXTIPINSEL13R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - External Interrupt 14 Pin Select"]
    #[inline]
    pub fn extipinsel14(&self) -> EXTIPINSEL14R {
        EXTIPINSEL14R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - External Interrupt 15 Pin Select"]
    #[inline]
    pub fn extipinsel15(&self) -> EXTIPINSEL15R {
        EXTIPINSEL15R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 839922192 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - External Interrupt 8 Pin Select"]
    #[inline]
    pub fn extipinsel8(&mut self) -> _EXTIPINSEL8W {
        _EXTIPINSEL8W { w: self }
    }
    #[doc = "Bits 4:5 - External Interrupt 9 Pin Select"]
    #[inline]
    pub fn extipinsel9(&mut self) -> _EXTIPINSEL9W {
        _EXTIPINSEL9W { w: self }
    }
    #[doc = "Bits 8:9 - External Interrupt 10 Pin Select"]
    #[inline]
    pub fn extipinsel10(&mut self) -> _EXTIPINSEL10W {
        _EXTIPINSEL10W { w: self }
    }
    #[doc = "Bits 12:13 - External Interrupt 11 Pin Select"]
    #[inline]
    pub fn extipinsel11(&mut self) -> _EXTIPINSEL11W {
        _EXTIPINSEL11W { w: self }
    }
    #[doc = "Bits 16:17 - External Interrupt 12 Pin Select"]
    #[inline]
    pub fn extipinsel12(&mut self) -> _EXTIPINSEL12W {
        _EXTIPINSEL12W { w: self }
    }
    #[doc = "Bits 20:21 - External Interrupt 13 Pin Select"]
    #[inline]
    pub fn extipinsel13(&mut self) -> _EXTIPINSEL13W {
        _EXTIPINSEL13W { w: self }
    }
    #[doc = "Bits 24:25 - External Interrupt 14 Pin Select"]
    #[inline]
    pub fn extipinsel14(&mut self) -> _EXTIPINSEL14W {
        _EXTIPINSEL14W { w: self }
    }
    #[doc = "Bits 28:29 - External Interrupt 15 Pin Select"]
    #[inline]
    pub fn extipinsel15(&mut self) -> _EXTIPINSEL15W {
        _EXTIPINSEL15W { w: self }
    }
}
