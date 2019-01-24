#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCANNEGSEL {
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
#[doc = "Possible values of the field `INPUT0NEGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT0NEGSELR {
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    INPUT1,
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    INPUT3,
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    INPUT5,
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    INPUT7,
}
impl INPUT0NEGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPUT0NEGSELR::INPUT1 => 0,
            INPUT0NEGSELR::INPUT3 => 1,
            INPUT0NEGSELR::INPUT5 => 2,
            INPUT0NEGSELR::INPUT7 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPUT0NEGSELR {
        match value {
            0 => INPUT0NEGSELR::INPUT1,
            1 => INPUT0NEGSELR::INPUT3,
            2 => INPUT0NEGSELR::INPUT5,
            3 => INPUT0NEGSELR::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline]
    pub fn is_input1(&self) -> bool {
        *self == INPUT0NEGSELR::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline]
    pub fn is_input3(&self) -> bool {
        *self == INPUT0NEGSELR::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline]
    pub fn is_input5(&self) -> bool {
        *self == INPUT0NEGSELR::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline]
    pub fn is_input7(&self) -> bool {
        *self == INPUT0NEGSELR::INPUT7
    }
}
#[doc = "Possible values of the field `INPUT2NEGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT2NEGSELR {
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    INPUT1,
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    INPUT3,
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    INPUT5,
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    INPUT7,
}
impl INPUT2NEGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPUT2NEGSELR::INPUT1 => 0,
            INPUT2NEGSELR::INPUT3 => 1,
            INPUT2NEGSELR::INPUT5 => 2,
            INPUT2NEGSELR::INPUT7 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPUT2NEGSELR {
        match value {
            0 => INPUT2NEGSELR::INPUT1,
            1 => INPUT2NEGSELR::INPUT3,
            2 => INPUT2NEGSELR::INPUT5,
            3 => INPUT2NEGSELR::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline]
    pub fn is_input1(&self) -> bool {
        *self == INPUT2NEGSELR::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline]
    pub fn is_input3(&self) -> bool {
        *self == INPUT2NEGSELR::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline]
    pub fn is_input5(&self) -> bool {
        *self == INPUT2NEGSELR::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline]
    pub fn is_input7(&self) -> bool {
        *self == INPUT2NEGSELR::INPUT7
    }
}
#[doc = "Possible values of the field `INPUT4NEGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT4NEGSELR {
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    INPUT1,
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    INPUT3,
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    INPUT5,
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    INPUT7,
}
impl INPUT4NEGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPUT4NEGSELR::INPUT1 => 0,
            INPUT4NEGSELR::INPUT3 => 1,
            INPUT4NEGSELR::INPUT5 => 2,
            INPUT4NEGSELR::INPUT7 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPUT4NEGSELR {
        match value {
            0 => INPUT4NEGSELR::INPUT1,
            1 => INPUT4NEGSELR::INPUT3,
            2 => INPUT4NEGSELR::INPUT5,
            3 => INPUT4NEGSELR::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline]
    pub fn is_input1(&self) -> bool {
        *self == INPUT4NEGSELR::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline]
    pub fn is_input3(&self) -> bool {
        *self == INPUT4NEGSELR::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline]
    pub fn is_input5(&self) -> bool {
        *self == INPUT4NEGSELR::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline]
    pub fn is_input7(&self) -> bool {
        *self == INPUT4NEGSELR::INPUT7
    }
}
#[doc = "Possible values of the field `INPUT6NEGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT6NEGSELR {
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    INPUT1,
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    INPUT3,
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    INPUT5,
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    INPUT7,
}
impl INPUT6NEGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPUT6NEGSELR::INPUT1 => 0,
            INPUT6NEGSELR::INPUT3 => 1,
            INPUT6NEGSELR::INPUT5 => 2,
            INPUT6NEGSELR::INPUT7 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPUT6NEGSELR {
        match value {
            0 => INPUT6NEGSELR::INPUT1,
            1 => INPUT6NEGSELR::INPUT3,
            2 => INPUT6NEGSELR::INPUT5,
            3 => INPUT6NEGSELR::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline]
    pub fn is_input1(&self) -> bool {
        *self == INPUT6NEGSELR::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline]
    pub fn is_input3(&self) -> bool {
        *self == INPUT6NEGSELR::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline]
    pub fn is_input5(&self) -> bool {
        *self == INPUT6NEGSELR::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline]
    pub fn is_input7(&self) -> bool {
        *self == INPUT6NEGSELR::INPUT7
    }
}
#[doc = "Possible values of the field `INPUT9NEGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT9NEGSELR {
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    INPUT8,
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    INPUT10,
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    INPUT12,
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    INPUT14,
}
impl INPUT9NEGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPUT9NEGSELR::INPUT8 => 0,
            INPUT9NEGSELR::INPUT10 => 1,
            INPUT9NEGSELR::INPUT12 => 2,
            INPUT9NEGSELR::INPUT14 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPUT9NEGSELR {
        match value {
            0 => INPUT9NEGSELR::INPUT8,
            1 => INPUT9NEGSELR::INPUT10,
            2 => INPUT9NEGSELR::INPUT12,
            3 => INPUT9NEGSELR::INPUT14,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT8`"]
    #[inline]
    pub fn is_input8(&self) -> bool {
        *self == INPUT9NEGSELR::INPUT8
    }
    #[doc = "Checks if the value of the field is `INPUT10`"]
    #[inline]
    pub fn is_input10(&self) -> bool {
        *self == INPUT9NEGSELR::INPUT10
    }
    #[doc = "Checks if the value of the field is `INPUT12`"]
    #[inline]
    pub fn is_input12(&self) -> bool {
        *self == INPUT9NEGSELR::INPUT12
    }
    #[doc = "Checks if the value of the field is `INPUT14`"]
    #[inline]
    pub fn is_input14(&self) -> bool {
        *self == INPUT9NEGSELR::INPUT14
    }
}
#[doc = "Possible values of the field `INPUT11NEGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT11NEGSELR {
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    INPUT8,
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    INPUT10,
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    INPUT12,
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    INPUT14,
}
impl INPUT11NEGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPUT11NEGSELR::INPUT8 => 0,
            INPUT11NEGSELR::INPUT10 => 1,
            INPUT11NEGSELR::INPUT12 => 2,
            INPUT11NEGSELR::INPUT14 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPUT11NEGSELR {
        match value {
            0 => INPUT11NEGSELR::INPUT8,
            1 => INPUT11NEGSELR::INPUT10,
            2 => INPUT11NEGSELR::INPUT12,
            3 => INPUT11NEGSELR::INPUT14,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT8`"]
    #[inline]
    pub fn is_input8(&self) -> bool {
        *self == INPUT11NEGSELR::INPUT8
    }
    #[doc = "Checks if the value of the field is `INPUT10`"]
    #[inline]
    pub fn is_input10(&self) -> bool {
        *self == INPUT11NEGSELR::INPUT10
    }
    #[doc = "Checks if the value of the field is `INPUT12`"]
    #[inline]
    pub fn is_input12(&self) -> bool {
        *self == INPUT11NEGSELR::INPUT12
    }
    #[doc = "Checks if the value of the field is `INPUT14`"]
    #[inline]
    pub fn is_input14(&self) -> bool {
        *self == INPUT11NEGSELR::INPUT14
    }
}
#[doc = "Possible values of the field `INPUT13NEGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT13NEGSELR {
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    INPUT8,
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    INPUT10,
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    INPUT12,
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    INPUT14,
}
impl INPUT13NEGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPUT13NEGSELR::INPUT8 => 0,
            INPUT13NEGSELR::INPUT10 => 1,
            INPUT13NEGSELR::INPUT12 => 2,
            INPUT13NEGSELR::INPUT14 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPUT13NEGSELR {
        match value {
            0 => INPUT13NEGSELR::INPUT8,
            1 => INPUT13NEGSELR::INPUT10,
            2 => INPUT13NEGSELR::INPUT12,
            3 => INPUT13NEGSELR::INPUT14,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT8`"]
    #[inline]
    pub fn is_input8(&self) -> bool {
        *self == INPUT13NEGSELR::INPUT8
    }
    #[doc = "Checks if the value of the field is `INPUT10`"]
    #[inline]
    pub fn is_input10(&self) -> bool {
        *self == INPUT13NEGSELR::INPUT10
    }
    #[doc = "Checks if the value of the field is `INPUT12`"]
    #[inline]
    pub fn is_input12(&self) -> bool {
        *self == INPUT13NEGSELR::INPUT12
    }
    #[doc = "Checks if the value of the field is `INPUT14`"]
    #[inline]
    pub fn is_input14(&self) -> bool {
        *self == INPUT13NEGSELR::INPUT14
    }
}
#[doc = "Possible values of the field `INPUT15NEGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT15NEGSELR {
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    INPUT8,
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    INPUT10,
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    INPUT12,
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    INPUT14,
}
impl INPUT15NEGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPUT15NEGSELR::INPUT8 => 0,
            INPUT15NEGSELR::INPUT10 => 1,
            INPUT15NEGSELR::INPUT12 => 2,
            INPUT15NEGSELR::INPUT14 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPUT15NEGSELR {
        match value {
            0 => INPUT15NEGSELR::INPUT8,
            1 => INPUT15NEGSELR::INPUT10,
            2 => INPUT15NEGSELR::INPUT12,
            3 => INPUT15NEGSELR::INPUT14,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT8`"]
    #[inline]
    pub fn is_input8(&self) -> bool {
        *self == INPUT15NEGSELR::INPUT8
    }
    #[doc = "Checks if the value of the field is `INPUT10`"]
    #[inline]
    pub fn is_input10(&self) -> bool {
        *self == INPUT15NEGSELR::INPUT10
    }
    #[doc = "Checks if the value of the field is `INPUT12`"]
    #[inline]
    pub fn is_input12(&self) -> bool {
        *self == INPUT15NEGSELR::INPUT12
    }
    #[doc = "Checks if the value of the field is `INPUT14`"]
    #[inline]
    pub fn is_input14(&self) -> bool {
        *self == INPUT15NEGSELR::INPUT14
    }
}
#[doc = "Values that can be written to the field `INPUT0NEGSEL`"]
pub enum INPUT0NEGSELW {
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    INPUT1,
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    INPUT3,
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    INPUT5,
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    INPUT7,
}
impl INPUT0NEGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPUT0NEGSELW::INPUT1 => 0,
            INPUT0NEGSELW::INPUT3 => 1,
            INPUT0NEGSELW::INPUT5 => 2,
            INPUT0NEGSELW::INPUT7 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUT0NEGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT0NEGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUT0NEGSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    #[inline]
    pub fn input1(self) -> &'a mut W {
        self.variant(INPUT0NEGSELW::INPUT1)
    }
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    #[inline]
    pub fn input3(self) -> &'a mut W {
        self.variant(INPUT0NEGSELW::INPUT3)
    }
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    #[inline]
    pub fn input5(self) -> &'a mut W {
        self.variant(INPUT0NEGSELW::INPUT5)
    }
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    #[inline]
    pub fn input7(self) -> &'a mut W {
        self.variant(INPUT0NEGSELW::INPUT7)
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
#[doc = "Values that can be written to the field `INPUT2NEGSEL`"]
pub enum INPUT2NEGSELW {
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    INPUT1,
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    INPUT3,
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    INPUT5,
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    INPUT7,
}
impl INPUT2NEGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPUT2NEGSELW::INPUT1 => 0,
            INPUT2NEGSELW::INPUT3 => 1,
            INPUT2NEGSELW::INPUT5 => 2,
            INPUT2NEGSELW::INPUT7 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUT2NEGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT2NEGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUT2NEGSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    #[inline]
    pub fn input1(self) -> &'a mut W {
        self.variant(INPUT2NEGSELW::INPUT1)
    }
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    #[inline]
    pub fn input3(self) -> &'a mut W {
        self.variant(INPUT2NEGSELW::INPUT3)
    }
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    #[inline]
    pub fn input5(self) -> &'a mut W {
        self.variant(INPUT2NEGSELW::INPUT5)
    }
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    #[inline]
    pub fn input7(self) -> &'a mut W {
        self.variant(INPUT2NEGSELW::INPUT7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INPUT4NEGSEL`"]
pub enum INPUT4NEGSELW {
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    INPUT1,
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    INPUT3,
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    INPUT5,
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    INPUT7,
}
impl INPUT4NEGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPUT4NEGSELW::INPUT1 => 0,
            INPUT4NEGSELW::INPUT3 => 1,
            INPUT4NEGSELW::INPUT5 => 2,
            INPUT4NEGSELW::INPUT7 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUT4NEGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT4NEGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUT4NEGSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    #[inline]
    pub fn input1(self) -> &'a mut W {
        self.variant(INPUT4NEGSELW::INPUT1)
    }
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    #[inline]
    pub fn input3(self) -> &'a mut W {
        self.variant(INPUT4NEGSELW::INPUT3)
    }
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    #[inline]
    pub fn input5(self) -> &'a mut W {
        self.variant(INPUT4NEGSELW::INPUT5)
    }
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    #[inline]
    pub fn input7(self) -> &'a mut W {
        self.variant(INPUT4NEGSELW::INPUT7)
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
#[doc = "Values that can be written to the field `INPUT6NEGSEL`"]
pub enum INPUT6NEGSELW {
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    INPUT1,
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    INPUT3,
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    INPUT5,
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    INPUT7,
}
impl INPUT6NEGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPUT6NEGSELW::INPUT1 => 0,
            INPUT6NEGSELW::INPUT3 => 1,
            INPUT6NEGSELW::INPUT5 => 2,
            INPUT6NEGSELW::INPUT7 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUT6NEGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT6NEGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUT6NEGSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    #[inline]
    pub fn input1(self) -> &'a mut W {
        self.variant(INPUT6NEGSELW::INPUT1)
    }
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    #[inline]
    pub fn input3(self) -> &'a mut W {
        self.variant(INPUT6NEGSELW::INPUT3)
    }
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    #[inline]
    pub fn input5(self) -> &'a mut W {
        self.variant(INPUT6NEGSELW::INPUT5)
    }
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    #[inline]
    pub fn input7(self) -> &'a mut W {
        self.variant(INPUT6NEGSELW::INPUT7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INPUT9NEGSEL`"]
pub enum INPUT9NEGSELW {
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    INPUT8,
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    INPUT10,
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    INPUT12,
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    INPUT14,
}
impl INPUT9NEGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPUT9NEGSELW::INPUT8 => 0,
            INPUT9NEGSELW::INPUT10 => 1,
            INPUT9NEGSELW::INPUT12 => 2,
            INPUT9NEGSELW::INPUT14 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUT9NEGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT9NEGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUT9NEGSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    #[inline]
    pub fn input8(self) -> &'a mut W {
        self.variant(INPUT9NEGSELW::INPUT8)
    }
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    #[inline]
    pub fn input10(self) -> &'a mut W {
        self.variant(INPUT9NEGSELW::INPUT10)
    }
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    #[inline]
    pub fn input12(self) -> &'a mut W {
        self.variant(INPUT9NEGSELW::INPUT12)
    }
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    #[inline]
    pub fn input14(self) -> &'a mut W {
        self.variant(INPUT9NEGSELW::INPUT14)
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
#[doc = "Values that can be written to the field `INPUT11NEGSEL`"]
pub enum INPUT11NEGSELW {
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    INPUT8,
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    INPUT10,
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    INPUT12,
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    INPUT14,
}
impl INPUT11NEGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPUT11NEGSELW::INPUT8 => 0,
            INPUT11NEGSELW::INPUT10 => 1,
            INPUT11NEGSELW::INPUT12 => 2,
            INPUT11NEGSELW::INPUT14 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUT11NEGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT11NEGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUT11NEGSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    #[inline]
    pub fn input8(self) -> &'a mut W {
        self.variant(INPUT11NEGSELW::INPUT8)
    }
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    #[inline]
    pub fn input10(self) -> &'a mut W {
        self.variant(INPUT11NEGSELW::INPUT10)
    }
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    #[inline]
    pub fn input12(self) -> &'a mut W {
        self.variant(INPUT11NEGSELW::INPUT12)
    }
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    #[inline]
    pub fn input14(self) -> &'a mut W {
        self.variant(INPUT11NEGSELW::INPUT14)
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
#[doc = "Values that can be written to the field `INPUT13NEGSEL`"]
pub enum INPUT13NEGSELW {
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    INPUT8,
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    INPUT10,
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    INPUT12,
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    INPUT14,
}
impl INPUT13NEGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPUT13NEGSELW::INPUT8 => 0,
            INPUT13NEGSELW::INPUT10 => 1,
            INPUT13NEGSELW::INPUT12 => 2,
            INPUT13NEGSELW::INPUT14 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUT13NEGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT13NEGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUT13NEGSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    #[inline]
    pub fn input8(self) -> &'a mut W {
        self.variant(INPUT13NEGSELW::INPUT8)
    }
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    #[inline]
    pub fn input10(self) -> &'a mut W {
        self.variant(INPUT13NEGSELW::INPUT10)
    }
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    #[inline]
    pub fn input12(self) -> &'a mut W {
        self.variant(INPUT13NEGSELW::INPUT12)
    }
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    #[inline]
    pub fn input14(self) -> &'a mut W {
        self.variant(INPUT13NEGSELW::INPUT14)
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
#[doc = "Values that can be written to the field `INPUT15NEGSEL`"]
pub enum INPUT15NEGSELW {
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    INPUT8,
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    INPUT10,
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    INPUT12,
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    INPUT14,
}
impl INPUT15NEGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPUT15NEGSELW::INPUT8 => 0,
            INPUT15NEGSELW::INPUT10 => 1,
            INPUT15NEGSELW::INPUT12 => 2,
            INPUT15NEGSELW::INPUT14 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUT15NEGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT15NEGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUT15NEGSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    #[inline]
    pub fn input8(self) -> &'a mut W {
        self.variant(INPUT15NEGSELW::INPUT8)
    }
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    #[inline]
    pub fn input10(self) -> &'a mut W {
        self.variant(INPUT15NEGSELW::INPUT10)
    }
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    #[inline]
    pub fn input12(self) -> &'a mut W {
        self.variant(INPUT15NEGSELW::INPUT12)
    }
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    #[inline]
    pub fn input14(self) -> &'a mut W {
        self.variant(INPUT15NEGSELW::INPUT14)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:1 - Negative Input Select Register for ADCn_INPUT0 in Differential Scan Mode"]
    #[inline]
    pub fn input0negsel(&self) -> INPUT0NEGSELR {
        INPUT0NEGSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Negative Input Select Register for ADCn_INPUT2 in Differential Scan Mode"]
    #[inline]
    pub fn input2negsel(&self) -> INPUT2NEGSELR {
        INPUT2NEGSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Negative Input Select Register for ADCn_INPUT4 in Differential Scan Mode"]
    #[inline]
    pub fn input4negsel(&self) -> INPUT4NEGSELR {
        INPUT4NEGSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Negative Input Select Register for ADCn_INPUT1 in Differential Scan Mode"]
    #[inline]
    pub fn input6negsel(&self) -> INPUT6NEGSELR {
        INPUT6NEGSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Negative Input Select Register for ADCn_INPUT9 in Differential Scan Mode"]
    #[inline]
    pub fn input9negsel(&self) -> INPUT9NEGSELR {
        INPUT9NEGSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Negative Input Select Register for ADCn_INPUT11 in Differential Scan Mode"]
    #[inline]
    pub fn input11negsel(&self) -> INPUT11NEGSELR {
        INPUT11NEGSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Negative Input Select Register for ADCn_INPUT13 in Differential Scan Mode"]
    #[inline]
    pub fn input13negsel(&self) -> INPUT13NEGSELR {
        INPUT13NEGSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Negative Input Select Register for ADCn_INPUT15 in Differential Scan Mode"]
    #[inline]
    pub fn input15negsel(&self) -> INPUT15NEGSELR {
        INPUT15NEGSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 14820 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Negative Input Select Register for ADCn_INPUT0 in Differential Scan Mode"]
    #[inline]
    pub fn input0negsel(&mut self) -> _INPUT0NEGSELW {
        _INPUT0NEGSELW { w: self }
    }
    #[doc = "Bits 2:3 - Negative Input Select Register for ADCn_INPUT2 in Differential Scan Mode"]
    #[inline]
    pub fn input2negsel(&mut self) -> _INPUT2NEGSELW {
        _INPUT2NEGSELW { w: self }
    }
    #[doc = "Bits 4:5 - Negative Input Select Register for ADCn_INPUT4 in Differential Scan Mode"]
    #[inline]
    pub fn input4negsel(&mut self) -> _INPUT4NEGSELW {
        _INPUT4NEGSELW { w: self }
    }
    #[doc = "Bits 6:7 - Negative Input Select Register for ADCn_INPUT1 in Differential Scan Mode"]
    #[inline]
    pub fn input6negsel(&mut self) -> _INPUT6NEGSELW {
        _INPUT6NEGSELW { w: self }
    }
    #[doc = "Bits 8:9 - Negative Input Select Register for ADCn_INPUT9 in Differential Scan Mode"]
    #[inline]
    pub fn input9negsel(&mut self) -> _INPUT9NEGSELW {
        _INPUT9NEGSELW { w: self }
    }
    #[doc = "Bits 10:11 - Negative Input Select Register for ADCn_INPUT11 in Differential Scan Mode"]
    #[inline]
    pub fn input11negsel(&mut self) -> _INPUT11NEGSELW {
        _INPUT11NEGSELW { w: self }
    }
    #[doc = "Bits 12:13 - Negative Input Select Register for ADCn_INPUT13 in Differential Scan Mode"]
    #[inline]
    pub fn input13negsel(&mut self) -> _INPUT13NEGSELW {
        _INPUT13NEGSELW { w: self }
    }
    #[doc = "Bits 14:15 - Negative Input Select Register for ADCn_INPUT15 in Differential Scan Mode"]
    #[inline]
    pub fn input15negsel(&mut self) -> _INPUT15NEGSELW {
        _INPUT15NEGSELW { w: self }
    }
}
