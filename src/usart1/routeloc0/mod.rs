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
#[doc = "Possible values of the field `RXLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXLOCR {
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
impl RXLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXLOCR::LOC0 => 0,
            RXLOCR::LOC1 => 1,
            RXLOCR::LOC2 => 2,
            RXLOCR::LOC3 => 3,
            RXLOCR::LOC4 => 4,
            RXLOCR::LOC5 => 5,
            RXLOCR::LOC6 => 6,
            RXLOCR::LOC7 => 7,
            RXLOCR::LOC8 => 8,
            RXLOCR::LOC9 => 9,
            RXLOCR::LOC10 => 10,
            RXLOCR::LOC11 => 11,
            RXLOCR::LOC12 => 12,
            RXLOCR::LOC13 => 13,
            RXLOCR::LOC14 => 14,
            RXLOCR::LOC15 => 15,
            RXLOCR::LOC16 => 16,
            RXLOCR::LOC17 => 17,
            RXLOCR::LOC18 => 18,
            RXLOCR::LOC19 => 19,
            RXLOCR::LOC20 => 20,
            RXLOCR::LOC21 => 21,
            RXLOCR::LOC22 => 22,
            RXLOCR::LOC23 => 23,
            RXLOCR::LOC24 => 24,
            RXLOCR::LOC25 => 25,
            RXLOCR::LOC26 => 26,
            RXLOCR::LOC27 => 27,
            RXLOCR::LOC28 => 28,
            RXLOCR::LOC29 => 29,
            RXLOCR::LOC30 => 30,
            RXLOCR::LOC31 => 31,
            RXLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXLOCR {
        match value {
            0 => RXLOCR::LOC0,
            1 => RXLOCR::LOC1,
            2 => RXLOCR::LOC2,
            3 => RXLOCR::LOC3,
            4 => RXLOCR::LOC4,
            5 => RXLOCR::LOC5,
            6 => RXLOCR::LOC6,
            7 => RXLOCR::LOC7,
            8 => RXLOCR::LOC8,
            9 => RXLOCR::LOC9,
            10 => RXLOCR::LOC10,
            11 => RXLOCR::LOC11,
            12 => RXLOCR::LOC12,
            13 => RXLOCR::LOC13,
            14 => RXLOCR::LOC14,
            15 => RXLOCR::LOC15,
            16 => RXLOCR::LOC16,
            17 => RXLOCR::LOC17,
            18 => RXLOCR::LOC18,
            19 => RXLOCR::LOC19,
            20 => RXLOCR::LOC20,
            21 => RXLOCR::LOC21,
            22 => RXLOCR::LOC22,
            23 => RXLOCR::LOC23,
            24 => RXLOCR::LOC24,
            25 => RXLOCR::LOC25,
            26 => RXLOCR::LOC26,
            27 => RXLOCR::LOC27,
            28 => RXLOCR::LOC28,
            29 => RXLOCR::LOC29,
            30 => RXLOCR::LOC30,
            31 => RXLOCR::LOC31,
            i => RXLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == RXLOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == RXLOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == RXLOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == RXLOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == RXLOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == RXLOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == RXLOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == RXLOCR::LOC7
    }
    #[doc = "Checks if the value of the field is `LOC8`"]
    #[inline]
    pub fn is_loc8(&self) -> bool {
        *self == RXLOCR::LOC8
    }
    #[doc = "Checks if the value of the field is `LOC9`"]
    #[inline]
    pub fn is_loc9(&self) -> bool {
        *self == RXLOCR::LOC9
    }
    #[doc = "Checks if the value of the field is `LOC10`"]
    #[inline]
    pub fn is_loc10(&self) -> bool {
        *self == RXLOCR::LOC10
    }
    #[doc = "Checks if the value of the field is `LOC11`"]
    #[inline]
    pub fn is_loc11(&self) -> bool {
        *self == RXLOCR::LOC11
    }
    #[doc = "Checks if the value of the field is `LOC12`"]
    #[inline]
    pub fn is_loc12(&self) -> bool {
        *self == RXLOCR::LOC12
    }
    #[doc = "Checks if the value of the field is `LOC13`"]
    #[inline]
    pub fn is_loc13(&self) -> bool {
        *self == RXLOCR::LOC13
    }
    #[doc = "Checks if the value of the field is `LOC14`"]
    #[inline]
    pub fn is_loc14(&self) -> bool {
        *self == RXLOCR::LOC14
    }
    #[doc = "Checks if the value of the field is `LOC15`"]
    #[inline]
    pub fn is_loc15(&self) -> bool {
        *self == RXLOCR::LOC15
    }
    #[doc = "Checks if the value of the field is `LOC16`"]
    #[inline]
    pub fn is_loc16(&self) -> bool {
        *self == RXLOCR::LOC16
    }
    #[doc = "Checks if the value of the field is `LOC17`"]
    #[inline]
    pub fn is_loc17(&self) -> bool {
        *self == RXLOCR::LOC17
    }
    #[doc = "Checks if the value of the field is `LOC18`"]
    #[inline]
    pub fn is_loc18(&self) -> bool {
        *self == RXLOCR::LOC18
    }
    #[doc = "Checks if the value of the field is `LOC19`"]
    #[inline]
    pub fn is_loc19(&self) -> bool {
        *self == RXLOCR::LOC19
    }
    #[doc = "Checks if the value of the field is `LOC20`"]
    #[inline]
    pub fn is_loc20(&self) -> bool {
        *self == RXLOCR::LOC20
    }
    #[doc = "Checks if the value of the field is `LOC21`"]
    #[inline]
    pub fn is_loc21(&self) -> bool {
        *self == RXLOCR::LOC21
    }
    #[doc = "Checks if the value of the field is `LOC22`"]
    #[inline]
    pub fn is_loc22(&self) -> bool {
        *self == RXLOCR::LOC22
    }
    #[doc = "Checks if the value of the field is `LOC23`"]
    #[inline]
    pub fn is_loc23(&self) -> bool {
        *self == RXLOCR::LOC23
    }
    #[doc = "Checks if the value of the field is `LOC24`"]
    #[inline]
    pub fn is_loc24(&self) -> bool {
        *self == RXLOCR::LOC24
    }
    #[doc = "Checks if the value of the field is `LOC25`"]
    #[inline]
    pub fn is_loc25(&self) -> bool {
        *self == RXLOCR::LOC25
    }
    #[doc = "Checks if the value of the field is `LOC26`"]
    #[inline]
    pub fn is_loc26(&self) -> bool {
        *self == RXLOCR::LOC26
    }
    #[doc = "Checks if the value of the field is `LOC27`"]
    #[inline]
    pub fn is_loc27(&self) -> bool {
        *self == RXLOCR::LOC27
    }
    #[doc = "Checks if the value of the field is `LOC28`"]
    #[inline]
    pub fn is_loc28(&self) -> bool {
        *self == RXLOCR::LOC28
    }
    #[doc = "Checks if the value of the field is `LOC29`"]
    #[inline]
    pub fn is_loc29(&self) -> bool {
        *self == RXLOCR::LOC29
    }
    #[doc = "Checks if the value of the field is `LOC30`"]
    #[inline]
    pub fn is_loc30(&self) -> bool {
        *self == RXLOCR::LOC30
    }
    #[doc = "Checks if the value of the field is `LOC31`"]
    #[inline]
    pub fn is_loc31(&self) -> bool {
        *self == RXLOCR::LOC31
    }
}
#[doc = "Possible values of the field `TXLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXLOCR {
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
impl TXLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXLOCR::LOC0 => 0,
            TXLOCR::LOC1 => 1,
            TXLOCR::LOC2 => 2,
            TXLOCR::LOC3 => 3,
            TXLOCR::LOC4 => 4,
            TXLOCR::LOC5 => 5,
            TXLOCR::LOC6 => 6,
            TXLOCR::LOC7 => 7,
            TXLOCR::LOC8 => 8,
            TXLOCR::LOC9 => 9,
            TXLOCR::LOC10 => 10,
            TXLOCR::LOC11 => 11,
            TXLOCR::LOC12 => 12,
            TXLOCR::LOC13 => 13,
            TXLOCR::LOC14 => 14,
            TXLOCR::LOC15 => 15,
            TXLOCR::LOC16 => 16,
            TXLOCR::LOC17 => 17,
            TXLOCR::LOC18 => 18,
            TXLOCR::LOC19 => 19,
            TXLOCR::LOC20 => 20,
            TXLOCR::LOC21 => 21,
            TXLOCR::LOC22 => 22,
            TXLOCR::LOC23 => 23,
            TXLOCR::LOC24 => 24,
            TXLOCR::LOC25 => 25,
            TXLOCR::LOC26 => 26,
            TXLOCR::LOC27 => 27,
            TXLOCR::LOC28 => 28,
            TXLOCR::LOC29 => 29,
            TXLOCR::LOC30 => 30,
            TXLOCR::LOC31 => 31,
            TXLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXLOCR {
        match value {
            0 => TXLOCR::LOC0,
            1 => TXLOCR::LOC1,
            2 => TXLOCR::LOC2,
            3 => TXLOCR::LOC3,
            4 => TXLOCR::LOC4,
            5 => TXLOCR::LOC5,
            6 => TXLOCR::LOC6,
            7 => TXLOCR::LOC7,
            8 => TXLOCR::LOC8,
            9 => TXLOCR::LOC9,
            10 => TXLOCR::LOC10,
            11 => TXLOCR::LOC11,
            12 => TXLOCR::LOC12,
            13 => TXLOCR::LOC13,
            14 => TXLOCR::LOC14,
            15 => TXLOCR::LOC15,
            16 => TXLOCR::LOC16,
            17 => TXLOCR::LOC17,
            18 => TXLOCR::LOC18,
            19 => TXLOCR::LOC19,
            20 => TXLOCR::LOC20,
            21 => TXLOCR::LOC21,
            22 => TXLOCR::LOC22,
            23 => TXLOCR::LOC23,
            24 => TXLOCR::LOC24,
            25 => TXLOCR::LOC25,
            26 => TXLOCR::LOC26,
            27 => TXLOCR::LOC27,
            28 => TXLOCR::LOC28,
            29 => TXLOCR::LOC29,
            30 => TXLOCR::LOC30,
            31 => TXLOCR::LOC31,
            i => TXLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == TXLOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == TXLOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == TXLOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == TXLOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == TXLOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == TXLOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == TXLOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == TXLOCR::LOC7
    }
    #[doc = "Checks if the value of the field is `LOC8`"]
    #[inline]
    pub fn is_loc8(&self) -> bool {
        *self == TXLOCR::LOC8
    }
    #[doc = "Checks if the value of the field is `LOC9`"]
    #[inline]
    pub fn is_loc9(&self) -> bool {
        *self == TXLOCR::LOC9
    }
    #[doc = "Checks if the value of the field is `LOC10`"]
    #[inline]
    pub fn is_loc10(&self) -> bool {
        *self == TXLOCR::LOC10
    }
    #[doc = "Checks if the value of the field is `LOC11`"]
    #[inline]
    pub fn is_loc11(&self) -> bool {
        *self == TXLOCR::LOC11
    }
    #[doc = "Checks if the value of the field is `LOC12`"]
    #[inline]
    pub fn is_loc12(&self) -> bool {
        *self == TXLOCR::LOC12
    }
    #[doc = "Checks if the value of the field is `LOC13`"]
    #[inline]
    pub fn is_loc13(&self) -> bool {
        *self == TXLOCR::LOC13
    }
    #[doc = "Checks if the value of the field is `LOC14`"]
    #[inline]
    pub fn is_loc14(&self) -> bool {
        *self == TXLOCR::LOC14
    }
    #[doc = "Checks if the value of the field is `LOC15`"]
    #[inline]
    pub fn is_loc15(&self) -> bool {
        *self == TXLOCR::LOC15
    }
    #[doc = "Checks if the value of the field is `LOC16`"]
    #[inline]
    pub fn is_loc16(&self) -> bool {
        *self == TXLOCR::LOC16
    }
    #[doc = "Checks if the value of the field is `LOC17`"]
    #[inline]
    pub fn is_loc17(&self) -> bool {
        *self == TXLOCR::LOC17
    }
    #[doc = "Checks if the value of the field is `LOC18`"]
    #[inline]
    pub fn is_loc18(&self) -> bool {
        *self == TXLOCR::LOC18
    }
    #[doc = "Checks if the value of the field is `LOC19`"]
    #[inline]
    pub fn is_loc19(&self) -> bool {
        *self == TXLOCR::LOC19
    }
    #[doc = "Checks if the value of the field is `LOC20`"]
    #[inline]
    pub fn is_loc20(&self) -> bool {
        *self == TXLOCR::LOC20
    }
    #[doc = "Checks if the value of the field is `LOC21`"]
    #[inline]
    pub fn is_loc21(&self) -> bool {
        *self == TXLOCR::LOC21
    }
    #[doc = "Checks if the value of the field is `LOC22`"]
    #[inline]
    pub fn is_loc22(&self) -> bool {
        *self == TXLOCR::LOC22
    }
    #[doc = "Checks if the value of the field is `LOC23`"]
    #[inline]
    pub fn is_loc23(&self) -> bool {
        *self == TXLOCR::LOC23
    }
    #[doc = "Checks if the value of the field is `LOC24`"]
    #[inline]
    pub fn is_loc24(&self) -> bool {
        *self == TXLOCR::LOC24
    }
    #[doc = "Checks if the value of the field is `LOC25`"]
    #[inline]
    pub fn is_loc25(&self) -> bool {
        *self == TXLOCR::LOC25
    }
    #[doc = "Checks if the value of the field is `LOC26`"]
    #[inline]
    pub fn is_loc26(&self) -> bool {
        *self == TXLOCR::LOC26
    }
    #[doc = "Checks if the value of the field is `LOC27`"]
    #[inline]
    pub fn is_loc27(&self) -> bool {
        *self == TXLOCR::LOC27
    }
    #[doc = "Checks if the value of the field is `LOC28`"]
    #[inline]
    pub fn is_loc28(&self) -> bool {
        *self == TXLOCR::LOC28
    }
    #[doc = "Checks if the value of the field is `LOC29`"]
    #[inline]
    pub fn is_loc29(&self) -> bool {
        *self == TXLOCR::LOC29
    }
    #[doc = "Checks if the value of the field is `LOC30`"]
    #[inline]
    pub fn is_loc30(&self) -> bool {
        *self == TXLOCR::LOC30
    }
    #[doc = "Checks if the value of the field is `LOC31`"]
    #[inline]
    pub fn is_loc31(&self) -> bool {
        *self == TXLOCR::LOC31
    }
}
#[doc = "Possible values of the field `CSLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSLOCR {
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
impl CSLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSLOCR::LOC0 => 0,
            CSLOCR::LOC1 => 1,
            CSLOCR::LOC2 => 2,
            CSLOCR::LOC3 => 3,
            CSLOCR::LOC4 => 4,
            CSLOCR::LOC5 => 5,
            CSLOCR::LOC6 => 6,
            CSLOCR::LOC7 => 7,
            CSLOCR::LOC8 => 8,
            CSLOCR::LOC9 => 9,
            CSLOCR::LOC10 => 10,
            CSLOCR::LOC11 => 11,
            CSLOCR::LOC12 => 12,
            CSLOCR::LOC13 => 13,
            CSLOCR::LOC14 => 14,
            CSLOCR::LOC15 => 15,
            CSLOCR::LOC16 => 16,
            CSLOCR::LOC17 => 17,
            CSLOCR::LOC18 => 18,
            CSLOCR::LOC19 => 19,
            CSLOCR::LOC20 => 20,
            CSLOCR::LOC21 => 21,
            CSLOCR::LOC22 => 22,
            CSLOCR::LOC23 => 23,
            CSLOCR::LOC24 => 24,
            CSLOCR::LOC25 => 25,
            CSLOCR::LOC26 => 26,
            CSLOCR::LOC27 => 27,
            CSLOCR::LOC28 => 28,
            CSLOCR::LOC29 => 29,
            CSLOCR::LOC30 => 30,
            CSLOCR::LOC31 => 31,
            CSLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSLOCR {
        match value {
            0 => CSLOCR::LOC0,
            1 => CSLOCR::LOC1,
            2 => CSLOCR::LOC2,
            3 => CSLOCR::LOC3,
            4 => CSLOCR::LOC4,
            5 => CSLOCR::LOC5,
            6 => CSLOCR::LOC6,
            7 => CSLOCR::LOC7,
            8 => CSLOCR::LOC8,
            9 => CSLOCR::LOC9,
            10 => CSLOCR::LOC10,
            11 => CSLOCR::LOC11,
            12 => CSLOCR::LOC12,
            13 => CSLOCR::LOC13,
            14 => CSLOCR::LOC14,
            15 => CSLOCR::LOC15,
            16 => CSLOCR::LOC16,
            17 => CSLOCR::LOC17,
            18 => CSLOCR::LOC18,
            19 => CSLOCR::LOC19,
            20 => CSLOCR::LOC20,
            21 => CSLOCR::LOC21,
            22 => CSLOCR::LOC22,
            23 => CSLOCR::LOC23,
            24 => CSLOCR::LOC24,
            25 => CSLOCR::LOC25,
            26 => CSLOCR::LOC26,
            27 => CSLOCR::LOC27,
            28 => CSLOCR::LOC28,
            29 => CSLOCR::LOC29,
            30 => CSLOCR::LOC30,
            31 => CSLOCR::LOC31,
            i => CSLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == CSLOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == CSLOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == CSLOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == CSLOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == CSLOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == CSLOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == CSLOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == CSLOCR::LOC7
    }
    #[doc = "Checks if the value of the field is `LOC8`"]
    #[inline]
    pub fn is_loc8(&self) -> bool {
        *self == CSLOCR::LOC8
    }
    #[doc = "Checks if the value of the field is `LOC9`"]
    #[inline]
    pub fn is_loc9(&self) -> bool {
        *self == CSLOCR::LOC9
    }
    #[doc = "Checks if the value of the field is `LOC10`"]
    #[inline]
    pub fn is_loc10(&self) -> bool {
        *self == CSLOCR::LOC10
    }
    #[doc = "Checks if the value of the field is `LOC11`"]
    #[inline]
    pub fn is_loc11(&self) -> bool {
        *self == CSLOCR::LOC11
    }
    #[doc = "Checks if the value of the field is `LOC12`"]
    #[inline]
    pub fn is_loc12(&self) -> bool {
        *self == CSLOCR::LOC12
    }
    #[doc = "Checks if the value of the field is `LOC13`"]
    #[inline]
    pub fn is_loc13(&self) -> bool {
        *self == CSLOCR::LOC13
    }
    #[doc = "Checks if the value of the field is `LOC14`"]
    #[inline]
    pub fn is_loc14(&self) -> bool {
        *self == CSLOCR::LOC14
    }
    #[doc = "Checks if the value of the field is `LOC15`"]
    #[inline]
    pub fn is_loc15(&self) -> bool {
        *self == CSLOCR::LOC15
    }
    #[doc = "Checks if the value of the field is `LOC16`"]
    #[inline]
    pub fn is_loc16(&self) -> bool {
        *self == CSLOCR::LOC16
    }
    #[doc = "Checks if the value of the field is `LOC17`"]
    #[inline]
    pub fn is_loc17(&self) -> bool {
        *self == CSLOCR::LOC17
    }
    #[doc = "Checks if the value of the field is `LOC18`"]
    #[inline]
    pub fn is_loc18(&self) -> bool {
        *self == CSLOCR::LOC18
    }
    #[doc = "Checks if the value of the field is `LOC19`"]
    #[inline]
    pub fn is_loc19(&self) -> bool {
        *self == CSLOCR::LOC19
    }
    #[doc = "Checks if the value of the field is `LOC20`"]
    #[inline]
    pub fn is_loc20(&self) -> bool {
        *self == CSLOCR::LOC20
    }
    #[doc = "Checks if the value of the field is `LOC21`"]
    #[inline]
    pub fn is_loc21(&self) -> bool {
        *self == CSLOCR::LOC21
    }
    #[doc = "Checks if the value of the field is `LOC22`"]
    #[inline]
    pub fn is_loc22(&self) -> bool {
        *self == CSLOCR::LOC22
    }
    #[doc = "Checks if the value of the field is `LOC23`"]
    #[inline]
    pub fn is_loc23(&self) -> bool {
        *self == CSLOCR::LOC23
    }
    #[doc = "Checks if the value of the field is `LOC24`"]
    #[inline]
    pub fn is_loc24(&self) -> bool {
        *self == CSLOCR::LOC24
    }
    #[doc = "Checks if the value of the field is `LOC25`"]
    #[inline]
    pub fn is_loc25(&self) -> bool {
        *self == CSLOCR::LOC25
    }
    #[doc = "Checks if the value of the field is `LOC26`"]
    #[inline]
    pub fn is_loc26(&self) -> bool {
        *self == CSLOCR::LOC26
    }
    #[doc = "Checks if the value of the field is `LOC27`"]
    #[inline]
    pub fn is_loc27(&self) -> bool {
        *self == CSLOCR::LOC27
    }
    #[doc = "Checks if the value of the field is `LOC28`"]
    #[inline]
    pub fn is_loc28(&self) -> bool {
        *self == CSLOCR::LOC28
    }
    #[doc = "Checks if the value of the field is `LOC29`"]
    #[inline]
    pub fn is_loc29(&self) -> bool {
        *self == CSLOCR::LOC29
    }
    #[doc = "Checks if the value of the field is `LOC30`"]
    #[inline]
    pub fn is_loc30(&self) -> bool {
        *self == CSLOCR::LOC30
    }
    #[doc = "Checks if the value of the field is `LOC31`"]
    #[inline]
    pub fn is_loc31(&self) -> bool {
        *self == CSLOCR::LOC31
    }
}
#[doc = "Possible values of the field `CLKLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKLOCR {
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
impl CLKLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKLOCR::LOC0 => 0,
            CLKLOCR::LOC1 => 1,
            CLKLOCR::LOC2 => 2,
            CLKLOCR::LOC3 => 3,
            CLKLOCR::LOC4 => 4,
            CLKLOCR::LOC5 => 5,
            CLKLOCR::LOC6 => 6,
            CLKLOCR::LOC7 => 7,
            CLKLOCR::LOC8 => 8,
            CLKLOCR::LOC9 => 9,
            CLKLOCR::LOC10 => 10,
            CLKLOCR::LOC11 => 11,
            CLKLOCR::LOC12 => 12,
            CLKLOCR::LOC13 => 13,
            CLKLOCR::LOC14 => 14,
            CLKLOCR::LOC15 => 15,
            CLKLOCR::LOC16 => 16,
            CLKLOCR::LOC17 => 17,
            CLKLOCR::LOC18 => 18,
            CLKLOCR::LOC19 => 19,
            CLKLOCR::LOC20 => 20,
            CLKLOCR::LOC21 => 21,
            CLKLOCR::LOC22 => 22,
            CLKLOCR::LOC23 => 23,
            CLKLOCR::LOC24 => 24,
            CLKLOCR::LOC25 => 25,
            CLKLOCR::LOC26 => 26,
            CLKLOCR::LOC27 => 27,
            CLKLOCR::LOC28 => 28,
            CLKLOCR::LOC29 => 29,
            CLKLOCR::LOC30 => 30,
            CLKLOCR::LOC31 => 31,
            CLKLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKLOCR {
        match value {
            0 => CLKLOCR::LOC0,
            1 => CLKLOCR::LOC1,
            2 => CLKLOCR::LOC2,
            3 => CLKLOCR::LOC3,
            4 => CLKLOCR::LOC4,
            5 => CLKLOCR::LOC5,
            6 => CLKLOCR::LOC6,
            7 => CLKLOCR::LOC7,
            8 => CLKLOCR::LOC8,
            9 => CLKLOCR::LOC9,
            10 => CLKLOCR::LOC10,
            11 => CLKLOCR::LOC11,
            12 => CLKLOCR::LOC12,
            13 => CLKLOCR::LOC13,
            14 => CLKLOCR::LOC14,
            15 => CLKLOCR::LOC15,
            16 => CLKLOCR::LOC16,
            17 => CLKLOCR::LOC17,
            18 => CLKLOCR::LOC18,
            19 => CLKLOCR::LOC19,
            20 => CLKLOCR::LOC20,
            21 => CLKLOCR::LOC21,
            22 => CLKLOCR::LOC22,
            23 => CLKLOCR::LOC23,
            24 => CLKLOCR::LOC24,
            25 => CLKLOCR::LOC25,
            26 => CLKLOCR::LOC26,
            27 => CLKLOCR::LOC27,
            28 => CLKLOCR::LOC28,
            29 => CLKLOCR::LOC29,
            30 => CLKLOCR::LOC30,
            31 => CLKLOCR::LOC31,
            i => CLKLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == CLKLOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == CLKLOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == CLKLOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == CLKLOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == CLKLOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == CLKLOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == CLKLOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == CLKLOCR::LOC7
    }
    #[doc = "Checks if the value of the field is `LOC8`"]
    #[inline]
    pub fn is_loc8(&self) -> bool {
        *self == CLKLOCR::LOC8
    }
    #[doc = "Checks if the value of the field is `LOC9`"]
    #[inline]
    pub fn is_loc9(&self) -> bool {
        *self == CLKLOCR::LOC9
    }
    #[doc = "Checks if the value of the field is `LOC10`"]
    #[inline]
    pub fn is_loc10(&self) -> bool {
        *self == CLKLOCR::LOC10
    }
    #[doc = "Checks if the value of the field is `LOC11`"]
    #[inline]
    pub fn is_loc11(&self) -> bool {
        *self == CLKLOCR::LOC11
    }
    #[doc = "Checks if the value of the field is `LOC12`"]
    #[inline]
    pub fn is_loc12(&self) -> bool {
        *self == CLKLOCR::LOC12
    }
    #[doc = "Checks if the value of the field is `LOC13`"]
    #[inline]
    pub fn is_loc13(&self) -> bool {
        *self == CLKLOCR::LOC13
    }
    #[doc = "Checks if the value of the field is `LOC14`"]
    #[inline]
    pub fn is_loc14(&self) -> bool {
        *self == CLKLOCR::LOC14
    }
    #[doc = "Checks if the value of the field is `LOC15`"]
    #[inline]
    pub fn is_loc15(&self) -> bool {
        *self == CLKLOCR::LOC15
    }
    #[doc = "Checks if the value of the field is `LOC16`"]
    #[inline]
    pub fn is_loc16(&self) -> bool {
        *self == CLKLOCR::LOC16
    }
    #[doc = "Checks if the value of the field is `LOC17`"]
    #[inline]
    pub fn is_loc17(&self) -> bool {
        *self == CLKLOCR::LOC17
    }
    #[doc = "Checks if the value of the field is `LOC18`"]
    #[inline]
    pub fn is_loc18(&self) -> bool {
        *self == CLKLOCR::LOC18
    }
    #[doc = "Checks if the value of the field is `LOC19`"]
    #[inline]
    pub fn is_loc19(&self) -> bool {
        *self == CLKLOCR::LOC19
    }
    #[doc = "Checks if the value of the field is `LOC20`"]
    #[inline]
    pub fn is_loc20(&self) -> bool {
        *self == CLKLOCR::LOC20
    }
    #[doc = "Checks if the value of the field is `LOC21`"]
    #[inline]
    pub fn is_loc21(&self) -> bool {
        *self == CLKLOCR::LOC21
    }
    #[doc = "Checks if the value of the field is `LOC22`"]
    #[inline]
    pub fn is_loc22(&self) -> bool {
        *self == CLKLOCR::LOC22
    }
    #[doc = "Checks if the value of the field is `LOC23`"]
    #[inline]
    pub fn is_loc23(&self) -> bool {
        *self == CLKLOCR::LOC23
    }
    #[doc = "Checks if the value of the field is `LOC24`"]
    #[inline]
    pub fn is_loc24(&self) -> bool {
        *self == CLKLOCR::LOC24
    }
    #[doc = "Checks if the value of the field is `LOC25`"]
    #[inline]
    pub fn is_loc25(&self) -> bool {
        *self == CLKLOCR::LOC25
    }
    #[doc = "Checks if the value of the field is `LOC26`"]
    #[inline]
    pub fn is_loc26(&self) -> bool {
        *self == CLKLOCR::LOC26
    }
    #[doc = "Checks if the value of the field is `LOC27`"]
    #[inline]
    pub fn is_loc27(&self) -> bool {
        *self == CLKLOCR::LOC27
    }
    #[doc = "Checks if the value of the field is `LOC28`"]
    #[inline]
    pub fn is_loc28(&self) -> bool {
        *self == CLKLOCR::LOC28
    }
    #[doc = "Checks if the value of the field is `LOC29`"]
    #[inline]
    pub fn is_loc29(&self) -> bool {
        *self == CLKLOCR::LOC29
    }
    #[doc = "Checks if the value of the field is `LOC30`"]
    #[inline]
    pub fn is_loc30(&self) -> bool {
        *self == CLKLOCR::LOC30
    }
    #[doc = "Checks if the value of the field is `LOC31`"]
    #[inline]
    pub fn is_loc31(&self) -> bool {
        *self == CLKLOCR::LOC31
    }
}
#[doc = "Values that can be written to the field `RXLOC`"]
pub enum RXLOCW {
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
impl RXLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXLOCW::LOC0 => 0,
            RXLOCW::LOC1 => 1,
            RXLOCW::LOC2 => 2,
            RXLOCW::LOC3 => 3,
            RXLOCW::LOC4 => 4,
            RXLOCW::LOC5 => 5,
            RXLOCW::LOC6 => 6,
            RXLOCW::LOC7 => 7,
            RXLOCW::LOC8 => 8,
            RXLOCW::LOC9 => 9,
            RXLOCW::LOC10 => 10,
            RXLOCW::LOC11 => 11,
            RXLOCW::LOC12 => 12,
            RXLOCW::LOC13 => 13,
            RXLOCW::LOC14 => 14,
            RXLOCW::LOC15 => 15,
            RXLOCW::LOC16 => 16,
            RXLOCW::LOC17 => 17,
            RXLOCW::LOC18 => 18,
            RXLOCW::LOC19 => 19,
            RXLOCW::LOC20 => 20,
            RXLOCW::LOC21 => 21,
            RXLOCW::LOC22 => 22,
            RXLOCW::LOC23 => 23,
            RXLOCW::LOC24 => 24,
            RXLOCW::LOC25 => 25,
            RXLOCW::LOC26 => 26,
            RXLOCW::LOC27 => 27,
            RXLOCW::LOC28 => 28,
            RXLOCW::LOC29 => 29,
            RXLOCW::LOC30 => 30,
            RXLOCW::LOC31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _RXLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(RXLOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(RXLOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(RXLOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(RXLOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(RXLOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(RXLOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(RXLOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(RXLOCW::LOC7)
    }
    #[doc = "Location 8"]
    #[inline]
    pub fn loc8(self) -> &'a mut W {
        self.variant(RXLOCW::LOC8)
    }
    #[doc = "Location 9"]
    #[inline]
    pub fn loc9(self) -> &'a mut W {
        self.variant(RXLOCW::LOC9)
    }
    #[doc = "Location 10"]
    #[inline]
    pub fn loc10(self) -> &'a mut W {
        self.variant(RXLOCW::LOC10)
    }
    #[doc = "Location 11"]
    #[inline]
    pub fn loc11(self) -> &'a mut W {
        self.variant(RXLOCW::LOC11)
    }
    #[doc = "Location 12"]
    #[inline]
    pub fn loc12(self) -> &'a mut W {
        self.variant(RXLOCW::LOC12)
    }
    #[doc = "Location 13"]
    #[inline]
    pub fn loc13(self) -> &'a mut W {
        self.variant(RXLOCW::LOC13)
    }
    #[doc = "Location 14"]
    #[inline]
    pub fn loc14(self) -> &'a mut W {
        self.variant(RXLOCW::LOC14)
    }
    #[doc = "Location 15"]
    #[inline]
    pub fn loc15(self) -> &'a mut W {
        self.variant(RXLOCW::LOC15)
    }
    #[doc = "Location 16"]
    #[inline]
    pub fn loc16(self) -> &'a mut W {
        self.variant(RXLOCW::LOC16)
    }
    #[doc = "Location 17"]
    #[inline]
    pub fn loc17(self) -> &'a mut W {
        self.variant(RXLOCW::LOC17)
    }
    #[doc = "Location 18"]
    #[inline]
    pub fn loc18(self) -> &'a mut W {
        self.variant(RXLOCW::LOC18)
    }
    #[doc = "Location 19"]
    #[inline]
    pub fn loc19(self) -> &'a mut W {
        self.variant(RXLOCW::LOC19)
    }
    #[doc = "Location 20"]
    #[inline]
    pub fn loc20(self) -> &'a mut W {
        self.variant(RXLOCW::LOC20)
    }
    #[doc = "Location 21"]
    #[inline]
    pub fn loc21(self) -> &'a mut W {
        self.variant(RXLOCW::LOC21)
    }
    #[doc = "Location 22"]
    #[inline]
    pub fn loc22(self) -> &'a mut W {
        self.variant(RXLOCW::LOC22)
    }
    #[doc = "Location 23"]
    #[inline]
    pub fn loc23(self) -> &'a mut W {
        self.variant(RXLOCW::LOC23)
    }
    #[doc = "Location 24"]
    #[inline]
    pub fn loc24(self) -> &'a mut W {
        self.variant(RXLOCW::LOC24)
    }
    #[doc = "Location 25"]
    #[inline]
    pub fn loc25(self) -> &'a mut W {
        self.variant(RXLOCW::LOC25)
    }
    #[doc = "Location 26"]
    #[inline]
    pub fn loc26(self) -> &'a mut W {
        self.variant(RXLOCW::LOC26)
    }
    #[doc = "Location 27"]
    #[inline]
    pub fn loc27(self) -> &'a mut W {
        self.variant(RXLOCW::LOC27)
    }
    #[doc = "Location 28"]
    #[inline]
    pub fn loc28(self) -> &'a mut W {
        self.variant(RXLOCW::LOC28)
    }
    #[doc = "Location 29"]
    #[inline]
    pub fn loc29(self) -> &'a mut W {
        self.variant(RXLOCW::LOC29)
    }
    #[doc = "Location 30"]
    #[inline]
    pub fn loc30(self) -> &'a mut W {
        self.variant(RXLOCW::LOC30)
    }
    #[doc = "Location 31"]
    #[inline]
    pub fn loc31(self) -> &'a mut W {
        self.variant(RXLOCW::LOC31)
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
#[doc = "Values that can be written to the field `TXLOC`"]
pub enum TXLOCW {
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
impl TXLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXLOCW::LOC0 => 0,
            TXLOCW::LOC1 => 1,
            TXLOCW::LOC2 => 2,
            TXLOCW::LOC3 => 3,
            TXLOCW::LOC4 => 4,
            TXLOCW::LOC5 => 5,
            TXLOCW::LOC6 => 6,
            TXLOCW::LOC7 => 7,
            TXLOCW::LOC8 => 8,
            TXLOCW::LOC9 => 9,
            TXLOCW::LOC10 => 10,
            TXLOCW::LOC11 => 11,
            TXLOCW::LOC12 => 12,
            TXLOCW::LOC13 => 13,
            TXLOCW::LOC14 => 14,
            TXLOCW::LOC15 => 15,
            TXLOCW::LOC16 => 16,
            TXLOCW::LOC17 => 17,
            TXLOCW::LOC18 => 18,
            TXLOCW::LOC19 => 19,
            TXLOCW::LOC20 => 20,
            TXLOCW::LOC21 => 21,
            TXLOCW::LOC22 => 22,
            TXLOCW::LOC23 => 23,
            TXLOCW::LOC24 => 24,
            TXLOCW::LOC25 => 25,
            TXLOCW::LOC26 => 26,
            TXLOCW::LOC27 => 27,
            TXLOCW::LOC28 => 28,
            TXLOCW::LOC29 => 29,
            TXLOCW::LOC30 => 30,
            TXLOCW::LOC31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _TXLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(TXLOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(TXLOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(TXLOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(TXLOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(TXLOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(TXLOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(TXLOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(TXLOCW::LOC7)
    }
    #[doc = "Location 8"]
    #[inline]
    pub fn loc8(self) -> &'a mut W {
        self.variant(TXLOCW::LOC8)
    }
    #[doc = "Location 9"]
    #[inline]
    pub fn loc9(self) -> &'a mut W {
        self.variant(TXLOCW::LOC9)
    }
    #[doc = "Location 10"]
    #[inline]
    pub fn loc10(self) -> &'a mut W {
        self.variant(TXLOCW::LOC10)
    }
    #[doc = "Location 11"]
    #[inline]
    pub fn loc11(self) -> &'a mut W {
        self.variant(TXLOCW::LOC11)
    }
    #[doc = "Location 12"]
    #[inline]
    pub fn loc12(self) -> &'a mut W {
        self.variant(TXLOCW::LOC12)
    }
    #[doc = "Location 13"]
    #[inline]
    pub fn loc13(self) -> &'a mut W {
        self.variant(TXLOCW::LOC13)
    }
    #[doc = "Location 14"]
    #[inline]
    pub fn loc14(self) -> &'a mut W {
        self.variant(TXLOCW::LOC14)
    }
    #[doc = "Location 15"]
    #[inline]
    pub fn loc15(self) -> &'a mut W {
        self.variant(TXLOCW::LOC15)
    }
    #[doc = "Location 16"]
    #[inline]
    pub fn loc16(self) -> &'a mut W {
        self.variant(TXLOCW::LOC16)
    }
    #[doc = "Location 17"]
    #[inline]
    pub fn loc17(self) -> &'a mut W {
        self.variant(TXLOCW::LOC17)
    }
    #[doc = "Location 18"]
    #[inline]
    pub fn loc18(self) -> &'a mut W {
        self.variant(TXLOCW::LOC18)
    }
    #[doc = "Location 19"]
    #[inline]
    pub fn loc19(self) -> &'a mut W {
        self.variant(TXLOCW::LOC19)
    }
    #[doc = "Location 20"]
    #[inline]
    pub fn loc20(self) -> &'a mut W {
        self.variant(TXLOCW::LOC20)
    }
    #[doc = "Location 21"]
    #[inline]
    pub fn loc21(self) -> &'a mut W {
        self.variant(TXLOCW::LOC21)
    }
    #[doc = "Location 22"]
    #[inline]
    pub fn loc22(self) -> &'a mut W {
        self.variant(TXLOCW::LOC22)
    }
    #[doc = "Location 23"]
    #[inline]
    pub fn loc23(self) -> &'a mut W {
        self.variant(TXLOCW::LOC23)
    }
    #[doc = "Location 24"]
    #[inline]
    pub fn loc24(self) -> &'a mut W {
        self.variant(TXLOCW::LOC24)
    }
    #[doc = "Location 25"]
    #[inline]
    pub fn loc25(self) -> &'a mut W {
        self.variant(TXLOCW::LOC25)
    }
    #[doc = "Location 26"]
    #[inline]
    pub fn loc26(self) -> &'a mut W {
        self.variant(TXLOCW::LOC26)
    }
    #[doc = "Location 27"]
    #[inline]
    pub fn loc27(self) -> &'a mut W {
        self.variant(TXLOCW::LOC27)
    }
    #[doc = "Location 28"]
    #[inline]
    pub fn loc28(self) -> &'a mut W {
        self.variant(TXLOCW::LOC28)
    }
    #[doc = "Location 29"]
    #[inline]
    pub fn loc29(self) -> &'a mut W {
        self.variant(TXLOCW::LOC29)
    }
    #[doc = "Location 30"]
    #[inline]
    pub fn loc30(self) -> &'a mut W {
        self.variant(TXLOCW::LOC30)
    }
    #[doc = "Location 31"]
    #[inline]
    pub fn loc31(self) -> &'a mut W {
        self.variant(TXLOCW::LOC31)
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
#[doc = "Values that can be written to the field `CSLOC`"]
pub enum CSLOCW {
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
impl CSLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSLOCW::LOC0 => 0,
            CSLOCW::LOC1 => 1,
            CSLOCW::LOC2 => 2,
            CSLOCW::LOC3 => 3,
            CSLOCW::LOC4 => 4,
            CSLOCW::LOC5 => 5,
            CSLOCW::LOC6 => 6,
            CSLOCW::LOC7 => 7,
            CSLOCW::LOC8 => 8,
            CSLOCW::LOC9 => 9,
            CSLOCW::LOC10 => 10,
            CSLOCW::LOC11 => 11,
            CSLOCW::LOC12 => 12,
            CSLOCW::LOC13 => 13,
            CSLOCW::LOC14 => 14,
            CSLOCW::LOC15 => 15,
            CSLOCW::LOC16 => 16,
            CSLOCW::LOC17 => 17,
            CSLOCW::LOC18 => 18,
            CSLOCW::LOC19 => 19,
            CSLOCW::LOC20 => 20,
            CSLOCW::LOC21 => 21,
            CSLOCW::LOC22 => 22,
            CSLOCW::LOC23 => 23,
            CSLOCW::LOC24 => 24,
            CSLOCW::LOC25 => 25,
            CSLOCW::LOC26 => 26,
            CSLOCW::LOC27 => 27,
            CSLOCW::LOC28 => 28,
            CSLOCW::LOC29 => 29,
            CSLOCW::LOC30 => 30,
            CSLOCW::LOC31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _CSLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CSLOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CSLOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CSLOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CSLOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CSLOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CSLOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CSLOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CSLOCW::LOC7)
    }
    #[doc = "Location 8"]
    #[inline]
    pub fn loc8(self) -> &'a mut W {
        self.variant(CSLOCW::LOC8)
    }
    #[doc = "Location 9"]
    #[inline]
    pub fn loc9(self) -> &'a mut W {
        self.variant(CSLOCW::LOC9)
    }
    #[doc = "Location 10"]
    #[inline]
    pub fn loc10(self) -> &'a mut W {
        self.variant(CSLOCW::LOC10)
    }
    #[doc = "Location 11"]
    #[inline]
    pub fn loc11(self) -> &'a mut W {
        self.variant(CSLOCW::LOC11)
    }
    #[doc = "Location 12"]
    #[inline]
    pub fn loc12(self) -> &'a mut W {
        self.variant(CSLOCW::LOC12)
    }
    #[doc = "Location 13"]
    #[inline]
    pub fn loc13(self) -> &'a mut W {
        self.variant(CSLOCW::LOC13)
    }
    #[doc = "Location 14"]
    #[inline]
    pub fn loc14(self) -> &'a mut W {
        self.variant(CSLOCW::LOC14)
    }
    #[doc = "Location 15"]
    #[inline]
    pub fn loc15(self) -> &'a mut W {
        self.variant(CSLOCW::LOC15)
    }
    #[doc = "Location 16"]
    #[inline]
    pub fn loc16(self) -> &'a mut W {
        self.variant(CSLOCW::LOC16)
    }
    #[doc = "Location 17"]
    #[inline]
    pub fn loc17(self) -> &'a mut W {
        self.variant(CSLOCW::LOC17)
    }
    #[doc = "Location 18"]
    #[inline]
    pub fn loc18(self) -> &'a mut W {
        self.variant(CSLOCW::LOC18)
    }
    #[doc = "Location 19"]
    #[inline]
    pub fn loc19(self) -> &'a mut W {
        self.variant(CSLOCW::LOC19)
    }
    #[doc = "Location 20"]
    #[inline]
    pub fn loc20(self) -> &'a mut W {
        self.variant(CSLOCW::LOC20)
    }
    #[doc = "Location 21"]
    #[inline]
    pub fn loc21(self) -> &'a mut W {
        self.variant(CSLOCW::LOC21)
    }
    #[doc = "Location 22"]
    #[inline]
    pub fn loc22(self) -> &'a mut W {
        self.variant(CSLOCW::LOC22)
    }
    #[doc = "Location 23"]
    #[inline]
    pub fn loc23(self) -> &'a mut W {
        self.variant(CSLOCW::LOC23)
    }
    #[doc = "Location 24"]
    #[inline]
    pub fn loc24(self) -> &'a mut W {
        self.variant(CSLOCW::LOC24)
    }
    #[doc = "Location 25"]
    #[inline]
    pub fn loc25(self) -> &'a mut W {
        self.variant(CSLOCW::LOC25)
    }
    #[doc = "Location 26"]
    #[inline]
    pub fn loc26(self) -> &'a mut W {
        self.variant(CSLOCW::LOC26)
    }
    #[doc = "Location 27"]
    #[inline]
    pub fn loc27(self) -> &'a mut W {
        self.variant(CSLOCW::LOC27)
    }
    #[doc = "Location 28"]
    #[inline]
    pub fn loc28(self) -> &'a mut W {
        self.variant(CSLOCW::LOC28)
    }
    #[doc = "Location 29"]
    #[inline]
    pub fn loc29(self) -> &'a mut W {
        self.variant(CSLOCW::LOC29)
    }
    #[doc = "Location 30"]
    #[inline]
    pub fn loc30(self) -> &'a mut W {
        self.variant(CSLOCW::LOC30)
    }
    #[doc = "Location 31"]
    #[inline]
    pub fn loc31(self) -> &'a mut W {
        self.variant(CSLOCW::LOC31)
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
#[doc = "Values that can be written to the field `CLKLOC`"]
pub enum CLKLOCW {
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
impl CLKLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKLOCW::LOC0 => 0,
            CLKLOCW::LOC1 => 1,
            CLKLOCW::LOC2 => 2,
            CLKLOCW::LOC3 => 3,
            CLKLOCW::LOC4 => 4,
            CLKLOCW::LOC5 => 5,
            CLKLOCW::LOC6 => 6,
            CLKLOCW::LOC7 => 7,
            CLKLOCW::LOC8 => 8,
            CLKLOCW::LOC9 => 9,
            CLKLOCW::LOC10 => 10,
            CLKLOCW::LOC11 => 11,
            CLKLOCW::LOC12 => 12,
            CLKLOCW::LOC13 => 13,
            CLKLOCW::LOC14 => 14,
            CLKLOCW::LOC15 => 15,
            CLKLOCW::LOC16 => 16,
            CLKLOCW::LOC17 => 17,
            CLKLOCW::LOC18 => 18,
            CLKLOCW::LOC19 => 19,
            CLKLOCW::LOC20 => 20,
            CLKLOCW::LOC21 => 21,
            CLKLOCW::LOC22 => 22,
            CLKLOCW::LOC23 => 23,
            CLKLOCW::LOC24 => 24,
            CLKLOCW::LOC25 => 25,
            CLKLOCW::LOC26 => 26,
            CLKLOCW::LOC27 => 27,
            CLKLOCW::LOC28 => 28,
            CLKLOCW::LOC29 => 29,
            CLKLOCW::LOC30 => 30,
            CLKLOCW::LOC31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC7)
    }
    #[doc = "Location 8"]
    #[inline]
    pub fn loc8(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC8)
    }
    #[doc = "Location 9"]
    #[inline]
    pub fn loc9(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC9)
    }
    #[doc = "Location 10"]
    #[inline]
    pub fn loc10(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC10)
    }
    #[doc = "Location 11"]
    #[inline]
    pub fn loc11(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC11)
    }
    #[doc = "Location 12"]
    #[inline]
    pub fn loc12(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC12)
    }
    #[doc = "Location 13"]
    #[inline]
    pub fn loc13(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC13)
    }
    #[doc = "Location 14"]
    #[inline]
    pub fn loc14(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC14)
    }
    #[doc = "Location 15"]
    #[inline]
    pub fn loc15(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC15)
    }
    #[doc = "Location 16"]
    #[inline]
    pub fn loc16(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC16)
    }
    #[doc = "Location 17"]
    #[inline]
    pub fn loc17(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC17)
    }
    #[doc = "Location 18"]
    #[inline]
    pub fn loc18(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC18)
    }
    #[doc = "Location 19"]
    #[inline]
    pub fn loc19(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC19)
    }
    #[doc = "Location 20"]
    #[inline]
    pub fn loc20(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC20)
    }
    #[doc = "Location 21"]
    #[inline]
    pub fn loc21(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC21)
    }
    #[doc = "Location 22"]
    #[inline]
    pub fn loc22(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC22)
    }
    #[doc = "Location 23"]
    #[inline]
    pub fn loc23(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC23)
    }
    #[doc = "Location 24"]
    #[inline]
    pub fn loc24(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC24)
    }
    #[doc = "Location 25"]
    #[inline]
    pub fn loc25(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC25)
    }
    #[doc = "Location 26"]
    #[inline]
    pub fn loc26(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC26)
    }
    #[doc = "Location 27"]
    #[inline]
    pub fn loc27(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC27)
    }
    #[doc = "Location 28"]
    #[inline]
    pub fn loc28(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC28)
    }
    #[doc = "Location 29"]
    #[inline]
    pub fn loc29(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC29)
    }
    #[doc = "Location 30"]
    #[inline]
    pub fn loc30(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC30)
    }
    #[doc = "Location 31"]
    #[inline]
    pub fn loc31(self) -> &'a mut W {
        self.variant(CLKLOCW::LOC31)
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
    pub fn rxloc(&self) -> RXLOCR {
        RXLOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline]
    pub fn txloc(&self) -> TXLOCR {
        TXLOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline]
    pub fn csloc(&self) -> CSLOCR {
        CSLOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline]
    pub fn clkloc(&self) -> CLKLOCR {
        CLKLOCR::_from({
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
    pub fn rxloc(&mut self) -> _RXLOCW {
        _RXLOCW { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline]
    pub fn txloc(&mut self) -> _TXLOCW {
        _TXLOCW { w: self }
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline]
    pub fn csloc(&mut self) -> _CSLOCW {
        _CSLOCW { w: self }
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline]
    pub fn clkloc(&mut self) -> _CLKLOCW {
        _CLKLOCW { w: self }
    }
}
