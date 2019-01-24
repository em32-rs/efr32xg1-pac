#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HYSTERESIS0 {
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
#[doc = "Possible values of the field `HYST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYSTR {
    #[doc = "No hysteresis"]
    HYST0,
    #[doc = "14 mV hysteresis"]
    HYST1,
    #[doc = "25 mV hysteresis"]
    HYST2,
    #[doc = "30 mV hysteresis"]
    HYST3,
    #[doc = "35 mV hysteresis"]
    HYST4,
    #[doc = "39 mV hysteresis"]
    HYST5,
    #[doc = "42 mV hysteresis"]
    HYST6,
    #[doc = "45 mV hysteresis"]
    HYST7,
    #[doc = "No hysteresis"]
    HYST8,
    #[doc = "-14 mV hysteresis"]
    HYST9,
    #[doc = "-25 mV hysteresis"]
    HYST10,
    #[doc = "-30 mV hysteresis"]
    HYST11,
    #[doc = "-35 mV hysteresis"]
    HYST12,
    #[doc = "-39 mV hysteresis"]
    HYST13,
    #[doc = "-42 mV hysteresis"]
    HYST14,
    #[doc = "-45 mV hysteresis"]
    HYST15,
}
impl HYSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HYSTR::HYST0 => 0,
            HYSTR::HYST1 => 1,
            HYSTR::HYST2 => 2,
            HYSTR::HYST3 => 3,
            HYSTR::HYST4 => 4,
            HYSTR::HYST5 => 5,
            HYSTR::HYST6 => 6,
            HYSTR::HYST7 => 7,
            HYSTR::HYST8 => 8,
            HYSTR::HYST9 => 9,
            HYSTR::HYST10 => 10,
            HYSTR::HYST11 => 11,
            HYSTR::HYST12 => 12,
            HYSTR::HYST13 => 13,
            HYSTR::HYST14 => 14,
            HYSTR::HYST15 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HYSTR {
        match value {
            0 => HYSTR::HYST0,
            1 => HYSTR::HYST1,
            2 => HYSTR::HYST2,
            3 => HYSTR::HYST3,
            4 => HYSTR::HYST4,
            5 => HYSTR::HYST5,
            6 => HYSTR::HYST6,
            7 => HYSTR::HYST7,
            8 => HYSTR::HYST8,
            9 => HYSTR::HYST9,
            10 => HYSTR::HYST10,
            11 => HYSTR::HYST11,
            12 => HYSTR::HYST12,
            13 => HYSTR::HYST13,
            14 => HYSTR::HYST14,
            15 => HYSTR::HYST15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HYST0`"]
    #[inline]
    pub fn is_hyst0(&self) -> bool {
        *self == HYSTR::HYST0
    }
    #[doc = "Checks if the value of the field is `HYST1`"]
    #[inline]
    pub fn is_hyst1(&self) -> bool {
        *self == HYSTR::HYST1
    }
    #[doc = "Checks if the value of the field is `HYST2`"]
    #[inline]
    pub fn is_hyst2(&self) -> bool {
        *self == HYSTR::HYST2
    }
    #[doc = "Checks if the value of the field is `HYST3`"]
    #[inline]
    pub fn is_hyst3(&self) -> bool {
        *self == HYSTR::HYST3
    }
    #[doc = "Checks if the value of the field is `HYST4`"]
    #[inline]
    pub fn is_hyst4(&self) -> bool {
        *self == HYSTR::HYST4
    }
    #[doc = "Checks if the value of the field is `HYST5`"]
    #[inline]
    pub fn is_hyst5(&self) -> bool {
        *self == HYSTR::HYST5
    }
    #[doc = "Checks if the value of the field is `HYST6`"]
    #[inline]
    pub fn is_hyst6(&self) -> bool {
        *self == HYSTR::HYST6
    }
    #[doc = "Checks if the value of the field is `HYST7`"]
    #[inline]
    pub fn is_hyst7(&self) -> bool {
        *self == HYSTR::HYST7
    }
    #[doc = "Checks if the value of the field is `HYST8`"]
    #[inline]
    pub fn is_hyst8(&self) -> bool {
        *self == HYSTR::HYST8
    }
    #[doc = "Checks if the value of the field is `HYST9`"]
    #[inline]
    pub fn is_hyst9(&self) -> bool {
        *self == HYSTR::HYST9
    }
    #[doc = "Checks if the value of the field is `HYST10`"]
    #[inline]
    pub fn is_hyst10(&self) -> bool {
        *self == HYSTR::HYST10
    }
    #[doc = "Checks if the value of the field is `HYST11`"]
    #[inline]
    pub fn is_hyst11(&self) -> bool {
        *self == HYSTR::HYST11
    }
    #[doc = "Checks if the value of the field is `HYST12`"]
    #[inline]
    pub fn is_hyst12(&self) -> bool {
        *self == HYSTR::HYST12
    }
    #[doc = "Checks if the value of the field is `HYST13`"]
    #[inline]
    pub fn is_hyst13(&self) -> bool {
        *self == HYSTR::HYST13
    }
    #[doc = "Checks if the value of the field is `HYST14`"]
    #[inline]
    pub fn is_hyst14(&self) -> bool {
        *self == HYSTR::HYST14
    }
    #[doc = "Checks if the value of the field is `HYST15`"]
    #[inline]
    pub fn is_hyst15(&self) -> bool {
        *self == HYSTR::HYST15
    }
}
#[doc = r" Value of the field"]
pub struct DIVVAR {
    bits: u8,
}
impl DIVVAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIVVBR {
    bits: u8,
}
impl DIVVBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `HYST`"]
pub enum HYSTW {
    #[doc = "No hysteresis"]
    HYST0,
    #[doc = "14 mV hysteresis"]
    HYST1,
    #[doc = "25 mV hysteresis"]
    HYST2,
    #[doc = "30 mV hysteresis"]
    HYST3,
    #[doc = "35 mV hysteresis"]
    HYST4,
    #[doc = "39 mV hysteresis"]
    HYST5,
    #[doc = "42 mV hysteresis"]
    HYST6,
    #[doc = "45 mV hysteresis"]
    HYST7,
    #[doc = "No hysteresis"]
    HYST8,
    #[doc = "-14 mV hysteresis"]
    HYST9,
    #[doc = "-25 mV hysteresis"]
    HYST10,
    #[doc = "-30 mV hysteresis"]
    HYST11,
    #[doc = "-35 mV hysteresis"]
    HYST12,
    #[doc = "-39 mV hysteresis"]
    HYST13,
    #[doc = "-42 mV hysteresis"]
    HYST14,
    #[doc = "-45 mV hysteresis"]
    HYST15,
}
impl HYSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HYSTW::HYST0 => 0,
            HYSTW::HYST1 => 1,
            HYSTW::HYST2 => 2,
            HYSTW::HYST3 => 3,
            HYSTW::HYST4 => 4,
            HYSTW::HYST5 => 5,
            HYSTW::HYST6 => 6,
            HYSTW::HYST7 => 7,
            HYSTW::HYST8 => 8,
            HYSTW::HYST9 => 9,
            HYSTW::HYST10 => 10,
            HYSTW::HYST11 => 11,
            HYSTW::HYST12 => 12,
            HYSTW::HYST13 => 13,
            HYSTW::HYST14 => 14,
            HYSTW::HYST15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HYSTW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HYSTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No hysteresis"]
    #[inline]
    pub fn hyst0(self) -> &'a mut W {
        self.variant(HYSTW::HYST0)
    }
    #[doc = "14 mV hysteresis"]
    #[inline]
    pub fn hyst1(self) -> &'a mut W {
        self.variant(HYSTW::HYST1)
    }
    #[doc = "25 mV hysteresis"]
    #[inline]
    pub fn hyst2(self) -> &'a mut W {
        self.variant(HYSTW::HYST2)
    }
    #[doc = "30 mV hysteresis"]
    #[inline]
    pub fn hyst3(self) -> &'a mut W {
        self.variant(HYSTW::HYST3)
    }
    #[doc = "35 mV hysteresis"]
    #[inline]
    pub fn hyst4(self) -> &'a mut W {
        self.variant(HYSTW::HYST4)
    }
    #[doc = "39 mV hysteresis"]
    #[inline]
    pub fn hyst5(self) -> &'a mut W {
        self.variant(HYSTW::HYST5)
    }
    #[doc = "42 mV hysteresis"]
    #[inline]
    pub fn hyst6(self) -> &'a mut W {
        self.variant(HYSTW::HYST6)
    }
    #[doc = "45 mV hysteresis"]
    #[inline]
    pub fn hyst7(self) -> &'a mut W {
        self.variant(HYSTW::HYST7)
    }
    #[doc = "No hysteresis"]
    #[inline]
    pub fn hyst8(self) -> &'a mut W {
        self.variant(HYSTW::HYST8)
    }
    #[doc = "-14 mV hysteresis"]
    #[inline]
    pub fn hyst9(self) -> &'a mut W {
        self.variant(HYSTW::HYST9)
    }
    #[doc = "-25 mV hysteresis"]
    #[inline]
    pub fn hyst10(self) -> &'a mut W {
        self.variant(HYSTW::HYST10)
    }
    #[doc = "-30 mV hysteresis"]
    #[inline]
    pub fn hyst11(self) -> &'a mut W {
        self.variant(HYSTW::HYST11)
    }
    #[doc = "-35 mV hysteresis"]
    #[inline]
    pub fn hyst12(self) -> &'a mut W {
        self.variant(HYSTW::HYST12)
    }
    #[doc = "-39 mV hysteresis"]
    #[inline]
    pub fn hyst13(self) -> &'a mut W {
        self.variant(HYSTW::HYST13)
    }
    #[doc = "-42 mV hysteresis"]
    #[inline]
    pub fn hyst14(self) -> &'a mut W {
        self.variant(HYSTW::HYST14)
    }
    #[doc = "-45 mV hysteresis"]
    #[inline]
    pub fn hyst15(self) -> &'a mut W {
        self.variant(HYSTW::HYST15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIVVAW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVVAW<'a> {
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
pub struct _DIVVBW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVVBW<'a> {
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
    #[doc = "Bits 0:3 - Hysteresis Select When ACMPOUT=0"]
    #[inline]
    pub fn hyst(&self) -> HYSTR {
        HYSTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:21 - Divider for VA Voltage When ACMPOUT=0"]
    #[inline]
    pub fn divva(&self) -> DIVVAR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIVVAR { bits }
    }
    #[doc = "Bits 24:29 - Divider for VB Voltage When ACMPOUT=0"]
    #[inline]
    pub fn divvb(&self) -> DIVVBR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIVVBR { bits }
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
    #[doc = "Bits 0:3 - Hysteresis Select When ACMPOUT=0"]
    #[inline]
    pub fn hyst(&mut self) -> _HYSTW {
        _HYSTW { w: self }
    }
    #[doc = "Bits 16:21 - Divider for VA Voltage When ACMPOUT=0"]
    #[inline]
    pub fn divva(&mut self) -> _DIVVAW {
        _DIVVAW { w: self }
    }
    #[doc = "Bits 24:29 - Divider for VB Voltage When ACMPOUT=0"]
    #[inline]
    pub fn divvb(&mut self) -> _DIVVBW {
        _DIVVBW { w: self }
    }
}
