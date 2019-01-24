#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ROUTELOC0 {
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
#[doc = "Possible values of the field `CC0LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC0LOCR {
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
    #[doc = "Location 17"]
    LOC17,
    #[doc = "Location 18"]
    LOC18,
    #[doc = "Location 19"]
    LOC19,
    #[doc = "Location 20"]
    LOC20,
    #[doc = "Location 21"]
    LOC21,
    #[doc = "Location 22"]
    LOC22,
    #[doc = "Location 23"]
    LOC23,
    #[doc = "Location 24"]
    LOC24,
    #[doc = "Location 25"]
    LOC25,
    #[doc = "Location 26"]
    LOC26,
    #[doc = "Location 27"]
    LOC27,
    #[doc = "Location 28"]
    LOC28,
    #[doc = "Location 29"]
    LOC29,
    #[doc = "Location 30"]
    LOC30,
    #[doc = "Location 31"]
    LOC31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CC0LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CC0LOCR::LOC0 => 0,
            CC0LOCR::LOC1 => 1,
            CC0LOCR::LOC2 => 2,
            CC0LOCR::LOC3 => 3,
            CC0LOCR::LOC4 => 4,
            CC0LOCR::LOC5 => 5,
            CC0LOCR::LOC6 => 6,
            CC0LOCR::LOC7 => 7,
            CC0LOCR::LOC8 => 8,
            CC0LOCR::LOC9 => 9,
            CC0LOCR::LOC10 => 10,
            CC0LOCR::LOC11 => 11,
            CC0LOCR::LOC12 => 12,
            CC0LOCR::LOC13 => 13,
            CC0LOCR::LOC14 => 14,
            CC0LOCR::LOC15 => 15,
            CC0LOCR::LOC16 => 16,
            CC0LOCR::LOC17 => 17,
            CC0LOCR::LOC18 => 18,
            CC0LOCR::LOC19 => 19,
            CC0LOCR::LOC20 => 20,
            CC0LOCR::LOC21 => 21,
            CC0LOCR::LOC22 => 22,
            CC0LOCR::LOC23 => 23,
            CC0LOCR::LOC24 => 24,
            CC0LOCR::LOC25 => 25,
            CC0LOCR::LOC26 => 26,
            CC0LOCR::LOC27 => 27,
            CC0LOCR::LOC28 => 28,
            CC0LOCR::LOC29 => 29,
            CC0LOCR::LOC30 => 30,
            CC0LOCR::LOC31 => 31,
            CC0LOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CC0LOCR {
        match value {
            0 => CC0LOCR::LOC0,
            1 => CC0LOCR::LOC1,
            2 => CC0LOCR::LOC2,
            3 => CC0LOCR::LOC3,
            4 => CC0LOCR::LOC4,
            5 => CC0LOCR::LOC5,
            6 => CC0LOCR::LOC6,
            7 => CC0LOCR::LOC7,
            8 => CC0LOCR::LOC8,
            9 => CC0LOCR::LOC9,
            10 => CC0LOCR::LOC10,
            11 => CC0LOCR::LOC11,
            12 => CC0LOCR::LOC12,
            13 => CC0LOCR::LOC13,
            14 => CC0LOCR::LOC14,
            15 => CC0LOCR::LOC15,
            16 => CC0LOCR::LOC16,
            17 => CC0LOCR::LOC17,
            18 => CC0LOCR::LOC18,
            19 => CC0LOCR::LOC19,
            20 => CC0LOCR::LOC20,
            21 => CC0LOCR::LOC21,
            22 => CC0LOCR::LOC22,
            23 => CC0LOCR::LOC23,
            24 => CC0LOCR::LOC24,
            25 => CC0LOCR::LOC25,
            26 => CC0LOCR::LOC26,
            27 => CC0LOCR::LOC27,
            28 => CC0LOCR::LOC28,
            29 => CC0LOCR::LOC29,
            30 => CC0LOCR::LOC30,
            31 => CC0LOCR::LOC31,
            i => CC0LOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == CC0LOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == CC0LOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == CC0LOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == CC0LOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == CC0LOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == CC0LOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == CC0LOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == CC0LOCR::LOC7
    }
    #[doc = "Checks if the value of the field is `LOC8`"]
    #[inline]
    pub fn is_loc8(&self) -> bool {
        *self == CC0LOCR::LOC8
    }
    #[doc = "Checks if the value of the field is `LOC9`"]
    #[inline]
    pub fn is_loc9(&self) -> bool {
        *self == CC0LOCR::LOC9
    }
    #[doc = "Checks if the value of the field is `LOC10`"]
    #[inline]
    pub fn is_loc10(&self) -> bool {
        *self == CC0LOCR::LOC10
    }
    #[doc = "Checks if the value of the field is `LOC11`"]
    #[inline]
    pub fn is_loc11(&self) -> bool {
        *self == CC0LOCR::LOC11
    }
    #[doc = "Checks if the value of the field is `LOC12`"]
    #[inline]
    pub fn is_loc12(&self) -> bool {
        *self == CC0LOCR::LOC12
    }
    #[doc = "Checks if the value of the field is `LOC13`"]
    #[inline]
    pub fn is_loc13(&self) -> bool {
        *self == CC0LOCR::LOC13
    }
    #[doc = "Checks if the value of the field is `LOC14`"]
    #[inline]
    pub fn is_loc14(&self) -> bool {
        *self == CC0LOCR::LOC14
    }
    #[doc = "Checks if the value of the field is `LOC15`"]
    #[inline]
    pub fn is_loc15(&self) -> bool {
        *self == CC0LOCR::LOC15
    }
    #[doc = "Checks if the value of the field is `LOC16`"]
    #[inline]
    pub fn is_loc16(&self) -> bool {
        *self == CC0LOCR::LOC16
    }
    #[doc = "Checks if the value of the field is `LOC17`"]
    #[inline]
    pub fn is_loc17(&self) -> bool {
        *self == CC0LOCR::LOC17
    }
    #[doc = "Checks if the value of the field is `LOC18`"]
    #[inline]
    pub fn is_loc18(&self) -> bool {
        *self == CC0LOCR::LOC18
    }
    #[doc = "Checks if the value of the field is `LOC19`"]
    #[inline]
    pub fn is_loc19(&self) -> bool {
        *self == CC0LOCR::LOC19
    }
    #[doc = "Checks if the value of the field is `LOC20`"]
    #[inline]
    pub fn is_loc20(&self) -> bool {
        *self == CC0LOCR::LOC20
    }
    #[doc = "Checks if the value of the field is `LOC21`"]
    #[inline]
    pub fn is_loc21(&self) -> bool {
        *self == CC0LOCR::LOC21
    }
    #[doc = "Checks if the value of the field is `LOC22`"]
    #[inline]
    pub fn is_loc22(&self) -> bool {
        *self == CC0LOCR::LOC22
    }
    #[doc = "Checks if the value of the field is `LOC23`"]
    #[inline]
    pub fn is_loc23(&self) -> bool {
        *self == CC0LOCR::LOC23
    }
    #[doc = "Checks if the value of the field is `LOC24`"]
    #[inline]
    pub fn is_loc24(&self) -> bool {
        *self == CC0LOCR::LOC24
    }
    #[doc = "Checks if the value of the field is `LOC25`"]
    #[inline]
    pub fn is_loc25(&self) -> bool {
        *self == CC0LOCR::LOC25
    }
    #[doc = "Checks if the value of the field is `LOC26`"]
    #[inline]
    pub fn is_loc26(&self) -> bool {
        *self == CC0LOCR::LOC26
    }
    #[doc = "Checks if the value of the field is `LOC27`"]
    #[inline]
    pub fn is_loc27(&self) -> bool {
        *self == CC0LOCR::LOC27
    }
    #[doc = "Checks if the value of the field is `LOC28`"]
    #[inline]
    pub fn is_loc28(&self) -> bool {
        *self == CC0LOCR::LOC28
    }
    #[doc = "Checks if the value of the field is `LOC29`"]
    #[inline]
    pub fn is_loc29(&self) -> bool {
        *self == CC0LOCR::LOC29
    }
    #[doc = "Checks if the value of the field is `LOC30`"]
    #[inline]
    pub fn is_loc30(&self) -> bool {
        *self == CC0LOCR::LOC30
    }
    #[doc = "Checks if the value of the field is `LOC31`"]
    #[inline]
    pub fn is_loc31(&self) -> bool {
        *self == CC0LOCR::LOC31
    }
}
#[doc = "Possible values of the field `CC1LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1LOCR {
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
    #[doc = "Location 17"]
    LOC17,
    #[doc = "Location 18"]
    LOC18,
    #[doc = "Location 19"]
    LOC19,
    #[doc = "Location 20"]
    LOC20,
    #[doc = "Location 21"]
    LOC21,
    #[doc = "Location 22"]
    LOC22,
    #[doc = "Location 23"]
    LOC23,
    #[doc = "Location 24"]
    LOC24,
    #[doc = "Location 25"]
    LOC25,
    #[doc = "Location 26"]
    LOC26,
    #[doc = "Location 27"]
    LOC27,
    #[doc = "Location 28"]
    LOC28,
    #[doc = "Location 29"]
    LOC29,
    #[doc = "Location 30"]
    LOC30,
    #[doc = "Location 31"]
    LOC31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CC1LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CC1LOCR::LOC0 => 0,
            CC1LOCR::LOC1 => 1,
            CC1LOCR::LOC2 => 2,
            CC1LOCR::LOC3 => 3,
            CC1LOCR::LOC4 => 4,
            CC1LOCR::LOC5 => 5,
            CC1LOCR::LOC6 => 6,
            CC1LOCR::LOC7 => 7,
            CC1LOCR::LOC8 => 8,
            CC1LOCR::LOC9 => 9,
            CC1LOCR::LOC10 => 10,
            CC1LOCR::LOC11 => 11,
            CC1LOCR::LOC12 => 12,
            CC1LOCR::LOC13 => 13,
            CC1LOCR::LOC14 => 14,
            CC1LOCR::LOC15 => 15,
            CC1LOCR::LOC16 => 16,
            CC1LOCR::LOC17 => 17,
            CC1LOCR::LOC18 => 18,
            CC1LOCR::LOC19 => 19,
            CC1LOCR::LOC20 => 20,
            CC1LOCR::LOC21 => 21,
            CC1LOCR::LOC22 => 22,
            CC1LOCR::LOC23 => 23,
            CC1LOCR::LOC24 => 24,
            CC1LOCR::LOC25 => 25,
            CC1LOCR::LOC26 => 26,
            CC1LOCR::LOC27 => 27,
            CC1LOCR::LOC28 => 28,
            CC1LOCR::LOC29 => 29,
            CC1LOCR::LOC30 => 30,
            CC1LOCR::LOC31 => 31,
            CC1LOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CC1LOCR {
        match value {
            0 => CC1LOCR::LOC0,
            1 => CC1LOCR::LOC1,
            2 => CC1LOCR::LOC2,
            3 => CC1LOCR::LOC3,
            4 => CC1LOCR::LOC4,
            5 => CC1LOCR::LOC5,
            6 => CC1LOCR::LOC6,
            7 => CC1LOCR::LOC7,
            8 => CC1LOCR::LOC8,
            9 => CC1LOCR::LOC9,
            10 => CC1LOCR::LOC10,
            11 => CC1LOCR::LOC11,
            12 => CC1LOCR::LOC12,
            13 => CC1LOCR::LOC13,
            14 => CC1LOCR::LOC14,
            15 => CC1LOCR::LOC15,
            16 => CC1LOCR::LOC16,
            17 => CC1LOCR::LOC17,
            18 => CC1LOCR::LOC18,
            19 => CC1LOCR::LOC19,
            20 => CC1LOCR::LOC20,
            21 => CC1LOCR::LOC21,
            22 => CC1LOCR::LOC22,
            23 => CC1LOCR::LOC23,
            24 => CC1LOCR::LOC24,
            25 => CC1LOCR::LOC25,
            26 => CC1LOCR::LOC26,
            27 => CC1LOCR::LOC27,
            28 => CC1LOCR::LOC28,
            29 => CC1LOCR::LOC29,
            30 => CC1LOCR::LOC30,
            31 => CC1LOCR::LOC31,
            i => CC1LOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == CC1LOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == CC1LOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == CC1LOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == CC1LOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == CC1LOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == CC1LOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == CC1LOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == CC1LOCR::LOC7
    }
    #[doc = "Checks if the value of the field is `LOC8`"]
    #[inline]
    pub fn is_loc8(&self) -> bool {
        *self == CC1LOCR::LOC8
    }
    #[doc = "Checks if the value of the field is `LOC9`"]
    #[inline]
    pub fn is_loc9(&self) -> bool {
        *self == CC1LOCR::LOC9
    }
    #[doc = "Checks if the value of the field is `LOC10`"]
    #[inline]
    pub fn is_loc10(&self) -> bool {
        *self == CC1LOCR::LOC10
    }
    #[doc = "Checks if the value of the field is `LOC11`"]
    #[inline]
    pub fn is_loc11(&self) -> bool {
        *self == CC1LOCR::LOC11
    }
    #[doc = "Checks if the value of the field is `LOC12`"]
    #[inline]
    pub fn is_loc12(&self) -> bool {
        *self == CC1LOCR::LOC12
    }
    #[doc = "Checks if the value of the field is `LOC13`"]
    #[inline]
    pub fn is_loc13(&self) -> bool {
        *self == CC1LOCR::LOC13
    }
    #[doc = "Checks if the value of the field is `LOC14`"]
    #[inline]
    pub fn is_loc14(&self) -> bool {
        *self == CC1LOCR::LOC14
    }
    #[doc = "Checks if the value of the field is `LOC15`"]
    #[inline]
    pub fn is_loc15(&self) -> bool {
        *self == CC1LOCR::LOC15
    }
    #[doc = "Checks if the value of the field is `LOC16`"]
    #[inline]
    pub fn is_loc16(&self) -> bool {
        *self == CC1LOCR::LOC16
    }
    #[doc = "Checks if the value of the field is `LOC17`"]
    #[inline]
    pub fn is_loc17(&self) -> bool {
        *self == CC1LOCR::LOC17
    }
    #[doc = "Checks if the value of the field is `LOC18`"]
    #[inline]
    pub fn is_loc18(&self) -> bool {
        *self == CC1LOCR::LOC18
    }
    #[doc = "Checks if the value of the field is `LOC19`"]
    #[inline]
    pub fn is_loc19(&self) -> bool {
        *self == CC1LOCR::LOC19
    }
    #[doc = "Checks if the value of the field is `LOC20`"]
    #[inline]
    pub fn is_loc20(&self) -> bool {
        *self == CC1LOCR::LOC20
    }
    #[doc = "Checks if the value of the field is `LOC21`"]
    #[inline]
    pub fn is_loc21(&self) -> bool {
        *self == CC1LOCR::LOC21
    }
    #[doc = "Checks if the value of the field is `LOC22`"]
    #[inline]
    pub fn is_loc22(&self) -> bool {
        *self == CC1LOCR::LOC22
    }
    #[doc = "Checks if the value of the field is `LOC23`"]
    #[inline]
    pub fn is_loc23(&self) -> bool {
        *self == CC1LOCR::LOC23
    }
    #[doc = "Checks if the value of the field is `LOC24`"]
    #[inline]
    pub fn is_loc24(&self) -> bool {
        *self == CC1LOCR::LOC24
    }
    #[doc = "Checks if the value of the field is `LOC25`"]
    #[inline]
    pub fn is_loc25(&self) -> bool {
        *self == CC1LOCR::LOC25
    }
    #[doc = "Checks if the value of the field is `LOC26`"]
    #[inline]
    pub fn is_loc26(&self) -> bool {
        *self == CC1LOCR::LOC26
    }
    #[doc = "Checks if the value of the field is `LOC27`"]
    #[inline]
    pub fn is_loc27(&self) -> bool {
        *self == CC1LOCR::LOC27
    }
    #[doc = "Checks if the value of the field is `LOC28`"]
    #[inline]
    pub fn is_loc28(&self) -> bool {
        *self == CC1LOCR::LOC28
    }
    #[doc = "Checks if the value of the field is `LOC29`"]
    #[inline]
    pub fn is_loc29(&self) -> bool {
        *self == CC1LOCR::LOC29
    }
    #[doc = "Checks if the value of the field is `LOC30`"]
    #[inline]
    pub fn is_loc30(&self) -> bool {
        *self == CC1LOCR::LOC30
    }
    #[doc = "Checks if the value of the field is `LOC31`"]
    #[inline]
    pub fn is_loc31(&self) -> bool {
        *self == CC1LOCR::LOC31
    }
}
#[doc = "Possible values of the field `CC2LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC2LOCR {
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
    #[doc = "Location 17"]
    LOC17,
    #[doc = "Location 18"]
    LOC18,
    #[doc = "Location 19"]
    LOC19,
    #[doc = "Location 20"]
    LOC20,
    #[doc = "Location 21"]
    LOC21,
    #[doc = "Location 22"]
    LOC22,
    #[doc = "Location 23"]
    LOC23,
    #[doc = "Location 24"]
    LOC24,
    #[doc = "Location 25"]
    LOC25,
    #[doc = "Location 26"]
    LOC26,
    #[doc = "Location 27"]
    LOC27,
    #[doc = "Location 28"]
    LOC28,
    #[doc = "Location 29"]
    LOC29,
    #[doc = "Location 30"]
    LOC30,
    #[doc = "Location 31"]
    LOC31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CC2LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CC2LOCR::LOC0 => 0,
            CC2LOCR::LOC1 => 1,
            CC2LOCR::LOC2 => 2,
            CC2LOCR::LOC3 => 3,
            CC2LOCR::LOC4 => 4,
            CC2LOCR::LOC5 => 5,
            CC2LOCR::LOC6 => 6,
            CC2LOCR::LOC7 => 7,
            CC2LOCR::LOC8 => 8,
            CC2LOCR::LOC9 => 9,
            CC2LOCR::LOC10 => 10,
            CC2LOCR::LOC11 => 11,
            CC2LOCR::LOC12 => 12,
            CC2LOCR::LOC13 => 13,
            CC2LOCR::LOC14 => 14,
            CC2LOCR::LOC15 => 15,
            CC2LOCR::LOC16 => 16,
            CC2LOCR::LOC17 => 17,
            CC2LOCR::LOC18 => 18,
            CC2LOCR::LOC19 => 19,
            CC2LOCR::LOC20 => 20,
            CC2LOCR::LOC21 => 21,
            CC2LOCR::LOC22 => 22,
            CC2LOCR::LOC23 => 23,
            CC2LOCR::LOC24 => 24,
            CC2LOCR::LOC25 => 25,
            CC2LOCR::LOC26 => 26,
            CC2LOCR::LOC27 => 27,
            CC2LOCR::LOC28 => 28,
            CC2LOCR::LOC29 => 29,
            CC2LOCR::LOC30 => 30,
            CC2LOCR::LOC31 => 31,
            CC2LOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CC2LOCR {
        match value {
            0 => CC2LOCR::LOC0,
            1 => CC2LOCR::LOC1,
            2 => CC2LOCR::LOC2,
            3 => CC2LOCR::LOC3,
            4 => CC2LOCR::LOC4,
            5 => CC2LOCR::LOC5,
            6 => CC2LOCR::LOC6,
            7 => CC2LOCR::LOC7,
            8 => CC2LOCR::LOC8,
            9 => CC2LOCR::LOC9,
            10 => CC2LOCR::LOC10,
            11 => CC2LOCR::LOC11,
            12 => CC2LOCR::LOC12,
            13 => CC2LOCR::LOC13,
            14 => CC2LOCR::LOC14,
            15 => CC2LOCR::LOC15,
            16 => CC2LOCR::LOC16,
            17 => CC2LOCR::LOC17,
            18 => CC2LOCR::LOC18,
            19 => CC2LOCR::LOC19,
            20 => CC2LOCR::LOC20,
            21 => CC2LOCR::LOC21,
            22 => CC2LOCR::LOC22,
            23 => CC2LOCR::LOC23,
            24 => CC2LOCR::LOC24,
            25 => CC2LOCR::LOC25,
            26 => CC2LOCR::LOC26,
            27 => CC2LOCR::LOC27,
            28 => CC2LOCR::LOC28,
            29 => CC2LOCR::LOC29,
            30 => CC2LOCR::LOC30,
            31 => CC2LOCR::LOC31,
            i => CC2LOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == CC2LOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == CC2LOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == CC2LOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == CC2LOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == CC2LOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == CC2LOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == CC2LOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == CC2LOCR::LOC7
    }
    #[doc = "Checks if the value of the field is `LOC8`"]
    #[inline]
    pub fn is_loc8(&self) -> bool {
        *self == CC2LOCR::LOC8
    }
    #[doc = "Checks if the value of the field is `LOC9`"]
    #[inline]
    pub fn is_loc9(&self) -> bool {
        *self == CC2LOCR::LOC9
    }
    #[doc = "Checks if the value of the field is `LOC10`"]
    #[inline]
    pub fn is_loc10(&self) -> bool {
        *self == CC2LOCR::LOC10
    }
    #[doc = "Checks if the value of the field is `LOC11`"]
    #[inline]
    pub fn is_loc11(&self) -> bool {
        *self == CC2LOCR::LOC11
    }
    #[doc = "Checks if the value of the field is `LOC12`"]
    #[inline]
    pub fn is_loc12(&self) -> bool {
        *self == CC2LOCR::LOC12
    }
    #[doc = "Checks if the value of the field is `LOC13`"]
    #[inline]
    pub fn is_loc13(&self) -> bool {
        *self == CC2LOCR::LOC13
    }
    #[doc = "Checks if the value of the field is `LOC14`"]
    #[inline]
    pub fn is_loc14(&self) -> bool {
        *self == CC2LOCR::LOC14
    }
    #[doc = "Checks if the value of the field is `LOC15`"]
    #[inline]
    pub fn is_loc15(&self) -> bool {
        *self == CC2LOCR::LOC15
    }
    #[doc = "Checks if the value of the field is `LOC16`"]
    #[inline]
    pub fn is_loc16(&self) -> bool {
        *self == CC2LOCR::LOC16
    }
    #[doc = "Checks if the value of the field is `LOC17`"]
    #[inline]
    pub fn is_loc17(&self) -> bool {
        *self == CC2LOCR::LOC17
    }
    #[doc = "Checks if the value of the field is `LOC18`"]
    #[inline]
    pub fn is_loc18(&self) -> bool {
        *self == CC2LOCR::LOC18
    }
    #[doc = "Checks if the value of the field is `LOC19`"]
    #[inline]
    pub fn is_loc19(&self) -> bool {
        *self == CC2LOCR::LOC19
    }
    #[doc = "Checks if the value of the field is `LOC20`"]
    #[inline]
    pub fn is_loc20(&self) -> bool {
        *self == CC2LOCR::LOC20
    }
    #[doc = "Checks if the value of the field is `LOC21`"]
    #[inline]
    pub fn is_loc21(&self) -> bool {
        *self == CC2LOCR::LOC21
    }
    #[doc = "Checks if the value of the field is `LOC22`"]
    #[inline]
    pub fn is_loc22(&self) -> bool {
        *self == CC2LOCR::LOC22
    }
    #[doc = "Checks if the value of the field is `LOC23`"]
    #[inline]
    pub fn is_loc23(&self) -> bool {
        *self == CC2LOCR::LOC23
    }
    #[doc = "Checks if the value of the field is `LOC24`"]
    #[inline]
    pub fn is_loc24(&self) -> bool {
        *self == CC2LOCR::LOC24
    }
    #[doc = "Checks if the value of the field is `LOC25`"]
    #[inline]
    pub fn is_loc25(&self) -> bool {
        *self == CC2LOCR::LOC25
    }
    #[doc = "Checks if the value of the field is `LOC26`"]
    #[inline]
    pub fn is_loc26(&self) -> bool {
        *self == CC2LOCR::LOC26
    }
    #[doc = "Checks if the value of the field is `LOC27`"]
    #[inline]
    pub fn is_loc27(&self) -> bool {
        *self == CC2LOCR::LOC27
    }
    #[doc = "Checks if the value of the field is `LOC28`"]
    #[inline]
    pub fn is_loc28(&self) -> bool {
        *self == CC2LOCR::LOC28
    }
    #[doc = "Checks if the value of the field is `LOC29`"]
    #[inline]
    pub fn is_loc29(&self) -> bool {
        *self == CC2LOCR::LOC29
    }
    #[doc = "Checks if the value of the field is `LOC30`"]
    #[inline]
    pub fn is_loc30(&self) -> bool {
        *self == CC2LOCR::LOC30
    }
    #[doc = "Checks if the value of the field is `LOC31`"]
    #[inline]
    pub fn is_loc31(&self) -> bool {
        *self == CC2LOCR::LOC31
    }
}
#[doc = "Possible values of the field `CC3LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC3LOCR {
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
    #[doc = "Location 17"]
    LOC17,
    #[doc = "Location 18"]
    LOC18,
    #[doc = "Location 19"]
    LOC19,
    #[doc = "Location 20"]
    LOC20,
    #[doc = "Location 21"]
    LOC21,
    #[doc = "Location 22"]
    LOC22,
    #[doc = "Location 23"]
    LOC23,
    #[doc = "Location 24"]
    LOC24,
    #[doc = "Location 25"]
    LOC25,
    #[doc = "Location 26"]
    LOC26,
    #[doc = "Location 27"]
    LOC27,
    #[doc = "Location 28"]
    LOC28,
    #[doc = "Location 29"]
    LOC29,
    #[doc = "Location 30"]
    LOC30,
    #[doc = "Location 31"]
    LOC31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CC3LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CC3LOCR::LOC0 => 0,
            CC3LOCR::LOC1 => 1,
            CC3LOCR::LOC2 => 2,
            CC3LOCR::LOC3 => 3,
            CC3LOCR::LOC4 => 4,
            CC3LOCR::LOC5 => 5,
            CC3LOCR::LOC6 => 6,
            CC3LOCR::LOC7 => 7,
            CC3LOCR::LOC8 => 8,
            CC3LOCR::LOC9 => 9,
            CC3LOCR::LOC10 => 10,
            CC3LOCR::LOC11 => 11,
            CC3LOCR::LOC12 => 12,
            CC3LOCR::LOC13 => 13,
            CC3LOCR::LOC14 => 14,
            CC3LOCR::LOC15 => 15,
            CC3LOCR::LOC16 => 16,
            CC3LOCR::LOC17 => 17,
            CC3LOCR::LOC18 => 18,
            CC3LOCR::LOC19 => 19,
            CC3LOCR::LOC20 => 20,
            CC3LOCR::LOC21 => 21,
            CC3LOCR::LOC22 => 22,
            CC3LOCR::LOC23 => 23,
            CC3LOCR::LOC24 => 24,
            CC3LOCR::LOC25 => 25,
            CC3LOCR::LOC26 => 26,
            CC3LOCR::LOC27 => 27,
            CC3LOCR::LOC28 => 28,
            CC3LOCR::LOC29 => 29,
            CC3LOCR::LOC30 => 30,
            CC3LOCR::LOC31 => 31,
            CC3LOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CC3LOCR {
        match value {
            0 => CC3LOCR::LOC0,
            1 => CC3LOCR::LOC1,
            2 => CC3LOCR::LOC2,
            3 => CC3LOCR::LOC3,
            4 => CC3LOCR::LOC4,
            5 => CC3LOCR::LOC5,
            6 => CC3LOCR::LOC6,
            7 => CC3LOCR::LOC7,
            8 => CC3LOCR::LOC8,
            9 => CC3LOCR::LOC9,
            10 => CC3LOCR::LOC10,
            11 => CC3LOCR::LOC11,
            12 => CC3LOCR::LOC12,
            13 => CC3LOCR::LOC13,
            14 => CC3LOCR::LOC14,
            15 => CC3LOCR::LOC15,
            16 => CC3LOCR::LOC16,
            17 => CC3LOCR::LOC17,
            18 => CC3LOCR::LOC18,
            19 => CC3LOCR::LOC19,
            20 => CC3LOCR::LOC20,
            21 => CC3LOCR::LOC21,
            22 => CC3LOCR::LOC22,
            23 => CC3LOCR::LOC23,
            24 => CC3LOCR::LOC24,
            25 => CC3LOCR::LOC25,
            26 => CC3LOCR::LOC26,
            27 => CC3LOCR::LOC27,
            28 => CC3LOCR::LOC28,
            29 => CC3LOCR::LOC29,
            30 => CC3LOCR::LOC30,
            31 => CC3LOCR::LOC31,
            i => CC3LOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == CC3LOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == CC3LOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == CC3LOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == CC3LOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == CC3LOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == CC3LOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == CC3LOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == CC3LOCR::LOC7
    }
    #[doc = "Checks if the value of the field is `LOC8`"]
    #[inline]
    pub fn is_loc8(&self) -> bool {
        *self == CC3LOCR::LOC8
    }
    #[doc = "Checks if the value of the field is `LOC9`"]
    #[inline]
    pub fn is_loc9(&self) -> bool {
        *self == CC3LOCR::LOC9
    }
    #[doc = "Checks if the value of the field is `LOC10`"]
    #[inline]
    pub fn is_loc10(&self) -> bool {
        *self == CC3LOCR::LOC10
    }
    #[doc = "Checks if the value of the field is `LOC11`"]
    #[inline]
    pub fn is_loc11(&self) -> bool {
        *self == CC3LOCR::LOC11
    }
    #[doc = "Checks if the value of the field is `LOC12`"]
    #[inline]
    pub fn is_loc12(&self) -> bool {
        *self == CC3LOCR::LOC12
    }
    #[doc = "Checks if the value of the field is `LOC13`"]
    #[inline]
    pub fn is_loc13(&self) -> bool {
        *self == CC3LOCR::LOC13
    }
    #[doc = "Checks if the value of the field is `LOC14`"]
    #[inline]
    pub fn is_loc14(&self) -> bool {
        *self == CC3LOCR::LOC14
    }
    #[doc = "Checks if the value of the field is `LOC15`"]
    #[inline]
    pub fn is_loc15(&self) -> bool {
        *self == CC3LOCR::LOC15
    }
    #[doc = "Checks if the value of the field is `LOC16`"]
    #[inline]
    pub fn is_loc16(&self) -> bool {
        *self == CC3LOCR::LOC16
    }
    #[doc = "Checks if the value of the field is `LOC17`"]
    #[inline]
    pub fn is_loc17(&self) -> bool {
        *self == CC3LOCR::LOC17
    }
    #[doc = "Checks if the value of the field is `LOC18`"]
    #[inline]
    pub fn is_loc18(&self) -> bool {
        *self == CC3LOCR::LOC18
    }
    #[doc = "Checks if the value of the field is `LOC19`"]
    #[inline]
    pub fn is_loc19(&self) -> bool {
        *self == CC3LOCR::LOC19
    }
    #[doc = "Checks if the value of the field is `LOC20`"]
    #[inline]
    pub fn is_loc20(&self) -> bool {
        *self == CC3LOCR::LOC20
    }
    #[doc = "Checks if the value of the field is `LOC21`"]
    #[inline]
    pub fn is_loc21(&self) -> bool {
        *self == CC3LOCR::LOC21
    }
    #[doc = "Checks if the value of the field is `LOC22`"]
    #[inline]
    pub fn is_loc22(&self) -> bool {
        *self == CC3LOCR::LOC22
    }
    #[doc = "Checks if the value of the field is `LOC23`"]
    #[inline]
    pub fn is_loc23(&self) -> bool {
        *self == CC3LOCR::LOC23
    }
    #[doc = "Checks if the value of the field is `LOC24`"]
    #[inline]
    pub fn is_loc24(&self) -> bool {
        *self == CC3LOCR::LOC24
    }
    #[doc = "Checks if the value of the field is `LOC25`"]
    #[inline]
    pub fn is_loc25(&self) -> bool {
        *self == CC3LOCR::LOC25
    }
    #[doc = "Checks if the value of the field is `LOC26`"]
    #[inline]
    pub fn is_loc26(&self) -> bool {
        *self == CC3LOCR::LOC26
    }
    #[doc = "Checks if the value of the field is `LOC27`"]
    #[inline]
    pub fn is_loc27(&self) -> bool {
        *self == CC3LOCR::LOC27
    }
    #[doc = "Checks if the value of the field is `LOC28`"]
    #[inline]
    pub fn is_loc28(&self) -> bool {
        *self == CC3LOCR::LOC28
    }
    #[doc = "Checks if the value of the field is `LOC29`"]
    #[inline]
    pub fn is_loc29(&self) -> bool {
        *self == CC3LOCR::LOC29
    }
    #[doc = "Checks if the value of the field is `LOC30`"]
    #[inline]
    pub fn is_loc30(&self) -> bool {
        *self == CC3LOCR::LOC30
    }
    #[doc = "Checks if the value of the field is `LOC31`"]
    #[inline]
    pub fn is_loc31(&self) -> bool {
        *self == CC3LOCR::LOC31
    }
}
#[doc = "Values that can be written to the field `CC0LOC`"]
pub enum CC0LOCW {
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
    #[doc = "Location 17"]
    LOC17,
    #[doc = "Location 18"]
    LOC18,
    #[doc = "Location 19"]
    LOC19,
    #[doc = "Location 20"]
    LOC20,
    #[doc = "Location 21"]
    LOC21,
    #[doc = "Location 22"]
    LOC22,
    #[doc = "Location 23"]
    LOC23,
    #[doc = "Location 24"]
    LOC24,
    #[doc = "Location 25"]
    LOC25,
    #[doc = "Location 26"]
    LOC26,
    #[doc = "Location 27"]
    LOC27,
    #[doc = "Location 28"]
    LOC28,
    #[doc = "Location 29"]
    LOC29,
    #[doc = "Location 30"]
    LOC30,
    #[doc = "Location 31"]
    LOC31,
}
impl CC0LOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CC0LOCW::LOC0 => 0,
            CC0LOCW::LOC1 => 1,
            CC0LOCW::LOC2 => 2,
            CC0LOCW::LOC3 => 3,
            CC0LOCW::LOC4 => 4,
            CC0LOCW::LOC5 => 5,
            CC0LOCW::LOC6 => 6,
            CC0LOCW::LOC7 => 7,
            CC0LOCW::LOC8 => 8,
            CC0LOCW::LOC9 => 9,
            CC0LOCW::LOC10 => 10,
            CC0LOCW::LOC11 => 11,
            CC0LOCW::LOC12 => 12,
            CC0LOCW::LOC13 => 13,
            CC0LOCW::LOC14 => 14,
            CC0LOCW::LOC15 => 15,
            CC0LOCW::LOC16 => 16,
            CC0LOCW::LOC17 => 17,
            CC0LOCW::LOC18 => 18,
            CC0LOCW::LOC19 => 19,
            CC0LOCW::LOC20 => 20,
            CC0LOCW::LOC21 => 21,
            CC0LOCW::LOC22 => 22,
            CC0LOCW::LOC23 => 23,
            CC0LOCW::LOC24 => 24,
            CC0LOCW::LOC25 => 25,
            CC0LOCW::LOC26 => 26,
            CC0LOCW::LOC27 => 27,
            CC0LOCW::LOC28 => 28,
            CC0LOCW::LOC29 => 29,
            CC0LOCW::LOC30 => 30,
            CC0LOCW::LOC31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC0LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _CC0LOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC0LOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC7)
    }
    #[doc = "Location 8"]
    #[inline]
    pub fn loc8(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC8)
    }
    #[doc = "Location 9"]
    #[inline]
    pub fn loc9(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC9)
    }
    #[doc = "Location 10"]
    #[inline]
    pub fn loc10(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC10)
    }
    #[doc = "Location 11"]
    #[inline]
    pub fn loc11(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC11)
    }
    #[doc = "Location 12"]
    #[inline]
    pub fn loc12(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC12)
    }
    #[doc = "Location 13"]
    #[inline]
    pub fn loc13(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC13)
    }
    #[doc = "Location 14"]
    #[inline]
    pub fn loc14(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC14)
    }
    #[doc = "Location 15"]
    #[inline]
    pub fn loc15(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC15)
    }
    #[doc = "Location 16"]
    #[inline]
    pub fn loc16(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC16)
    }
    #[doc = "Location 17"]
    #[inline]
    pub fn loc17(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC17)
    }
    #[doc = "Location 18"]
    #[inline]
    pub fn loc18(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC18)
    }
    #[doc = "Location 19"]
    #[inline]
    pub fn loc19(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC19)
    }
    #[doc = "Location 20"]
    #[inline]
    pub fn loc20(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC20)
    }
    #[doc = "Location 21"]
    #[inline]
    pub fn loc21(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC21)
    }
    #[doc = "Location 22"]
    #[inline]
    pub fn loc22(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC22)
    }
    #[doc = "Location 23"]
    #[inline]
    pub fn loc23(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC23)
    }
    #[doc = "Location 24"]
    #[inline]
    pub fn loc24(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC24)
    }
    #[doc = "Location 25"]
    #[inline]
    pub fn loc25(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC25)
    }
    #[doc = "Location 26"]
    #[inline]
    pub fn loc26(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC26)
    }
    #[doc = "Location 27"]
    #[inline]
    pub fn loc27(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC27)
    }
    #[doc = "Location 28"]
    #[inline]
    pub fn loc28(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC28)
    }
    #[doc = "Location 29"]
    #[inline]
    pub fn loc29(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC29)
    }
    #[doc = "Location 30"]
    #[inline]
    pub fn loc30(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC30)
    }
    #[doc = "Location 31"]
    #[inline]
    pub fn loc31(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC31)
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
#[doc = "Values that can be written to the field `CC1LOC`"]
pub enum CC1LOCW {
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
    #[doc = "Location 17"]
    LOC17,
    #[doc = "Location 18"]
    LOC18,
    #[doc = "Location 19"]
    LOC19,
    #[doc = "Location 20"]
    LOC20,
    #[doc = "Location 21"]
    LOC21,
    #[doc = "Location 22"]
    LOC22,
    #[doc = "Location 23"]
    LOC23,
    #[doc = "Location 24"]
    LOC24,
    #[doc = "Location 25"]
    LOC25,
    #[doc = "Location 26"]
    LOC26,
    #[doc = "Location 27"]
    LOC27,
    #[doc = "Location 28"]
    LOC28,
    #[doc = "Location 29"]
    LOC29,
    #[doc = "Location 30"]
    LOC30,
    #[doc = "Location 31"]
    LOC31,
}
impl CC1LOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CC1LOCW::LOC0 => 0,
            CC1LOCW::LOC1 => 1,
            CC1LOCW::LOC2 => 2,
            CC1LOCW::LOC3 => 3,
            CC1LOCW::LOC4 => 4,
            CC1LOCW::LOC5 => 5,
            CC1LOCW::LOC6 => 6,
            CC1LOCW::LOC7 => 7,
            CC1LOCW::LOC8 => 8,
            CC1LOCW::LOC9 => 9,
            CC1LOCW::LOC10 => 10,
            CC1LOCW::LOC11 => 11,
            CC1LOCW::LOC12 => 12,
            CC1LOCW::LOC13 => 13,
            CC1LOCW::LOC14 => 14,
            CC1LOCW::LOC15 => 15,
            CC1LOCW::LOC16 => 16,
            CC1LOCW::LOC17 => 17,
            CC1LOCW::LOC18 => 18,
            CC1LOCW::LOC19 => 19,
            CC1LOCW::LOC20 => 20,
            CC1LOCW::LOC21 => 21,
            CC1LOCW::LOC22 => 22,
            CC1LOCW::LOC23 => 23,
            CC1LOCW::LOC24 => 24,
            CC1LOCW::LOC25 => 25,
            CC1LOCW::LOC26 => 26,
            CC1LOCW::LOC27 => 27,
            CC1LOCW::LOC28 => 28,
            CC1LOCW::LOC29 => 29,
            CC1LOCW::LOC30 => 30,
            CC1LOCW::LOC31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC1LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _CC1LOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC1LOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC7)
    }
    #[doc = "Location 8"]
    #[inline]
    pub fn loc8(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC8)
    }
    #[doc = "Location 9"]
    #[inline]
    pub fn loc9(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC9)
    }
    #[doc = "Location 10"]
    #[inline]
    pub fn loc10(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC10)
    }
    #[doc = "Location 11"]
    #[inline]
    pub fn loc11(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC11)
    }
    #[doc = "Location 12"]
    #[inline]
    pub fn loc12(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC12)
    }
    #[doc = "Location 13"]
    #[inline]
    pub fn loc13(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC13)
    }
    #[doc = "Location 14"]
    #[inline]
    pub fn loc14(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC14)
    }
    #[doc = "Location 15"]
    #[inline]
    pub fn loc15(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC15)
    }
    #[doc = "Location 16"]
    #[inline]
    pub fn loc16(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC16)
    }
    #[doc = "Location 17"]
    #[inline]
    pub fn loc17(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC17)
    }
    #[doc = "Location 18"]
    #[inline]
    pub fn loc18(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC18)
    }
    #[doc = "Location 19"]
    #[inline]
    pub fn loc19(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC19)
    }
    #[doc = "Location 20"]
    #[inline]
    pub fn loc20(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC20)
    }
    #[doc = "Location 21"]
    #[inline]
    pub fn loc21(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC21)
    }
    #[doc = "Location 22"]
    #[inline]
    pub fn loc22(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC22)
    }
    #[doc = "Location 23"]
    #[inline]
    pub fn loc23(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC23)
    }
    #[doc = "Location 24"]
    #[inline]
    pub fn loc24(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC24)
    }
    #[doc = "Location 25"]
    #[inline]
    pub fn loc25(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC25)
    }
    #[doc = "Location 26"]
    #[inline]
    pub fn loc26(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC26)
    }
    #[doc = "Location 27"]
    #[inline]
    pub fn loc27(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC27)
    }
    #[doc = "Location 28"]
    #[inline]
    pub fn loc28(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC28)
    }
    #[doc = "Location 29"]
    #[inline]
    pub fn loc29(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC29)
    }
    #[doc = "Location 30"]
    #[inline]
    pub fn loc30(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC30)
    }
    #[doc = "Location 31"]
    #[inline]
    pub fn loc31(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC31)
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
#[doc = "Values that can be written to the field `CC2LOC`"]
pub enum CC2LOCW {
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
    #[doc = "Location 17"]
    LOC17,
    #[doc = "Location 18"]
    LOC18,
    #[doc = "Location 19"]
    LOC19,
    #[doc = "Location 20"]
    LOC20,
    #[doc = "Location 21"]
    LOC21,
    #[doc = "Location 22"]
    LOC22,
    #[doc = "Location 23"]
    LOC23,
    #[doc = "Location 24"]
    LOC24,
    #[doc = "Location 25"]
    LOC25,
    #[doc = "Location 26"]
    LOC26,
    #[doc = "Location 27"]
    LOC27,
    #[doc = "Location 28"]
    LOC28,
    #[doc = "Location 29"]
    LOC29,
    #[doc = "Location 30"]
    LOC30,
    #[doc = "Location 31"]
    LOC31,
}
impl CC2LOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CC2LOCW::LOC0 => 0,
            CC2LOCW::LOC1 => 1,
            CC2LOCW::LOC2 => 2,
            CC2LOCW::LOC3 => 3,
            CC2LOCW::LOC4 => 4,
            CC2LOCW::LOC5 => 5,
            CC2LOCW::LOC6 => 6,
            CC2LOCW::LOC7 => 7,
            CC2LOCW::LOC8 => 8,
            CC2LOCW::LOC9 => 9,
            CC2LOCW::LOC10 => 10,
            CC2LOCW::LOC11 => 11,
            CC2LOCW::LOC12 => 12,
            CC2LOCW::LOC13 => 13,
            CC2LOCW::LOC14 => 14,
            CC2LOCW::LOC15 => 15,
            CC2LOCW::LOC16 => 16,
            CC2LOCW::LOC17 => 17,
            CC2LOCW::LOC18 => 18,
            CC2LOCW::LOC19 => 19,
            CC2LOCW::LOC20 => 20,
            CC2LOCW::LOC21 => 21,
            CC2LOCW::LOC22 => 22,
            CC2LOCW::LOC23 => 23,
            CC2LOCW::LOC24 => 24,
            CC2LOCW::LOC25 => 25,
            CC2LOCW::LOC26 => 26,
            CC2LOCW::LOC27 => 27,
            CC2LOCW::LOC28 => 28,
            CC2LOCW::LOC29 => 29,
            CC2LOCW::LOC30 => 30,
            CC2LOCW::LOC31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC2LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _CC2LOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC2LOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC7)
    }
    #[doc = "Location 8"]
    #[inline]
    pub fn loc8(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC8)
    }
    #[doc = "Location 9"]
    #[inline]
    pub fn loc9(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC9)
    }
    #[doc = "Location 10"]
    #[inline]
    pub fn loc10(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC10)
    }
    #[doc = "Location 11"]
    #[inline]
    pub fn loc11(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC11)
    }
    #[doc = "Location 12"]
    #[inline]
    pub fn loc12(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC12)
    }
    #[doc = "Location 13"]
    #[inline]
    pub fn loc13(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC13)
    }
    #[doc = "Location 14"]
    #[inline]
    pub fn loc14(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC14)
    }
    #[doc = "Location 15"]
    #[inline]
    pub fn loc15(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC15)
    }
    #[doc = "Location 16"]
    #[inline]
    pub fn loc16(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC16)
    }
    #[doc = "Location 17"]
    #[inline]
    pub fn loc17(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC17)
    }
    #[doc = "Location 18"]
    #[inline]
    pub fn loc18(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC18)
    }
    #[doc = "Location 19"]
    #[inline]
    pub fn loc19(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC19)
    }
    #[doc = "Location 20"]
    #[inline]
    pub fn loc20(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC20)
    }
    #[doc = "Location 21"]
    #[inline]
    pub fn loc21(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC21)
    }
    #[doc = "Location 22"]
    #[inline]
    pub fn loc22(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC22)
    }
    #[doc = "Location 23"]
    #[inline]
    pub fn loc23(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC23)
    }
    #[doc = "Location 24"]
    #[inline]
    pub fn loc24(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC24)
    }
    #[doc = "Location 25"]
    #[inline]
    pub fn loc25(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC25)
    }
    #[doc = "Location 26"]
    #[inline]
    pub fn loc26(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC26)
    }
    #[doc = "Location 27"]
    #[inline]
    pub fn loc27(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC27)
    }
    #[doc = "Location 28"]
    #[inline]
    pub fn loc28(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC28)
    }
    #[doc = "Location 29"]
    #[inline]
    pub fn loc29(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC29)
    }
    #[doc = "Location 30"]
    #[inline]
    pub fn loc30(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC30)
    }
    #[doc = "Location 31"]
    #[inline]
    pub fn loc31(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC31)
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
#[doc = "Values that can be written to the field `CC3LOC`"]
pub enum CC3LOCW {
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
    #[doc = "Location 17"]
    LOC17,
    #[doc = "Location 18"]
    LOC18,
    #[doc = "Location 19"]
    LOC19,
    #[doc = "Location 20"]
    LOC20,
    #[doc = "Location 21"]
    LOC21,
    #[doc = "Location 22"]
    LOC22,
    #[doc = "Location 23"]
    LOC23,
    #[doc = "Location 24"]
    LOC24,
    #[doc = "Location 25"]
    LOC25,
    #[doc = "Location 26"]
    LOC26,
    #[doc = "Location 27"]
    LOC27,
    #[doc = "Location 28"]
    LOC28,
    #[doc = "Location 29"]
    LOC29,
    #[doc = "Location 30"]
    LOC30,
    #[doc = "Location 31"]
    LOC31,
}
impl CC3LOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CC3LOCW::LOC0 => 0,
            CC3LOCW::LOC1 => 1,
            CC3LOCW::LOC2 => 2,
            CC3LOCW::LOC3 => 3,
            CC3LOCW::LOC4 => 4,
            CC3LOCW::LOC5 => 5,
            CC3LOCW::LOC6 => 6,
            CC3LOCW::LOC7 => 7,
            CC3LOCW::LOC8 => 8,
            CC3LOCW::LOC9 => 9,
            CC3LOCW::LOC10 => 10,
            CC3LOCW::LOC11 => 11,
            CC3LOCW::LOC12 => 12,
            CC3LOCW::LOC13 => 13,
            CC3LOCW::LOC14 => 14,
            CC3LOCW::LOC15 => 15,
            CC3LOCW::LOC16 => 16,
            CC3LOCW::LOC17 => 17,
            CC3LOCW::LOC18 => 18,
            CC3LOCW::LOC19 => 19,
            CC3LOCW::LOC20 => 20,
            CC3LOCW::LOC21 => 21,
            CC3LOCW::LOC22 => 22,
            CC3LOCW::LOC23 => 23,
            CC3LOCW::LOC24 => 24,
            CC3LOCW::LOC25 => 25,
            CC3LOCW::LOC26 => 26,
            CC3LOCW::LOC27 => 27,
            CC3LOCW::LOC28 => 28,
            CC3LOCW::LOC29 => 29,
            CC3LOCW::LOC30 => 30,
            CC3LOCW::LOC31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC3LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _CC3LOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC3LOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC7)
    }
    #[doc = "Location 8"]
    #[inline]
    pub fn loc8(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC8)
    }
    #[doc = "Location 9"]
    #[inline]
    pub fn loc9(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC9)
    }
    #[doc = "Location 10"]
    #[inline]
    pub fn loc10(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC10)
    }
    #[doc = "Location 11"]
    #[inline]
    pub fn loc11(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC11)
    }
    #[doc = "Location 12"]
    #[inline]
    pub fn loc12(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC12)
    }
    #[doc = "Location 13"]
    #[inline]
    pub fn loc13(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC13)
    }
    #[doc = "Location 14"]
    #[inline]
    pub fn loc14(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC14)
    }
    #[doc = "Location 15"]
    #[inline]
    pub fn loc15(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC15)
    }
    #[doc = "Location 16"]
    #[inline]
    pub fn loc16(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC16)
    }
    #[doc = "Location 17"]
    #[inline]
    pub fn loc17(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC17)
    }
    #[doc = "Location 18"]
    #[inline]
    pub fn loc18(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC18)
    }
    #[doc = "Location 19"]
    #[inline]
    pub fn loc19(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC19)
    }
    #[doc = "Location 20"]
    #[inline]
    pub fn loc20(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC20)
    }
    #[doc = "Location 21"]
    #[inline]
    pub fn loc21(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC21)
    }
    #[doc = "Location 22"]
    #[inline]
    pub fn loc22(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC22)
    }
    #[doc = "Location 23"]
    #[inline]
    pub fn loc23(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC23)
    }
    #[doc = "Location 24"]
    #[inline]
    pub fn loc24(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC24)
    }
    #[doc = "Location 25"]
    #[inline]
    pub fn loc25(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC25)
    }
    #[doc = "Location 26"]
    #[inline]
    pub fn loc26(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC26)
    }
    #[doc = "Location 27"]
    #[inline]
    pub fn loc27(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC27)
    }
    #[doc = "Location 28"]
    #[inline]
    pub fn loc28(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC28)
    }
    #[doc = "Location 29"]
    #[inline]
    pub fn loc29(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC29)
    }
    #[doc = "Location 30"]
    #[inline]
    pub fn loc30(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC30)
    }
    #[doc = "Location 31"]
    #[inline]
    pub fn loc31(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC31)
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
    pub fn cc0loc(&self) -> CC0LOCR {
        CC0LOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline]
    pub fn cc1loc(&self) -> CC1LOCR {
        CC1LOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline]
    pub fn cc2loc(&self) -> CC2LOCR {
        CC2LOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline]
    pub fn cc3loc(&self) -> CC3LOCR {
        CC3LOCR::_from({
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
    pub fn cc0loc(&mut self) -> _CC0LOCW {
        _CC0LOCW { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline]
    pub fn cc1loc(&mut self) -> _CC1LOCW {
        _CC1LOCW { w: self }
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline]
    pub fn cc2loc(&mut self) -> _CC2LOCW {
        _CC2LOCW { w: self }
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline]
    pub fn cc3loc(&mut self) -> _CC3LOCW {
        _CC3LOCW { w: self }
    }
}
