#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIMING {
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
#[doc = "Possible values of the field `TXDELAY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDELAYR {
    #[doc = "Disable - TXDELAY in USARTn_CTRL can be used for legacy"]
    DISABLE,
    #[doc = "Start of transmission is delayed for 1 baud-times"]
    ONE,
    #[doc = "Start of transmission is delayed for 2 baud-times"]
    TWO,
    #[doc = "Start of transmission is delayed for 3 baud-times"]
    THREE,
    #[doc = "Start of transmission is delayed for 7 baud-times"]
    SEVEN,
    #[doc = "Start of transmission is delayed for TCMPVAL0 baud-times"]
    TCMP0,
    #[doc = "Start of transmission is delayed for TCMPVAL1 baud-times"]
    TCMP1,
    #[doc = "Start of transmission is delayed for TCMPVAL2 baud-times"]
    TCMP2,
}
impl TXDELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXDELAYR::DISABLE => 0,
            TXDELAYR::ONE => 1,
            TXDELAYR::TWO => 2,
            TXDELAYR::THREE => 3,
            TXDELAYR::SEVEN => 4,
            TXDELAYR::TCMP0 => 5,
            TXDELAYR::TCMP1 => 6,
            TXDELAYR::TCMP2 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXDELAYR {
        match value {
            0 => TXDELAYR::DISABLE,
            1 => TXDELAYR::ONE,
            2 => TXDELAYR::TWO,
            3 => TXDELAYR::THREE,
            4 => TXDELAYR::SEVEN,
            5 => TXDELAYR::TCMP0,
            6 => TXDELAYR::TCMP1,
            7 => TXDELAYR::TCMP2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TXDELAYR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == TXDELAYR::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline]
    pub fn is_two(&self) -> bool {
        *self == TXDELAYR::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline]
    pub fn is_three(&self) -> bool {
        *self == TXDELAYR::THREE
    }
    #[doc = "Checks if the value of the field is `SEVEN`"]
    #[inline]
    pub fn is_seven(&self) -> bool {
        *self == TXDELAYR::SEVEN
    }
    #[doc = "Checks if the value of the field is `TCMP0`"]
    #[inline]
    pub fn is_tcmp0(&self) -> bool {
        *self == TXDELAYR::TCMP0
    }
    #[doc = "Checks if the value of the field is `TCMP1`"]
    #[inline]
    pub fn is_tcmp1(&self) -> bool {
        *self == TXDELAYR::TCMP1
    }
    #[doc = "Checks if the value of the field is `TCMP2`"]
    #[inline]
    pub fn is_tcmp2(&self) -> bool {
        *self == TXDELAYR::TCMP2
    }
}
#[doc = "Possible values of the field `CSSETUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSETUPR {
    #[doc = "CS is not asserted before start of transmission"]
    ZERO,
    #[doc = "CS is asserted for 1 baud-times before start of transmission"]
    ONE,
    #[doc = "CS is asserted for 2 baud-times before start of transmission"]
    TWO,
    #[doc = "CS is asserted for 3 baud-times before start of transmission"]
    THREE,
    #[doc = "CS is asserted for 7 baud-times before start of transmission"]
    SEVEN,
    #[doc = "CS is asserted before the start of transmission for TCMPVAL0 baud-times"]
    TCMP0,
    #[doc = "CS is asserted before the start of transmission for TCMPVAL1 baud-times"]
    TCMP1,
    #[doc = "CS is asserted before the start of transmission for TCMPVAL2 baud-times"]
    TCMP2,
}
impl CSSETUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSSETUPR::ZERO => 0,
            CSSETUPR::ONE => 1,
            CSSETUPR::TWO => 2,
            CSSETUPR::THREE => 3,
            CSSETUPR::SEVEN => 4,
            CSSETUPR::TCMP0 => 5,
            CSSETUPR::TCMP1 => 6,
            CSSETUPR::TCMP2 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSSETUPR {
        match value {
            0 => CSSETUPR::ZERO,
            1 => CSSETUPR::ONE,
            2 => CSSETUPR::TWO,
            3 => CSSETUPR::THREE,
            4 => CSSETUPR::SEVEN,
            5 => CSSETUPR::TCMP0,
            6 => CSSETUPR::TCMP1,
            7 => CSSETUPR::TCMP2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CSSETUPR::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CSSETUPR::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline]
    pub fn is_two(&self) -> bool {
        *self == CSSETUPR::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline]
    pub fn is_three(&self) -> bool {
        *self == CSSETUPR::THREE
    }
    #[doc = "Checks if the value of the field is `SEVEN`"]
    #[inline]
    pub fn is_seven(&self) -> bool {
        *self == CSSETUPR::SEVEN
    }
    #[doc = "Checks if the value of the field is `TCMP0`"]
    #[inline]
    pub fn is_tcmp0(&self) -> bool {
        *self == CSSETUPR::TCMP0
    }
    #[doc = "Checks if the value of the field is `TCMP1`"]
    #[inline]
    pub fn is_tcmp1(&self) -> bool {
        *self == CSSETUPR::TCMP1
    }
    #[doc = "Checks if the value of the field is `TCMP2`"]
    #[inline]
    pub fn is_tcmp2(&self) -> bool {
        *self == CSSETUPR::TCMP2
    }
}
#[doc = "Possible values of the field `ICS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICSR {
    #[doc = "There is no space between charcters"]
    ZERO,
    #[doc = "Create a space of 1 baud-times before start of transmission "]
    ONE,
    #[doc = "Create a space of 2 baud-times before start of transmission"]
    TWO,
    #[doc = "Create a space of 3 baud-times before start of transmission"]
    THREE,
    #[doc = "Create a space of 7 baud-times before start of transmission"]
    SEVEN,
    #[doc = "Create a space of before the start of transmission for TCMPVAL0 baud-times"]
    TCMP0,
    #[doc = "Create a space of before the start of transmission for TCMPVAL1 baud-times"]
    TCMP1,
    #[doc = "Create a space of before the start of transmission for TCMPVAL2 baud-times"]
    TCMP2,
}
impl ICSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICSR::ZERO => 0,
            ICSR::ONE => 1,
            ICSR::TWO => 2,
            ICSR::THREE => 3,
            ICSR::SEVEN => 4,
            ICSR::TCMP0 => 5,
            ICSR::TCMP1 => 6,
            ICSR::TCMP2 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICSR {
        match value {
            0 => ICSR::ZERO,
            1 => ICSR::ONE,
            2 => ICSR::TWO,
            3 => ICSR::THREE,
            4 => ICSR::SEVEN,
            5 => ICSR::TCMP0,
            6 => ICSR::TCMP1,
            7 => ICSR::TCMP2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == ICSR::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == ICSR::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline]
    pub fn is_two(&self) -> bool {
        *self == ICSR::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline]
    pub fn is_three(&self) -> bool {
        *self == ICSR::THREE
    }
    #[doc = "Checks if the value of the field is `SEVEN`"]
    #[inline]
    pub fn is_seven(&self) -> bool {
        *self == ICSR::SEVEN
    }
    #[doc = "Checks if the value of the field is `TCMP0`"]
    #[inline]
    pub fn is_tcmp0(&self) -> bool {
        *self == ICSR::TCMP0
    }
    #[doc = "Checks if the value of the field is `TCMP1`"]
    #[inline]
    pub fn is_tcmp1(&self) -> bool {
        *self == ICSR::TCMP1
    }
    #[doc = "Checks if the value of the field is `TCMP2`"]
    #[inline]
    pub fn is_tcmp2(&self) -> bool {
        *self == ICSR::TCMP2
    }
}
#[doc = "Possible values of the field `CSHOLD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSHOLDR {
    #[doc = "Disable CS being asserted after the end of transmission"]
    ZERO,
    #[doc = "CS is asserted for 1 baud-times after the end of transmission"]
    ONE,
    #[doc = "CS is asserted for 2 baud-times after the end of transmission"]
    TWO,
    #[doc = "CS is asserted for 3 baud-times after the end of transmission"]
    THREE,
    #[doc = "CS is asserted for 7 baud-times after the end of transmission"]
    SEVEN,
    #[doc = "CS is asserted after the end of transmission for TCMPVAL0 baud-times"]
    TCMP0,
    #[doc = "CS is asserted after the end of transmission for TCMPVAL1 baud-times"]
    TCMP1,
    #[doc = "CS is asserted after the end of transmission for TCMPVAL2 baud-times"]
    TCMP2,
}
impl CSHOLDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSHOLDR::ZERO => 0,
            CSHOLDR::ONE => 1,
            CSHOLDR::TWO => 2,
            CSHOLDR::THREE => 3,
            CSHOLDR::SEVEN => 4,
            CSHOLDR::TCMP0 => 5,
            CSHOLDR::TCMP1 => 6,
            CSHOLDR::TCMP2 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSHOLDR {
        match value {
            0 => CSHOLDR::ZERO,
            1 => CSHOLDR::ONE,
            2 => CSHOLDR::TWO,
            3 => CSHOLDR::THREE,
            4 => CSHOLDR::SEVEN,
            5 => CSHOLDR::TCMP0,
            6 => CSHOLDR::TCMP1,
            7 => CSHOLDR::TCMP2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CSHOLDR::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CSHOLDR::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline]
    pub fn is_two(&self) -> bool {
        *self == CSHOLDR::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline]
    pub fn is_three(&self) -> bool {
        *self == CSHOLDR::THREE
    }
    #[doc = "Checks if the value of the field is `SEVEN`"]
    #[inline]
    pub fn is_seven(&self) -> bool {
        *self == CSHOLDR::SEVEN
    }
    #[doc = "Checks if the value of the field is `TCMP0`"]
    #[inline]
    pub fn is_tcmp0(&self) -> bool {
        *self == CSHOLDR::TCMP0
    }
    #[doc = "Checks if the value of the field is `TCMP1`"]
    #[inline]
    pub fn is_tcmp1(&self) -> bool {
        *self == CSHOLDR::TCMP1
    }
    #[doc = "Checks if the value of the field is `TCMP2`"]
    #[inline]
    pub fn is_tcmp2(&self) -> bool {
        *self == CSHOLDR::TCMP2
    }
}
#[doc = "Values that can be written to the field `TXDELAY`"]
pub enum TXDELAYW {
    #[doc = "Disable - TXDELAY in USARTn_CTRL can be used for legacy"]
    DISABLE,
    #[doc = "Start of transmission is delayed for 1 baud-times"]
    ONE,
    #[doc = "Start of transmission is delayed for 2 baud-times"]
    TWO,
    #[doc = "Start of transmission is delayed for 3 baud-times"]
    THREE,
    #[doc = "Start of transmission is delayed for 7 baud-times"]
    SEVEN,
    #[doc = "Start of transmission is delayed for TCMPVAL0 baud-times"]
    TCMP0,
    #[doc = "Start of transmission is delayed for TCMPVAL1 baud-times"]
    TCMP1,
    #[doc = "Start of transmission is delayed for TCMPVAL2 baud-times"]
    TCMP2,
}
impl TXDELAYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXDELAYW::DISABLE => 0,
            TXDELAYW::ONE => 1,
            TXDELAYW::TWO => 2,
            TXDELAYW::THREE => 3,
            TXDELAYW::SEVEN => 4,
            TXDELAYW::TCMP0 => 5,
            TXDELAYW::TCMP1 => 6,
            TXDELAYW::TCMP2 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXDELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDELAYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXDELAYW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disable - TXDELAY in USARTn_CTRL can be used for legacy"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXDELAYW::DISABLE)
    }
    #[doc = "Start of transmission is delayed for 1 baud-times"]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(TXDELAYW::ONE)
    }
    #[doc = "Start of transmission is delayed for 2 baud-times"]
    #[inline]
    pub fn two(self) -> &'a mut W {
        self.variant(TXDELAYW::TWO)
    }
    #[doc = "Start of transmission is delayed for 3 baud-times"]
    #[inline]
    pub fn three(self) -> &'a mut W {
        self.variant(TXDELAYW::THREE)
    }
    #[doc = "Start of transmission is delayed for 7 baud-times"]
    #[inline]
    pub fn seven(self) -> &'a mut W {
        self.variant(TXDELAYW::SEVEN)
    }
    #[doc = "Start of transmission is delayed for TCMPVAL0 baud-times"]
    #[inline]
    pub fn tcmp0(self) -> &'a mut W {
        self.variant(TXDELAYW::TCMP0)
    }
    #[doc = "Start of transmission is delayed for TCMPVAL1 baud-times"]
    #[inline]
    pub fn tcmp1(self) -> &'a mut W {
        self.variant(TXDELAYW::TCMP1)
    }
    #[doc = "Start of transmission is delayed for TCMPVAL2 baud-times"]
    #[inline]
    pub fn tcmp2(self) -> &'a mut W {
        self.variant(TXDELAYW::TCMP2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CSSETUP`"]
pub enum CSSETUPW {
    #[doc = "CS is not asserted before start of transmission"]
    ZERO,
    #[doc = "CS is asserted for 1 baud-times before start of transmission"]
    ONE,
    #[doc = "CS is asserted for 2 baud-times before start of transmission"]
    TWO,
    #[doc = "CS is asserted for 3 baud-times before start of transmission"]
    THREE,
    #[doc = "CS is asserted for 7 baud-times before start of transmission"]
    SEVEN,
    #[doc = "CS is asserted before the start of transmission for TCMPVAL0 baud-times"]
    TCMP0,
    #[doc = "CS is asserted before the start of transmission for TCMPVAL1 baud-times"]
    TCMP1,
    #[doc = "CS is asserted before the start of transmission for TCMPVAL2 baud-times"]
    TCMP2,
}
impl CSSETUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSSETUPW::ZERO => 0,
            CSSETUPW::ONE => 1,
            CSSETUPW::TWO => 2,
            CSSETUPW::THREE => 3,
            CSSETUPW::SEVEN => 4,
            CSSETUPW::TCMP0 => 5,
            CSSETUPW::TCMP1 => 6,
            CSSETUPW::TCMP2 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSSETUPW<'a> {
    w: &'a mut W,
}
impl<'a> _CSSETUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSSETUPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CS is not asserted before start of transmission"]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CSSETUPW::ZERO)
    }
    #[doc = "CS is asserted for 1 baud-times before start of transmission"]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CSSETUPW::ONE)
    }
    #[doc = "CS is asserted for 2 baud-times before start of transmission"]
    #[inline]
    pub fn two(self) -> &'a mut W {
        self.variant(CSSETUPW::TWO)
    }
    #[doc = "CS is asserted for 3 baud-times before start of transmission"]
    #[inline]
    pub fn three(self) -> &'a mut W {
        self.variant(CSSETUPW::THREE)
    }
    #[doc = "CS is asserted for 7 baud-times before start of transmission"]
    #[inline]
    pub fn seven(self) -> &'a mut W {
        self.variant(CSSETUPW::SEVEN)
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL0 baud-times"]
    #[inline]
    pub fn tcmp0(self) -> &'a mut W {
        self.variant(CSSETUPW::TCMP0)
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL1 baud-times"]
    #[inline]
    pub fn tcmp1(self) -> &'a mut W {
        self.variant(CSSETUPW::TCMP1)
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL2 baud-times"]
    #[inline]
    pub fn tcmp2(self) -> &'a mut W {
        self.variant(CSSETUPW::TCMP2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ICS`"]
pub enum ICSW {
    #[doc = "There is no space between charcters"]
    ZERO,
    #[doc = "Create a space of 1 baud-times before start of transmission "]
    ONE,
    #[doc = "Create a space of 2 baud-times before start of transmission"]
    TWO,
    #[doc = "Create a space of 3 baud-times before start of transmission"]
    THREE,
    #[doc = "Create a space of 7 baud-times before start of transmission"]
    SEVEN,
    #[doc = "Create a space of before the start of transmission for TCMPVAL0 baud-times"]
    TCMP0,
    #[doc = "Create a space of before the start of transmission for TCMPVAL1 baud-times"]
    TCMP1,
    #[doc = "Create a space of before the start of transmission for TCMPVAL2 baud-times"]
    TCMP2,
}
impl ICSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICSW::ZERO => 0,
            ICSW::ONE => 1,
            ICSW::TWO => 2,
            ICSW::THREE => 3,
            ICSW::SEVEN => 4,
            ICSW::TCMP0 => 5,
            ICSW::TCMP1 => 6,
            ICSW::TCMP2 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICSW<'a> {
    w: &'a mut W,
}
impl<'a> _ICSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "There is no space between charcters"]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(ICSW::ZERO)
    }
    #[doc = "Create a space of 1 baud-times before start of transmission"]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(ICSW::ONE)
    }
    #[doc = "Create a space of 2 baud-times before start of transmission"]
    #[inline]
    pub fn two(self) -> &'a mut W {
        self.variant(ICSW::TWO)
    }
    #[doc = "Create a space of 3 baud-times before start of transmission"]
    #[inline]
    pub fn three(self) -> &'a mut W {
        self.variant(ICSW::THREE)
    }
    #[doc = "Create a space of 7 baud-times before start of transmission"]
    #[inline]
    pub fn seven(self) -> &'a mut W {
        self.variant(ICSW::SEVEN)
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL0 baud-times"]
    #[inline]
    pub fn tcmp0(self) -> &'a mut W {
        self.variant(ICSW::TCMP0)
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL1 baud-times"]
    #[inline]
    pub fn tcmp1(self) -> &'a mut W {
        self.variant(ICSW::TCMP1)
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL2 baud-times"]
    #[inline]
    pub fn tcmp2(self) -> &'a mut W {
        self.variant(ICSW::TCMP2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CSHOLD`"]
pub enum CSHOLDW {
    #[doc = "Disable CS being asserted after the end of transmission"]
    ZERO,
    #[doc = "CS is asserted for 1 baud-times after the end of transmission"]
    ONE,
    #[doc = "CS is asserted for 2 baud-times after the end of transmission"]
    TWO,
    #[doc = "CS is asserted for 3 baud-times after the end of transmission"]
    THREE,
    #[doc = "CS is asserted for 7 baud-times after the end of transmission"]
    SEVEN,
    #[doc = "CS is asserted after the end of transmission for TCMPVAL0 baud-times"]
    TCMP0,
    #[doc = "CS is asserted after the end of transmission for TCMPVAL1 baud-times"]
    TCMP1,
    #[doc = "CS is asserted after the end of transmission for TCMPVAL2 baud-times"]
    TCMP2,
}
impl CSHOLDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSHOLDW::ZERO => 0,
            CSHOLDW::ONE => 1,
            CSHOLDW::TWO => 2,
            CSHOLDW::THREE => 3,
            CSHOLDW::SEVEN => 4,
            CSHOLDW::TCMP0 => 5,
            CSHOLDW::TCMP1 => 6,
            CSHOLDW::TCMP2 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSHOLDW<'a> {
    w: &'a mut W,
}
impl<'a> _CSHOLDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSHOLDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disable CS being asserted after the end of transmission"]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CSHOLDW::ZERO)
    }
    #[doc = "CS is asserted for 1 baud-times after the end of transmission"]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CSHOLDW::ONE)
    }
    #[doc = "CS is asserted for 2 baud-times after the end of transmission"]
    #[inline]
    pub fn two(self) -> &'a mut W {
        self.variant(CSHOLDW::TWO)
    }
    #[doc = "CS is asserted for 3 baud-times after the end of transmission"]
    #[inline]
    pub fn three(self) -> &'a mut W {
        self.variant(CSHOLDW::THREE)
    }
    #[doc = "CS is asserted for 7 baud-times after the end of transmission"]
    #[inline]
    pub fn seven(self) -> &'a mut W {
        self.variant(CSHOLDW::SEVEN)
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL0 baud-times"]
    #[inline]
    pub fn tcmp0(self) -> &'a mut W {
        self.variant(CSHOLDW::TCMP0)
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL1 baud-times"]
    #[inline]
    pub fn tcmp1(self) -> &'a mut W {
        self.variant(CSHOLDW::TCMP1)
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL2 baud-times"]
    #[inline]
    pub fn tcmp2(self) -> &'a mut W {
        self.variant(CSHOLDW::TCMP2)
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
    #[doc = "Bits 16:18 - TX Frame Start Delay"]
    #[inline]
    pub fn txdelay(&self) -> TXDELAYR {
        TXDELAYR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:22 - Chip Select Setup"]
    #[inline]
    pub fn cssetup(&self) -> CSSETUPR {
        CSSETUPR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:26 - Inter-character Spacing"]
    #[inline]
    pub fn ics(&self) -> ICSR {
        ICSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:30 - Chip Select Hold"]
    #[inline]
    pub fn cshold(&self) -> CSHOLDR {
        CSHOLDR::_from({
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
    #[doc = "Bits 16:18 - TX Frame Start Delay"]
    #[inline]
    pub fn txdelay(&mut self) -> _TXDELAYW {
        _TXDELAYW { w: self }
    }
    #[doc = "Bits 20:22 - Chip Select Setup"]
    #[inline]
    pub fn cssetup(&mut self) -> _CSSETUPW {
        _CSSETUPW { w: self }
    }
    #[doc = "Bits 24:26 - Inter-character Spacing"]
    #[inline]
    pub fn ics(&mut self) -> _ICSW {
        _ICSW { w: self }
    }
    #[doc = "Bits 28:30 - Chip Select Hold"]
    #[inline]
    pub fn cshold(&mut self) -> _CSHOLDW {
        _CSHOLDW { w: self }
    }
}
