#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRSSEL {
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
#[doc = "Possible values of the field `PRSSTARTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSSTARTSELR {
    #[doc = "PRS Channel 0 selected as input"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected as input"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected as input"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected as input"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected as input"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected as input"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected as input"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected as input"]
    PRSCH7,
    #[doc = "PRS Channel 8 selected as input"]
    PRSCH8,
    #[doc = "PRS Channel 9 selected as input"]
    PRSCH9,
    #[doc = "PRS Channel 10 selected as input"]
    PRSCH10,
    #[doc = "PRS Channel 11 selected as input"]
    PRSCH11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRSSTARTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRSSTARTSELR::PRSCH0 => 0,
            PRSSTARTSELR::PRSCH1 => 1,
            PRSSTARTSELR::PRSCH2 => 2,
            PRSSTARTSELR::PRSCH3 => 3,
            PRSSTARTSELR::PRSCH4 => 4,
            PRSSTARTSELR::PRSCH5 => 5,
            PRSSTARTSELR::PRSCH6 => 6,
            PRSSTARTSELR::PRSCH7 => 7,
            PRSSTARTSELR::PRSCH8 => 8,
            PRSSTARTSELR::PRSCH9 => 9,
            PRSSTARTSELR::PRSCH10 => 10,
            PRSSTARTSELR::PRSCH11 => 11,
            PRSSTARTSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRSSTARTSELR {
        match value {
            0 => PRSSTARTSELR::PRSCH0,
            1 => PRSSTARTSELR::PRSCH1,
            2 => PRSSTARTSELR::PRSCH2,
            3 => PRSSTARTSELR::PRSCH3,
            4 => PRSSTARTSELR::PRSCH4,
            5 => PRSSTARTSELR::PRSCH5,
            6 => PRSSTARTSELR::PRSCH6,
            7 => PRSSTARTSELR::PRSCH7,
            8 => PRSSTARTSELR::PRSCH8,
            9 => PRSSTARTSELR::PRSCH9,
            10 => PRSSTARTSELR::PRSCH10,
            11 => PRSSTARTSELR::PRSCH11,
            i => PRSSTARTSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSTARTSELR::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSTARTSELR::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSTARTSELR::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSTARTSELR::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSTARTSELR::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSTARTSELR::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSTARTSELR::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSTARTSELR::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSTARTSELR::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSTARTSELR::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSTARTSELR::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSTARTSELR::PRSCH11
    }
}
#[doc = "Possible values of the field `PRSSTOPSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSSTOPSELR {
    #[doc = "PRS Channel 0 selected as input"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected as input"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected as input"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected as input"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected as input"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected as input"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected as input"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected as input"]
    PRSCH7,
    #[doc = "PRS Channel 8 selected as input"]
    PRSCH8,
    #[doc = "PRS Channel 9 selected as input"]
    PRSCH9,
    #[doc = "PRS Channel 10 selected as input"]
    PRSCH10,
    #[doc = "PRS Channel 11 selected as input"]
    PRSCH11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRSSTOPSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRSSTOPSELR::PRSCH0 => 0,
            PRSSTOPSELR::PRSCH1 => 1,
            PRSSTOPSELR::PRSCH2 => 2,
            PRSSTOPSELR::PRSCH3 => 3,
            PRSSTOPSELR::PRSCH4 => 4,
            PRSSTOPSELR::PRSCH5 => 5,
            PRSSTOPSELR::PRSCH6 => 6,
            PRSSTOPSELR::PRSCH7 => 7,
            PRSSTOPSELR::PRSCH8 => 8,
            PRSSTOPSELR::PRSCH9 => 9,
            PRSSTOPSELR::PRSCH10 => 10,
            PRSSTOPSELR::PRSCH11 => 11,
            PRSSTOPSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRSSTOPSELR {
        match value {
            0 => PRSSTOPSELR::PRSCH0,
            1 => PRSSTOPSELR::PRSCH1,
            2 => PRSSTOPSELR::PRSCH2,
            3 => PRSSTOPSELR::PRSCH3,
            4 => PRSSTOPSELR::PRSCH4,
            5 => PRSSTOPSELR::PRSCH5,
            6 => PRSSTOPSELR::PRSCH6,
            7 => PRSSTOPSELR::PRSCH7,
            8 => PRSSTOPSELR::PRSCH8,
            9 => PRSSTOPSELR::PRSCH9,
            10 => PRSSTOPSELR::PRSCH10,
            11 => PRSSTOPSELR::PRSCH11,
            i => PRSSTOPSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSTOPSELR::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSTOPSELR::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSTOPSELR::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSTOPSELR::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSTOPSELR::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSTOPSELR::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSTOPSELR::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSTOPSELR::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSTOPSELR::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSTOPSELR::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSTOPSELR::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSTOPSELR::PRSCH11
    }
}
#[doc = "Possible values of the field `PRSCLEARSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSCLEARSELR {
    #[doc = "PRS Channel 0 selected as input"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected as input"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected as input"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected as input"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected as input"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected as input"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected as input"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected as input"]
    PRSCH7,
    #[doc = "PRS Channel 8 selected as input"]
    PRSCH8,
    #[doc = "PRS Channel 9 selected as input"]
    PRSCH9,
    #[doc = "PRS Channel 10 selected as input"]
    PRSCH10,
    #[doc = "PRS Channel 11 selected as input"]
    PRSCH11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRSCLEARSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRSCLEARSELR::PRSCH0 => 0,
            PRSCLEARSELR::PRSCH1 => 1,
            PRSCLEARSELR::PRSCH2 => 2,
            PRSCLEARSELR::PRSCH3 => 3,
            PRSCLEARSELR::PRSCH4 => 4,
            PRSCLEARSELR::PRSCH5 => 5,
            PRSCLEARSELR::PRSCH6 => 6,
            PRSCLEARSELR::PRSCH7 => 7,
            PRSCLEARSELR::PRSCH8 => 8,
            PRSCLEARSELR::PRSCH9 => 9,
            PRSCLEARSELR::PRSCH10 => 10,
            PRSCLEARSELR::PRSCH11 => 11,
            PRSCLEARSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRSCLEARSELR {
        match value {
            0 => PRSCLEARSELR::PRSCH0,
            1 => PRSCLEARSELR::PRSCH1,
            2 => PRSCLEARSELR::PRSCH2,
            3 => PRSCLEARSELR::PRSCH3,
            4 => PRSCLEARSELR::PRSCH4,
            5 => PRSCLEARSELR::PRSCH5,
            6 => PRSCLEARSELR::PRSCH6,
            7 => PRSCLEARSELR::PRSCH7,
            8 => PRSCLEARSELR::PRSCH8,
            9 => PRSCLEARSELR::PRSCH9,
            10 => PRSCLEARSELR::PRSCH10,
            11 => PRSCLEARSELR::PRSCH11,
            i => PRSCLEARSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSCLEARSELR::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSCLEARSELR::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSCLEARSELR::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSCLEARSELR::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSCLEARSELR::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSCLEARSELR::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSCLEARSELR::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSCLEARSELR::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSCLEARSELR::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSCLEARSELR::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSCLEARSELR::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSCLEARSELR::PRSCH11
    }
}
#[doc = "Possible values of the field `PRSSTARTMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSSTARTMODER {
    #[doc = "PRS cannot start the LETIMER"]
    NONE,
    #[doc = "Rising edge of selected PRS input can start the LETIMER"]
    RISING,
    #[doc = "Falling edge of selected PRS input can start the LETIMER"]
    FALLING,
    #[doc = "Both the rising or falling edge of the selected PRS input can start the LETIMER"]
    BOTH,
}
impl PRSSTARTMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRSSTARTMODER::NONE => 0,
            PRSSTARTMODER::RISING => 1,
            PRSSTARTMODER::FALLING => 2,
            PRSSTARTMODER::BOTH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRSSTARTMODER {
        match value {
            0 => PRSSTARTMODER::NONE,
            1 => PRSSTARTMODER::RISING,
            2 => PRSSTARTMODER::FALLING,
            3 => PRSSTARTMODER::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == PRSSTARTMODER::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == PRSSTARTMODER::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == PRSSTARTMODER::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == PRSSTARTMODER::BOTH
    }
}
#[doc = "Possible values of the field `PRSSTOPMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSSTOPMODER {
    #[doc = "PRS cannot stop the LETIMER"]
    NONE,
    #[doc = "Rising edge of selected PRS input can stop the LETIMER"]
    RISING,
    #[doc = "Falling edge of selected PRS input can stop the LETIMER"]
    FALLING,
    #[doc = "Both the rising or falling edge of the selected PRS input can stop the LETIMER"]
    BOTH,
}
impl PRSSTOPMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRSSTOPMODER::NONE => 0,
            PRSSTOPMODER::RISING => 1,
            PRSSTOPMODER::FALLING => 2,
            PRSSTOPMODER::BOTH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRSSTOPMODER {
        match value {
            0 => PRSSTOPMODER::NONE,
            1 => PRSSTOPMODER::RISING,
            2 => PRSSTOPMODER::FALLING,
            3 => PRSSTOPMODER::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == PRSSTOPMODER::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == PRSSTOPMODER::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == PRSSTOPMODER::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == PRSSTOPMODER::BOTH
    }
}
#[doc = "Possible values of the field `PRSCLEARMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSCLEARMODER {
    #[doc = "PRS cannot clear the LETIMER"]
    NONE,
    #[doc = "Rising edge of selected PRS input can clear the LETIMER"]
    RISING,
    #[doc = "Falling edge of selected PRS input can clear the LETIMER"]
    FALLING,
    #[doc = "Both the rising or falling edge of the selected PRS input can clear the LETIMER"]
    BOTH,
}
impl PRSCLEARMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRSCLEARMODER::NONE => 0,
            PRSCLEARMODER::RISING => 1,
            PRSCLEARMODER::FALLING => 2,
            PRSCLEARMODER::BOTH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRSCLEARMODER {
        match value {
            0 => PRSCLEARMODER::NONE,
            1 => PRSCLEARMODER::RISING,
            2 => PRSCLEARMODER::FALLING,
            3 => PRSCLEARMODER::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == PRSCLEARMODER::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == PRSCLEARMODER::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == PRSCLEARMODER::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == PRSCLEARMODER::BOTH
    }
}
#[doc = "Values that can be written to the field `PRSSTARTSEL`"]
pub enum PRSSTARTSELW {
    #[doc = "PRS Channel 0 selected as input"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected as input"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected as input"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected as input"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected as input"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected as input"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected as input"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected as input"]
    PRSCH7,
    #[doc = "PRS Channel 8 selected as input"]
    PRSCH8,
    #[doc = "PRS Channel 9 selected as input"]
    PRSCH9,
    #[doc = "PRS Channel 10 selected as input"]
    PRSCH10,
    #[doc = "PRS Channel 11 selected as input"]
    PRSCH11,
}
impl PRSSTARTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRSSTARTSELW::PRSCH0 => 0,
            PRSSTARTSELW::PRSCH1 => 1,
            PRSSTARTSELW::PRSCH2 => 2,
            PRSSTARTSELW::PRSCH3 => 3,
            PRSSTARTSELW::PRSCH4 => 4,
            PRSSTARTSELW::PRSCH5 => 5,
            PRSSTARTSELW::PRSCH6 => 6,
            PRSSTARTSELW::PRSCH7 => 7,
            PRSSTARTSELW::PRSCH8 => 8,
            PRSSTARTSELW::PRSCH9 => 9,
            PRSSTARTSELW::PRSCH10 => 10,
            PRSSTARTSELW::PRSCH11 => 11,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRSSTARTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSSTARTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRSSTARTSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSTARTSELW::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSTARTSELW::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSTARTSELW::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSTARTSELW::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSTARTSELW::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSTARTSELW::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSTARTSELW::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSTARTSELW::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSTARTSELW::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSTARTSELW::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSTARTSELW::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSTARTSELW::PRSCH11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRSSTOPSEL`"]
pub enum PRSSTOPSELW {
    #[doc = "PRS Channel 0 selected as input"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected as input"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected as input"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected as input"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected as input"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected as input"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected as input"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected as input"]
    PRSCH7,
    #[doc = "PRS Channel 8 selected as input"]
    PRSCH8,
    #[doc = "PRS Channel 9 selected as input"]
    PRSCH9,
    #[doc = "PRS Channel 10 selected as input"]
    PRSCH10,
    #[doc = "PRS Channel 11 selected as input"]
    PRSCH11,
}
impl PRSSTOPSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRSSTOPSELW::PRSCH0 => 0,
            PRSSTOPSELW::PRSCH1 => 1,
            PRSSTOPSELW::PRSCH2 => 2,
            PRSSTOPSELW::PRSCH3 => 3,
            PRSSTOPSELW::PRSCH4 => 4,
            PRSSTOPSELW::PRSCH5 => 5,
            PRSSTOPSELW::PRSCH6 => 6,
            PRSSTOPSELW::PRSCH7 => 7,
            PRSSTOPSELW::PRSCH8 => 8,
            PRSSTOPSELW::PRSCH9 => 9,
            PRSSTOPSELW::PRSCH10 => 10,
            PRSSTOPSELW::PRSCH11 => 11,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRSSTOPSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSSTOPSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRSSTOPSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSTOPSELW::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSTOPSELW::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSTOPSELW::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSTOPSELW::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSTOPSELW::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSTOPSELW::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSTOPSELW::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSTOPSELW::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSTOPSELW::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSTOPSELW::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSTOPSELW::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSTOPSELW::PRSCH11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRSCLEARSEL`"]
pub enum PRSCLEARSELW {
    #[doc = "PRS Channel 0 selected as input"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected as input"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected as input"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected as input"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected as input"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected as input"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected as input"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected as input"]
    PRSCH7,
    #[doc = "PRS Channel 8 selected as input"]
    PRSCH8,
    #[doc = "PRS Channel 9 selected as input"]
    PRSCH9,
    #[doc = "PRS Channel 10 selected as input"]
    PRSCH10,
    #[doc = "PRS Channel 11 selected as input"]
    PRSCH11,
}
impl PRSCLEARSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRSCLEARSELW::PRSCH0 => 0,
            PRSCLEARSELW::PRSCH1 => 1,
            PRSCLEARSELW::PRSCH2 => 2,
            PRSCLEARSELW::PRSCH3 => 3,
            PRSCLEARSELW::PRSCH4 => 4,
            PRSCLEARSELW::PRSCH5 => 5,
            PRSCLEARSELW::PRSCH6 => 6,
            PRSCLEARSELW::PRSCH7 => 7,
            PRSCLEARSELW::PRSCH8 => 8,
            PRSCLEARSELW::PRSCH9 => 9,
            PRSCLEARSELW::PRSCH10 => 10,
            PRSCLEARSELW::PRSCH11 => 11,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRSCLEARSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSCLEARSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRSCLEARSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSCLEARSELW::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSCLEARSELW::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSCLEARSELW::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSCLEARSELW::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSCLEARSELW::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSCLEARSELW::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSCLEARSELW::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSCLEARSELW::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSCLEARSELW::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSCLEARSELW::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSCLEARSELW::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSCLEARSELW::PRSCH11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRSSTARTMODE`"]
pub enum PRSSTARTMODEW {
    #[doc = "PRS cannot start the LETIMER"]
    NONE,
    #[doc = "Rising edge of selected PRS input can start the LETIMER"]
    RISING,
    #[doc = "Falling edge of selected PRS input can start the LETIMER"]
    FALLING,
    #[doc = "Both the rising or falling edge of the selected PRS input can start the LETIMER"]
    BOTH,
}
impl PRSSTARTMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRSSTARTMODEW::NONE => 0,
            PRSSTARTMODEW::RISING => 1,
            PRSSTARTMODEW::FALLING => 2,
            PRSSTARTMODEW::BOTH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRSSTARTMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSSTARTMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRSSTARTMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "PRS cannot start the LETIMER"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PRSSTARTMODEW::NONE)
    }
    #[doc = "Rising edge of selected PRS input can start the LETIMER"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(PRSSTARTMODEW::RISING)
    }
    #[doc = "Falling edge of selected PRS input can start the LETIMER"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(PRSSTARTMODEW::FALLING)
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can start the LETIMER"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(PRSSTARTMODEW::BOTH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRSSTOPMODE`"]
pub enum PRSSTOPMODEW {
    #[doc = "PRS cannot stop the LETIMER"]
    NONE,
    #[doc = "Rising edge of selected PRS input can stop the LETIMER"]
    RISING,
    #[doc = "Falling edge of selected PRS input can stop the LETIMER"]
    FALLING,
    #[doc = "Both the rising or falling edge of the selected PRS input can stop the LETIMER"]
    BOTH,
}
impl PRSSTOPMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRSSTOPMODEW::NONE => 0,
            PRSSTOPMODEW::RISING => 1,
            PRSSTOPMODEW::FALLING => 2,
            PRSSTOPMODEW::BOTH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRSSTOPMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSSTOPMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRSSTOPMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "PRS cannot stop the LETIMER"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PRSSTOPMODEW::NONE)
    }
    #[doc = "Rising edge of selected PRS input can stop the LETIMER"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(PRSSTOPMODEW::RISING)
    }
    #[doc = "Falling edge of selected PRS input can stop the LETIMER"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(PRSSTOPMODEW::FALLING)
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can stop the LETIMER"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(PRSSTOPMODEW::BOTH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRSCLEARMODE`"]
pub enum PRSCLEARMODEW {
    #[doc = "PRS cannot clear the LETIMER"]
    NONE,
    #[doc = "Rising edge of selected PRS input can clear the LETIMER"]
    RISING,
    #[doc = "Falling edge of selected PRS input can clear the LETIMER"]
    FALLING,
    #[doc = "Both the rising or falling edge of the selected PRS input can clear the LETIMER"]
    BOTH,
}
impl PRSCLEARMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRSCLEARMODEW::NONE => 0,
            PRSCLEARMODEW::RISING => 1,
            PRSCLEARMODEW::FALLING => 2,
            PRSCLEARMODEW::BOTH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRSCLEARMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSCLEARMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRSCLEARMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "PRS cannot clear the LETIMER"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PRSCLEARMODEW::NONE)
    }
    #[doc = "Rising edge of selected PRS input can clear the LETIMER"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(PRSCLEARMODEW::RISING)
    }
    #[doc = "Falling edge of selected PRS input can clear the LETIMER"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(PRSCLEARMODEW::FALLING)
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can clear the LETIMER"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(PRSCLEARMODEW::BOTH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
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
    #[doc = "Bits 0:3 - PRS Start Select"]
    #[inline]
    pub fn prsstartsel(&self) -> PRSSTARTSELR {
        PRSSTARTSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:9 - PRS Stop Select"]
    #[inline]
    pub fn prsstopsel(&self) -> PRSSTOPSELR {
        PRSSTOPSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - PRS Clear Select"]
    #[inline]
    pub fn prsclearsel(&self) -> PRSCLEARSELR {
        PRSCLEARSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - PRS Start Mode"]
    #[inline]
    pub fn prsstartmode(&self) -> PRSSTARTMODER {
        PRSSTARTMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - PRS Stop Mode"]
    #[inline]
    pub fn prsstopmode(&self) -> PRSSTOPMODER {
        PRSSTOPMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - PRS Clear Mode"]
    #[inline]
    pub fn prsclearmode(&self) -> PRSCLEARMODER {
        PRSCLEARMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
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
    #[doc = "Bits 0:3 - PRS Start Select"]
    #[inline]
    pub fn prsstartsel(&mut self) -> _PRSSTARTSELW {
        _PRSSTARTSELW { w: self }
    }
    #[doc = "Bits 6:9 - PRS Stop Select"]
    #[inline]
    pub fn prsstopsel(&mut self) -> _PRSSTOPSELW {
        _PRSSTOPSELW { w: self }
    }
    #[doc = "Bits 12:15 - PRS Clear Select"]
    #[inline]
    pub fn prsclearsel(&mut self) -> _PRSCLEARSELW {
        _PRSCLEARSELW { w: self }
    }
    #[doc = "Bits 18:19 - PRS Start Mode"]
    #[inline]
    pub fn prsstartmode(&mut self) -> _PRSSTARTMODEW {
        _PRSSTARTMODEW { w: self }
    }
    #[doc = "Bits 22:23 - PRS Stop Mode"]
    #[inline]
    pub fn prsstopmode(&mut self) -> _PRSSTOPMODEW {
        _PRSSTOPMODEW { w: self }
    }
    #[doc = "Bits 26:27 - PRS Clear Mode"]
    #[inline]
    pub fn prsclearmode(&mut self) -> _PRSCLEARMODEW {
        _PRSCLEARMODEW { w: self }
    }
}
