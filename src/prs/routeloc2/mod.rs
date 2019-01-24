#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ROUTELOC2 {
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
#[doc = "Possible values of the field `CH8LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH8LOCR {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
    #[doc = "Location 4"]
    LOC4,
    #[doc = "Location 5"]
    LOC5,
    #[doc = "Location 6"]
    LOC6,
    #[doc = "Location 7"]
    LOC7,
    #[doc = "Location 8"]
    LOC8,
    #[doc = "Location 9"]
    LOC9,
    #[doc = "Location 10"]
    LOC10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CH8LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH8LOCR::LOC0 => 0,
            CH8LOCR::LOC1 => 1,
            CH8LOCR::LOC2 => 2,
            CH8LOCR::LOC3 => 3,
            CH8LOCR::LOC4 => 4,
            CH8LOCR::LOC5 => 5,
            CH8LOCR::LOC6 => 6,
            CH8LOCR::LOC7 => 7,
            CH8LOCR::LOC8 => 8,
            CH8LOCR::LOC9 => 9,
            CH8LOCR::LOC10 => 10,
            CH8LOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH8LOCR {
        match value {
            0 => CH8LOCR::LOC0,
            1 => CH8LOCR::LOC1,
            2 => CH8LOCR::LOC2,
            3 => CH8LOCR::LOC3,
            4 => CH8LOCR::LOC4,
            5 => CH8LOCR::LOC5,
            6 => CH8LOCR::LOC6,
            7 => CH8LOCR::LOC7,
            8 => CH8LOCR::LOC8,
            9 => CH8LOCR::LOC9,
            10 => CH8LOCR::LOC10,
            i => CH8LOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == CH8LOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == CH8LOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == CH8LOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == CH8LOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == CH8LOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == CH8LOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == CH8LOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == CH8LOCR::LOC7
    }
    #[doc = "Checks if the value of the field is `LOC8`"]
    #[inline]
    pub fn is_loc8(&self) -> bool {
        *self == CH8LOCR::LOC8
    }
    #[doc = "Checks if the value of the field is `LOC9`"]
    #[inline]
    pub fn is_loc9(&self) -> bool {
        *self == CH8LOCR::LOC9
    }
    #[doc = "Checks if the value of the field is `LOC10`"]
    #[inline]
    pub fn is_loc10(&self) -> bool {
        *self == CH8LOCR::LOC10
    }
}
#[doc = "Possible values of the field `CH9LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH9LOCR {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
    #[doc = "Location 4"]
    LOC4,
    #[doc = "Location 5"]
    LOC5,
    #[doc = "Location 6"]
    LOC6,
    #[doc = "Location 7"]
    LOC7,
    #[doc = "Location 8"]
    LOC8,
    #[doc = "Location 9"]
    LOC9,
    #[doc = "Location 10"]
    LOC10,
    #[doc = "Location 11"]
    LOC11,
    #[doc = "Location 12"]
    LOC12,
    #[doc = "Location 13"]
    LOC13,
    #[doc = "Location 14"]
    LOC14,
    #[doc = "Location 15"]
    LOC15,
    #[doc = "Location 16"]
    LOC16,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CH9LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH9LOCR::LOC0 => 0,
            CH9LOCR::LOC1 => 1,
            CH9LOCR::LOC2 => 2,
            CH9LOCR::LOC3 => 3,
            CH9LOCR::LOC4 => 4,
            CH9LOCR::LOC5 => 5,
            CH9LOCR::LOC6 => 6,
            CH9LOCR::LOC7 => 7,
            CH9LOCR::LOC8 => 8,
            CH9LOCR::LOC9 => 9,
            CH9LOCR::LOC10 => 10,
            CH9LOCR::LOC11 => 11,
            CH9LOCR::LOC12 => 12,
            CH9LOCR::LOC13 => 13,
            CH9LOCR::LOC14 => 14,
            CH9LOCR::LOC15 => 15,
            CH9LOCR::LOC16 => 16,
            CH9LOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH9LOCR {
        match value {
            0 => CH9LOCR::LOC0,
            1 => CH9LOCR::LOC1,
            2 => CH9LOCR::LOC2,
            3 => CH9LOCR::LOC3,
            4 => CH9LOCR::LOC4,
            5 => CH9LOCR::LOC5,
            6 => CH9LOCR::LOC6,
            7 => CH9LOCR::LOC7,
            8 => CH9LOCR::LOC8,
            9 => CH9LOCR::LOC9,
            10 => CH9LOCR::LOC10,
            11 => CH9LOCR::LOC11,
            12 => CH9LOCR::LOC12,
            13 => CH9LOCR::LOC13,
            14 => CH9LOCR::LOC14,
            15 => CH9LOCR::LOC15,
            16 => CH9LOCR::LOC16,
            i => CH9LOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == CH9LOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == CH9LOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == CH9LOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == CH9LOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == CH9LOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == CH9LOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == CH9LOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == CH9LOCR::LOC7
    }
    #[doc = "Checks if the value of the field is `LOC8`"]
    #[inline]
    pub fn is_loc8(&self) -> bool {
        *self == CH9LOCR::LOC8
    }
    #[doc = "Checks if the value of the field is `LOC9`"]
    #[inline]
    pub fn is_loc9(&self) -> bool {
        *self == CH9LOCR::LOC9
    }
    #[doc = "Checks if the value of the field is `LOC10`"]
    #[inline]
    pub fn is_loc10(&self) -> bool {
        *self == CH9LOCR::LOC10
    }
    #[doc = "Checks if the value of the field is `LOC11`"]
    #[inline]
    pub fn is_loc11(&self) -> bool {
        *self == CH9LOCR::LOC11
    }
    #[doc = "Checks if the value of the field is `LOC12`"]
    #[inline]
    pub fn is_loc12(&self) -> bool {
        *self == CH9LOCR::LOC12
    }
    #[doc = "Checks if the value of the field is `LOC13`"]
    #[inline]
    pub fn is_loc13(&self) -> bool {
        *self == CH9LOCR::LOC13
    }
    #[doc = "Checks if the value of the field is `LOC14`"]
    #[inline]
    pub fn is_loc14(&self) -> bool {
        *self == CH9LOCR::LOC14
    }
    #[doc = "Checks if the value of the field is `LOC15`"]
    #[inline]
    pub fn is_loc15(&self) -> bool {
        *self == CH9LOCR::LOC15
    }
    #[doc = "Checks if the value of the field is `LOC16`"]
    #[inline]
    pub fn is_loc16(&self) -> bool {
        *self == CH9LOCR::LOC16
    }
}
#[doc = "Possible values of the field `CH10LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH10LOCR {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
    #[doc = "Location 4"]
    LOC4,
    #[doc = "Location 5"]
    LOC5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CH10LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH10LOCR::LOC0 => 0,
            CH10LOCR::LOC1 => 1,
            CH10LOCR::LOC2 => 2,
            CH10LOCR::LOC3 => 3,
            CH10LOCR::LOC4 => 4,
            CH10LOCR::LOC5 => 5,
            CH10LOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH10LOCR {
        match value {
            0 => CH10LOCR::LOC0,
            1 => CH10LOCR::LOC1,
            2 => CH10LOCR::LOC2,
            3 => CH10LOCR::LOC3,
            4 => CH10LOCR::LOC4,
            5 => CH10LOCR::LOC5,
            i => CH10LOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == CH10LOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == CH10LOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == CH10LOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == CH10LOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == CH10LOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == CH10LOCR::LOC5
    }
}
#[doc = "Possible values of the field `CH11LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH11LOCR {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
    #[doc = "Location 4"]
    LOC4,
    #[doc = "Location 5"]
    LOC5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CH11LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH11LOCR::LOC0 => 0,
            CH11LOCR::LOC1 => 1,
            CH11LOCR::LOC2 => 2,
            CH11LOCR::LOC3 => 3,
            CH11LOCR::LOC4 => 4,
            CH11LOCR::LOC5 => 5,
            CH11LOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH11LOCR {
        match value {
            0 => CH11LOCR::LOC0,
            1 => CH11LOCR::LOC1,
            2 => CH11LOCR::LOC2,
            3 => CH11LOCR::LOC3,
            4 => CH11LOCR::LOC4,
            5 => CH11LOCR::LOC5,
            i => CH11LOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == CH11LOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == CH11LOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == CH11LOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == CH11LOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == CH11LOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == CH11LOCR::LOC5
    }
}
#[doc = "Values that can be written to the field `CH8LOC`"]
pub enum CH8LOCW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
    #[doc = "Location 4"]
    LOC4,
    #[doc = "Location 5"]
    LOC5,
    #[doc = "Location 6"]
    LOC6,
    #[doc = "Location 7"]
    LOC7,
    #[doc = "Location 8"]
    LOC8,
    #[doc = "Location 9"]
    LOC9,
    #[doc = "Location 10"]
    LOC10,
}
impl CH8LOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CH8LOCW::LOC0 => 0,
            CH8LOCW::LOC1 => 1,
            CH8LOCW::LOC2 => 2,
            CH8LOCW::LOC3 => 3,
            CH8LOCW::LOC4 => 4,
            CH8LOCW::LOC5 => 5,
            CH8LOCW::LOC6 => 6,
            CH8LOCW::LOC7 => 7,
            CH8LOCW::LOC8 => 8,
            CH8LOCW::LOC9 => 9,
            CH8LOCW::LOC10 => 10,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH8LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH8LOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH8LOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH8LOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH8LOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH8LOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CH8LOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CH8LOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CH8LOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CH8LOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CH8LOCW::LOC7)
    }
    #[doc = "Location 8"]
    #[inline]
    pub fn loc8(self) -> &'a mut W {
        self.variant(CH8LOCW::LOC8)
    }
    #[doc = "Location 9"]
    #[inline]
    pub fn loc9(self) -> &'a mut W {
        self.variant(CH8LOCW::LOC9)
    }
    #[doc = "Location 10"]
    #[inline]
    pub fn loc10(self) -> &'a mut W {
        self.variant(CH8LOCW::LOC10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CH9LOC`"]
pub enum CH9LOCW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
    #[doc = "Location 4"]
    LOC4,
    #[doc = "Location 5"]
    LOC5,
    #[doc = "Location 6"]
    LOC6,
    #[doc = "Location 7"]
    LOC7,
    #[doc = "Location 8"]
    LOC8,
    #[doc = "Location 9"]
    LOC9,
    #[doc = "Location 10"]
    LOC10,
    #[doc = "Location 11"]
    LOC11,
    #[doc = "Location 12"]
    LOC12,
    #[doc = "Location 13"]
    LOC13,
    #[doc = "Location 14"]
    LOC14,
    #[doc = "Location 15"]
    LOC15,
    #[doc = "Location 16"]
    LOC16,
}
impl CH9LOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CH9LOCW::LOC0 => 0,
            CH9LOCW::LOC1 => 1,
            CH9LOCW::LOC2 => 2,
            CH9LOCW::LOC3 => 3,
            CH9LOCW::LOC4 => 4,
            CH9LOCW::LOC5 => 5,
            CH9LOCW::LOC6 => 6,
            CH9LOCW::LOC7 => 7,
            CH9LOCW::LOC8 => 8,
            CH9LOCW::LOC9 => 9,
            CH9LOCW::LOC10 => 10,
            CH9LOCW::LOC11 => 11,
            CH9LOCW::LOC12 => 12,
            CH9LOCW::LOC13 => 13,
            CH9LOCW::LOC14 => 14,
            CH9LOCW::LOC15 => 15,
            CH9LOCW::LOC16 => 16,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH9LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH9LOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH9LOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH9LOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH9LOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH9LOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CH9LOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CH9LOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CH9LOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CH9LOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CH9LOCW::LOC7)
    }
    #[doc = "Location 8"]
    #[inline]
    pub fn loc8(self) -> &'a mut W {
        self.variant(CH9LOCW::LOC8)
    }
    #[doc = "Location 9"]
    #[inline]
    pub fn loc9(self) -> &'a mut W {
        self.variant(CH9LOCW::LOC9)
    }
    #[doc = "Location 10"]
    #[inline]
    pub fn loc10(self) -> &'a mut W {
        self.variant(CH9LOCW::LOC10)
    }
    #[doc = "Location 11"]
    #[inline]
    pub fn loc11(self) -> &'a mut W {
        self.variant(CH9LOCW::LOC11)
    }
    #[doc = "Location 12"]
    #[inline]
    pub fn loc12(self) -> &'a mut W {
        self.variant(CH9LOCW::LOC12)
    }
    #[doc = "Location 13"]
    #[inline]
    pub fn loc13(self) -> &'a mut W {
        self.variant(CH9LOCW::LOC13)
    }
    #[doc = "Location 14"]
    #[inline]
    pub fn loc14(self) -> &'a mut W {
        self.variant(CH9LOCW::LOC14)
    }
    #[doc = "Location 15"]
    #[inline]
    pub fn loc15(self) -> &'a mut W {
        self.variant(CH9LOCW::LOC15)
    }
    #[doc = "Location 16"]
    #[inline]
    pub fn loc16(self) -> &'a mut W {
        self.variant(CH9LOCW::LOC16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CH10LOC`"]
pub enum CH10LOCW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
    #[doc = "Location 4"]
    LOC4,
    #[doc = "Location 5"]
    LOC5,
}
impl CH10LOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CH10LOCW::LOC0 => 0,
            CH10LOCW::LOC1 => 1,
            CH10LOCW::LOC2 => 2,
            CH10LOCW::LOC3 => 3,
            CH10LOCW::LOC4 => 4,
            CH10LOCW::LOC5 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH10LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH10LOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH10LOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH10LOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH10LOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH10LOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CH10LOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CH10LOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CH10LOCW::LOC5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CH11LOC`"]
pub enum CH11LOCW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
    #[doc = "Location 4"]
    LOC4,
    #[doc = "Location 5"]
    LOC5,
}
impl CH11LOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CH11LOCW::LOC0 => 0,
            CH11LOCW::LOC1 => 1,
            CH11LOCW::LOC2 => 2,
            CH11LOCW::LOC3 => 3,
            CH11LOCW::LOC4 => 4,
            CH11LOCW::LOC5 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH11LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH11LOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH11LOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH11LOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH11LOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH11LOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CH11LOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CH11LOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CH11LOCW::LOC5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline]
    pub fn ch8loc(&self) -> CH8LOCR {
        CH8LOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline]
    pub fn ch9loc(&self) -> CH9LOCR {
        CH9LOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline]
    pub fn ch10loc(&self) -> CH10LOCR {
        CH10LOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline]
    pub fn ch11loc(&self) -> CH11LOCR {
        CH11LOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline]
    pub fn ch8loc(&mut self) -> _CH8LOCW {
        _CH8LOCW { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline]
    pub fn ch9loc(&mut self) -> _CH9LOCW {
        _CH9LOCW { w: self }
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline]
    pub fn ch10loc(&mut self) -> _CH10LOCW {
        _CH10LOCW { w: self }
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline]
    pub fn ch11loc(&mut self) -> _CH11LOCW {
        _CH11LOCW { w: self }
    }
}
