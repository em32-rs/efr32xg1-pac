#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCANCTRLX {
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
#[doc = "Possible values of the field `VREFSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFSELR {
    #[doc = "Internal 0.83V Bandgap reference"]
    VBGR,
    #[doc = "Scaled AVDD: AVDD*(the VREF attenuation factor)"]
    VDDXWATT,
    #[doc = "Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)"]
    VREFPWATT,
    #[doc = "Raw single ended external Vref: ADCn_EXTP"]
    VREFP,
    #[doc = "Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)"]
    VREFPNWATT,
    #[doc = "Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)"]
    VREFPN,
    #[doc = "Internal Bandgap reference at low setting 0.78V"]
    VBGRLOW,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl VREFSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VREFSELR::VBGR => 0,
            VREFSELR::VDDXWATT => 1,
            VREFSELR::VREFPWATT => 2,
            VREFSELR::VREFP => 3,
            VREFSELR::VREFPNWATT => 5,
            VREFSELR::VREFPN => 6,
            VREFSELR::VBGRLOW => 7,
            VREFSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VREFSELR {
        match value {
            0 => VREFSELR::VBGR,
            1 => VREFSELR::VDDXWATT,
            2 => VREFSELR::VREFPWATT,
            3 => VREFSELR::VREFP,
            5 => VREFSELR::VREFPNWATT,
            6 => VREFSELR::VREFPN,
            7 => VREFSELR::VBGRLOW,
            i => VREFSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VBGR`"]
    #[inline]
    pub fn is_vbgr(&self) -> bool {
        *self == VREFSELR::VBGR
    }
    #[doc = "Checks if the value of the field is `VDDXWATT`"]
    #[inline]
    pub fn is_vddxwatt(&self) -> bool {
        *self == VREFSELR::VDDXWATT
    }
    #[doc = "Checks if the value of the field is `VREFPWATT`"]
    #[inline]
    pub fn is_vrefpwatt(&self) -> bool {
        *self == VREFSELR::VREFPWATT
    }
    #[doc = "Checks if the value of the field is `VREFP`"]
    #[inline]
    pub fn is_vrefp(&self) -> bool {
        *self == VREFSELR::VREFP
    }
    #[doc = "Checks if the value of the field is `VREFPNWATT`"]
    #[inline]
    pub fn is_vrefpnwatt(&self) -> bool {
        *self == VREFSELR::VREFPNWATT
    }
    #[doc = "Checks if the value of the field is `VREFPN`"]
    #[inline]
    pub fn is_vrefpn(&self) -> bool {
        *self == VREFSELR::VREFPN
    }
    #[doc = "Checks if the value of the field is `VBGRLOW`"]
    #[inline]
    pub fn is_vbgrlow(&self) -> bool {
        *self == VREFSELR::VBGRLOW
    }
}
#[doc = r" Value of the field"]
pub struct VREFATTFIXR {
    bits: bool,
}
impl VREFATTFIXR {
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
pub struct VREFATTR {
    bits: u8,
}
impl VREFATTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VINATTR {
    bits: u8,
}
impl VINATTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DVLR {
    bits: u8,
}
impl DVLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FIFOOFACTR {
    bits: bool,
}
impl FIFOOFACTR {
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
pub struct PRSMODER {
    bits: bool,
}
impl PRSMODER {
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
    #[doc = "PRS ch 0 triggers scan sequence"]
    PRSCH0,
    #[doc = "PRS ch 1 triggers scan sequence"]
    PRSCH1,
    #[doc = "PRS ch 2 triggers scan sequence"]
    PRSCH2,
    #[doc = "PRS ch 3 triggers scan sequence"]
    PRSCH3,
    #[doc = "PRS ch 4 triggers scan sequence"]
    PRSCH4,
    #[doc = "PRS ch 5 triggers scan sequence"]
    PRSCH5,
    #[doc = "PRS ch 6 triggers scan sequence"]
    PRSCH6,
    #[doc = "PRS ch 7 triggers scan sequence"]
    PRSCH7,
    #[doc = "PRS ch 8 triggers scan sequence"]
    PRSCH8,
    #[doc = "PRS ch 9 triggers scan sequence"]
    PRSCH9,
    #[doc = "PRS ch 10 triggers scan sequence"]
    PRSCH10,
    #[doc = "PRS ch 11 triggers scan sequence"]
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
#[doc = r" Value of the field"]
pub struct CONVSTARTDELAYR {
    bits: u8,
}
impl CONVSTARTDELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CONVSTARTDELAYENR {
    bits: bool,
}
impl CONVSTARTDELAYENR {
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
#[doc = "Values that can be written to the field `VREFSEL`"]
pub enum VREFSELW {
    #[doc = "Internal 0.83V Bandgap reference"]
    VBGR,
    #[doc = "Scaled AVDD: AVDD*(the VREF attenuation factor)"]
    VDDXWATT,
    #[doc = "Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)"]
    VREFPWATT,
    #[doc = "Raw single ended external Vref: ADCn_EXTP"]
    VREFP,
    #[doc = "Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)"]
    VREFPNWATT,
    #[doc = "Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)"]
    VREFPN,
    #[doc = "Internal Bandgap reference at low setting 0.78V"]
    VBGRLOW,
}
impl VREFSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VREFSELW::VBGR => 0,
            VREFSELW::VDDXWATT => 1,
            VREFSELW::VREFPWATT => 2,
            VREFSELW::VREFP => 3,
            VREFSELW::VREFPNWATT => 5,
            VREFSELW::VREFPN => 6,
            VREFSELW::VBGRLOW => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VREFSELW<'a> {
    w: &'a mut W,
}
impl<'a> _VREFSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VREFSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Internal 0.83V Bandgap reference"]
    #[inline]
    pub fn vbgr(self) -> &'a mut W {
        self.variant(VREFSELW::VBGR)
    }
    #[doc = "Scaled AVDD: AVDD*(the VREF attenuation factor)"]
    #[inline]
    pub fn vddxwatt(self) -> &'a mut W {
        self.variant(VREFSELW::VDDXWATT)
    }
    #[doc = "Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)"]
    #[inline]
    pub fn vrefpwatt(self) -> &'a mut W {
        self.variant(VREFSELW::VREFPWATT)
    }
    #[doc = "Raw single ended external Vref: ADCn_EXTP"]
    #[inline]
    pub fn vrefp(self) -> &'a mut W {
        self.variant(VREFSELW::VREFP)
    }
    #[doc = "Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)"]
    #[inline]
    pub fn vrefpnwatt(self) -> &'a mut W {
        self.variant(VREFSELW::VREFPNWATT)
    }
    #[doc = "Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)"]
    #[inline]
    pub fn vrefpn(self) -> &'a mut W {
        self.variant(VREFSELW::VREFPN)
    }
    #[doc = "Internal Bandgap reference at low setting 0.78V"]
    #[inline]
    pub fn vbgrlow(self) -> &'a mut W {
        self.variant(VREFSELW::VBGRLOW)
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
#[doc = r" Proxy"]
pub struct _VREFATTFIXW<'a> {
    w: &'a mut W,
}
impl<'a> _VREFATTFIXW<'a> {
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
#[doc = r" Proxy"]
pub struct _VREFATTW<'a> {
    w: &'a mut W,
}
impl<'a> _VREFATTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VINATTW<'a> {
    w: &'a mut W,
}
impl<'a> _VINATTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DVLW<'a> {
    w: &'a mut W,
}
impl<'a> _DVLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FIFOOFACTW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFOOFACTW<'a> {
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
pub struct _PRSMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSMODEW<'a> {
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
    #[doc = "PRS ch 0 triggers scan sequence"]
    PRSCH0,
    #[doc = "PRS ch 1 triggers scan sequence"]
    PRSCH1,
    #[doc = "PRS ch 2 triggers scan sequence"]
    PRSCH2,
    #[doc = "PRS ch 3 triggers scan sequence"]
    PRSCH3,
    #[doc = "PRS ch 4 triggers scan sequence"]
    PRSCH4,
    #[doc = "PRS ch 5 triggers scan sequence"]
    PRSCH5,
    #[doc = "PRS ch 6 triggers scan sequence"]
    PRSCH6,
    #[doc = "PRS ch 7 triggers scan sequence"]
    PRSCH7,
    #[doc = "PRS ch 8 triggers scan sequence"]
    PRSCH8,
    #[doc = "PRS ch 9 triggers scan sequence"]
    PRSCH9,
    #[doc = "PRS ch 10 triggers scan sequence"]
    PRSCH10,
    #[doc = "PRS ch 11 triggers scan sequence"]
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
    #[doc = "PRS ch 0 triggers scan sequence"]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH0)
    }
    #[doc = "PRS ch 1 triggers scan sequence"]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH1)
    }
    #[doc = "PRS ch 2 triggers scan sequence"]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH2)
    }
    #[doc = "PRS ch 3 triggers scan sequence"]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH3)
    }
    #[doc = "PRS ch 4 triggers scan sequence"]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH4)
    }
    #[doc = "PRS ch 5 triggers scan sequence"]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH5)
    }
    #[doc = "PRS ch 6 triggers scan sequence"]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH6)
    }
    #[doc = "PRS ch 7 triggers scan sequence"]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH7)
    }
    #[doc = "PRS ch 8 triggers scan sequence"]
    #[inline]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH8)
    }
    #[doc = "PRS ch 9 triggers scan sequence"]
    #[inline]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH9)
    }
    #[doc = "PRS ch 10 triggers scan sequence"]
    #[inline]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH10)
    }
    #[doc = "PRS ch 11 triggers scan sequence"]
    #[inline]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CONVSTARTDELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _CONVSTARTDELAYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CONVSTARTDELAYENW<'a> {
    w: &'a mut W,
}
impl<'a> _CONVSTARTDELAYENW<'a> {
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
        const OFFSET: u8 = 27;
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
    #[doc = "Bits 0:2 - Scan Channel Reference Selection"]
    #[inline]
    pub fn vrefsel(&self) -> VREFSELR {
        VREFSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Enable Fixed Scaling on VREF"]
    #[inline]
    pub fn vrefattfix(&self) -> VREFATTFIXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VREFATTFIXR { bits }
    }
    #[doc = "Bits 4:7 - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
    #[inline]
    pub fn vrefatt(&self) -> VREFATTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VREFATTR { bits }
    }
    #[doc = "Bits 8:11 - Code for VIN Attenuation Factor"]
    #[inline]
    pub fn vinatt(&self) -> VINATTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VINATTR { bits }
    }
    #[doc = "Bits 12:13 - Scan DV Level Select"]
    #[inline]
    pub fn dvl(&self) -> DVLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DVLR { bits }
    }
    #[doc = "Bit 14 - Scan FIFO Overflow Action"]
    #[inline]
    pub fn fifoofact(&self) -> FIFOOFACTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FIFOOFACTR { bits }
    }
    #[doc = "Bit 16 - Scan PRS Trigger Mode"]
    #[inline]
    pub fn prsmode(&self) -> PRSMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRSMODER { bits }
    }
    #[doc = "Bits 17:20 - Scan Sequence PRS Trigger Select"]
    #[inline]
    pub fn prssel(&self) -> PRSSELR {
        PRSSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:26 - Delay Next Conversion Start If CONVSTARTDELAYEN is Set"]
    #[inline]
    pub fn convstartdelay(&self) -> CONVSTARTDELAYR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CONVSTARTDELAYR { bits }
    }
    #[doc = "Bit 27 - Enable Delaying Next Conversion Start"]
    #[inline]
    pub fn convstartdelayen(&self) -> CONVSTARTDELAYENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CONVSTARTDELAYENR { bits }
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
    #[doc = "Bits 0:2 - Scan Channel Reference Selection"]
    #[inline]
    pub fn vrefsel(&mut self) -> _VREFSELW {
        _VREFSELW { w: self }
    }
    #[doc = "Bit 3 - Enable Fixed Scaling on VREF"]
    #[inline]
    pub fn vrefattfix(&mut self) -> _VREFATTFIXW {
        _VREFATTFIXW { w: self }
    }
    #[doc = "Bits 4:7 - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
    #[inline]
    pub fn vrefatt(&mut self) -> _VREFATTW {
        _VREFATTW { w: self }
    }
    #[doc = "Bits 8:11 - Code for VIN Attenuation Factor"]
    #[inline]
    pub fn vinatt(&mut self) -> _VINATTW {
        _VINATTW { w: self }
    }
    #[doc = "Bits 12:13 - Scan DV Level Select"]
    #[inline]
    pub fn dvl(&mut self) -> _DVLW {
        _DVLW { w: self }
    }
    #[doc = "Bit 14 - Scan FIFO Overflow Action"]
    #[inline]
    pub fn fifoofact(&mut self) -> _FIFOOFACTW {
        _FIFOOFACTW { w: self }
    }
    #[doc = "Bit 16 - Scan PRS Trigger Mode"]
    #[inline]
    pub fn prsmode(&mut self) -> _PRSMODEW {
        _PRSMODEW { w: self }
    }
    #[doc = "Bits 17:20 - Scan Sequence PRS Trigger Select"]
    #[inline]
    pub fn prssel(&mut self) -> _PRSSELW {
        _PRSSELW { w: self }
    }
    #[doc = "Bits 24:26 - Delay Next Conversion Start If CONVSTARTDELAYEN is Set"]
    #[inline]
    pub fn convstartdelay(&mut self) -> _CONVSTARTDELAYW {
        _CONVSTARTDELAYW { w: self }
    }
    #[doc = "Bit 27 - Enable Delaying Next Conversion Start"]
    #[inline]
    pub fn convstartdelayen(&mut self) -> _CONVSTARTDELAYENW {
        _CONVSTARTDELAYENW { w: self }
    }
}
