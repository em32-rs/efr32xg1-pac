#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EXTIPINSELL {
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
#[doc = "Possible values of the field `EXTIPINSEL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPINSEL0R {
    #[doc = "Pin 0"]
    PIN0,
    #[doc = "Pin 1"]
    PIN1,
    #[doc = "Pin 2"]
    PIN2,
    #[doc = "Pin 3"]
    PIN3,
}
impl EXTIPINSEL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPINSEL0R::PIN0 => 0,
            EXTIPINSEL0R::PIN1 => 1,
            EXTIPINSEL0R::PIN2 => 2,
            EXTIPINSEL0R::PIN3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPINSEL0R {
        match value {
            0 => EXTIPINSEL0R::PIN0,
            1 => EXTIPINSEL0R::PIN1,
            2 => EXTIPINSEL0R::PIN2,
            3 => EXTIPINSEL0R::PIN3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline]
    pub fn is_pin0(&self) -> bool {
        *self == EXTIPINSEL0R::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline]
    pub fn is_pin1(&self) -> bool {
        *self == EXTIPINSEL0R::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline]
    pub fn is_pin2(&self) -> bool {
        *self == EXTIPINSEL0R::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline]
    pub fn is_pin3(&self) -> bool {
        *self == EXTIPINSEL0R::PIN3
    }
}
#[doc = "Possible values of the field `EXTIPINSEL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPINSEL1R {
    #[doc = "Pin 0"]
    PIN0,
    #[doc = "Pin 1"]
    PIN1,
    #[doc = "Pin 2"]
    PIN2,
    #[doc = "Pin 3"]
    PIN3,
}
impl EXTIPINSEL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPINSEL1R::PIN0 => 0,
            EXTIPINSEL1R::PIN1 => 1,
            EXTIPINSEL1R::PIN2 => 2,
            EXTIPINSEL1R::PIN3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPINSEL1R {
        match value {
            0 => EXTIPINSEL1R::PIN0,
            1 => EXTIPINSEL1R::PIN1,
            2 => EXTIPINSEL1R::PIN2,
            3 => EXTIPINSEL1R::PIN3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline]
    pub fn is_pin0(&self) -> bool {
        *self == EXTIPINSEL1R::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline]
    pub fn is_pin1(&self) -> bool {
        *self == EXTIPINSEL1R::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline]
    pub fn is_pin2(&self) -> bool {
        *self == EXTIPINSEL1R::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline]
    pub fn is_pin3(&self) -> bool {
        *self == EXTIPINSEL1R::PIN3
    }
}
#[doc = "Possible values of the field `EXTIPINSEL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPINSEL2R {
    #[doc = "Pin 0"]
    PIN0,
    #[doc = "Pin 1"]
    PIN1,
    #[doc = "Pin 2"]
    PIN2,
    #[doc = "Pin 3"]
    PIN3,
}
impl EXTIPINSEL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPINSEL2R::PIN0 => 0,
            EXTIPINSEL2R::PIN1 => 1,
            EXTIPINSEL2R::PIN2 => 2,
            EXTIPINSEL2R::PIN3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPINSEL2R {
        match value {
            0 => EXTIPINSEL2R::PIN0,
            1 => EXTIPINSEL2R::PIN1,
            2 => EXTIPINSEL2R::PIN2,
            3 => EXTIPINSEL2R::PIN3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline]
    pub fn is_pin0(&self) -> bool {
        *self == EXTIPINSEL2R::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline]
    pub fn is_pin1(&self) -> bool {
        *self == EXTIPINSEL2R::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline]
    pub fn is_pin2(&self) -> bool {
        *self == EXTIPINSEL2R::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline]
    pub fn is_pin3(&self) -> bool {
        *self == EXTIPINSEL2R::PIN3
    }
}
#[doc = "Possible values of the field `EXTIPINSEL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPINSEL3R {
    #[doc = "Pin 0"]
    PIN0,
    #[doc = "Pin 1"]
    PIN1,
    #[doc = "Pin 2"]
    PIN2,
    #[doc = "Pin 3"]
    PIN3,
}
impl EXTIPINSEL3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPINSEL3R::PIN0 => 0,
            EXTIPINSEL3R::PIN1 => 1,
            EXTIPINSEL3R::PIN2 => 2,
            EXTIPINSEL3R::PIN3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPINSEL3R {
        match value {
            0 => EXTIPINSEL3R::PIN0,
            1 => EXTIPINSEL3R::PIN1,
            2 => EXTIPINSEL3R::PIN2,
            3 => EXTIPINSEL3R::PIN3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline]
    pub fn is_pin0(&self) -> bool {
        *self == EXTIPINSEL3R::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline]
    pub fn is_pin1(&self) -> bool {
        *self == EXTIPINSEL3R::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline]
    pub fn is_pin2(&self) -> bool {
        *self == EXTIPINSEL3R::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline]
    pub fn is_pin3(&self) -> bool {
        *self == EXTIPINSEL3R::PIN3
    }
}
#[doc = "Possible values of the field `EXTIPINSEL4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPINSEL4R {
    #[doc = "Pin 4"]
    PIN4,
    #[doc = "Pin 5"]
    PIN5,
    #[doc = "Pin 6"]
    PIN6,
    #[doc = "Pin 7"]
    PIN7,
}
impl EXTIPINSEL4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPINSEL4R::PIN4 => 0,
            EXTIPINSEL4R::PIN5 => 1,
            EXTIPINSEL4R::PIN6 => 2,
            EXTIPINSEL4R::PIN7 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPINSEL4R {
        match value {
            0 => EXTIPINSEL4R::PIN4,
            1 => EXTIPINSEL4R::PIN5,
            2 => EXTIPINSEL4R::PIN6,
            3 => EXTIPINSEL4R::PIN7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN4`"]
    #[inline]
    pub fn is_pin4(&self) -> bool {
        *self == EXTIPINSEL4R::PIN4
    }
    #[doc = "Checks if the value of the field is `PIN5`"]
    #[inline]
    pub fn is_pin5(&self) -> bool {
        *self == EXTIPINSEL4R::PIN5
    }
    #[doc = "Checks if the value of the field is `PIN6`"]
    #[inline]
    pub fn is_pin6(&self) -> bool {
        *self == EXTIPINSEL4R::PIN6
    }
    #[doc = "Checks if the value of the field is `PIN7`"]
    #[inline]
    pub fn is_pin7(&self) -> bool {
        *self == EXTIPINSEL4R::PIN7
    }
}
#[doc = "Possible values of the field `EXTIPINSEL5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPINSEL5R {
    #[doc = "Pin 4"]
    PIN4,
    #[doc = "Pin 5"]
    PIN5,
    #[doc = "Pin 6"]
    PIN6,
    #[doc = "Pin 7"]
    PIN7,
}
impl EXTIPINSEL5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPINSEL5R::PIN4 => 0,
            EXTIPINSEL5R::PIN5 => 1,
            EXTIPINSEL5R::PIN6 => 2,
            EXTIPINSEL5R::PIN7 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPINSEL5R {
        match value {
            0 => EXTIPINSEL5R::PIN4,
            1 => EXTIPINSEL5R::PIN5,
            2 => EXTIPINSEL5R::PIN6,
            3 => EXTIPINSEL5R::PIN7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN4`"]
    #[inline]
    pub fn is_pin4(&self) -> bool {
        *self == EXTIPINSEL5R::PIN4
    }
    #[doc = "Checks if the value of the field is `PIN5`"]
    #[inline]
    pub fn is_pin5(&self) -> bool {
        *self == EXTIPINSEL5R::PIN5
    }
    #[doc = "Checks if the value of the field is `PIN6`"]
    #[inline]
    pub fn is_pin6(&self) -> bool {
        *self == EXTIPINSEL5R::PIN6
    }
    #[doc = "Checks if the value of the field is `PIN7`"]
    #[inline]
    pub fn is_pin7(&self) -> bool {
        *self == EXTIPINSEL5R::PIN7
    }
}
#[doc = "Possible values of the field `EXTIPINSEL6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPINSEL6R {
    #[doc = "Pin 4"]
    PIN4,
    #[doc = "Pin 5"]
    PIN5,
    #[doc = "Pin 6"]
    PIN6,
    #[doc = "Pin 7"]
    PIN7,
}
impl EXTIPINSEL6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPINSEL6R::PIN4 => 0,
            EXTIPINSEL6R::PIN5 => 1,
            EXTIPINSEL6R::PIN6 => 2,
            EXTIPINSEL6R::PIN7 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPINSEL6R {
        match value {
            0 => EXTIPINSEL6R::PIN4,
            1 => EXTIPINSEL6R::PIN5,
            2 => EXTIPINSEL6R::PIN6,
            3 => EXTIPINSEL6R::PIN7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN4`"]
    #[inline]
    pub fn is_pin4(&self) -> bool {
        *self == EXTIPINSEL6R::PIN4
    }
    #[doc = "Checks if the value of the field is `PIN5`"]
    #[inline]
    pub fn is_pin5(&self) -> bool {
        *self == EXTIPINSEL6R::PIN5
    }
    #[doc = "Checks if the value of the field is `PIN6`"]
    #[inline]
    pub fn is_pin6(&self) -> bool {
        *self == EXTIPINSEL6R::PIN6
    }
    #[doc = "Checks if the value of the field is `PIN7`"]
    #[inline]
    pub fn is_pin7(&self) -> bool {
        *self == EXTIPINSEL6R::PIN7
    }
}
#[doc = "Possible values of the field `EXTIPINSEL7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPINSEL7R {
    #[doc = "Pin 4"]
    PIN4,
    #[doc = "Pin 5"]
    PIN5,
    #[doc = "Pin 6"]
    PIN6,
    #[doc = "Pin 7"]
    PIN7,
}
impl EXTIPINSEL7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPINSEL7R::PIN4 => 0,
            EXTIPINSEL7R::PIN5 => 1,
            EXTIPINSEL7R::PIN6 => 2,
            EXTIPINSEL7R::PIN7 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPINSEL7R {
        match value {
            0 => EXTIPINSEL7R::PIN4,
            1 => EXTIPINSEL7R::PIN5,
            2 => EXTIPINSEL7R::PIN6,
            3 => EXTIPINSEL7R::PIN7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN4`"]
    #[inline]
    pub fn is_pin4(&self) -> bool {
        *self == EXTIPINSEL7R::PIN4
    }
    #[doc = "Checks if the value of the field is `PIN5`"]
    #[inline]
    pub fn is_pin5(&self) -> bool {
        *self == EXTIPINSEL7R::PIN5
    }
    #[doc = "Checks if the value of the field is `PIN6`"]
    #[inline]
    pub fn is_pin6(&self) -> bool {
        *self == EXTIPINSEL7R::PIN6
    }
    #[doc = "Checks if the value of the field is `PIN7`"]
    #[inline]
    pub fn is_pin7(&self) -> bool {
        *self == EXTIPINSEL7R::PIN7
    }
}
#[doc = "Values that can be written to the field `EXTIPINSEL0`"]
pub enum EXTIPINSEL0W {
    #[doc = "Pin 0"]
    PIN0,
    #[doc = "Pin 1"]
    PIN1,
    #[doc = "Pin 2"]
    PIN2,
    #[doc = "Pin 3"]
    PIN3,
}
impl EXTIPINSEL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPINSEL0W::PIN0 => 0,
            EXTIPINSEL0W::PIN1 => 1,
            EXTIPINSEL0W::PIN2 => 2,
            EXTIPINSEL0W::PIN3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPINSEL0W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPINSEL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPINSEL0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pin 0"]
    #[inline]
    pub fn pin0(self) -> &'a mut W {
        self.variant(EXTIPINSEL0W::PIN0)
    }
    #[doc = "Pin 1"]
    #[inline]
    pub fn pin1(self) -> &'a mut W {
        self.variant(EXTIPINSEL0W::PIN1)
    }
    #[doc = "Pin 2"]
    #[inline]
    pub fn pin2(self) -> &'a mut W {
        self.variant(EXTIPINSEL0W::PIN2)
    }
    #[doc = "Pin 3"]
    #[inline]
    pub fn pin3(self) -> &'a mut W {
        self.variant(EXTIPINSEL0W::PIN3)
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
#[doc = "Values that can be written to the field `EXTIPINSEL1`"]
pub enum EXTIPINSEL1W {
    #[doc = "Pin 0"]
    PIN0,
    #[doc = "Pin 1"]
    PIN1,
    #[doc = "Pin 2"]
    PIN2,
    #[doc = "Pin 3"]
    PIN3,
}
impl EXTIPINSEL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPINSEL1W::PIN0 => 0,
            EXTIPINSEL1W::PIN1 => 1,
            EXTIPINSEL1W::PIN2 => 2,
            EXTIPINSEL1W::PIN3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPINSEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPINSEL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPINSEL1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pin 0"]
    #[inline]
    pub fn pin0(self) -> &'a mut W {
        self.variant(EXTIPINSEL1W::PIN0)
    }
    #[doc = "Pin 1"]
    #[inline]
    pub fn pin1(self) -> &'a mut W {
        self.variant(EXTIPINSEL1W::PIN1)
    }
    #[doc = "Pin 2"]
    #[inline]
    pub fn pin2(self) -> &'a mut W {
        self.variant(EXTIPINSEL1W::PIN2)
    }
    #[doc = "Pin 3"]
    #[inline]
    pub fn pin3(self) -> &'a mut W {
        self.variant(EXTIPINSEL1W::PIN3)
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
#[doc = "Values that can be written to the field `EXTIPINSEL2`"]
pub enum EXTIPINSEL2W {
    #[doc = "Pin 0"]
    PIN0,
    #[doc = "Pin 1"]
    PIN1,
    #[doc = "Pin 2"]
    PIN2,
    #[doc = "Pin 3"]
    PIN3,
}
impl EXTIPINSEL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPINSEL2W::PIN0 => 0,
            EXTIPINSEL2W::PIN1 => 1,
            EXTIPINSEL2W::PIN2 => 2,
            EXTIPINSEL2W::PIN3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPINSEL2W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPINSEL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPINSEL2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pin 0"]
    #[inline]
    pub fn pin0(self) -> &'a mut W {
        self.variant(EXTIPINSEL2W::PIN0)
    }
    #[doc = "Pin 1"]
    #[inline]
    pub fn pin1(self) -> &'a mut W {
        self.variant(EXTIPINSEL2W::PIN1)
    }
    #[doc = "Pin 2"]
    #[inline]
    pub fn pin2(self) -> &'a mut W {
        self.variant(EXTIPINSEL2W::PIN2)
    }
    #[doc = "Pin 3"]
    #[inline]
    pub fn pin3(self) -> &'a mut W {
        self.variant(EXTIPINSEL2W::PIN3)
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
#[doc = "Values that can be written to the field `EXTIPINSEL3`"]
pub enum EXTIPINSEL3W {
    #[doc = "Pin 0"]
    PIN0,
    #[doc = "Pin 1"]
    PIN1,
    #[doc = "Pin 2"]
    PIN2,
    #[doc = "Pin 3"]
    PIN3,
}
impl EXTIPINSEL3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPINSEL3W::PIN0 => 0,
            EXTIPINSEL3W::PIN1 => 1,
            EXTIPINSEL3W::PIN2 => 2,
            EXTIPINSEL3W::PIN3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPINSEL3W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPINSEL3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPINSEL3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pin 0"]
    #[inline]
    pub fn pin0(self) -> &'a mut W {
        self.variant(EXTIPINSEL3W::PIN0)
    }
    #[doc = "Pin 1"]
    #[inline]
    pub fn pin1(self) -> &'a mut W {
        self.variant(EXTIPINSEL3W::PIN1)
    }
    #[doc = "Pin 2"]
    #[inline]
    pub fn pin2(self) -> &'a mut W {
        self.variant(EXTIPINSEL3W::PIN2)
    }
    #[doc = "Pin 3"]
    #[inline]
    pub fn pin3(self) -> &'a mut W {
        self.variant(EXTIPINSEL3W::PIN3)
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
#[doc = "Values that can be written to the field `EXTIPINSEL4`"]
pub enum EXTIPINSEL4W {
    #[doc = "Pin 4"]
    PIN4,
    #[doc = "Pin 5"]
    PIN5,
    #[doc = "Pin 6"]
    PIN6,
    #[doc = "Pin 7"]
    PIN7,
}
impl EXTIPINSEL4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPINSEL4W::PIN4 => 0,
            EXTIPINSEL4W::PIN5 => 1,
            EXTIPINSEL4W::PIN6 => 2,
            EXTIPINSEL4W::PIN7 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPINSEL4W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPINSEL4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPINSEL4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pin 4"]
    #[inline]
    pub fn pin4(self) -> &'a mut W {
        self.variant(EXTIPINSEL4W::PIN4)
    }
    #[doc = "Pin 5"]
    #[inline]
    pub fn pin5(self) -> &'a mut W {
        self.variant(EXTIPINSEL4W::PIN5)
    }
    #[doc = "Pin 6"]
    #[inline]
    pub fn pin6(self) -> &'a mut W {
        self.variant(EXTIPINSEL4W::PIN6)
    }
    #[doc = "Pin 7"]
    #[inline]
    pub fn pin7(self) -> &'a mut W {
        self.variant(EXTIPINSEL4W::PIN7)
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
#[doc = "Values that can be written to the field `EXTIPINSEL5`"]
pub enum EXTIPINSEL5W {
    #[doc = "Pin 4"]
    PIN4,
    #[doc = "Pin 5"]
    PIN5,
    #[doc = "Pin 6"]
    PIN6,
    #[doc = "Pin 7"]
    PIN7,
}
impl EXTIPINSEL5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPINSEL5W::PIN4 => 0,
            EXTIPINSEL5W::PIN5 => 1,
            EXTIPINSEL5W::PIN6 => 2,
            EXTIPINSEL5W::PIN7 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPINSEL5W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPINSEL5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPINSEL5W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pin 4"]
    #[inline]
    pub fn pin4(self) -> &'a mut W {
        self.variant(EXTIPINSEL5W::PIN4)
    }
    #[doc = "Pin 5"]
    #[inline]
    pub fn pin5(self) -> &'a mut W {
        self.variant(EXTIPINSEL5W::PIN5)
    }
    #[doc = "Pin 6"]
    #[inline]
    pub fn pin6(self) -> &'a mut W {
        self.variant(EXTIPINSEL5W::PIN6)
    }
    #[doc = "Pin 7"]
    #[inline]
    pub fn pin7(self) -> &'a mut W {
        self.variant(EXTIPINSEL5W::PIN7)
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
#[doc = "Values that can be written to the field `EXTIPINSEL6`"]
pub enum EXTIPINSEL6W {
    #[doc = "Pin 4"]
    PIN4,
    #[doc = "Pin 5"]
    PIN5,
    #[doc = "Pin 6"]
    PIN6,
    #[doc = "Pin 7"]
    PIN7,
}
impl EXTIPINSEL6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPINSEL6W::PIN4 => 0,
            EXTIPINSEL6W::PIN5 => 1,
            EXTIPINSEL6W::PIN6 => 2,
            EXTIPINSEL6W::PIN7 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPINSEL6W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPINSEL6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPINSEL6W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pin 4"]
    #[inline]
    pub fn pin4(self) -> &'a mut W {
        self.variant(EXTIPINSEL6W::PIN4)
    }
    #[doc = "Pin 5"]
    #[inline]
    pub fn pin5(self) -> &'a mut W {
        self.variant(EXTIPINSEL6W::PIN5)
    }
    #[doc = "Pin 6"]
    #[inline]
    pub fn pin6(self) -> &'a mut W {
        self.variant(EXTIPINSEL6W::PIN6)
    }
    #[doc = "Pin 7"]
    #[inline]
    pub fn pin7(self) -> &'a mut W {
        self.variant(EXTIPINSEL6W::PIN7)
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
#[doc = "Values that can be written to the field `EXTIPINSEL7`"]
pub enum EXTIPINSEL7W {
    #[doc = "Pin 4"]
    PIN4,
    #[doc = "Pin 5"]
    PIN5,
    #[doc = "Pin 6"]
    PIN6,
    #[doc = "Pin 7"]
    PIN7,
}
impl EXTIPINSEL7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPINSEL7W::PIN4 => 0,
            EXTIPINSEL7W::PIN5 => 1,
            EXTIPINSEL7W::PIN6 => 2,
            EXTIPINSEL7W::PIN7 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPINSEL7W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPINSEL7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPINSEL7W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pin 4"]
    #[inline]
    pub fn pin4(self) -> &'a mut W {
        self.variant(EXTIPINSEL7W::PIN4)
    }
    #[doc = "Pin 5"]
    #[inline]
    pub fn pin5(self) -> &'a mut W {
        self.variant(EXTIPINSEL7W::PIN5)
    }
    #[doc = "Pin 6"]
    #[inline]
    pub fn pin6(self) -> &'a mut W {
        self.variant(EXTIPINSEL7W::PIN6)
    }
    #[doc = "Pin 7"]
    #[inline]
    pub fn pin7(self) -> &'a mut W {
        self.variant(EXTIPINSEL7W::PIN7)
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
    #[doc = "Bits 0:1 - External Interrupt 0 Pin Select"]
    #[inline]
    pub fn extipinsel0(&self) -> EXTIPINSEL0R {
        EXTIPINSEL0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - External Interrupt 1 Pin Select"]
    #[inline]
    pub fn extipinsel1(&self) -> EXTIPINSEL1R {
        EXTIPINSEL1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - External Interrupt 2 Pin Select"]
    #[inline]
    pub fn extipinsel2(&self) -> EXTIPINSEL2R {
        EXTIPINSEL2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - External Interrupt 3 Pin Select"]
    #[inline]
    pub fn extipinsel3(&self) -> EXTIPINSEL3R {
        EXTIPINSEL3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - External Interrupt 4 Pin Select"]
    #[inline]
    pub fn extipinsel4(&self) -> EXTIPINSEL4R {
        EXTIPINSEL4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - External Interrupt 5 Pin Select"]
    #[inline]
    pub fn extipinsel5(&self) -> EXTIPINSEL5R {
        EXTIPINSEL5R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - External Interrupt 6 Pin Select"]
    #[inline]
    pub fn extipinsel6(&self) -> EXTIPINSEL6R {
        EXTIPINSEL6R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - External Interrupt 7 Pin Select"]
    #[inline]
    pub fn extipinsel7(&self) -> EXTIPINSEL7R {
        EXTIPINSEL7R::_from({
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
    #[doc = "Bits 0:1 - External Interrupt 0 Pin Select"]
    #[inline]
    pub fn extipinsel0(&mut self) -> _EXTIPINSEL0W {
        _EXTIPINSEL0W { w: self }
    }
    #[doc = "Bits 4:5 - External Interrupt 1 Pin Select"]
    #[inline]
    pub fn extipinsel1(&mut self) -> _EXTIPINSEL1W {
        _EXTIPINSEL1W { w: self }
    }
    #[doc = "Bits 8:9 - External Interrupt 2 Pin Select"]
    #[inline]
    pub fn extipinsel2(&mut self) -> _EXTIPINSEL2W {
        _EXTIPINSEL2W { w: self }
    }
    #[doc = "Bits 12:13 - External Interrupt 3 Pin Select"]
    #[inline]
    pub fn extipinsel3(&mut self) -> _EXTIPINSEL3W {
        _EXTIPINSEL3W { w: self }
    }
    #[doc = "Bits 16:17 - External Interrupt 4 Pin Select"]
    #[inline]
    pub fn extipinsel4(&mut self) -> _EXTIPINSEL4W {
        _EXTIPINSEL4W { w: self }
    }
    #[doc = "Bits 20:21 - External Interrupt 5 Pin Select"]
    #[inline]
    pub fn extipinsel5(&mut self) -> _EXTIPINSEL5W {
        _EXTIPINSEL5W { w: self }
    }
    #[doc = "Bits 24:25 - External Interrupt 6 Pin Select"]
    #[inline]
    pub fn extipinsel6(&mut self) -> _EXTIPINSEL6W {
        _EXTIPINSEL6W { w: self }
    }
    #[doc = "Bits 28:29 - External Interrupt 7 Pin Select"]
    #[inline]
    pub fn extipinsel7(&mut self) -> _EXTIPINSEL7W {
        _EXTIPINSEL7W { w: self }
    }
}
