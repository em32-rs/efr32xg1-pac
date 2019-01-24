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
pub struct ENR {
    bits: bool,
}
impl ENR {
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
pub struct CURSINKR {
    bits: bool,
}
impl CURSINKR {
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
pub struct MINOUTTRANSR {
    bits: bool,
}
impl MINOUTTRANSR {
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
pub struct APORTOUTENR {
    bits: bool,
}
impl APORTOUTENR {
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
#[doc = "Possible values of the field `APORTOUTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APORTOUTSELR {
    #[doc = "APORT1X Channel 0"]
    APORT1XCH0,
    #[doc = "APORT1Y Channel 1"]
    APORT1YCH1,
    #[doc = "APORT1X Channel 2"]
    APORT1XCH2,
    #[doc = "APORT1Y Channel 3"]
    APORT1YCH3,
    #[doc = "APORT1X Channel 4"]
    APORT1XCH4,
    #[doc = "APORT1Y Channel 5"]
    APORT1YCH5,
    #[doc = "APORT1X Channel 6"]
    APORT1XCH6,
    #[doc = "APORT1Y Channel 7"]
    APORT1YCH7,
    #[doc = "APORT1X Channel 8"]
    APORT1XCH8,
    #[doc = "APORT1Y Channel 9"]
    APORT1YCH9,
    #[doc = "APORT1X Channel 10"]
    APORT1XCH10,
    #[doc = "APORT1Y Channel 11"]
    APORT1YCH11,
    #[doc = "APORT1X Channel 12"]
    APORT1XCH12,
    #[doc = "APORT1Y Channel 13"]
    APORT1YCH13,
    #[doc = "APORT1X Channel 14"]
    APORT1XCH14,
    #[doc = "APORT1Y Channel 15"]
    APORT1YCH15,
    #[doc = "APORT1X Channel 16"]
    APORT1XCH16,
    #[doc = "APORT1Y Channel 17"]
    APORT1YCH17,
    #[doc = "APORT1X Channel 18"]
    APORT1XCH18,
    #[doc = "APORT1Y Channel 19"]
    APORT1YCH19,
    #[doc = "APORT1X Channel 20"]
    APORT1XCH20,
    #[doc = "APORT1Y Channel 21"]
    APORT1YCH21,
    #[doc = "APORT1X Channel 22"]
    APORT1XCH22,
    #[doc = "APORT1Y Channel 23"]
    APORT1YCH23,
    #[doc = "APORT1X Channel 24"]
    APORT1XCH24,
    #[doc = "APORT1Y Channel 25"]
    APORT1YCH25,
    #[doc = "APORT1X Channel 26"]
    APORT1XCH26,
    #[doc = "APORT1Y Channel 27"]
    APORT1YCH27,
    #[doc = "APORT1X Channel 28"]
    APORT1XCH28,
    #[doc = "APORT1Y Channel 29"]
    APORT1YCH29,
    #[doc = "APORT1X Channel 30"]
    APORT1XCH30,
    #[doc = "APORT1Y Channel 31"]
    APORT1YCH31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl APORTOUTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            APORTOUTSELR::APORT1XCH0 => 32,
            APORTOUTSELR::APORT1YCH1 => 33,
            APORTOUTSELR::APORT1XCH2 => 34,
            APORTOUTSELR::APORT1YCH3 => 35,
            APORTOUTSELR::APORT1XCH4 => 36,
            APORTOUTSELR::APORT1YCH5 => 37,
            APORTOUTSELR::APORT1XCH6 => 38,
            APORTOUTSELR::APORT1YCH7 => 39,
            APORTOUTSELR::APORT1XCH8 => 40,
            APORTOUTSELR::APORT1YCH9 => 41,
            APORTOUTSELR::APORT1XCH10 => 42,
            APORTOUTSELR::APORT1YCH11 => 43,
            APORTOUTSELR::APORT1XCH12 => 44,
            APORTOUTSELR::APORT1YCH13 => 45,
            APORTOUTSELR::APORT1XCH14 => 46,
            APORTOUTSELR::APORT1YCH15 => 47,
            APORTOUTSELR::APORT1XCH16 => 48,
            APORTOUTSELR::APORT1YCH17 => 49,
            APORTOUTSELR::APORT1XCH18 => 50,
            APORTOUTSELR::APORT1YCH19 => 51,
            APORTOUTSELR::APORT1XCH20 => 52,
            APORTOUTSELR::APORT1YCH21 => 53,
            APORTOUTSELR::APORT1XCH22 => 54,
            APORTOUTSELR::APORT1YCH23 => 55,
            APORTOUTSELR::APORT1XCH24 => 56,
            APORTOUTSELR::APORT1YCH25 => 57,
            APORTOUTSELR::APORT1XCH26 => 58,
            APORTOUTSELR::APORT1YCH27 => 59,
            APORTOUTSELR::APORT1XCH28 => 60,
            APORTOUTSELR::APORT1YCH29 => 61,
            APORTOUTSELR::APORT1XCH30 => 62,
            APORTOUTSELR::APORT1YCH31 => 63,
            APORTOUTSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> APORTOUTSELR {
        match value {
            32 => APORTOUTSELR::APORT1XCH0,
            33 => APORTOUTSELR::APORT1YCH1,
            34 => APORTOUTSELR::APORT1XCH2,
            35 => APORTOUTSELR::APORT1YCH3,
            36 => APORTOUTSELR::APORT1XCH4,
            37 => APORTOUTSELR::APORT1YCH5,
            38 => APORTOUTSELR::APORT1XCH6,
            39 => APORTOUTSELR::APORT1YCH7,
            40 => APORTOUTSELR::APORT1XCH8,
            41 => APORTOUTSELR::APORT1YCH9,
            42 => APORTOUTSELR::APORT1XCH10,
            43 => APORTOUTSELR::APORT1YCH11,
            44 => APORTOUTSELR::APORT1XCH12,
            45 => APORTOUTSELR::APORT1YCH13,
            46 => APORTOUTSELR::APORT1XCH14,
            47 => APORTOUTSELR::APORT1YCH15,
            48 => APORTOUTSELR::APORT1XCH16,
            49 => APORTOUTSELR::APORT1YCH17,
            50 => APORTOUTSELR::APORT1XCH18,
            51 => APORTOUTSELR::APORT1YCH19,
            52 => APORTOUTSELR::APORT1XCH20,
            53 => APORTOUTSELR::APORT1YCH21,
            54 => APORTOUTSELR::APORT1XCH22,
            55 => APORTOUTSELR::APORT1YCH23,
            56 => APORTOUTSELR::APORT1XCH24,
            57 => APORTOUTSELR::APORT1YCH25,
            58 => APORTOUTSELR::APORT1XCH26,
            59 => APORTOUTSELR::APORT1YCH27,
            60 => APORTOUTSELR::APORT1XCH28,
            61 => APORTOUTSELR::APORT1YCH29,
            62 => APORTOUTSELR::APORT1XCH30,
            63 => APORTOUTSELR::APORT1YCH31,
            i => APORTOUTSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT1XCH0`"]
    #[inline]
    pub fn is_aport1xch0(&self) -> bool {
        *self == APORTOUTSELR::APORT1XCH0
    }
    #[doc = "Checks if the value of the field is `APORT1YCH1`"]
    #[inline]
    pub fn is_aport1ych1(&self) -> bool {
        *self == APORTOUTSELR::APORT1YCH1
    }
    #[doc = "Checks if the value of the field is `APORT1XCH2`"]
    #[inline]
    pub fn is_aport1xch2(&self) -> bool {
        *self == APORTOUTSELR::APORT1XCH2
    }
    #[doc = "Checks if the value of the field is `APORT1YCH3`"]
    #[inline]
    pub fn is_aport1ych3(&self) -> bool {
        *self == APORTOUTSELR::APORT1YCH3
    }
    #[doc = "Checks if the value of the field is `APORT1XCH4`"]
    #[inline]
    pub fn is_aport1xch4(&self) -> bool {
        *self == APORTOUTSELR::APORT1XCH4
    }
    #[doc = "Checks if the value of the field is `APORT1YCH5`"]
    #[inline]
    pub fn is_aport1ych5(&self) -> bool {
        *self == APORTOUTSELR::APORT1YCH5
    }
    #[doc = "Checks if the value of the field is `APORT1XCH6`"]
    #[inline]
    pub fn is_aport1xch6(&self) -> bool {
        *self == APORTOUTSELR::APORT1XCH6
    }
    #[doc = "Checks if the value of the field is `APORT1YCH7`"]
    #[inline]
    pub fn is_aport1ych7(&self) -> bool {
        *self == APORTOUTSELR::APORT1YCH7
    }
    #[doc = "Checks if the value of the field is `APORT1XCH8`"]
    #[inline]
    pub fn is_aport1xch8(&self) -> bool {
        *self == APORTOUTSELR::APORT1XCH8
    }
    #[doc = "Checks if the value of the field is `APORT1YCH9`"]
    #[inline]
    pub fn is_aport1ych9(&self) -> bool {
        *self == APORTOUTSELR::APORT1YCH9
    }
    #[doc = "Checks if the value of the field is `APORT1XCH10`"]
    #[inline]
    pub fn is_aport1xch10(&self) -> bool {
        *self == APORTOUTSELR::APORT1XCH10
    }
    #[doc = "Checks if the value of the field is `APORT1YCH11`"]
    #[inline]
    pub fn is_aport1ych11(&self) -> bool {
        *self == APORTOUTSELR::APORT1YCH11
    }
    #[doc = "Checks if the value of the field is `APORT1XCH12`"]
    #[inline]
    pub fn is_aport1xch12(&self) -> bool {
        *self == APORTOUTSELR::APORT1XCH12
    }
    #[doc = "Checks if the value of the field is `APORT1YCH13`"]
    #[inline]
    pub fn is_aport1ych13(&self) -> bool {
        *self == APORTOUTSELR::APORT1YCH13
    }
    #[doc = "Checks if the value of the field is `APORT1XCH14`"]
    #[inline]
    pub fn is_aport1xch14(&self) -> bool {
        *self == APORTOUTSELR::APORT1XCH14
    }
    #[doc = "Checks if the value of the field is `APORT1YCH15`"]
    #[inline]
    pub fn is_aport1ych15(&self) -> bool {
        *self == APORTOUTSELR::APORT1YCH15
    }
    #[doc = "Checks if the value of the field is `APORT1XCH16`"]
    #[inline]
    pub fn is_aport1xch16(&self) -> bool {
        *self == APORTOUTSELR::APORT1XCH16
    }
    #[doc = "Checks if the value of the field is `APORT1YCH17`"]
    #[inline]
    pub fn is_aport1ych17(&self) -> bool {
        *self == APORTOUTSELR::APORT1YCH17
    }
    #[doc = "Checks if the value of the field is `APORT1XCH18`"]
    #[inline]
    pub fn is_aport1xch18(&self) -> bool {
        *self == APORTOUTSELR::APORT1XCH18
    }
    #[doc = "Checks if the value of the field is `APORT1YCH19`"]
    #[inline]
    pub fn is_aport1ych19(&self) -> bool {
        *self == APORTOUTSELR::APORT1YCH19
    }
    #[doc = "Checks if the value of the field is `APORT1XCH20`"]
    #[inline]
    pub fn is_aport1xch20(&self) -> bool {
        *self == APORTOUTSELR::APORT1XCH20
    }
    #[doc = "Checks if the value of the field is `APORT1YCH21`"]
    #[inline]
    pub fn is_aport1ych21(&self) -> bool {
        *self == APORTOUTSELR::APORT1YCH21
    }
    #[doc = "Checks if the value of the field is `APORT1XCH22`"]
    #[inline]
    pub fn is_aport1xch22(&self) -> bool {
        *self == APORTOUTSELR::APORT1XCH22
    }
    #[doc = "Checks if the value of the field is `APORT1YCH23`"]
    #[inline]
    pub fn is_aport1ych23(&self) -> bool {
        *self == APORTOUTSELR::APORT1YCH23
    }
    #[doc = "Checks if the value of the field is `APORT1XCH24`"]
    #[inline]
    pub fn is_aport1xch24(&self) -> bool {
        *self == APORTOUTSELR::APORT1XCH24
    }
    #[doc = "Checks if the value of the field is `APORT1YCH25`"]
    #[inline]
    pub fn is_aport1ych25(&self) -> bool {
        *self == APORTOUTSELR::APORT1YCH25
    }
    #[doc = "Checks if the value of the field is `APORT1XCH26`"]
    #[inline]
    pub fn is_aport1xch26(&self) -> bool {
        *self == APORTOUTSELR::APORT1XCH26
    }
    #[doc = "Checks if the value of the field is `APORT1YCH27`"]
    #[inline]
    pub fn is_aport1ych27(&self) -> bool {
        *self == APORTOUTSELR::APORT1YCH27
    }
    #[doc = "Checks if the value of the field is `APORT1XCH28`"]
    #[inline]
    pub fn is_aport1xch28(&self) -> bool {
        *self == APORTOUTSELR::APORT1XCH28
    }
    #[doc = "Checks if the value of the field is `APORT1YCH29`"]
    #[inline]
    pub fn is_aport1ych29(&self) -> bool {
        *self == APORTOUTSELR::APORT1YCH29
    }
    #[doc = "Checks if the value of the field is `APORT1XCH30`"]
    #[inline]
    pub fn is_aport1xch30(&self) -> bool {
        *self == APORTOUTSELR::APORT1XCH30
    }
    #[doc = "Checks if the value of the field is `APORT1YCH31`"]
    #[inline]
    pub fn is_aport1ych31(&self) -> bool {
        *self == APORTOUTSELR::APORT1YCH31
    }
}
#[doc = r" Value of the field"]
pub struct PWRSELR {
    bits: bool,
}
impl PWRSELR {
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
pub struct EM2DELAYR {
    bits: bool,
}
impl EM2DELAYR {
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
pub struct APORTMASTERDISR {
    bits: bool,
}
impl APORTMASTERDISR {
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
pub struct APORTOUTENPRSR {
    bits: bool,
}
impl APORTOUTENPRSR {
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
#[doc = "Possible values of the field `PRSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSSELR {
    #[doc = "PRS Channel 0 selected."]
    PRSCH0,
    #[doc = "PRS Channel 1 selected."]
    PRSCH1,
    #[doc = "PRS Channel 2 selected."]
    PRSCH2,
    #[doc = "PRS Channel 3 selected."]
    PRSCH3,
    #[doc = "PRS Channel 4 selected."]
    PRSCH4,
    #[doc = "PRS Channel 5 selected."]
    PRSCH5,
    #[doc = "PRS Channel 6 selected."]
    PRSCH6,
    #[doc = "PRS Channel 7 selected."]
    PRSCH7,
    #[doc = "PRS Channel 8 selected."]
    PRSCH8,
    #[doc = "PRS Channel 9 selected."]
    PRSCH9,
    #[doc = "PRS Channel 10 selected."]
    PRSCH10,
    #[doc = "PRS Channel 11 selected."]
    PRSCH11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRSSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRSSELR::PRSCH0 => 0,
            PRSSELR::PRSCH1 => 1,
            PRSSELR::PRSCH2 => 2,
            PRSSELR::PRSCH3 => 3,
            PRSSELR::PRSCH4 => 4,
            PRSSELR::PRSCH5 => 5,
            PRSSELR::PRSCH6 => 6,
            PRSSELR::PRSCH7 => 7,
            PRSSELR::PRSCH8 => 8,
            PRSSELR::PRSCH9 => 9,
            PRSSELR::PRSCH10 => 10,
            PRSSELR::PRSCH11 => 11,
            PRSSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRSSELR {
        match value {
            0 => PRSSELR::PRSCH0,
            1 => PRSSELR::PRSCH1,
            2 => PRSSELR::PRSCH2,
            3 => PRSSELR::PRSCH3,
            4 => PRSSELR::PRSCH4,
            5 => PRSSELR::PRSCH5,
            6 => PRSSELR::PRSCH6,
            7 => PRSSELR::PRSCH7,
            8 => PRSSELR::PRSCH8,
            9 => PRSSELR::PRSCH9,
            10 => PRSSELR::PRSCH10,
            11 => PRSSELR::PRSCH11,
            i => PRSSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSELR::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSELR::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSELR::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSELR::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSELR::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSELR::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSELR::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSELR::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSELR::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSELR::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSELR::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSELR::PRSCH11
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
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
pub struct _CURSINKW<'a> {
    w: &'a mut W,
}
impl<'a> _CURSINKW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MINOUTTRANSW<'a> {
    w: &'a mut W,
}
impl<'a> _MINOUTTRANSW<'a> {
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
pub struct _APORTOUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _APORTOUTENW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `APORTOUTSEL`"]
pub enum APORTOUTSELW {
    #[doc = "APORT1X Channel 0"]
    APORT1XCH0,
    #[doc = "APORT1Y Channel 1"]
    APORT1YCH1,
    #[doc = "APORT1X Channel 2"]
    APORT1XCH2,
    #[doc = "APORT1Y Channel 3"]
    APORT1YCH3,
    #[doc = "APORT1X Channel 4"]
    APORT1XCH4,
    #[doc = "APORT1Y Channel 5"]
    APORT1YCH5,
    #[doc = "APORT1X Channel 6"]
    APORT1XCH6,
    #[doc = "APORT1Y Channel 7"]
    APORT1YCH7,
    #[doc = "APORT1X Channel 8"]
    APORT1XCH8,
    #[doc = "APORT1Y Channel 9"]
    APORT1YCH9,
    #[doc = "APORT1X Channel 10"]
    APORT1XCH10,
    #[doc = "APORT1Y Channel 11"]
    APORT1YCH11,
    #[doc = "APORT1X Channel 12"]
    APORT1XCH12,
    #[doc = "APORT1Y Channel 13"]
    APORT1YCH13,
    #[doc = "APORT1X Channel 14"]
    APORT1XCH14,
    #[doc = "APORT1Y Channel 15"]
    APORT1YCH15,
    #[doc = "APORT1X Channel 16"]
    APORT1XCH16,
    #[doc = "APORT1Y Channel 17"]
    APORT1YCH17,
    #[doc = "APORT1X Channel 18"]
    APORT1XCH18,
    #[doc = "APORT1Y Channel 19"]
    APORT1YCH19,
    #[doc = "APORT1X Channel 20"]
    APORT1XCH20,
    #[doc = "APORT1Y Channel 21"]
    APORT1YCH21,
    #[doc = "APORT1X Channel 22"]
    APORT1XCH22,
    #[doc = "APORT1Y Channel 23"]
    APORT1YCH23,
    #[doc = "APORT1X Channel 24"]
    APORT1XCH24,
    #[doc = "APORT1Y Channel 25"]
    APORT1YCH25,
    #[doc = "APORT1X Channel 26"]
    APORT1XCH26,
    #[doc = "APORT1Y Channel 27"]
    APORT1YCH27,
    #[doc = "APORT1X Channel 28"]
    APORT1XCH28,
    #[doc = "APORT1Y Channel 29"]
    APORT1YCH29,
    #[doc = "APORT1X Channel 30"]
    APORT1XCH30,
    #[doc = "APORT1Y Channel 31"]
    APORT1YCH31,
}
impl APORTOUTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            APORTOUTSELW::APORT1XCH0 => 32,
            APORTOUTSELW::APORT1YCH1 => 33,
            APORTOUTSELW::APORT1XCH2 => 34,
            APORTOUTSELW::APORT1YCH3 => 35,
            APORTOUTSELW::APORT1XCH4 => 36,
            APORTOUTSELW::APORT1YCH5 => 37,
            APORTOUTSELW::APORT1XCH6 => 38,
            APORTOUTSELW::APORT1YCH7 => 39,
            APORTOUTSELW::APORT1XCH8 => 40,
            APORTOUTSELW::APORT1YCH9 => 41,
            APORTOUTSELW::APORT1XCH10 => 42,
            APORTOUTSELW::APORT1YCH11 => 43,
            APORTOUTSELW::APORT1XCH12 => 44,
            APORTOUTSELW::APORT1YCH13 => 45,
            APORTOUTSELW::APORT1XCH14 => 46,
            APORTOUTSELW::APORT1YCH15 => 47,
            APORTOUTSELW::APORT1XCH16 => 48,
            APORTOUTSELW::APORT1YCH17 => 49,
            APORTOUTSELW::APORT1XCH18 => 50,
            APORTOUTSELW::APORT1YCH19 => 51,
            APORTOUTSELW::APORT1XCH20 => 52,
            APORTOUTSELW::APORT1YCH21 => 53,
            APORTOUTSELW::APORT1XCH22 => 54,
            APORTOUTSELW::APORT1YCH23 => 55,
            APORTOUTSELW::APORT1XCH24 => 56,
            APORTOUTSELW::APORT1YCH25 => 57,
            APORTOUTSELW::APORT1XCH26 => 58,
            APORTOUTSELW::APORT1YCH27 => 59,
            APORTOUTSELW::APORT1XCH28 => 60,
            APORTOUTSELW::APORT1YCH29 => 61,
            APORTOUTSELW::APORT1XCH30 => 62,
            APORTOUTSELW::APORT1YCH31 => 63,
        }
    }
}
#[doc = r" Proxy"]
pub struct _APORTOUTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _APORTOUTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: APORTOUTSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "APORT1X Channel 0"]
    #[inline]
    pub fn aport1xch0(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1XCH0)
    }
    #[doc = "APORT1Y Channel 1"]
    #[inline]
    pub fn aport1ych1(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1YCH1)
    }
    #[doc = "APORT1X Channel 2"]
    #[inline]
    pub fn aport1xch2(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1XCH2)
    }
    #[doc = "APORT1Y Channel 3"]
    #[inline]
    pub fn aport1ych3(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1YCH3)
    }
    #[doc = "APORT1X Channel 4"]
    #[inline]
    pub fn aport1xch4(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1XCH4)
    }
    #[doc = "APORT1Y Channel 5"]
    #[inline]
    pub fn aport1ych5(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1YCH5)
    }
    #[doc = "APORT1X Channel 6"]
    #[inline]
    pub fn aport1xch6(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1XCH6)
    }
    #[doc = "APORT1Y Channel 7"]
    #[inline]
    pub fn aport1ych7(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1YCH7)
    }
    #[doc = "APORT1X Channel 8"]
    #[inline]
    pub fn aport1xch8(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1XCH8)
    }
    #[doc = "APORT1Y Channel 9"]
    #[inline]
    pub fn aport1ych9(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1YCH9)
    }
    #[doc = "APORT1X Channel 10"]
    #[inline]
    pub fn aport1xch10(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1XCH10)
    }
    #[doc = "APORT1Y Channel 11"]
    #[inline]
    pub fn aport1ych11(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1YCH11)
    }
    #[doc = "APORT1X Channel 12"]
    #[inline]
    pub fn aport1xch12(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1XCH12)
    }
    #[doc = "APORT1Y Channel 13"]
    #[inline]
    pub fn aport1ych13(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1YCH13)
    }
    #[doc = "APORT1X Channel 14"]
    #[inline]
    pub fn aport1xch14(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1XCH14)
    }
    #[doc = "APORT1Y Channel 15"]
    #[inline]
    pub fn aport1ych15(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1YCH15)
    }
    #[doc = "APORT1X Channel 16"]
    #[inline]
    pub fn aport1xch16(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1XCH16)
    }
    #[doc = "APORT1Y Channel 17"]
    #[inline]
    pub fn aport1ych17(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1YCH17)
    }
    #[doc = "APORT1X Channel 18"]
    #[inline]
    pub fn aport1xch18(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1XCH18)
    }
    #[doc = "APORT1Y Channel 19"]
    #[inline]
    pub fn aport1ych19(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1YCH19)
    }
    #[doc = "APORT1X Channel 20"]
    #[inline]
    pub fn aport1xch20(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1XCH20)
    }
    #[doc = "APORT1Y Channel 21"]
    #[inline]
    pub fn aport1ych21(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1YCH21)
    }
    #[doc = "APORT1X Channel 22"]
    #[inline]
    pub fn aport1xch22(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1XCH22)
    }
    #[doc = "APORT1Y Channel 23"]
    #[inline]
    pub fn aport1ych23(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1YCH23)
    }
    #[doc = "APORT1X Channel 24"]
    #[inline]
    pub fn aport1xch24(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1XCH24)
    }
    #[doc = "APORT1Y Channel 25"]
    #[inline]
    pub fn aport1ych25(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1YCH25)
    }
    #[doc = "APORT1X Channel 26"]
    #[inline]
    pub fn aport1xch26(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1XCH26)
    }
    #[doc = "APORT1Y Channel 27"]
    #[inline]
    pub fn aport1ych27(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1YCH27)
    }
    #[doc = "APORT1X Channel 28"]
    #[inline]
    pub fn aport1xch28(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1XCH28)
    }
    #[doc = "APORT1Y Channel 29"]
    #[inline]
    pub fn aport1ych29(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1YCH29)
    }
    #[doc = "APORT1X Channel 30"]
    #[inline]
    pub fn aport1xch30(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1XCH30)
    }
    #[doc = "APORT1Y Channel 31"]
    #[inline]
    pub fn aport1ych31(self) -> &'a mut W {
        self.variant(APORTOUTSELW::APORT1YCH31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PWRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRSELW<'a> {
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
pub struct _EM2DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _EM2DELAYW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _APORTMASTERDISW<'a> {
    w: &'a mut W,
}
impl<'a> _APORTMASTERDISW<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _APORTOUTENPRSW<'a> {
    w: &'a mut W,
}
impl<'a> _APORTOUTENPRSW<'a> {
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
#[doc = "Values that can be written to the field `PRSSEL`"]
pub enum PRSSELW {
    #[doc = "PRS Channel 0 selected."]
    PRSCH0,
    #[doc = "PRS Channel 1 selected."]
    PRSCH1,
    #[doc = "PRS Channel 2 selected."]
    PRSCH2,
    #[doc = "PRS Channel 3 selected."]
    PRSCH3,
    #[doc = "PRS Channel 4 selected."]
    PRSCH4,
    #[doc = "PRS Channel 5 selected."]
    PRSCH5,
    #[doc = "PRS Channel 6 selected."]
    PRSCH6,
    #[doc = "PRS Channel 7 selected."]
    PRSCH7,
    #[doc = "PRS Channel 8 selected."]
    PRSCH8,
    #[doc = "PRS Channel 9 selected."]
    PRSCH9,
    #[doc = "PRS Channel 10 selected."]
    PRSCH10,
    #[doc = "PRS Channel 11 selected."]
    PRSCH11,
}
impl PRSSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRSSELW::PRSCH0 => 0,
            PRSSELW::PRSCH1 => 1,
            PRSSELW::PRSCH2 => 2,
            PRSSELW::PRSCH3 => 3,
            PRSSELW::PRSCH4 => 4,
            PRSSELW::PRSCH5 => 5,
            PRSSELW::PRSCH6 => 6,
            PRSSELW::PRSCH7 => 7,
            PRSSELW::PRSCH8 => 8,
            PRSSELW::PRSCH9 => 9,
            PRSSELW::PRSCH10 => 10,
            PRSSELW::PRSCH11 => 11,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRSSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected."]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
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
    #[doc = "Bit 0 - Current DAC Enable"]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENR { bits }
    }
    #[doc = "Bit 1 - Current Sink Enable"]
    #[inline]
    pub fn cursink(&self) -> CURSINKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CURSINKR { bits }
    }
    #[doc = "Bit 2 - Minimum Output Transition Enable"]
    #[inline]
    pub fn minouttrans(&self) -> MINOUTTRANSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MINOUTTRANSR { bits }
    }
    #[doc = "Bit 3 - APORT Output Enable"]
    #[inline]
    pub fn aportouten(&self) -> APORTOUTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APORTOUTENR { bits }
    }
    #[doc = "Bits 4:11 - APORT Output Select"]
    #[inline]
    pub fn aportoutsel(&self) -> APORTOUTSELR {
        APORTOUTSELR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Power Select"]
    #[inline]
    pub fn pwrsel(&self) -> PWRSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PWRSELR { bits }
    }
    #[doc = "Bit 13 - EM2 Delay"]
    #[inline]
    pub fn em2delay(&self) -> EM2DELAYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EM2DELAYR { bits }
    }
    #[doc = "Bit 14 - APORT Bus Master Disable"]
    #[inline]
    pub fn aportmasterdis(&self) -> APORTMASTERDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APORTMASTERDISR { bits }
    }
    #[doc = "Bit 16 - PRS Controlled APORT Output Enable"]
    #[inline]
    pub fn aportoutenprs(&self) -> APORTOUTENPRSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APORTOUTENPRSR { bits }
    }
    #[doc = "Bits 20:23 - IDAC Output Enable PRS Channel Select"]
    #[inline]
    pub fn prssel(&self) -> PRSSELR {
        PRSSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
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
    #[doc = "Bit 0 - Current DAC Enable"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bit 1 - Current Sink Enable"]
    #[inline]
    pub fn cursink(&mut self) -> _CURSINKW {
        _CURSINKW { w: self }
    }
    #[doc = "Bit 2 - Minimum Output Transition Enable"]
    #[inline]
    pub fn minouttrans(&mut self) -> _MINOUTTRANSW {
        _MINOUTTRANSW { w: self }
    }
    #[doc = "Bit 3 - APORT Output Enable"]
    #[inline]
    pub fn aportouten(&mut self) -> _APORTOUTENW {
        _APORTOUTENW { w: self }
    }
    #[doc = "Bits 4:11 - APORT Output Select"]
    #[inline]
    pub fn aportoutsel(&mut self) -> _APORTOUTSELW {
        _APORTOUTSELW { w: self }
    }
    #[doc = "Bit 12 - Power Select"]
    #[inline]
    pub fn pwrsel(&mut self) -> _PWRSELW {
        _PWRSELW { w: self }
    }
    #[doc = "Bit 13 - EM2 Delay"]
    #[inline]
    pub fn em2delay(&mut self) -> _EM2DELAYW {
        _EM2DELAYW { w: self }
    }
    #[doc = "Bit 14 - APORT Bus Master Disable"]
    #[inline]
    pub fn aportmasterdis(&mut self) -> _APORTMASTERDISW {
        _APORTMASTERDISW { w: self }
    }
    #[doc = "Bit 16 - PRS Controlled APORT Output Enable"]
    #[inline]
    pub fn aportoutenprs(&mut self) -> _APORTOUTENPRSW {
        _APORTOUTENPRSW { w: self }
    }
    #[doc = "Bits 20:23 - IDAC Output Enable PRS Channel Select"]
    #[inline]
    pub fn prssel(&mut self) -> _PRSSELW {
        _PRSSELW { w: self }
    }
}
