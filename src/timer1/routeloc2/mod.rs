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
#[doc = "Possible values of the field `CDTI0LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDTI0LOCR {
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
impl CDTI0LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CDTI0LOCR::LOC0 => 0,
            CDTI0LOCR::LOC1 => 1,
            CDTI0LOCR::LOC2 => 2,
            CDTI0LOCR::LOC3 => 3,
            CDTI0LOCR::LOC4 => 4,
            CDTI0LOCR::LOC5 => 5,
            CDTI0LOCR::LOC6 => 6,
            CDTI0LOCR::LOC7 => 7,
            CDTI0LOCR::LOC8 => 8,
            CDTI0LOCR::LOC9 => 9,
            CDTI0LOCR::LOC10 => 10,
            CDTI0LOCR::LOC11 => 11,
            CDTI0LOCR::LOC12 => 12,
            CDTI0LOCR::LOC13 => 13,
            CDTI0LOCR::LOC14 => 14,
            CDTI0LOCR::LOC15 => 15,
            CDTI0LOCR::LOC16 => 16,
            CDTI0LOCR::LOC17 => 17,
            CDTI0LOCR::LOC18 => 18,
            CDTI0LOCR::LOC19 => 19,
            CDTI0LOCR::LOC20 => 20,
            CDTI0LOCR::LOC21 => 21,
            CDTI0LOCR::LOC22 => 22,
            CDTI0LOCR::LOC23 => 23,
            CDTI0LOCR::LOC24 => 24,
            CDTI0LOCR::LOC25 => 25,
            CDTI0LOCR::LOC26 => 26,
            CDTI0LOCR::LOC27 => 27,
            CDTI0LOCR::LOC28 => 28,
            CDTI0LOCR::LOC29 => 29,
            CDTI0LOCR::LOC30 => 30,
            CDTI0LOCR::LOC31 => 31,
            CDTI0LOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CDTI0LOCR {
        match value {
            0 => CDTI0LOCR::LOC0,
            1 => CDTI0LOCR::LOC1,
            2 => CDTI0LOCR::LOC2,
            3 => CDTI0LOCR::LOC3,
            4 => CDTI0LOCR::LOC4,
            5 => CDTI0LOCR::LOC5,
            6 => CDTI0LOCR::LOC6,
            7 => CDTI0LOCR::LOC7,
            8 => CDTI0LOCR::LOC8,
            9 => CDTI0LOCR::LOC9,
            10 => CDTI0LOCR::LOC10,
            11 => CDTI0LOCR::LOC11,
            12 => CDTI0LOCR::LOC12,
            13 => CDTI0LOCR::LOC13,
            14 => CDTI0LOCR::LOC14,
            15 => CDTI0LOCR::LOC15,
            16 => CDTI0LOCR::LOC16,
            17 => CDTI0LOCR::LOC17,
            18 => CDTI0LOCR::LOC18,
            19 => CDTI0LOCR::LOC19,
            20 => CDTI0LOCR::LOC20,
            21 => CDTI0LOCR::LOC21,
            22 => CDTI0LOCR::LOC22,
            23 => CDTI0LOCR::LOC23,
            24 => CDTI0LOCR::LOC24,
            25 => CDTI0LOCR::LOC25,
            26 => CDTI0LOCR::LOC26,
            27 => CDTI0LOCR::LOC27,
            28 => CDTI0LOCR::LOC28,
            29 => CDTI0LOCR::LOC29,
            30 => CDTI0LOCR::LOC30,
            31 => CDTI0LOCR::LOC31,
            i => CDTI0LOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == CDTI0LOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == CDTI0LOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == CDTI0LOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == CDTI0LOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == CDTI0LOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == CDTI0LOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == CDTI0LOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == CDTI0LOCR::LOC7
    }
    #[doc = "Checks if the value of the field is `LOC8`"]
    #[inline]
    pub fn is_loc8(&self) -> bool {
        *self == CDTI0LOCR::LOC8
    }
    #[doc = "Checks if the value of the field is `LOC9`"]
    #[inline]
    pub fn is_loc9(&self) -> bool {
        *self == CDTI0LOCR::LOC9
    }
    #[doc = "Checks if the value of the field is `LOC10`"]
    #[inline]
    pub fn is_loc10(&self) -> bool {
        *self == CDTI0LOCR::LOC10
    }
    #[doc = "Checks if the value of the field is `LOC11`"]
    #[inline]
    pub fn is_loc11(&self) -> bool {
        *self == CDTI0LOCR::LOC11
    }
    #[doc = "Checks if the value of the field is `LOC12`"]
    #[inline]
    pub fn is_loc12(&self) -> bool {
        *self == CDTI0LOCR::LOC12
    }
    #[doc = "Checks if the value of the field is `LOC13`"]
    #[inline]
    pub fn is_loc13(&self) -> bool {
        *self == CDTI0LOCR::LOC13
    }
    #[doc = "Checks if the value of the field is `LOC14`"]
    #[inline]
    pub fn is_loc14(&self) -> bool {
        *self == CDTI0LOCR::LOC14
    }
    #[doc = "Checks if the value of the field is `LOC15`"]
    #[inline]
    pub fn is_loc15(&self) -> bool {
        *self == CDTI0LOCR::LOC15
    }
    #[doc = "Checks if the value of the field is `LOC16`"]
    #[inline]
    pub fn is_loc16(&self) -> bool {
        *self == CDTI0LOCR::LOC16
    }
    #[doc = "Checks if the value of the field is `LOC17`"]
    #[inline]
    pub fn is_loc17(&self) -> bool {
        *self == CDTI0LOCR::LOC17
    }
    #[doc = "Checks if the value of the field is `LOC18`"]
    #[inline]
    pub fn is_loc18(&self) -> bool {
        *self == CDTI0LOCR::LOC18
    }
    #[doc = "Checks if the value of the field is `LOC19`"]
    #[inline]
    pub fn is_loc19(&self) -> bool {
        *self == CDTI0LOCR::LOC19
    }
    #[doc = "Checks if the value of the field is `LOC20`"]
    #[inline]
    pub fn is_loc20(&self) -> bool {
        *self == CDTI0LOCR::LOC20
    }
    #[doc = "Checks if the value of the field is `LOC21`"]
    #[inline]
    pub fn is_loc21(&self) -> bool {
        *self == CDTI0LOCR::LOC21
    }
    #[doc = "Checks if the value of the field is `LOC22`"]
    #[inline]
    pub fn is_loc22(&self) -> bool {
        *self == CDTI0LOCR::LOC22
    }
    #[doc = "Checks if the value of the field is `LOC23`"]
    #[inline]
    pub fn is_loc23(&self) -> bool {
        *self == CDTI0LOCR::LOC23
    }
    #[doc = "Checks if the value of the field is `LOC24`"]
    #[inline]
    pub fn is_loc24(&self) -> bool {
        *self == CDTI0LOCR::LOC24
    }
    #[doc = "Checks if the value of the field is `LOC25`"]
    #[inline]
    pub fn is_loc25(&self) -> bool {
        *self == CDTI0LOCR::LOC25
    }
    #[doc = "Checks if the value of the field is `LOC26`"]
    #[inline]
    pub fn is_loc26(&self) -> bool {
        *self == CDTI0LOCR::LOC26
    }
    #[doc = "Checks if the value of the field is `LOC27`"]
    #[inline]
    pub fn is_loc27(&self) -> bool {
        *self == CDTI0LOCR::LOC27
    }
    #[doc = "Checks if the value of the field is `LOC28`"]
    #[inline]
    pub fn is_loc28(&self) -> bool {
        *self == CDTI0LOCR::LOC28
    }
    #[doc = "Checks if the value of the field is `LOC29`"]
    #[inline]
    pub fn is_loc29(&self) -> bool {
        *self == CDTI0LOCR::LOC29
    }
    #[doc = "Checks if the value of the field is `LOC30`"]
    #[inline]
    pub fn is_loc30(&self) -> bool {
        *self == CDTI0LOCR::LOC30
    }
    #[doc = "Checks if the value of the field is `LOC31`"]
    #[inline]
    pub fn is_loc31(&self) -> bool {
        *self == CDTI0LOCR::LOC31
    }
}
#[doc = "Possible values of the field `CDTI1LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDTI1LOCR {
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
impl CDTI1LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CDTI1LOCR::LOC0 => 0,
            CDTI1LOCR::LOC1 => 1,
            CDTI1LOCR::LOC2 => 2,
            CDTI1LOCR::LOC3 => 3,
            CDTI1LOCR::LOC4 => 4,
            CDTI1LOCR::LOC5 => 5,
            CDTI1LOCR::LOC6 => 6,
            CDTI1LOCR::LOC7 => 7,
            CDTI1LOCR::LOC8 => 8,
            CDTI1LOCR::LOC9 => 9,
            CDTI1LOCR::LOC10 => 10,
            CDTI1LOCR::LOC11 => 11,
            CDTI1LOCR::LOC12 => 12,
            CDTI1LOCR::LOC13 => 13,
            CDTI1LOCR::LOC14 => 14,
            CDTI1LOCR::LOC15 => 15,
            CDTI1LOCR::LOC16 => 16,
            CDTI1LOCR::LOC17 => 17,
            CDTI1LOCR::LOC18 => 18,
            CDTI1LOCR::LOC19 => 19,
            CDTI1LOCR::LOC20 => 20,
            CDTI1LOCR::LOC21 => 21,
            CDTI1LOCR::LOC22 => 22,
            CDTI1LOCR::LOC23 => 23,
            CDTI1LOCR::LOC24 => 24,
            CDTI1LOCR::LOC25 => 25,
            CDTI1LOCR::LOC26 => 26,
            CDTI1LOCR::LOC27 => 27,
            CDTI1LOCR::LOC28 => 28,
            CDTI1LOCR::LOC29 => 29,
            CDTI1LOCR::LOC30 => 30,
            CDTI1LOCR::LOC31 => 31,
            CDTI1LOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CDTI1LOCR {
        match value {
            0 => CDTI1LOCR::LOC0,
            1 => CDTI1LOCR::LOC1,
            2 => CDTI1LOCR::LOC2,
            3 => CDTI1LOCR::LOC3,
            4 => CDTI1LOCR::LOC4,
            5 => CDTI1LOCR::LOC5,
            6 => CDTI1LOCR::LOC6,
            7 => CDTI1LOCR::LOC7,
            8 => CDTI1LOCR::LOC8,
            9 => CDTI1LOCR::LOC9,
            10 => CDTI1LOCR::LOC10,
            11 => CDTI1LOCR::LOC11,
            12 => CDTI1LOCR::LOC12,
            13 => CDTI1LOCR::LOC13,
            14 => CDTI1LOCR::LOC14,
            15 => CDTI1LOCR::LOC15,
            16 => CDTI1LOCR::LOC16,
            17 => CDTI1LOCR::LOC17,
            18 => CDTI1LOCR::LOC18,
            19 => CDTI1LOCR::LOC19,
            20 => CDTI1LOCR::LOC20,
            21 => CDTI1LOCR::LOC21,
            22 => CDTI1LOCR::LOC22,
            23 => CDTI1LOCR::LOC23,
            24 => CDTI1LOCR::LOC24,
            25 => CDTI1LOCR::LOC25,
            26 => CDTI1LOCR::LOC26,
            27 => CDTI1LOCR::LOC27,
            28 => CDTI1LOCR::LOC28,
            29 => CDTI1LOCR::LOC29,
            30 => CDTI1LOCR::LOC30,
            31 => CDTI1LOCR::LOC31,
            i => CDTI1LOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == CDTI1LOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == CDTI1LOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == CDTI1LOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == CDTI1LOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == CDTI1LOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == CDTI1LOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == CDTI1LOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == CDTI1LOCR::LOC7
    }
    #[doc = "Checks if the value of the field is `LOC8`"]
    #[inline]
    pub fn is_loc8(&self) -> bool {
        *self == CDTI1LOCR::LOC8
    }
    #[doc = "Checks if the value of the field is `LOC9`"]
    #[inline]
    pub fn is_loc9(&self) -> bool {
        *self == CDTI1LOCR::LOC9
    }
    #[doc = "Checks if the value of the field is `LOC10`"]
    #[inline]
    pub fn is_loc10(&self) -> bool {
        *self == CDTI1LOCR::LOC10
    }
    #[doc = "Checks if the value of the field is `LOC11`"]
    #[inline]
    pub fn is_loc11(&self) -> bool {
        *self == CDTI1LOCR::LOC11
    }
    #[doc = "Checks if the value of the field is `LOC12`"]
    #[inline]
    pub fn is_loc12(&self) -> bool {
        *self == CDTI1LOCR::LOC12
    }
    #[doc = "Checks if the value of the field is `LOC13`"]
    #[inline]
    pub fn is_loc13(&self) -> bool {
        *self == CDTI1LOCR::LOC13
    }
    #[doc = "Checks if the value of the field is `LOC14`"]
    #[inline]
    pub fn is_loc14(&self) -> bool {
        *self == CDTI1LOCR::LOC14
    }
    #[doc = "Checks if the value of the field is `LOC15`"]
    #[inline]
    pub fn is_loc15(&self) -> bool {
        *self == CDTI1LOCR::LOC15
    }
    #[doc = "Checks if the value of the field is `LOC16`"]
    #[inline]
    pub fn is_loc16(&self) -> bool {
        *self == CDTI1LOCR::LOC16
    }
    #[doc = "Checks if the value of the field is `LOC17`"]
    #[inline]
    pub fn is_loc17(&self) -> bool {
        *self == CDTI1LOCR::LOC17
    }
    #[doc = "Checks if the value of the field is `LOC18`"]
    #[inline]
    pub fn is_loc18(&self) -> bool {
        *self == CDTI1LOCR::LOC18
    }
    #[doc = "Checks if the value of the field is `LOC19`"]
    #[inline]
    pub fn is_loc19(&self) -> bool {
        *self == CDTI1LOCR::LOC19
    }
    #[doc = "Checks if the value of the field is `LOC20`"]
    #[inline]
    pub fn is_loc20(&self) -> bool {
        *self == CDTI1LOCR::LOC20
    }
    #[doc = "Checks if the value of the field is `LOC21`"]
    #[inline]
    pub fn is_loc21(&self) -> bool {
        *self == CDTI1LOCR::LOC21
    }
    #[doc = "Checks if the value of the field is `LOC22`"]
    #[inline]
    pub fn is_loc22(&self) -> bool {
        *self == CDTI1LOCR::LOC22
    }
    #[doc = "Checks if the value of the field is `LOC23`"]
    #[inline]
    pub fn is_loc23(&self) -> bool {
        *self == CDTI1LOCR::LOC23
    }
    #[doc = "Checks if the value of the field is `LOC24`"]
    #[inline]
    pub fn is_loc24(&self) -> bool {
        *self == CDTI1LOCR::LOC24
    }
    #[doc = "Checks if the value of the field is `LOC25`"]
    #[inline]
    pub fn is_loc25(&self) -> bool {
        *self == CDTI1LOCR::LOC25
    }
    #[doc = "Checks if the value of the field is `LOC26`"]
    #[inline]
    pub fn is_loc26(&self) -> bool {
        *self == CDTI1LOCR::LOC26
    }
    #[doc = "Checks if the value of the field is `LOC27`"]
    #[inline]
    pub fn is_loc27(&self) -> bool {
        *self == CDTI1LOCR::LOC27
    }
    #[doc = "Checks if the value of the field is `LOC28`"]
    #[inline]
    pub fn is_loc28(&self) -> bool {
        *self == CDTI1LOCR::LOC28
    }
    #[doc = "Checks if the value of the field is `LOC29`"]
    #[inline]
    pub fn is_loc29(&self) -> bool {
        *self == CDTI1LOCR::LOC29
    }
    #[doc = "Checks if the value of the field is `LOC30`"]
    #[inline]
    pub fn is_loc30(&self) -> bool {
        *self == CDTI1LOCR::LOC30
    }
    #[doc = "Checks if the value of the field is `LOC31`"]
    #[inline]
    pub fn is_loc31(&self) -> bool {
        *self == CDTI1LOCR::LOC31
    }
}
#[doc = "Possible values of the field `CDTI2LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDTI2LOCR {
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
impl CDTI2LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CDTI2LOCR::LOC0 => 0,
            CDTI2LOCR::LOC1 => 1,
            CDTI2LOCR::LOC2 => 2,
            CDTI2LOCR::LOC3 => 3,
            CDTI2LOCR::LOC4 => 4,
            CDTI2LOCR::LOC5 => 5,
            CDTI2LOCR::LOC6 => 6,
            CDTI2LOCR::LOC7 => 7,
            CDTI2LOCR::LOC8 => 8,
            CDTI2LOCR::LOC9 => 9,
            CDTI2LOCR::LOC10 => 10,
            CDTI2LOCR::LOC11 => 11,
            CDTI2LOCR::LOC12 => 12,
            CDTI2LOCR::LOC13 => 13,
            CDTI2LOCR::LOC14 => 14,
            CDTI2LOCR::LOC15 => 15,
            CDTI2LOCR::LOC16 => 16,
            CDTI2LOCR::LOC17 => 17,
            CDTI2LOCR::LOC18 => 18,
            CDTI2LOCR::LOC19 => 19,
            CDTI2LOCR::LOC20 => 20,
            CDTI2LOCR::LOC21 => 21,
            CDTI2LOCR::LOC22 => 22,
            CDTI2LOCR::LOC23 => 23,
            CDTI2LOCR::LOC24 => 24,
            CDTI2LOCR::LOC25 => 25,
            CDTI2LOCR::LOC26 => 26,
            CDTI2LOCR::LOC27 => 27,
            CDTI2LOCR::LOC28 => 28,
            CDTI2LOCR::LOC29 => 29,
            CDTI2LOCR::LOC30 => 30,
            CDTI2LOCR::LOC31 => 31,
            CDTI2LOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CDTI2LOCR {
        match value {
            0 => CDTI2LOCR::LOC0,
            1 => CDTI2LOCR::LOC1,
            2 => CDTI2LOCR::LOC2,
            3 => CDTI2LOCR::LOC3,
            4 => CDTI2LOCR::LOC4,
            5 => CDTI2LOCR::LOC5,
            6 => CDTI2LOCR::LOC6,
            7 => CDTI2LOCR::LOC7,
            8 => CDTI2LOCR::LOC8,
            9 => CDTI2LOCR::LOC9,
            10 => CDTI2LOCR::LOC10,
            11 => CDTI2LOCR::LOC11,
            12 => CDTI2LOCR::LOC12,
            13 => CDTI2LOCR::LOC13,
            14 => CDTI2LOCR::LOC14,
            15 => CDTI2LOCR::LOC15,
            16 => CDTI2LOCR::LOC16,
            17 => CDTI2LOCR::LOC17,
            18 => CDTI2LOCR::LOC18,
            19 => CDTI2LOCR::LOC19,
            20 => CDTI2LOCR::LOC20,
            21 => CDTI2LOCR::LOC21,
            22 => CDTI2LOCR::LOC22,
            23 => CDTI2LOCR::LOC23,
            24 => CDTI2LOCR::LOC24,
            25 => CDTI2LOCR::LOC25,
            26 => CDTI2LOCR::LOC26,
            27 => CDTI2LOCR::LOC27,
            28 => CDTI2LOCR::LOC28,
            29 => CDTI2LOCR::LOC29,
            30 => CDTI2LOCR::LOC30,
            31 => CDTI2LOCR::LOC31,
            i => CDTI2LOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == CDTI2LOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == CDTI2LOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == CDTI2LOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == CDTI2LOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == CDTI2LOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == CDTI2LOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == CDTI2LOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == CDTI2LOCR::LOC7
    }
    #[doc = "Checks if the value of the field is `LOC8`"]
    #[inline]
    pub fn is_loc8(&self) -> bool {
        *self == CDTI2LOCR::LOC8
    }
    #[doc = "Checks if the value of the field is `LOC9`"]
    #[inline]
    pub fn is_loc9(&self) -> bool {
        *self == CDTI2LOCR::LOC9
    }
    #[doc = "Checks if the value of the field is `LOC10`"]
    #[inline]
    pub fn is_loc10(&self) -> bool {
        *self == CDTI2LOCR::LOC10
    }
    #[doc = "Checks if the value of the field is `LOC11`"]
    #[inline]
    pub fn is_loc11(&self) -> bool {
        *self == CDTI2LOCR::LOC11
    }
    #[doc = "Checks if the value of the field is `LOC12`"]
    #[inline]
    pub fn is_loc12(&self) -> bool {
        *self == CDTI2LOCR::LOC12
    }
    #[doc = "Checks if the value of the field is `LOC13`"]
    #[inline]
    pub fn is_loc13(&self) -> bool {
        *self == CDTI2LOCR::LOC13
    }
    #[doc = "Checks if the value of the field is `LOC14`"]
    #[inline]
    pub fn is_loc14(&self) -> bool {
        *self == CDTI2LOCR::LOC14
    }
    #[doc = "Checks if the value of the field is `LOC15`"]
    #[inline]
    pub fn is_loc15(&self) -> bool {
        *self == CDTI2LOCR::LOC15
    }
    #[doc = "Checks if the value of the field is `LOC16`"]
    #[inline]
    pub fn is_loc16(&self) -> bool {
        *self == CDTI2LOCR::LOC16
    }
    #[doc = "Checks if the value of the field is `LOC17`"]
    #[inline]
    pub fn is_loc17(&self) -> bool {
        *self == CDTI2LOCR::LOC17
    }
    #[doc = "Checks if the value of the field is `LOC18`"]
    #[inline]
    pub fn is_loc18(&self) -> bool {
        *self == CDTI2LOCR::LOC18
    }
    #[doc = "Checks if the value of the field is `LOC19`"]
    #[inline]
    pub fn is_loc19(&self) -> bool {
        *self == CDTI2LOCR::LOC19
    }
    #[doc = "Checks if the value of the field is `LOC20`"]
    #[inline]
    pub fn is_loc20(&self) -> bool {
        *self == CDTI2LOCR::LOC20
    }
    #[doc = "Checks if the value of the field is `LOC21`"]
    #[inline]
    pub fn is_loc21(&self) -> bool {
        *self == CDTI2LOCR::LOC21
    }
    #[doc = "Checks if the value of the field is `LOC22`"]
    #[inline]
    pub fn is_loc22(&self) -> bool {
        *self == CDTI2LOCR::LOC22
    }
    #[doc = "Checks if the value of the field is `LOC23`"]
    #[inline]
    pub fn is_loc23(&self) -> bool {
        *self == CDTI2LOCR::LOC23
    }
    #[doc = "Checks if the value of the field is `LOC24`"]
    #[inline]
    pub fn is_loc24(&self) -> bool {
        *self == CDTI2LOCR::LOC24
    }
    #[doc = "Checks if the value of the field is `LOC25`"]
    #[inline]
    pub fn is_loc25(&self) -> bool {
        *self == CDTI2LOCR::LOC25
    }
    #[doc = "Checks if the value of the field is `LOC26`"]
    #[inline]
    pub fn is_loc26(&self) -> bool {
        *self == CDTI2LOCR::LOC26
    }
    #[doc = "Checks if the value of the field is `LOC27`"]
    #[inline]
    pub fn is_loc27(&self) -> bool {
        *self == CDTI2LOCR::LOC27
    }
    #[doc = "Checks if the value of the field is `LOC28`"]
    #[inline]
    pub fn is_loc28(&self) -> bool {
        *self == CDTI2LOCR::LOC28
    }
    #[doc = "Checks if the value of the field is `LOC29`"]
    #[inline]
    pub fn is_loc29(&self) -> bool {
        *self == CDTI2LOCR::LOC29
    }
    #[doc = "Checks if the value of the field is `LOC30`"]
    #[inline]
    pub fn is_loc30(&self) -> bool {
        *self == CDTI2LOCR::LOC30
    }
    #[doc = "Checks if the value of the field is `LOC31`"]
    #[inline]
    pub fn is_loc31(&self) -> bool {
        *self == CDTI2LOCR::LOC31
    }
}
#[doc = "Values that can be written to the field `CDTI0LOC`"]
pub enum CDTI0LOCW {
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
impl CDTI0LOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CDTI0LOCW::LOC0 => 0,
            CDTI0LOCW::LOC1 => 1,
            CDTI0LOCW::LOC2 => 2,
            CDTI0LOCW::LOC3 => 3,
            CDTI0LOCW::LOC4 => 4,
            CDTI0LOCW::LOC5 => 5,
            CDTI0LOCW::LOC6 => 6,
            CDTI0LOCW::LOC7 => 7,
            CDTI0LOCW::LOC8 => 8,
            CDTI0LOCW::LOC9 => 9,
            CDTI0LOCW::LOC10 => 10,
            CDTI0LOCW::LOC11 => 11,
            CDTI0LOCW::LOC12 => 12,
            CDTI0LOCW::LOC13 => 13,
            CDTI0LOCW::LOC14 => 14,
            CDTI0LOCW::LOC15 => 15,
            CDTI0LOCW::LOC16 => 16,
            CDTI0LOCW::LOC17 => 17,
            CDTI0LOCW::LOC18 => 18,
            CDTI0LOCW::LOC19 => 19,
            CDTI0LOCW::LOC20 => 20,
            CDTI0LOCW::LOC21 => 21,
            CDTI0LOCW::LOC22 => 22,
            CDTI0LOCW::LOC23 => 23,
            CDTI0LOCW::LOC24 => 24,
            CDTI0LOCW::LOC25 => 25,
            CDTI0LOCW::LOC26 => 26,
            CDTI0LOCW::LOC27 => 27,
            CDTI0LOCW::LOC28 => 28,
            CDTI0LOCW::LOC29 => 29,
            CDTI0LOCW::LOC30 => 30,
            CDTI0LOCW::LOC31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDTI0LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _CDTI0LOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDTI0LOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC7)
    }
    #[doc = "Location 8"]
    #[inline]
    pub fn loc8(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC8)
    }
    #[doc = "Location 9"]
    #[inline]
    pub fn loc9(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC9)
    }
    #[doc = "Location 10"]
    #[inline]
    pub fn loc10(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC10)
    }
    #[doc = "Location 11"]
    #[inline]
    pub fn loc11(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC11)
    }
    #[doc = "Location 12"]
    #[inline]
    pub fn loc12(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC12)
    }
    #[doc = "Location 13"]
    #[inline]
    pub fn loc13(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC13)
    }
    #[doc = "Location 14"]
    #[inline]
    pub fn loc14(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC14)
    }
    #[doc = "Location 15"]
    #[inline]
    pub fn loc15(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC15)
    }
    #[doc = "Location 16"]
    #[inline]
    pub fn loc16(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC16)
    }
    #[doc = "Location 17"]
    #[inline]
    pub fn loc17(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC17)
    }
    #[doc = "Location 18"]
    #[inline]
    pub fn loc18(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC18)
    }
    #[doc = "Location 19"]
    #[inline]
    pub fn loc19(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC19)
    }
    #[doc = "Location 20"]
    #[inline]
    pub fn loc20(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC20)
    }
    #[doc = "Location 21"]
    #[inline]
    pub fn loc21(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC21)
    }
    #[doc = "Location 22"]
    #[inline]
    pub fn loc22(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC22)
    }
    #[doc = "Location 23"]
    #[inline]
    pub fn loc23(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC23)
    }
    #[doc = "Location 24"]
    #[inline]
    pub fn loc24(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC24)
    }
    #[doc = "Location 25"]
    #[inline]
    pub fn loc25(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC25)
    }
    #[doc = "Location 26"]
    #[inline]
    pub fn loc26(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC26)
    }
    #[doc = "Location 27"]
    #[inline]
    pub fn loc27(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC27)
    }
    #[doc = "Location 28"]
    #[inline]
    pub fn loc28(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC28)
    }
    #[doc = "Location 29"]
    #[inline]
    pub fn loc29(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC29)
    }
    #[doc = "Location 30"]
    #[inline]
    pub fn loc30(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC30)
    }
    #[doc = "Location 31"]
    #[inline]
    pub fn loc31(self) -> &'a mut W {
        self.variant(CDTI0LOCW::LOC31)
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
#[doc = "Values that can be written to the field `CDTI1LOC`"]
pub enum CDTI1LOCW {
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
impl CDTI1LOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CDTI1LOCW::LOC0 => 0,
            CDTI1LOCW::LOC1 => 1,
            CDTI1LOCW::LOC2 => 2,
            CDTI1LOCW::LOC3 => 3,
            CDTI1LOCW::LOC4 => 4,
            CDTI1LOCW::LOC5 => 5,
            CDTI1LOCW::LOC6 => 6,
            CDTI1LOCW::LOC7 => 7,
            CDTI1LOCW::LOC8 => 8,
            CDTI1LOCW::LOC9 => 9,
            CDTI1LOCW::LOC10 => 10,
            CDTI1LOCW::LOC11 => 11,
            CDTI1LOCW::LOC12 => 12,
            CDTI1LOCW::LOC13 => 13,
            CDTI1LOCW::LOC14 => 14,
            CDTI1LOCW::LOC15 => 15,
            CDTI1LOCW::LOC16 => 16,
            CDTI1LOCW::LOC17 => 17,
            CDTI1LOCW::LOC18 => 18,
            CDTI1LOCW::LOC19 => 19,
            CDTI1LOCW::LOC20 => 20,
            CDTI1LOCW::LOC21 => 21,
            CDTI1LOCW::LOC22 => 22,
            CDTI1LOCW::LOC23 => 23,
            CDTI1LOCW::LOC24 => 24,
            CDTI1LOCW::LOC25 => 25,
            CDTI1LOCW::LOC26 => 26,
            CDTI1LOCW::LOC27 => 27,
            CDTI1LOCW::LOC28 => 28,
            CDTI1LOCW::LOC29 => 29,
            CDTI1LOCW::LOC30 => 30,
            CDTI1LOCW::LOC31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDTI1LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _CDTI1LOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDTI1LOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC7)
    }
    #[doc = "Location 8"]
    #[inline]
    pub fn loc8(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC8)
    }
    #[doc = "Location 9"]
    #[inline]
    pub fn loc9(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC9)
    }
    #[doc = "Location 10"]
    #[inline]
    pub fn loc10(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC10)
    }
    #[doc = "Location 11"]
    #[inline]
    pub fn loc11(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC11)
    }
    #[doc = "Location 12"]
    #[inline]
    pub fn loc12(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC12)
    }
    #[doc = "Location 13"]
    #[inline]
    pub fn loc13(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC13)
    }
    #[doc = "Location 14"]
    #[inline]
    pub fn loc14(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC14)
    }
    #[doc = "Location 15"]
    #[inline]
    pub fn loc15(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC15)
    }
    #[doc = "Location 16"]
    #[inline]
    pub fn loc16(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC16)
    }
    #[doc = "Location 17"]
    #[inline]
    pub fn loc17(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC17)
    }
    #[doc = "Location 18"]
    #[inline]
    pub fn loc18(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC18)
    }
    #[doc = "Location 19"]
    #[inline]
    pub fn loc19(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC19)
    }
    #[doc = "Location 20"]
    #[inline]
    pub fn loc20(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC20)
    }
    #[doc = "Location 21"]
    #[inline]
    pub fn loc21(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC21)
    }
    #[doc = "Location 22"]
    #[inline]
    pub fn loc22(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC22)
    }
    #[doc = "Location 23"]
    #[inline]
    pub fn loc23(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC23)
    }
    #[doc = "Location 24"]
    #[inline]
    pub fn loc24(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC24)
    }
    #[doc = "Location 25"]
    #[inline]
    pub fn loc25(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC25)
    }
    #[doc = "Location 26"]
    #[inline]
    pub fn loc26(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC26)
    }
    #[doc = "Location 27"]
    #[inline]
    pub fn loc27(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC27)
    }
    #[doc = "Location 28"]
    #[inline]
    pub fn loc28(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC28)
    }
    #[doc = "Location 29"]
    #[inline]
    pub fn loc29(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC29)
    }
    #[doc = "Location 30"]
    #[inline]
    pub fn loc30(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC30)
    }
    #[doc = "Location 31"]
    #[inline]
    pub fn loc31(self) -> &'a mut W {
        self.variant(CDTI1LOCW::LOC31)
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
#[doc = "Values that can be written to the field `CDTI2LOC`"]
pub enum CDTI2LOCW {
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
impl CDTI2LOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CDTI2LOCW::LOC0 => 0,
            CDTI2LOCW::LOC1 => 1,
            CDTI2LOCW::LOC2 => 2,
            CDTI2LOCW::LOC3 => 3,
            CDTI2LOCW::LOC4 => 4,
            CDTI2LOCW::LOC5 => 5,
            CDTI2LOCW::LOC6 => 6,
            CDTI2LOCW::LOC7 => 7,
            CDTI2LOCW::LOC8 => 8,
            CDTI2LOCW::LOC9 => 9,
            CDTI2LOCW::LOC10 => 10,
            CDTI2LOCW::LOC11 => 11,
            CDTI2LOCW::LOC12 => 12,
            CDTI2LOCW::LOC13 => 13,
            CDTI2LOCW::LOC14 => 14,
            CDTI2LOCW::LOC15 => 15,
            CDTI2LOCW::LOC16 => 16,
            CDTI2LOCW::LOC17 => 17,
            CDTI2LOCW::LOC18 => 18,
            CDTI2LOCW::LOC19 => 19,
            CDTI2LOCW::LOC20 => 20,
            CDTI2LOCW::LOC21 => 21,
            CDTI2LOCW::LOC22 => 22,
            CDTI2LOCW::LOC23 => 23,
            CDTI2LOCW::LOC24 => 24,
            CDTI2LOCW::LOC25 => 25,
            CDTI2LOCW::LOC26 => 26,
            CDTI2LOCW::LOC27 => 27,
            CDTI2LOCW::LOC28 => 28,
            CDTI2LOCW::LOC29 => 29,
            CDTI2LOCW::LOC30 => 30,
            CDTI2LOCW::LOC31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDTI2LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _CDTI2LOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDTI2LOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC7)
    }
    #[doc = "Location 8"]
    #[inline]
    pub fn loc8(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC8)
    }
    #[doc = "Location 9"]
    #[inline]
    pub fn loc9(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC9)
    }
    #[doc = "Location 10"]
    #[inline]
    pub fn loc10(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC10)
    }
    #[doc = "Location 11"]
    #[inline]
    pub fn loc11(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC11)
    }
    #[doc = "Location 12"]
    #[inline]
    pub fn loc12(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC12)
    }
    #[doc = "Location 13"]
    #[inline]
    pub fn loc13(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC13)
    }
    #[doc = "Location 14"]
    #[inline]
    pub fn loc14(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC14)
    }
    #[doc = "Location 15"]
    #[inline]
    pub fn loc15(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC15)
    }
    #[doc = "Location 16"]
    #[inline]
    pub fn loc16(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC16)
    }
    #[doc = "Location 17"]
    #[inline]
    pub fn loc17(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC17)
    }
    #[doc = "Location 18"]
    #[inline]
    pub fn loc18(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC18)
    }
    #[doc = "Location 19"]
    #[inline]
    pub fn loc19(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC19)
    }
    #[doc = "Location 20"]
    #[inline]
    pub fn loc20(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC20)
    }
    #[doc = "Location 21"]
    #[inline]
    pub fn loc21(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC21)
    }
    #[doc = "Location 22"]
    #[inline]
    pub fn loc22(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC22)
    }
    #[doc = "Location 23"]
    #[inline]
    pub fn loc23(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC23)
    }
    #[doc = "Location 24"]
    #[inline]
    pub fn loc24(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC24)
    }
    #[doc = "Location 25"]
    #[inline]
    pub fn loc25(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC25)
    }
    #[doc = "Location 26"]
    #[inline]
    pub fn loc26(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC26)
    }
    #[doc = "Location 27"]
    #[inline]
    pub fn loc27(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC27)
    }
    #[doc = "Location 28"]
    #[inline]
    pub fn loc28(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC28)
    }
    #[doc = "Location 29"]
    #[inline]
    pub fn loc29(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC29)
    }
    #[doc = "Location 30"]
    #[inline]
    pub fn loc30(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC30)
    }
    #[doc = "Location 31"]
    #[inline]
    pub fn loc31(self) -> &'a mut W {
        self.variant(CDTI2LOCW::LOC31)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline]
    pub fn cdti0loc(&self) -> CDTI0LOCR {
        CDTI0LOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline]
    pub fn cdti1loc(&self) -> CDTI1LOCR {
        CDTI1LOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline]
    pub fn cdti2loc(&self) -> CDTI2LOCR {
        CDTI2LOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
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
    pub fn cdti0loc(&mut self) -> _CDTI0LOCW {
        _CDTI0LOCW { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline]
    pub fn cdti1loc(&mut self) -> _CDTI1LOCW {
        _CDTI1LOCW { w: self }
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline]
    pub fn cdti2loc(&mut self) -> _CDTI2LOCW {
        _CDTI2LOCW { w: self }
    }
}
