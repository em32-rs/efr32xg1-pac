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
#[doc = "Possible values of the field `S0INLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0INLOCR {
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
impl S0INLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            S0INLOCR::LOC0 => 0,
            S0INLOCR::LOC1 => 1,
            S0INLOCR::LOC2 => 2,
            S0INLOCR::LOC3 => 3,
            S0INLOCR::LOC4 => 4,
            S0INLOCR::LOC5 => 5,
            S0INLOCR::LOC6 => 6,
            S0INLOCR::LOC7 => 7,
            S0INLOCR::LOC8 => 8,
            S0INLOCR::LOC9 => 9,
            S0INLOCR::LOC10 => 10,
            S0INLOCR::LOC11 => 11,
            S0INLOCR::LOC12 => 12,
            S0INLOCR::LOC13 => 13,
            S0INLOCR::LOC14 => 14,
            S0INLOCR::LOC15 => 15,
            S0INLOCR::LOC16 => 16,
            S0INLOCR::LOC17 => 17,
            S0INLOCR::LOC18 => 18,
            S0INLOCR::LOC19 => 19,
            S0INLOCR::LOC20 => 20,
            S0INLOCR::LOC21 => 21,
            S0INLOCR::LOC22 => 22,
            S0INLOCR::LOC23 => 23,
            S0INLOCR::LOC24 => 24,
            S0INLOCR::LOC25 => 25,
            S0INLOCR::LOC26 => 26,
            S0INLOCR::LOC27 => 27,
            S0INLOCR::LOC28 => 28,
            S0INLOCR::LOC29 => 29,
            S0INLOCR::LOC30 => 30,
            S0INLOCR::LOC31 => 31,
            S0INLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> S0INLOCR {
        match value {
            0 => S0INLOCR::LOC0,
            1 => S0INLOCR::LOC1,
            2 => S0INLOCR::LOC2,
            3 => S0INLOCR::LOC3,
            4 => S0INLOCR::LOC4,
            5 => S0INLOCR::LOC5,
            6 => S0INLOCR::LOC6,
            7 => S0INLOCR::LOC7,
            8 => S0INLOCR::LOC8,
            9 => S0INLOCR::LOC9,
            10 => S0INLOCR::LOC10,
            11 => S0INLOCR::LOC11,
            12 => S0INLOCR::LOC12,
            13 => S0INLOCR::LOC13,
            14 => S0INLOCR::LOC14,
            15 => S0INLOCR::LOC15,
            16 => S0INLOCR::LOC16,
            17 => S0INLOCR::LOC17,
            18 => S0INLOCR::LOC18,
            19 => S0INLOCR::LOC19,
            20 => S0INLOCR::LOC20,
            21 => S0INLOCR::LOC21,
            22 => S0INLOCR::LOC22,
            23 => S0INLOCR::LOC23,
            24 => S0INLOCR::LOC24,
            25 => S0INLOCR::LOC25,
            26 => S0INLOCR::LOC26,
            27 => S0INLOCR::LOC27,
            28 => S0INLOCR::LOC28,
            29 => S0INLOCR::LOC29,
            30 => S0INLOCR::LOC30,
            31 => S0INLOCR::LOC31,
            i => S0INLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == S0INLOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == S0INLOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == S0INLOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == S0INLOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == S0INLOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == S0INLOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == S0INLOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == S0INLOCR::LOC7
    }
    #[doc = "Checks if the value of the field is `LOC8`"]
    #[inline]
    pub fn is_loc8(&self) -> bool {
        *self == S0INLOCR::LOC8
    }
    #[doc = "Checks if the value of the field is `LOC9`"]
    #[inline]
    pub fn is_loc9(&self) -> bool {
        *self == S0INLOCR::LOC9
    }
    #[doc = "Checks if the value of the field is `LOC10`"]
    #[inline]
    pub fn is_loc10(&self) -> bool {
        *self == S0INLOCR::LOC10
    }
    #[doc = "Checks if the value of the field is `LOC11`"]
    #[inline]
    pub fn is_loc11(&self) -> bool {
        *self == S0INLOCR::LOC11
    }
    #[doc = "Checks if the value of the field is `LOC12`"]
    #[inline]
    pub fn is_loc12(&self) -> bool {
        *self == S0INLOCR::LOC12
    }
    #[doc = "Checks if the value of the field is `LOC13`"]
    #[inline]
    pub fn is_loc13(&self) -> bool {
        *self == S0INLOCR::LOC13
    }
    #[doc = "Checks if the value of the field is `LOC14`"]
    #[inline]
    pub fn is_loc14(&self) -> bool {
        *self == S0INLOCR::LOC14
    }
    #[doc = "Checks if the value of the field is `LOC15`"]
    #[inline]
    pub fn is_loc15(&self) -> bool {
        *self == S0INLOCR::LOC15
    }
    #[doc = "Checks if the value of the field is `LOC16`"]
    #[inline]
    pub fn is_loc16(&self) -> bool {
        *self == S0INLOCR::LOC16
    }
    #[doc = "Checks if the value of the field is `LOC17`"]
    #[inline]
    pub fn is_loc17(&self) -> bool {
        *self == S0INLOCR::LOC17
    }
    #[doc = "Checks if the value of the field is `LOC18`"]
    #[inline]
    pub fn is_loc18(&self) -> bool {
        *self == S0INLOCR::LOC18
    }
    #[doc = "Checks if the value of the field is `LOC19`"]
    #[inline]
    pub fn is_loc19(&self) -> bool {
        *self == S0INLOCR::LOC19
    }
    #[doc = "Checks if the value of the field is `LOC20`"]
    #[inline]
    pub fn is_loc20(&self) -> bool {
        *self == S0INLOCR::LOC20
    }
    #[doc = "Checks if the value of the field is `LOC21`"]
    #[inline]
    pub fn is_loc21(&self) -> bool {
        *self == S0INLOCR::LOC21
    }
    #[doc = "Checks if the value of the field is `LOC22`"]
    #[inline]
    pub fn is_loc22(&self) -> bool {
        *self == S0INLOCR::LOC22
    }
    #[doc = "Checks if the value of the field is `LOC23`"]
    #[inline]
    pub fn is_loc23(&self) -> bool {
        *self == S0INLOCR::LOC23
    }
    #[doc = "Checks if the value of the field is `LOC24`"]
    #[inline]
    pub fn is_loc24(&self) -> bool {
        *self == S0INLOCR::LOC24
    }
    #[doc = "Checks if the value of the field is `LOC25`"]
    #[inline]
    pub fn is_loc25(&self) -> bool {
        *self == S0INLOCR::LOC25
    }
    #[doc = "Checks if the value of the field is `LOC26`"]
    #[inline]
    pub fn is_loc26(&self) -> bool {
        *self == S0INLOCR::LOC26
    }
    #[doc = "Checks if the value of the field is `LOC27`"]
    #[inline]
    pub fn is_loc27(&self) -> bool {
        *self == S0INLOCR::LOC27
    }
    #[doc = "Checks if the value of the field is `LOC28`"]
    #[inline]
    pub fn is_loc28(&self) -> bool {
        *self == S0INLOCR::LOC28
    }
    #[doc = "Checks if the value of the field is `LOC29`"]
    #[inline]
    pub fn is_loc29(&self) -> bool {
        *self == S0INLOCR::LOC29
    }
    #[doc = "Checks if the value of the field is `LOC30`"]
    #[inline]
    pub fn is_loc30(&self) -> bool {
        *self == S0INLOCR::LOC30
    }
    #[doc = "Checks if the value of the field is `LOC31`"]
    #[inline]
    pub fn is_loc31(&self) -> bool {
        *self == S0INLOCR::LOC31
    }
}
#[doc = "Possible values of the field `S1INLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1INLOCR {
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
impl S1INLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            S1INLOCR::LOC0 => 0,
            S1INLOCR::LOC1 => 1,
            S1INLOCR::LOC2 => 2,
            S1INLOCR::LOC3 => 3,
            S1INLOCR::LOC4 => 4,
            S1INLOCR::LOC5 => 5,
            S1INLOCR::LOC6 => 6,
            S1INLOCR::LOC7 => 7,
            S1INLOCR::LOC8 => 8,
            S1INLOCR::LOC9 => 9,
            S1INLOCR::LOC10 => 10,
            S1INLOCR::LOC11 => 11,
            S1INLOCR::LOC12 => 12,
            S1INLOCR::LOC13 => 13,
            S1INLOCR::LOC14 => 14,
            S1INLOCR::LOC15 => 15,
            S1INLOCR::LOC16 => 16,
            S1INLOCR::LOC17 => 17,
            S1INLOCR::LOC18 => 18,
            S1INLOCR::LOC19 => 19,
            S1INLOCR::LOC20 => 20,
            S1INLOCR::LOC21 => 21,
            S1INLOCR::LOC22 => 22,
            S1INLOCR::LOC23 => 23,
            S1INLOCR::LOC24 => 24,
            S1INLOCR::LOC25 => 25,
            S1INLOCR::LOC26 => 26,
            S1INLOCR::LOC27 => 27,
            S1INLOCR::LOC28 => 28,
            S1INLOCR::LOC29 => 29,
            S1INLOCR::LOC30 => 30,
            S1INLOCR::LOC31 => 31,
            S1INLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> S1INLOCR {
        match value {
            0 => S1INLOCR::LOC0,
            1 => S1INLOCR::LOC1,
            2 => S1INLOCR::LOC2,
            3 => S1INLOCR::LOC3,
            4 => S1INLOCR::LOC4,
            5 => S1INLOCR::LOC5,
            6 => S1INLOCR::LOC6,
            7 => S1INLOCR::LOC7,
            8 => S1INLOCR::LOC8,
            9 => S1INLOCR::LOC9,
            10 => S1INLOCR::LOC10,
            11 => S1INLOCR::LOC11,
            12 => S1INLOCR::LOC12,
            13 => S1INLOCR::LOC13,
            14 => S1INLOCR::LOC14,
            15 => S1INLOCR::LOC15,
            16 => S1INLOCR::LOC16,
            17 => S1INLOCR::LOC17,
            18 => S1INLOCR::LOC18,
            19 => S1INLOCR::LOC19,
            20 => S1INLOCR::LOC20,
            21 => S1INLOCR::LOC21,
            22 => S1INLOCR::LOC22,
            23 => S1INLOCR::LOC23,
            24 => S1INLOCR::LOC24,
            25 => S1INLOCR::LOC25,
            26 => S1INLOCR::LOC26,
            27 => S1INLOCR::LOC27,
            28 => S1INLOCR::LOC28,
            29 => S1INLOCR::LOC29,
            30 => S1INLOCR::LOC30,
            31 => S1INLOCR::LOC31,
            i => S1INLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == S1INLOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == S1INLOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == S1INLOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == S1INLOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == S1INLOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == S1INLOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == S1INLOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == S1INLOCR::LOC7
    }
    #[doc = "Checks if the value of the field is `LOC8`"]
    #[inline]
    pub fn is_loc8(&self) -> bool {
        *self == S1INLOCR::LOC8
    }
    #[doc = "Checks if the value of the field is `LOC9`"]
    #[inline]
    pub fn is_loc9(&self) -> bool {
        *self == S1INLOCR::LOC9
    }
    #[doc = "Checks if the value of the field is `LOC10`"]
    #[inline]
    pub fn is_loc10(&self) -> bool {
        *self == S1INLOCR::LOC10
    }
    #[doc = "Checks if the value of the field is `LOC11`"]
    #[inline]
    pub fn is_loc11(&self) -> bool {
        *self == S1INLOCR::LOC11
    }
    #[doc = "Checks if the value of the field is `LOC12`"]
    #[inline]
    pub fn is_loc12(&self) -> bool {
        *self == S1INLOCR::LOC12
    }
    #[doc = "Checks if the value of the field is `LOC13`"]
    #[inline]
    pub fn is_loc13(&self) -> bool {
        *self == S1INLOCR::LOC13
    }
    #[doc = "Checks if the value of the field is `LOC14`"]
    #[inline]
    pub fn is_loc14(&self) -> bool {
        *self == S1INLOCR::LOC14
    }
    #[doc = "Checks if the value of the field is `LOC15`"]
    #[inline]
    pub fn is_loc15(&self) -> bool {
        *self == S1INLOCR::LOC15
    }
    #[doc = "Checks if the value of the field is `LOC16`"]
    #[inline]
    pub fn is_loc16(&self) -> bool {
        *self == S1INLOCR::LOC16
    }
    #[doc = "Checks if the value of the field is `LOC17`"]
    #[inline]
    pub fn is_loc17(&self) -> bool {
        *self == S1INLOCR::LOC17
    }
    #[doc = "Checks if the value of the field is `LOC18`"]
    #[inline]
    pub fn is_loc18(&self) -> bool {
        *self == S1INLOCR::LOC18
    }
    #[doc = "Checks if the value of the field is `LOC19`"]
    #[inline]
    pub fn is_loc19(&self) -> bool {
        *self == S1INLOCR::LOC19
    }
    #[doc = "Checks if the value of the field is `LOC20`"]
    #[inline]
    pub fn is_loc20(&self) -> bool {
        *self == S1INLOCR::LOC20
    }
    #[doc = "Checks if the value of the field is `LOC21`"]
    #[inline]
    pub fn is_loc21(&self) -> bool {
        *self == S1INLOCR::LOC21
    }
    #[doc = "Checks if the value of the field is `LOC22`"]
    #[inline]
    pub fn is_loc22(&self) -> bool {
        *self == S1INLOCR::LOC22
    }
    #[doc = "Checks if the value of the field is `LOC23`"]
    #[inline]
    pub fn is_loc23(&self) -> bool {
        *self == S1INLOCR::LOC23
    }
    #[doc = "Checks if the value of the field is `LOC24`"]
    #[inline]
    pub fn is_loc24(&self) -> bool {
        *self == S1INLOCR::LOC24
    }
    #[doc = "Checks if the value of the field is `LOC25`"]
    #[inline]
    pub fn is_loc25(&self) -> bool {
        *self == S1INLOCR::LOC25
    }
    #[doc = "Checks if the value of the field is `LOC26`"]
    #[inline]
    pub fn is_loc26(&self) -> bool {
        *self == S1INLOCR::LOC26
    }
    #[doc = "Checks if the value of the field is `LOC27`"]
    #[inline]
    pub fn is_loc27(&self) -> bool {
        *self == S1INLOCR::LOC27
    }
    #[doc = "Checks if the value of the field is `LOC28`"]
    #[inline]
    pub fn is_loc28(&self) -> bool {
        *self == S1INLOCR::LOC28
    }
    #[doc = "Checks if the value of the field is `LOC29`"]
    #[inline]
    pub fn is_loc29(&self) -> bool {
        *self == S1INLOCR::LOC29
    }
    #[doc = "Checks if the value of the field is `LOC30`"]
    #[inline]
    pub fn is_loc30(&self) -> bool {
        *self == S1INLOCR::LOC30
    }
    #[doc = "Checks if the value of the field is `LOC31`"]
    #[inline]
    pub fn is_loc31(&self) -> bool {
        *self == S1INLOCR::LOC31
    }
}
#[doc = "Values that can be written to the field `S0INLOC`"]
pub enum S0INLOCW {
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
impl S0INLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            S0INLOCW::LOC0 => 0,
            S0INLOCW::LOC1 => 1,
            S0INLOCW::LOC2 => 2,
            S0INLOCW::LOC3 => 3,
            S0INLOCW::LOC4 => 4,
            S0INLOCW::LOC5 => 5,
            S0INLOCW::LOC6 => 6,
            S0INLOCW::LOC7 => 7,
            S0INLOCW::LOC8 => 8,
            S0INLOCW::LOC9 => 9,
            S0INLOCW::LOC10 => 10,
            S0INLOCW::LOC11 => 11,
            S0INLOCW::LOC12 => 12,
            S0INLOCW::LOC13 => 13,
            S0INLOCW::LOC14 => 14,
            S0INLOCW::LOC15 => 15,
            S0INLOCW::LOC16 => 16,
            S0INLOCW::LOC17 => 17,
            S0INLOCW::LOC18 => 18,
            S0INLOCW::LOC19 => 19,
            S0INLOCW::LOC20 => 20,
            S0INLOCW::LOC21 => 21,
            S0INLOCW::LOC22 => 22,
            S0INLOCW::LOC23 => 23,
            S0INLOCW::LOC24 => 24,
            S0INLOCW::LOC25 => 25,
            S0INLOCW::LOC26 => 26,
            S0INLOCW::LOC27 => 27,
            S0INLOCW::LOC28 => 28,
            S0INLOCW::LOC29 => 29,
            S0INLOCW::LOC30 => 30,
            S0INLOCW::LOC31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S0INLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _S0INLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S0INLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC7)
    }
    #[doc = "Location 8"]
    #[inline]
    pub fn loc8(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC8)
    }
    #[doc = "Location 9"]
    #[inline]
    pub fn loc9(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC9)
    }
    #[doc = "Location 10"]
    #[inline]
    pub fn loc10(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC10)
    }
    #[doc = "Location 11"]
    #[inline]
    pub fn loc11(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC11)
    }
    #[doc = "Location 12"]
    #[inline]
    pub fn loc12(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC12)
    }
    #[doc = "Location 13"]
    #[inline]
    pub fn loc13(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC13)
    }
    #[doc = "Location 14"]
    #[inline]
    pub fn loc14(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC14)
    }
    #[doc = "Location 15"]
    #[inline]
    pub fn loc15(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC15)
    }
    #[doc = "Location 16"]
    #[inline]
    pub fn loc16(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC16)
    }
    #[doc = "Location 17"]
    #[inline]
    pub fn loc17(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC17)
    }
    #[doc = "Location 18"]
    #[inline]
    pub fn loc18(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC18)
    }
    #[doc = "Location 19"]
    #[inline]
    pub fn loc19(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC19)
    }
    #[doc = "Location 20"]
    #[inline]
    pub fn loc20(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC20)
    }
    #[doc = "Location 21"]
    #[inline]
    pub fn loc21(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC21)
    }
    #[doc = "Location 22"]
    #[inline]
    pub fn loc22(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC22)
    }
    #[doc = "Location 23"]
    #[inline]
    pub fn loc23(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC23)
    }
    #[doc = "Location 24"]
    #[inline]
    pub fn loc24(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC24)
    }
    #[doc = "Location 25"]
    #[inline]
    pub fn loc25(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC25)
    }
    #[doc = "Location 26"]
    #[inline]
    pub fn loc26(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC26)
    }
    #[doc = "Location 27"]
    #[inline]
    pub fn loc27(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC27)
    }
    #[doc = "Location 28"]
    #[inline]
    pub fn loc28(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC28)
    }
    #[doc = "Location 29"]
    #[inline]
    pub fn loc29(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC29)
    }
    #[doc = "Location 30"]
    #[inline]
    pub fn loc30(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC30)
    }
    #[doc = "Location 31"]
    #[inline]
    pub fn loc31(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC31)
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
#[doc = "Values that can be written to the field `S1INLOC`"]
pub enum S1INLOCW {
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
impl S1INLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            S1INLOCW::LOC0 => 0,
            S1INLOCW::LOC1 => 1,
            S1INLOCW::LOC2 => 2,
            S1INLOCW::LOC3 => 3,
            S1INLOCW::LOC4 => 4,
            S1INLOCW::LOC5 => 5,
            S1INLOCW::LOC6 => 6,
            S1INLOCW::LOC7 => 7,
            S1INLOCW::LOC8 => 8,
            S1INLOCW::LOC9 => 9,
            S1INLOCW::LOC10 => 10,
            S1INLOCW::LOC11 => 11,
            S1INLOCW::LOC12 => 12,
            S1INLOCW::LOC13 => 13,
            S1INLOCW::LOC14 => 14,
            S1INLOCW::LOC15 => 15,
            S1INLOCW::LOC16 => 16,
            S1INLOCW::LOC17 => 17,
            S1INLOCW::LOC18 => 18,
            S1INLOCW::LOC19 => 19,
            S1INLOCW::LOC20 => 20,
            S1INLOCW::LOC21 => 21,
            S1INLOCW::LOC22 => 22,
            S1INLOCW::LOC23 => 23,
            S1INLOCW::LOC24 => 24,
            S1INLOCW::LOC25 => 25,
            S1INLOCW::LOC26 => 26,
            S1INLOCW::LOC27 => 27,
            S1INLOCW::LOC28 => 28,
            S1INLOCW::LOC29 => 29,
            S1INLOCW::LOC30 => 30,
            S1INLOCW::LOC31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S1INLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _S1INLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S1INLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC7)
    }
    #[doc = "Location 8"]
    #[inline]
    pub fn loc8(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC8)
    }
    #[doc = "Location 9"]
    #[inline]
    pub fn loc9(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC9)
    }
    #[doc = "Location 10"]
    #[inline]
    pub fn loc10(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC10)
    }
    #[doc = "Location 11"]
    #[inline]
    pub fn loc11(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC11)
    }
    #[doc = "Location 12"]
    #[inline]
    pub fn loc12(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC12)
    }
    #[doc = "Location 13"]
    #[inline]
    pub fn loc13(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC13)
    }
    #[doc = "Location 14"]
    #[inline]
    pub fn loc14(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC14)
    }
    #[doc = "Location 15"]
    #[inline]
    pub fn loc15(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC15)
    }
    #[doc = "Location 16"]
    #[inline]
    pub fn loc16(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC16)
    }
    #[doc = "Location 17"]
    #[inline]
    pub fn loc17(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC17)
    }
    #[doc = "Location 18"]
    #[inline]
    pub fn loc18(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC18)
    }
    #[doc = "Location 19"]
    #[inline]
    pub fn loc19(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC19)
    }
    #[doc = "Location 20"]
    #[inline]
    pub fn loc20(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC20)
    }
    #[doc = "Location 21"]
    #[inline]
    pub fn loc21(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC21)
    }
    #[doc = "Location 22"]
    #[inline]
    pub fn loc22(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC22)
    }
    #[doc = "Location 23"]
    #[inline]
    pub fn loc23(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC23)
    }
    #[doc = "Location 24"]
    #[inline]
    pub fn loc24(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC24)
    }
    #[doc = "Location 25"]
    #[inline]
    pub fn loc25(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC25)
    }
    #[doc = "Location 26"]
    #[inline]
    pub fn loc26(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC26)
    }
    #[doc = "Location 27"]
    #[inline]
    pub fn loc27(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC27)
    }
    #[doc = "Location 28"]
    #[inline]
    pub fn loc28(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC28)
    }
    #[doc = "Location 29"]
    #[inline]
    pub fn loc29(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC29)
    }
    #[doc = "Location 30"]
    #[inline]
    pub fn loc30(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC30)
    }
    #[doc = "Location 31"]
    #[inline]
    pub fn loc31(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC31)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline]
    pub fn s0inloc(&self) -> S0INLOCR {
        S0INLOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline]
    pub fn s1inloc(&self) -> S1INLOCR {
        S1INLOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
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
    pub fn s0inloc(&mut self) -> _S0INLOCW {
        _S0INLOCW { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline]
    pub fn s1inloc(&mut self) -> _S1INLOCW {
        _S1INLOCW { w: self }
    }
}
