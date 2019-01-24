#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CALCTRL {
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
#[doc = "Possible values of the field `UPSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPSELR {
    #[doc = "Select HFXO as up-counter"]
    HFXO,
    #[doc = "Select LFXO as up-counter"]
    LFXO,
    #[doc = "Select HFRCO as up-counter"]
    HFRCO,
    #[doc = "Select LFRCO as up-counter"]
    LFRCO,
    #[doc = "Select AUXHFRCO as up-counter"]
    AUXHFRCO,
    #[doc = "Select PRS input selected by PRSUPSEL as up-counter"]
    PRS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl UPSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UPSELR::HFXO => 0,
            UPSELR::LFXO => 1,
            UPSELR::HFRCO => 2,
            UPSELR::LFRCO => 3,
            UPSELR::AUXHFRCO => 4,
            UPSELR::PRS => 5,
            UPSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UPSELR {
        match value {
            0 => UPSELR::HFXO,
            1 => UPSELR::LFXO,
            2 => UPSELR::HFRCO,
            3 => UPSELR::LFRCO,
            4 => UPSELR::AUXHFRCO,
            5 => UPSELR::PRS,
            i => UPSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline]
    pub fn is_hfxo(&self) -> bool {
        *self == UPSELR::HFXO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline]
    pub fn is_lfxo(&self) -> bool {
        *self == UPSELR::LFXO
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline]
    pub fn is_hfrco(&self) -> bool {
        *self == UPSELR::HFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline]
    pub fn is_lfrco(&self) -> bool {
        *self == UPSELR::LFRCO
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline]
    pub fn is_auxhfrco(&self) -> bool {
        *self == UPSELR::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline]
    pub fn is_prs(&self) -> bool {
        *self == UPSELR::PRS
    }
}
#[doc = "Possible values of the field `DOWNSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOWNSELR {
    #[doc = "Select HFCLK for down-counter"]
    HFCLK,
    #[doc = "Select HFXO for down-counter"]
    HFXO,
    #[doc = "Select LFXO for down-counter"]
    LFXO,
    #[doc = "Select HFRCO for down-counter"]
    HFRCO,
    #[doc = "Select LFRCO for down-counter"]
    LFRCO,
    #[doc = "Select AUXHFRCO for down-counter"]
    AUXHFRCO,
    #[doc = "Select PRS input selected by PRSDOWNSEL as down-counter"]
    PRS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DOWNSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DOWNSELR::HFCLK => 0,
            DOWNSELR::HFXO => 1,
            DOWNSELR::LFXO => 2,
            DOWNSELR::HFRCO => 3,
            DOWNSELR::LFRCO => 4,
            DOWNSELR::AUXHFRCO => 5,
            DOWNSELR::PRS => 6,
            DOWNSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DOWNSELR {
        match value {
            0 => DOWNSELR::HFCLK,
            1 => DOWNSELR::HFXO,
            2 => DOWNSELR::LFXO,
            3 => DOWNSELR::HFRCO,
            4 => DOWNSELR::LFRCO,
            5 => DOWNSELR::AUXHFRCO,
            6 => DOWNSELR::PRS,
            i => DOWNSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `HFCLK`"]
    #[inline]
    pub fn is_hfclk(&self) -> bool {
        *self == DOWNSELR::HFCLK
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline]
    pub fn is_hfxo(&self) -> bool {
        *self == DOWNSELR::HFXO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline]
    pub fn is_lfxo(&self) -> bool {
        *self == DOWNSELR::LFXO
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline]
    pub fn is_hfrco(&self) -> bool {
        *self == DOWNSELR::HFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline]
    pub fn is_lfrco(&self) -> bool {
        *self == DOWNSELR::LFRCO
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline]
    pub fn is_auxhfrco(&self) -> bool {
        *self == DOWNSELR::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline]
    pub fn is_prs(&self) -> bool {
        *self == DOWNSELR::PRS
    }
}
#[doc = r" Value of the field"]
pub struct CONTR {
    bits: bool,
}
impl CONTR {
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
#[doc = "Possible values of the field `PRSUPSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSUPSELR {
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
impl PRSUPSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRSUPSELR::PRSCH0 => 0,
            PRSUPSELR::PRSCH1 => 1,
            PRSUPSELR::PRSCH2 => 2,
            PRSUPSELR::PRSCH3 => 3,
            PRSUPSELR::PRSCH4 => 4,
            PRSUPSELR::PRSCH5 => 5,
            PRSUPSELR::PRSCH6 => 6,
            PRSUPSELR::PRSCH7 => 7,
            PRSUPSELR::PRSCH8 => 8,
            PRSUPSELR::PRSCH9 => 9,
            PRSUPSELR::PRSCH10 => 10,
            PRSUPSELR::PRSCH11 => 11,
            PRSUPSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRSUPSELR {
        match value {
            0 => PRSUPSELR::PRSCH0,
            1 => PRSUPSELR::PRSCH1,
            2 => PRSUPSELR::PRSCH2,
            3 => PRSUPSELR::PRSCH3,
            4 => PRSUPSELR::PRSCH4,
            5 => PRSUPSELR::PRSCH5,
            6 => PRSUPSELR::PRSCH6,
            7 => PRSUPSELR::PRSCH7,
            8 => PRSUPSELR::PRSCH8,
            9 => PRSUPSELR::PRSCH9,
            10 => PRSUPSELR::PRSCH10,
            11 => PRSUPSELR::PRSCH11,
            i => PRSUPSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSUPSELR::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSUPSELR::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSUPSELR::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSUPSELR::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSUPSELR::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSUPSELR::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSUPSELR::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSUPSELR::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSUPSELR::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSUPSELR::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSUPSELR::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSUPSELR::PRSCH11
    }
}
#[doc = "Possible values of the field `PRSDOWNSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSDOWNSELR {
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
impl PRSDOWNSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRSDOWNSELR::PRSCH0 => 0,
            PRSDOWNSELR::PRSCH1 => 1,
            PRSDOWNSELR::PRSCH2 => 2,
            PRSDOWNSELR::PRSCH3 => 3,
            PRSDOWNSELR::PRSCH4 => 4,
            PRSDOWNSELR::PRSCH5 => 5,
            PRSDOWNSELR::PRSCH6 => 6,
            PRSDOWNSELR::PRSCH7 => 7,
            PRSDOWNSELR::PRSCH8 => 8,
            PRSDOWNSELR::PRSCH9 => 9,
            PRSDOWNSELR::PRSCH10 => 10,
            PRSDOWNSELR::PRSCH11 => 11,
            PRSDOWNSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRSDOWNSELR {
        match value {
            0 => PRSDOWNSELR::PRSCH0,
            1 => PRSDOWNSELR::PRSCH1,
            2 => PRSDOWNSELR::PRSCH2,
            3 => PRSDOWNSELR::PRSCH3,
            4 => PRSDOWNSELR::PRSCH4,
            5 => PRSDOWNSELR::PRSCH5,
            6 => PRSDOWNSELR::PRSCH6,
            7 => PRSDOWNSELR::PRSCH7,
            8 => PRSDOWNSELR::PRSCH8,
            9 => PRSDOWNSELR::PRSCH9,
            10 => PRSDOWNSELR::PRSCH10,
            11 => PRSDOWNSELR::PRSCH11,
            i => PRSDOWNSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSDOWNSELR::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSDOWNSELR::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSDOWNSELR::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSDOWNSELR::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSDOWNSELR::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSDOWNSELR::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSDOWNSELR::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSDOWNSELR::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSDOWNSELR::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSDOWNSELR::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSDOWNSELR::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSDOWNSELR::PRSCH11
    }
}
#[doc = "Values that can be written to the field `UPSEL`"]
pub enum UPSELW {
    #[doc = "Select HFXO as up-counter"]
    HFXO,
    #[doc = "Select LFXO as up-counter"]
    LFXO,
    #[doc = "Select HFRCO as up-counter"]
    HFRCO,
    #[doc = "Select LFRCO as up-counter"]
    LFRCO,
    #[doc = "Select AUXHFRCO as up-counter"]
    AUXHFRCO,
    #[doc = "Select PRS input selected by PRSUPSEL as up-counter"]
    PRS,
}
impl UPSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UPSELW::HFXO => 0,
            UPSELW::LFXO => 1,
            UPSELW::HFRCO => 2,
            UPSELW::LFRCO => 3,
            UPSELW::AUXHFRCO => 4,
            UPSELW::PRS => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UPSELW<'a> {
    w: &'a mut W,
}
impl<'a> _UPSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UPSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select HFXO as up-counter"]
    #[inline]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(UPSELW::HFXO)
    }
    #[doc = "Select LFXO as up-counter"]
    #[inline]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(UPSELW::LFXO)
    }
    #[doc = "Select HFRCO as up-counter"]
    #[inline]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(UPSELW::HFRCO)
    }
    #[doc = "Select LFRCO as up-counter"]
    #[inline]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(UPSELW::LFRCO)
    }
    #[doc = "Select AUXHFRCO as up-counter"]
    #[inline]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(UPSELW::AUXHFRCO)
    }
    #[doc = "Select PRS input selected by PRSUPSEL as up-counter"]
    #[inline]
    pub fn prs(self) -> &'a mut W {
        self.variant(UPSELW::PRS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DOWNSEL`"]
pub enum DOWNSELW {
    #[doc = "Select HFCLK for down-counter"]
    HFCLK,
    #[doc = "Select HFXO for down-counter"]
    HFXO,
    #[doc = "Select LFXO for down-counter"]
    LFXO,
    #[doc = "Select HFRCO for down-counter"]
    HFRCO,
    #[doc = "Select LFRCO for down-counter"]
    LFRCO,
    #[doc = "Select AUXHFRCO for down-counter"]
    AUXHFRCO,
    #[doc = "Select PRS input selected by PRSDOWNSEL as down-counter"]
    PRS,
}
impl DOWNSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DOWNSELW::HFCLK => 0,
            DOWNSELW::HFXO => 1,
            DOWNSELW::LFXO => 2,
            DOWNSELW::HFRCO => 3,
            DOWNSELW::LFRCO => 4,
            DOWNSELW::AUXHFRCO => 5,
            DOWNSELW::PRS => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOWNSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DOWNSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOWNSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select HFCLK for down-counter"]
    #[inline]
    pub fn hfclk(self) -> &'a mut W {
        self.variant(DOWNSELW::HFCLK)
    }
    #[doc = "Select HFXO for down-counter"]
    #[inline]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(DOWNSELW::HFXO)
    }
    #[doc = "Select LFXO for down-counter"]
    #[inline]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(DOWNSELW::LFXO)
    }
    #[doc = "Select HFRCO for down-counter"]
    #[inline]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(DOWNSELW::HFRCO)
    }
    #[doc = "Select LFRCO for down-counter"]
    #[inline]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(DOWNSELW::LFRCO)
    }
    #[doc = "Select AUXHFRCO for down-counter"]
    #[inline]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(DOWNSELW::AUXHFRCO)
    }
    #[doc = "Select PRS input selected by PRSDOWNSEL as down-counter"]
    #[inline]
    pub fn prs(self) -> &'a mut W {
        self.variant(DOWNSELW::PRS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CONTW<'a> {
    w: &'a mut W,
}
impl<'a> _CONTW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRSUPSEL`"]
pub enum PRSUPSELW {
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
impl PRSUPSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRSUPSELW::PRSCH0 => 0,
            PRSUPSELW::PRSCH1 => 1,
            PRSUPSELW::PRSCH2 => 2,
            PRSUPSELW::PRSCH3 => 3,
            PRSUPSELW::PRSCH4 => 4,
            PRSUPSELW::PRSCH5 => 5,
            PRSUPSELW::PRSCH6 => 6,
            PRSUPSELW::PRSCH7 => 7,
            PRSUPSELW::PRSCH8 => 8,
            PRSUPSELW::PRSCH9 => 9,
            PRSUPSELW::PRSCH10 => 10,
            PRSUPSELW::PRSCH11 => 11,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRSUPSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSUPSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRSUPSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSUPSELW::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSUPSELW::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSUPSELW::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSUPSELW::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSUPSELW::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSUPSELW::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSUPSELW::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSUPSELW::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSUPSELW::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSUPSELW::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSUPSELW::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSUPSELW::PRSCH11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRSDOWNSEL`"]
pub enum PRSDOWNSELW {
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
impl PRSDOWNSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRSDOWNSELW::PRSCH0 => 0,
            PRSDOWNSELW::PRSCH1 => 1,
            PRSDOWNSELW::PRSCH2 => 2,
            PRSDOWNSELW::PRSCH3 => 3,
            PRSDOWNSELW::PRSCH4 => 4,
            PRSDOWNSELW::PRSCH5 => 5,
            PRSDOWNSELW::PRSCH6 => 6,
            PRSDOWNSELW::PRSCH7 => 7,
            PRSDOWNSELW::PRSCH8 => 8,
            PRSDOWNSELW::PRSCH9 => 9,
            PRSDOWNSELW::PRSCH10 => 10,
            PRSDOWNSELW::PRSCH11 => 11,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRSDOWNSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSDOWNSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRSDOWNSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSDOWNSELW::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSDOWNSELW::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSDOWNSELW::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSDOWNSELW::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSDOWNSELW::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSDOWNSELW::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSDOWNSELW::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSDOWNSELW::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSDOWNSELW::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSDOWNSELW::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSDOWNSELW::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSDOWNSELW::PRSCH11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:2 - Calibration Up-counter Select"]
    #[inline]
    pub fn upsel(&self) -> UPSELR {
        UPSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Calibration Down-counter Select"]
    #[inline]
    pub fn downsel(&self) -> DOWNSELR {
        DOWNSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Continuous Calibration"]
    #[inline]
    pub fn cont(&self) -> CONTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CONTR { bits }
    }
    #[doc = "Bits 16:19 - PRS Select for PRS Input When Selected in UPSEL"]
    #[inline]
    pub fn prsupsel(&self) -> PRSUPSELR {
        PRSUPSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - PRS Select for PRS Input When Selected in DOWNSEL"]
    #[inline]
    pub fn prsdownsel(&self) -> PRSDOWNSELR {
        PRSDOWNSELR::_from({
            const MASK: u8 = 15;
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
    #[doc = "Bits 0:2 - Calibration Up-counter Select"]
    #[inline]
    pub fn upsel(&mut self) -> _UPSELW {
        _UPSELW { w: self }
    }
    #[doc = "Bits 4:6 - Calibration Down-counter Select"]
    #[inline]
    pub fn downsel(&mut self) -> _DOWNSELW {
        _DOWNSELW { w: self }
    }
    #[doc = "Bit 8 - Continuous Calibration"]
    #[inline]
    pub fn cont(&mut self) -> _CONTW {
        _CONTW { w: self }
    }
    #[doc = "Bits 16:19 - PRS Select for PRS Input When Selected in UPSEL"]
    #[inline]
    pub fn prsupsel(&mut self) -> _PRSUPSELW {
        _PRSUPSELW { w: self }
    }
    #[doc = "Bits 24:27 - PRS Select for PRS Input When Selected in DOWNSEL"]
    #[inline]
    pub fn prsdownsel(&mut self) -> _PRSDOWNSELW {
        _PRSDOWNSELW { w: self }
    }
}
