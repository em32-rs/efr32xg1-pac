#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INPUTSEL {
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
pub struct POSSELR {
    bits: u8,
}
impl POSSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NEGSELR {
    bits: u8,
}
impl NEGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `VASEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VASELR {
    #[doc = "ACMPVDD"]
    VDD,
    #[doc = "APORT2Y Channel 0"]
    APORT2YCH0,
    #[doc = "APORT2Y Channel 2"]
    APORT2YCH2,
    #[doc = "APORT2Y Channel 4"]
    APORT2YCH4,
    #[doc = "APORT2Y Channel 6"]
    APORT2YCH6,
    #[doc = "APORT2Y Channel 8"]
    APORT2YCH8,
    #[doc = "APORT2Y Channel 10"]
    APORT2YCH10,
    #[doc = "APORT2Y Channel 12"]
    APORT2YCH12,
    #[doc = "APORT2Y Channel 14"]
    APORT2YCH14,
    #[doc = "APORT2Y Channel 16"]
    APORT2YCH16,
    #[doc = "APORT2Y Channel 18"]
    APORT2YCH18,
    #[doc = "APORT2Y Channel 20"]
    APORT2YCH20,
    #[doc = "APORT2Y Channel 22"]
    APORT2YCH22,
    #[doc = "APORT2Y Channel 24"]
    APORT2YCH24,
    #[doc = "APORT2Y Channel 26"]
    APORT2YCH26,
    #[doc = "APORT2Y Channel 28"]
    APORT2YCH28,
    #[doc = "APORT2Y Channel 30"]
    APORT2YCH30,
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
impl VASELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VASELR::VDD => 0,
            VASELR::APORT2YCH0 => 1,
            VASELR::APORT2YCH2 => 3,
            VASELR::APORT2YCH4 => 5,
            VASELR::APORT2YCH6 => 7,
            VASELR::APORT2YCH8 => 9,
            VASELR::APORT2YCH10 => 11,
            VASELR::APORT2YCH12 => 13,
            VASELR::APORT2YCH14 => 15,
            VASELR::APORT2YCH16 => 17,
            VASELR::APORT2YCH18 => 19,
            VASELR::APORT2YCH20 => 21,
            VASELR::APORT2YCH22 => 23,
            VASELR::APORT2YCH24 => 25,
            VASELR::APORT2YCH26 => 27,
            VASELR::APORT2YCH28 => 29,
            VASELR::APORT2YCH30 => 31,
            VASELR::APORT1XCH0 => 32,
            VASELR::APORT1YCH1 => 33,
            VASELR::APORT1XCH2 => 34,
            VASELR::APORT1YCH3 => 35,
            VASELR::APORT1XCH4 => 36,
            VASELR::APORT1YCH5 => 37,
            VASELR::APORT1XCH6 => 38,
            VASELR::APORT1YCH7 => 39,
            VASELR::APORT1XCH8 => 40,
            VASELR::APORT1YCH9 => 41,
            VASELR::APORT1XCH10 => 42,
            VASELR::APORT1YCH11 => 43,
            VASELR::APORT1XCH12 => 44,
            VASELR::APORT1YCH13 => 45,
            VASELR::APORT1XCH14 => 46,
            VASELR::APORT1YCH15 => 47,
            VASELR::APORT1XCH16 => 48,
            VASELR::APORT1YCH17 => 49,
            VASELR::APORT1XCH18 => 50,
            VASELR::APORT1YCH19 => 51,
            VASELR::APORT1XCH20 => 52,
            VASELR::APORT1YCH21 => 53,
            VASELR::APORT1XCH22 => 54,
            VASELR::APORT1YCH23 => 55,
            VASELR::APORT1XCH24 => 56,
            VASELR::APORT1YCH25 => 57,
            VASELR::APORT1XCH26 => 58,
            VASELR::APORT1YCH27 => 59,
            VASELR::APORT1XCH28 => 60,
            VASELR::APORT1YCH29 => 61,
            VASELR::APORT1XCH30 => 62,
            VASELR::APORT1YCH31 => 63,
            VASELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VASELR {
        match value {
            0 => VASELR::VDD,
            1 => VASELR::APORT2YCH0,
            3 => VASELR::APORT2YCH2,
            5 => VASELR::APORT2YCH4,
            7 => VASELR::APORT2YCH6,
            9 => VASELR::APORT2YCH8,
            11 => VASELR::APORT2YCH10,
            13 => VASELR::APORT2YCH12,
            15 => VASELR::APORT2YCH14,
            17 => VASELR::APORT2YCH16,
            19 => VASELR::APORT2YCH18,
            21 => VASELR::APORT2YCH20,
            23 => VASELR::APORT2YCH22,
            25 => VASELR::APORT2YCH24,
            27 => VASELR::APORT2YCH26,
            29 => VASELR::APORT2YCH28,
            31 => VASELR::APORT2YCH30,
            32 => VASELR::APORT1XCH0,
            33 => VASELR::APORT1YCH1,
            34 => VASELR::APORT1XCH2,
            35 => VASELR::APORT1YCH3,
            36 => VASELR::APORT1XCH4,
            37 => VASELR::APORT1YCH5,
            38 => VASELR::APORT1XCH6,
            39 => VASELR::APORT1YCH7,
            40 => VASELR::APORT1XCH8,
            41 => VASELR::APORT1YCH9,
            42 => VASELR::APORT1XCH10,
            43 => VASELR::APORT1YCH11,
            44 => VASELR::APORT1XCH12,
            45 => VASELR::APORT1YCH13,
            46 => VASELR::APORT1XCH14,
            47 => VASELR::APORT1YCH15,
            48 => VASELR::APORT1XCH16,
            49 => VASELR::APORT1YCH17,
            50 => VASELR::APORT1XCH18,
            51 => VASELR::APORT1YCH19,
            52 => VASELR::APORT1XCH20,
            53 => VASELR::APORT1YCH21,
            54 => VASELR::APORT1XCH22,
            55 => VASELR::APORT1YCH23,
            56 => VASELR::APORT1XCH24,
            57 => VASELR::APORT1YCH25,
            58 => VASELR::APORT1XCH26,
            59 => VASELR::APORT1YCH27,
            60 => VASELR::APORT1XCH28,
            61 => VASELR::APORT1YCH29,
            62 => VASELR::APORT1XCH30,
            63 => VASELR::APORT1YCH31,
            i => VASELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline]
    pub fn is_vdd(&self) -> bool {
        *self == VASELR::VDD
    }
    #[doc = "Checks if the value of the field is `APORT2YCH0`"]
    #[inline]
    pub fn is_aport2ych0(&self) -> bool {
        *self == VASELR::APORT2YCH0
    }
    #[doc = "Checks if the value of the field is `APORT2YCH2`"]
    #[inline]
    pub fn is_aport2ych2(&self) -> bool {
        *self == VASELR::APORT2YCH2
    }
    #[doc = "Checks if the value of the field is `APORT2YCH4`"]
    #[inline]
    pub fn is_aport2ych4(&self) -> bool {
        *self == VASELR::APORT2YCH4
    }
    #[doc = "Checks if the value of the field is `APORT2YCH6`"]
    #[inline]
    pub fn is_aport2ych6(&self) -> bool {
        *self == VASELR::APORT2YCH6
    }
    #[doc = "Checks if the value of the field is `APORT2YCH8`"]
    #[inline]
    pub fn is_aport2ych8(&self) -> bool {
        *self == VASELR::APORT2YCH8
    }
    #[doc = "Checks if the value of the field is `APORT2YCH10`"]
    #[inline]
    pub fn is_aport2ych10(&self) -> bool {
        *self == VASELR::APORT2YCH10
    }
    #[doc = "Checks if the value of the field is `APORT2YCH12`"]
    #[inline]
    pub fn is_aport2ych12(&self) -> bool {
        *self == VASELR::APORT2YCH12
    }
    #[doc = "Checks if the value of the field is `APORT2YCH14`"]
    #[inline]
    pub fn is_aport2ych14(&self) -> bool {
        *self == VASELR::APORT2YCH14
    }
    #[doc = "Checks if the value of the field is `APORT2YCH16`"]
    #[inline]
    pub fn is_aport2ych16(&self) -> bool {
        *self == VASELR::APORT2YCH16
    }
    #[doc = "Checks if the value of the field is `APORT2YCH18`"]
    #[inline]
    pub fn is_aport2ych18(&self) -> bool {
        *self == VASELR::APORT2YCH18
    }
    #[doc = "Checks if the value of the field is `APORT2YCH20`"]
    #[inline]
    pub fn is_aport2ych20(&self) -> bool {
        *self == VASELR::APORT2YCH20
    }
    #[doc = "Checks if the value of the field is `APORT2YCH22`"]
    #[inline]
    pub fn is_aport2ych22(&self) -> bool {
        *self == VASELR::APORT2YCH22
    }
    #[doc = "Checks if the value of the field is `APORT2YCH24`"]
    #[inline]
    pub fn is_aport2ych24(&self) -> bool {
        *self == VASELR::APORT2YCH24
    }
    #[doc = "Checks if the value of the field is `APORT2YCH26`"]
    #[inline]
    pub fn is_aport2ych26(&self) -> bool {
        *self == VASELR::APORT2YCH26
    }
    #[doc = "Checks if the value of the field is `APORT2YCH28`"]
    #[inline]
    pub fn is_aport2ych28(&self) -> bool {
        *self == VASELR::APORT2YCH28
    }
    #[doc = "Checks if the value of the field is `APORT2YCH30`"]
    #[inline]
    pub fn is_aport2ych30(&self) -> bool {
        *self == VASELR::APORT2YCH30
    }
    #[doc = "Checks if the value of the field is `APORT1XCH0`"]
    #[inline]
    pub fn is_aport1xch0(&self) -> bool {
        *self == VASELR::APORT1XCH0
    }
    #[doc = "Checks if the value of the field is `APORT1YCH1`"]
    #[inline]
    pub fn is_aport1ych1(&self) -> bool {
        *self == VASELR::APORT1YCH1
    }
    #[doc = "Checks if the value of the field is `APORT1XCH2`"]
    #[inline]
    pub fn is_aport1xch2(&self) -> bool {
        *self == VASELR::APORT1XCH2
    }
    #[doc = "Checks if the value of the field is `APORT1YCH3`"]
    #[inline]
    pub fn is_aport1ych3(&self) -> bool {
        *self == VASELR::APORT1YCH3
    }
    #[doc = "Checks if the value of the field is `APORT1XCH4`"]
    #[inline]
    pub fn is_aport1xch4(&self) -> bool {
        *self == VASELR::APORT1XCH4
    }
    #[doc = "Checks if the value of the field is `APORT1YCH5`"]
    #[inline]
    pub fn is_aport1ych5(&self) -> bool {
        *self == VASELR::APORT1YCH5
    }
    #[doc = "Checks if the value of the field is `APORT1XCH6`"]
    #[inline]
    pub fn is_aport1xch6(&self) -> bool {
        *self == VASELR::APORT1XCH6
    }
    #[doc = "Checks if the value of the field is `APORT1YCH7`"]
    #[inline]
    pub fn is_aport1ych7(&self) -> bool {
        *self == VASELR::APORT1YCH7
    }
    #[doc = "Checks if the value of the field is `APORT1XCH8`"]
    #[inline]
    pub fn is_aport1xch8(&self) -> bool {
        *self == VASELR::APORT1XCH8
    }
    #[doc = "Checks if the value of the field is `APORT1YCH9`"]
    #[inline]
    pub fn is_aport1ych9(&self) -> bool {
        *self == VASELR::APORT1YCH9
    }
    #[doc = "Checks if the value of the field is `APORT1XCH10`"]
    #[inline]
    pub fn is_aport1xch10(&self) -> bool {
        *self == VASELR::APORT1XCH10
    }
    #[doc = "Checks if the value of the field is `APORT1YCH11`"]
    #[inline]
    pub fn is_aport1ych11(&self) -> bool {
        *self == VASELR::APORT1YCH11
    }
    #[doc = "Checks if the value of the field is `APORT1XCH12`"]
    #[inline]
    pub fn is_aport1xch12(&self) -> bool {
        *self == VASELR::APORT1XCH12
    }
    #[doc = "Checks if the value of the field is `APORT1YCH13`"]
    #[inline]
    pub fn is_aport1ych13(&self) -> bool {
        *self == VASELR::APORT1YCH13
    }
    #[doc = "Checks if the value of the field is `APORT1XCH14`"]
    #[inline]
    pub fn is_aport1xch14(&self) -> bool {
        *self == VASELR::APORT1XCH14
    }
    #[doc = "Checks if the value of the field is `APORT1YCH15`"]
    #[inline]
    pub fn is_aport1ych15(&self) -> bool {
        *self == VASELR::APORT1YCH15
    }
    #[doc = "Checks if the value of the field is `APORT1XCH16`"]
    #[inline]
    pub fn is_aport1xch16(&self) -> bool {
        *self == VASELR::APORT1XCH16
    }
    #[doc = "Checks if the value of the field is `APORT1YCH17`"]
    #[inline]
    pub fn is_aport1ych17(&self) -> bool {
        *self == VASELR::APORT1YCH17
    }
    #[doc = "Checks if the value of the field is `APORT1XCH18`"]
    #[inline]
    pub fn is_aport1xch18(&self) -> bool {
        *self == VASELR::APORT1XCH18
    }
    #[doc = "Checks if the value of the field is `APORT1YCH19`"]
    #[inline]
    pub fn is_aport1ych19(&self) -> bool {
        *self == VASELR::APORT1YCH19
    }
    #[doc = "Checks if the value of the field is `APORT1XCH20`"]
    #[inline]
    pub fn is_aport1xch20(&self) -> bool {
        *self == VASELR::APORT1XCH20
    }
    #[doc = "Checks if the value of the field is `APORT1YCH21`"]
    #[inline]
    pub fn is_aport1ych21(&self) -> bool {
        *self == VASELR::APORT1YCH21
    }
    #[doc = "Checks if the value of the field is `APORT1XCH22`"]
    #[inline]
    pub fn is_aport1xch22(&self) -> bool {
        *self == VASELR::APORT1XCH22
    }
    #[doc = "Checks if the value of the field is `APORT1YCH23`"]
    #[inline]
    pub fn is_aport1ych23(&self) -> bool {
        *self == VASELR::APORT1YCH23
    }
    #[doc = "Checks if the value of the field is `APORT1XCH24`"]
    #[inline]
    pub fn is_aport1xch24(&self) -> bool {
        *self == VASELR::APORT1XCH24
    }
    #[doc = "Checks if the value of the field is `APORT1YCH25`"]
    #[inline]
    pub fn is_aport1ych25(&self) -> bool {
        *self == VASELR::APORT1YCH25
    }
    #[doc = "Checks if the value of the field is `APORT1XCH26`"]
    #[inline]
    pub fn is_aport1xch26(&self) -> bool {
        *self == VASELR::APORT1XCH26
    }
    #[doc = "Checks if the value of the field is `APORT1YCH27`"]
    #[inline]
    pub fn is_aport1ych27(&self) -> bool {
        *self == VASELR::APORT1YCH27
    }
    #[doc = "Checks if the value of the field is `APORT1XCH28`"]
    #[inline]
    pub fn is_aport1xch28(&self) -> bool {
        *self == VASELR::APORT1XCH28
    }
    #[doc = "Checks if the value of the field is `APORT1YCH29`"]
    #[inline]
    pub fn is_aport1ych29(&self) -> bool {
        *self == VASELR::APORT1YCH29
    }
    #[doc = "Checks if the value of the field is `APORT1XCH30`"]
    #[inline]
    pub fn is_aport1xch30(&self) -> bool {
        *self == VASELR::APORT1XCH30
    }
    #[doc = "Checks if the value of the field is `APORT1YCH31`"]
    #[inline]
    pub fn is_aport1ych31(&self) -> bool {
        *self == VASELR::APORT1YCH31
    }
}
#[doc = r" Value of the field"]
pub struct VBSELR {
    bits: bool,
}
impl VBSELR {
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
pub struct VLPSELR {
    bits: bool,
}
impl VLPSELR {
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
pub struct CSRESENR {
    bits: bool,
}
impl CSRESENR {
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
#[doc = "Possible values of the field `CSRESSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSRESSELR {
    #[doc = "Internal capacitive sense resistor value 0"]
    RES0,
    #[doc = "Internal capacitive sense resistor value 1"]
    RES1,
    #[doc = "Internal capacitive sense resistor value 2"]
    RES2,
    #[doc = "Internal capacitive sense resistor value 3"]
    RES3,
    #[doc = "Internal capacitive sense resistor value 4"]
    RES4,
    #[doc = "Internal capacitive sense resistor value 5"]
    RES5,
    #[doc = "Internal capacitive sense resistor value 6"]
    RES6,
    #[doc = "Internal capacitive sense resistor value 7"]
    RES7,
}
impl CSRESSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSRESSELR::RES0 => 0,
            CSRESSELR::RES1 => 1,
            CSRESSELR::RES2 => 2,
            CSRESSELR::RES3 => 3,
            CSRESSELR::RES4 => 4,
            CSRESSELR::RES5 => 5,
            CSRESSELR::RES6 => 6,
            CSRESSELR::RES7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSRESSELR {
        match value {
            0 => CSRESSELR::RES0,
            1 => CSRESSELR::RES1,
            2 => CSRESSELR::RES2,
            3 => CSRESSELR::RES3,
            4 => CSRESSELR::RES4,
            5 => CSRESSELR::RES5,
            6 => CSRESSELR::RES6,
            7 => CSRESSELR::RES7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RES0`"]
    #[inline]
    pub fn is_res0(&self) -> bool {
        *self == CSRESSELR::RES0
    }
    #[doc = "Checks if the value of the field is `RES1`"]
    #[inline]
    pub fn is_res1(&self) -> bool {
        *self == CSRESSELR::RES1
    }
    #[doc = "Checks if the value of the field is `RES2`"]
    #[inline]
    pub fn is_res2(&self) -> bool {
        *self == CSRESSELR::RES2
    }
    #[doc = "Checks if the value of the field is `RES3`"]
    #[inline]
    pub fn is_res3(&self) -> bool {
        *self == CSRESSELR::RES3
    }
    #[doc = "Checks if the value of the field is `RES4`"]
    #[inline]
    pub fn is_res4(&self) -> bool {
        *self == CSRESSELR::RES4
    }
    #[doc = "Checks if the value of the field is `RES5`"]
    #[inline]
    pub fn is_res5(&self) -> bool {
        *self == CSRESSELR::RES5
    }
    #[doc = "Checks if the value of the field is `RES6`"]
    #[inline]
    pub fn is_res6(&self) -> bool {
        *self == CSRESSELR::RES6
    }
    #[doc = "Checks if the value of the field is `RES7`"]
    #[inline]
    pub fn is_res7(&self) -> bool {
        *self == CSRESSELR::RES7
    }
}
#[doc = r" Proxy"]
pub struct _POSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _POSSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NEGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _NEGSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VASEL`"]
pub enum VASELW {
    #[doc = "ACMPVDD"]
    VDD,
    #[doc = "APORT2Y Channel 0"]
    APORT2YCH0,
    #[doc = "APORT2Y Channel 2"]
    APORT2YCH2,
    #[doc = "APORT2Y Channel 4"]
    APORT2YCH4,
    #[doc = "APORT2Y Channel 6"]
    APORT2YCH6,
    #[doc = "APORT2Y Channel 8"]
    APORT2YCH8,
    #[doc = "APORT2Y Channel 10"]
    APORT2YCH10,
    #[doc = "APORT2Y Channel 12"]
    APORT2YCH12,
    #[doc = "APORT2Y Channel 14"]
    APORT2YCH14,
    #[doc = "APORT2Y Channel 16"]
    APORT2YCH16,
    #[doc = "APORT2Y Channel 18"]
    APORT2YCH18,
    #[doc = "APORT2Y Channel 20"]
    APORT2YCH20,
    #[doc = "APORT2Y Channel 22"]
    APORT2YCH22,
    #[doc = "APORT2Y Channel 24"]
    APORT2YCH24,
    #[doc = "APORT2Y Channel 26"]
    APORT2YCH26,
    #[doc = "APORT2Y Channel 28"]
    APORT2YCH28,
    #[doc = "APORT2Y Channel 30"]
    APORT2YCH30,
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
impl VASELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VASELW::VDD => 0,
            VASELW::APORT2YCH0 => 1,
            VASELW::APORT2YCH2 => 3,
            VASELW::APORT2YCH4 => 5,
            VASELW::APORT2YCH6 => 7,
            VASELW::APORT2YCH8 => 9,
            VASELW::APORT2YCH10 => 11,
            VASELW::APORT2YCH12 => 13,
            VASELW::APORT2YCH14 => 15,
            VASELW::APORT2YCH16 => 17,
            VASELW::APORT2YCH18 => 19,
            VASELW::APORT2YCH20 => 21,
            VASELW::APORT2YCH22 => 23,
            VASELW::APORT2YCH24 => 25,
            VASELW::APORT2YCH26 => 27,
            VASELW::APORT2YCH28 => 29,
            VASELW::APORT2YCH30 => 31,
            VASELW::APORT1XCH0 => 32,
            VASELW::APORT1YCH1 => 33,
            VASELW::APORT1XCH2 => 34,
            VASELW::APORT1YCH3 => 35,
            VASELW::APORT1XCH4 => 36,
            VASELW::APORT1YCH5 => 37,
            VASELW::APORT1XCH6 => 38,
            VASELW::APORT1YCH7 => 39,
            VASELW::APORT1XCH8 => 40,
            VASELW::APORT1YCH9 => 41,
            VASELW::APORT1XCH10 => 42,
            VASELW::APORT1YCH11 => 43,
            VASELW::APORT1XCH12 => 44,
            VASELW::APORT1YCH13 => 45,
            VASELW::APORT1XCH14 => 46,
            VASELW::APORT1YCH15 => 47,
            VASELW::APORT1XCH16 => 48,
            VASELW::APORT1YCH17 => 49,
            VASELW::APORT1XCH18 => 50,
            VASELW::APORT1YCH19 => 51,
            VASELW::APORT1XCH20 => 52,
            VASELW::APORT1YCH21 => 53,
            VASELW::APORT1XCH22 => 54,
            VASELW::APORT1YCH23 => 55,
            VASELW::APORT1XCH24 => 56,
            VASELW::APORT1YCH25 => 57,
            VASELW::APORT1XCH26 => 58,
            VASELW::APORT1YCH27 => 59,
            VASELW::APORT1XCH28 => 60,
            VASELW::APORT1YCH29 => 61,
            VASELW::APORT1XCH30 => 62,
            VASELW::APORT1YCH31 => 63,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VASELW<'a> {
    w: &'a mut W,
}
impl<'a> _VASELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VASELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ACMPVDD"]
    #[inline]
    pub fn vdd(self) -> &'a mut W {
        self.variant(VASELW::VDD)
    }
    #[doc = "APORT2Y Channel 0"]
    #[inline]
    pub fn aport2ych0(self) -> &'a mut W {
        self.variant(VASELW::APORT2YCH0)
    }
    #[doc = "APORT2Y Channel 2"]
    #[inline]
    pub fn aport2ych2(self) -> &'a mut W {
        self.variant(VASELW::APORT2YCH2)
    }
    #[doc = "APORT2Y Channel 4"]
    #[inline]
    pub fn aport2ych4(self) -> &'a mut W {
        self.variant(VASELW::APORT2YCH4)
    }
    #[doc = "APORT2Y Channel 6"]
    #[inline]
    pub fn aport2ych6(self) -> &'a mut W {
        self.variant(VASELW::APORT2YCH6)
    }
    #[doc = "APORT2Y Channel 8"]
    #[inline]
    pub fn aport2ych8(self) -> &'a mut W {
        self.variant(VASELW::APORT2YCH8)
    }
    #[doc = "APORT2Y Channel 10"]
    #[inline]
    pub fn aport2ych10(self) -> &'a mut W {
        self.variant(VASELW::APORT2YCH10)
    }
    #[doc = "APORT2Y Channel 12"]
    #[inline]
    pub fn aport2ych12(self) -> &'a mut W {
        self.variant(VASELW::APORT2YCH12)
    }
    #[doc = "APORT2Y Channel 14"]
    #[inline]
    pub fn aport2ych14(self) -> &'a mut W {
        self.variant(VASELW::APORT2YCH14)
    }
    #[doc = "APORT2Y Channel 16"]
    #[inline]
    pub fn aport2ych16(self) -> &'a mut W {
        self.variant(VASELW::APORT2YCH16)
    }
    #[doc = "APORT2Y Channel 18"]
    #[inline]
    pub fn aport2ych18(self) -> &'a mut W {
        self.variant(VASELW::APORT2YCH18)
    }
    #[doc = "APORT2Y Channel 20"]
    #[inline]
    pub fn aport2ych20(self) -> &'a mut W {
        self.variant(VASELW::APORT2YCH20)
    }
    #[doc = "APORT2Y Channel 22"]
    #[inline]
    pub fn aport2ych22(self) -> &'a mut W {
        self.variant(VASELW::APORT2YCH22)
    }
    #[doc = "APORT2Y Channel 24"]
    #[inline]
    pub fn aport2ych24(self) -> &'a mut W {
        self.variant(VASELW::APORT2YCH24)
    }
    #[doc = "APORT2Y Channel 26"]
    #[inline]
    pub fn aport2ych26(self) -> &'a mut W {
        self.variant(VASELW::APORT2YCH26)
    }
    #[doc = "APORT2Y Channel 28"]
    #[inline]
    pub fn aport2ych28(self) -> &'a mut W {
        self.variant(VASELW::APORT2YCH28)
    }
    #[doc = "APORT2Y Channel 30"]
    #[inline]
    pub fn aport2ych30(self) -> &'a mut W {
        self.variant(VASELW::APORT2YCH30)
    }
    #[doc = "APORT1X Channel 0"]
    #[inline]
    pub fn aport1xch0(self) -> &'a mut W {
        self.variant(VASELW::APORT1XCH0)
    }
    #[doc = "APORT1Y Channel 1"]
    #[inline]
    pub fn aport1ych1(self) -> &'a mut W {
        self.variant(VASELW::APORT1YCH1)
    }
    #[doc = "APORT1X Channel 2"]
    #[inline]
    pub fn aport1xch2(self) -> &'a mut W {
        self.variant(VASELW::APORT1XCH2)
    }
    #[doc = "APORT1Y Channel 3"]
    #[inline]
    pub fn aport1ych3(self) -> &'a mut W {
        self.variant(VASELW::APORT1YCH3)
    }
    #[doc = "APORT1X Channel 4"]
    #[inline]
    pub fn aport1xch4(self) -> &'a mut W {
        self.variant(VASELW::APORT1XCH4)
    }
    #[doc = "APORT1Y Channel 5"]
    #[inline]
    pub fn aport1ych5(self) -> &'a mut W {
        self.variant(VASELW::APORT1YCH5)
    }
    #[doc = "APORT1X Channel 6"]
    #[inline]
    pub fn aport1xch6(self) -> &'a mut W {
        self.variant(VASELW::APORT1XCH6)
    }
    #[doc = "APORT1Y Channel 7"]
    #[inline]
    pub fn aport1ych7(self) -> &'a mut W {
        self.variant(VASELW::APORT1YCH7)
    }
    #[doc = "APORT1X Channel 8"]
    #[inline]
    pub fn aport1xch8(self) -> &'a mut W {
        self.variant(VASELW::APORT1XCH8)
    }
    #[doc = "APORT1Y Channel 9"]
    #[inline]
    pub fn aport1ych9(self) -> &'a mut W {
        self.variant(VASELW::APORT1YCH9)
    }
    #[doc = "APORT1X Channel 10"]
    #[inline]
    pub fn aport1xch10(self) -> &'a mut W {
        self.variant(VASELW::APORT1XCH10)
    }
    #[doc = "APORT1Y Channel 11"]
    #[inline]
    pub fn aport1ych11(self) -> &'a mut W {
        self.variant(VASELW::APORT1YCH11)
    }
    #[doc = "APORT1X Channel 12"]
    #[inline]
    pub fn aport1xch12(self) -> &'a mut W {
        self.variant(VASELW::APORT1XCH12)
    }
    #[doc = "APORT1Y Channel 13"]
    #[inline]
    pub fn aport1ych13(self) -> &'a mut W {
        self.variant(VASELW::APORT1YCH13)
    }
    #[doc = "APORT1X Channel 14"]
    #[inline]
    pub fn aport1xch14(self) -> &'a mut W {
        self.variant(VASELW::APORT1XCH14)
    }
    #[doc = "APORT1Y Channel 15"]
    #[inline]
    pub fn aport1ych15(self) -> &'a mut W {
        self.variant(VASELW::APORT1YCH15)
    }
    #[doc = "APORT1X Channel 16"]
    #[inline]
    pub fn aport1xch16(self) -> &'a mut W {
        self.variant(VASELW::APORT1XCH16)
    }
    #[doc = "APORT1Y Channel 17"]
    #[inline]
    pub fn aport1ych17(self) -> &'a mut W {
        self.variant(VASELW::APORT1YCH17)
    }
    #[doc = "APORT1X Channel 18"]
    #[inline]
    pub fn aport1xch18(self) -> &'a mut W {
        self.variant(VASELW::APORT1XCH18)
    }
    #[doc = "APORT1Y Channel 19"]
    #[inline]
    pub fn aport1ych19(self) -> &'a mut W {
        self.variant(VASELW::APORT1YCH19)
    }
    #[doc = "APORT1X Channel 20"]
    #[inline]
    pub fn aport1xch20(self) -> &'a mut W {
        self.variant(VASELW::APORT1XCH20)
    }
    #[doc = "APORT1Y Channel 21"]
    #[inline]
    pub fn aport1ych21(self) -> &'a mut W {
        self.variant(VASELW::APORT1YCH21)
    }
    #[doc = "APORT1X Channel 22"]
    #[inline]
    pub fn aport1xch22(self) -> &'a mut W {
        self.variant(VASELW::APORT1XCH22)
    }
    #[doc = "APORT1Y Channel 23"]
    #[inline]
    pub fn aport1ych23(self) -> &'a mut W {
        self.variant(VASELW::APORT1YCH23)
    }
    #[doc = "APORT1X Channel 24"]
    #[inline]
    pub fn aport1xch24(self) -> &'a mut W {
        self.variant(VASELW::APORT1XCH24)
    }
    #[doc = "APORT1Y Channel 25"]
    #[inline]
    pub fn aport1ych25(self) -> &'a mut W {
        self.variant(VASELW::APORT1YCH25)
    }
    #[doc = "APORT1X Channel 26"]
    #[inline]
    pub fn aport1xch26(self) -> &'a mut W {
        self.variant(VASELW::APORT1XCH26)
    }
    #[doc = "APORT1Y Channel 27"]
    #[inline]
    pub fn aport1ych27(self) -> &'a mut W {
        self.variant(VASELW::APORT1YCH27)
    }
    #[doc = "APORT1X Channel 28"]
    #[inline]
    pub fn aport1xch28(self) -> &'a mut W {
        self.variant(VASELW::APORT1XCH28)
    }
    #[doc = "APORT1Y Channel 29"]
    #[inline]
    pub fn aport1ych29(self) -> &'a mut W {
        self.variant(VASELW::APORT1YCH29)
    }
    #[doc = "APORT1X Channel 30"]
    #[inline]
    pub fn aport1xch30(self) -> &'a mut W {
        self.variant(VASELW::APORT1XCH30)
    }
    #[doc = "APORT1Y Channel 31"]
    #[inline]
    pub fn aport1ych31(self) -> &'a mut W {
        self.variant(VASELW::APORT1YCH31)
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
#[doc = r" Proxy"]
pub struct _VBSELW<'a> {
    w: &'a mut W,
}
impl<'a> _VBSELW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VLPSELW<'a> {
    w: &'a mut W,
}
impl<'a> _VLPSELW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CSRESENW<'a> {
    w: &'a mut W,
}
impl<'a> _CSRESENW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CSRESSEL`"]
pub enum CSRESSELW {
    #[doc = "Internal capacitive sense resistor value 0"]
    RES0,
    #[doc = "Internal capacitive sense resistor value 1"]
    RES1,
    #[doc = "Internal capacitive sense resistor value 2"]
    RES2,
    #[doc = "Internal capacitive sense resistor value 3"]
    RES3,
    #[doc = "Internal capacitive sense resistor value 4"]
    RES4,
    #[doc = "Internal capacitive sense resistor value 5"]
    RES5,
    #[doc = "Internal capacitive sense resistor value 6"]
    RES6,
    #[doc = "Internal capacitive sense resistor value 7"]
    RES7,
}
impl CSRESSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSRESSELW::RES0 => 0,
            CSRESSELW::RES1 => 1,
            CSRESSELW::RES2 => 2,
            CSRESSELW::RES3 => 3,
            CSRESSELW::RES4 => 4,
            CSRESSELW::RES5 => 5,
            CSRESSELW::RES6 => 6,
            CSRESSELW::RES7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSRESSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CSRESSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSRESSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Internal capacitive sense resistor value 0"]
    #[inline]
    pub fn res0(self) -> &'a mut W {
        self.variant(CSRESSELW::RES0)
    }
    #[doc = "Internal capacitive sense resistor value 1"]
    #[inline]
    pub fn res1(self) -> &'a mut W {
        self.variant(CSRESSELW::RES1)
    }
    #[doc = "Internal capacitive sense resistor value 2"]
    #[inline]
    pub fn res2(self) -> &'a mut W {
        self.variant(CSRESSELW::RES2)
    }
    #[doc = "Internal capacitive sense resistor value 3"]
    #[inline]
    pub fn res3(self) -> &'a mut W {
        self.variant(CSRESSELW::RES3)
    }
    #[doc = "Internal capacitive sense resistor value 4"]
    #[inline]
    pub fn res4(self) -> &'a mut W {
        self.variant(CSRESSELW::RES4)
    }
    #[doc = "Internal capacitive sense resistor value 5"]
    #[inline]
    pub fn res5(self) -> &'a mut W {
        self.variant(CSRESSELW::RES5)
    }
    #[doc = "Internal capacitive sense resistor value 6"]
    #[inline]
    pub fn res6(self) -> &'a mut W {
        self.variant(CSRESSELW::RES6)
    }
    #[doc = "Internal capacitive sense resistor value 7"]
    #[inline]
    pub fn res7(self) -> &'a mut W {
        self.variant(CSRESSELW::RES7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 0:7 - Positive Input Select"]
    #[inline]
    pub fn possel(&self) -> POSSELR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        POSSELR { bits }
    }
    #[doc = "Bits 8:15 - Negative Input Select"]
    #[inline]
    pub fn negsel(&self) -> NEGSELR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NEGSELR { bits }
    }
    #[doc = "Bits 16:21 - VA Selection"]
    #[inline]
    pub fn vasel(&self) -> VASELR {
        VASELR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - VB Selection"]
    #[inline]
    pub fn vbsel(&self) -> VBSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VBSELR { bits }
    }
    #[doc = "Bit 24 - Low-Power Sampled Voltage Selection"]
    #[inline]
    pub fn vlpsel(&self) -> VLPSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VLPSELR { bits }
    }
    #[doc = "Bit 26 - Capacitive Sense Mode Internal Resistor Enable"]
    #[inline]
    pub fn csresen(&self) -> CSRESENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSRESENR { bits }
    }
    #[doc = "Bits 28:30 - Capacitive Sense Mode Internal Resistor Select"]
    #[inline]
    pub fn csressel(&self) -> CSRESSELR {
        CSRESSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:7 - Positive Input Select"]
    #[inline]
    pub fn possel(&mut self) -> _POSSELW {
        _POSSELW { w: self }
    }
    #[doc = "Bits 8:15 - Negative Input Select"]
    #[inline]
    pub fn negsel(&mut self) -> _NEGSELW {
        _NEGSELW { w: self }
    }
    #[doc = "Bits 16:21 - VA Selection"]
    #[inline]
    pub fn vasel(&mut self) -> _VASELW {
        _VASELW { w: self }
    }
    #[doc = "Bit 22 - VB Selection"]
    #[inline]
    pub fn vbsel(&mut self) -> _VBSELW {
        _VBSELW { w: self }
    }
    #[doc = "Bit 24 - Low-Power Sampled Voltage Selection"]
    #[inline]
    pub fn vlpsel(&mut self) -> _VLPSELW {
        _VLPSELW { w: self }
    }
    #[doc = "Bit 26 - Capacitive Sense Mode Internal Resistor Enable"]
    #[inline]
    pub fn csresen(&mut self) -> _CSRESENW {
        _CSRESENW { w: self }
    }
    #[doc = "Bits 28:30 - Capacitive Sense Mode Internal Resistor Select"]
    #[inline]
    pub fn csressel(&mut self) -> _CSRESSELW {
        _CSRESSELW { w: self }
    }
}
