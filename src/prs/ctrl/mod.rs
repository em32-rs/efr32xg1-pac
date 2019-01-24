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
pub struct SEVONPRSR {
    bits: bool,
}
impl SEVONPRSR {
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
#[doc = "Possible values of the field `SEVONPRSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEVONPRSSELR {
    #[doc = "PRS Channel 0 selected"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected"]
    PRSCH7,
    #[doc = "PRS Channel 8 selected"]
    PRSCH8,
    #[doc = "PRS Channel 9 selected"]
    PRSCH9,
    #[doc = "PRS Channel 10 selected"]
    PRSCH10,
    #[doc = "PRS Channel 11 selected"]
    PRSCH11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SEVONPRSSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SEVONPRSSELR::PRSCH0 => 0,
            SEVONPRSSELR::PRSCH1 => 1,
            SEVONPRSSELR::PRSCH2 => 2,
            SEVONPRSSELR::PRSCH3 => 3,
            SEVONPRSSELR::PRSCH4 => 4,
            SEVONPRSSELR::PRSCH5 => 5,
            SEVONPRSSELR::PRSCH6 => 6,
            SEVONPRSSELR::PRSCH7 => 7,
            SEVONPRSSELR::PRSCH8 => 8,
            SEVONPRSSELR::PRSCH9 => 9,
            SEVONPRSSELR::PRSCH10 => 10,
            SEVONPRSSELR::PRSCH11 => 11,
            SEVONPRSSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SEVONPRSSELR {
        match value {
            0 => SEVONPRSSELR::PRSCH0,
            1 => SEVONPRSSELR::PRSCH1,
            2 => SEVONPRSSELR::PRSCH2,
            3 => SEVONPRSSELR::PRSCH3,
            4 => SEVONPRSSELR::PRSCH4,
            5 => SEVONPRSSELR::PRSCH5,
            6 => SEVONPRSSELR::PRSCH6,
            7 => SEVONPRSSELR::PRSCH7,
            8 => SEVONPRSSELR::PRSCH8,
            9 => SEVONPRSSELR::PRSCH9,
            10 => SEVONPRSSELR::PRSCH10,
            11 => SEVONPRSSELR::PRSCH11,
            i => SEVONPRSSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == SEVONPRSSELR::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == SEVONPRSSELR::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == SEVONPRSSELR::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == SEVONPRSSELR::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == SEVONPRSSELR::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == SEVONPRSSELR::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline]
    pub fn is_prsch6(&self) -> bool {
        *self == SEVONPRSSELR::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline]
    pub fn is_prsch7(&self) -> bool {
        *self == SEVONPRSSELR::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline]
    pub fn is_prsch8(&self) -> bool {
        *self == SEVONPRSSELR::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline]
    pub fn is_prsch9(&self) -> bool {
        *self == SEVONPRSSELR::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline]
    pub fn is_prsch10(&self) -> bool {
        *self == SEVONPRSSELR::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline]
    pub fn is_prsch11(&self) -> bool {
        *self == SEVONPRSSELR::PRSCH11
    }
}
#[doc = r" Proxy"]
pub struct _SEVONPRSW<'a> {
    w: &'a mut W,
}
impl<'a> _SEVONPRSW<'a> {
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
#[doc = "Values that can be written to the field `SEVONPRSSEL`"]
pub enum SEVONPRSSELW {
    #[doc = "PRS Channel 0 selected"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected"]
    PRSCH7,
    #[doc = "PRS Channel 8 selected"]
    PRSCH8,
    #[doc = "PRS Channel 9 selected"]
    PRSCH9,
    #[doc = "PRS Channel 10 selected"]
    PRSCH10,
    #[doc = "PRS Channel 11 selected"]
    PRSCH11,
}
impl SEVONPRSSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SEVONPRSSELW::PRSCH0 => 0,
            SEVONPRSSELW::PRSCH1 => 1,
            SEVONPRSSELW::PRSCH2 => 2,
            SEVONPRSSELW::PRSCH3 => 3,
            SEVONPRSSELW::PRSCH4 => 4,
            SEVONPRSSELW::PRSCH5 => 5,
            SEVONPRSSELW::PRSCH6 => 6,
            SEVONPRSSELW::PRSCH7 => 7,
            SEVONPRSSELW::PRSCH8 => 8,
            SEVONPRSSELW::PRSCH9 => 9,
            SEVONPRSSELW::PRSCH10 => 10,
            SEVONPRSSELW::PRSCH11 => 11,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEVONPRSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SEVONPRSSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEVONPRSSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected"]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(SEVONPRSSELW::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(SEVONPRSSELW::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(SEVONPRSSELW::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(SEVONPRSSELW::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(SEVONPRSSELW::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(SEVONPRSSELW::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(SEVONPRSSELW::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(SEVONPRSSELW::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected"]
    #[inline]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(SEVONPRSSELW::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected"]
    #[inline]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(SEVONPRSSELW::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected"]
    #[inline]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(SEVONPRSSELW::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected"]
    #[inline]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(SEVONPRSSELW::PRSCH11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Set Event on PRS"]
    #[inline]
    pub fn sevonprs(&self) -> SEVONPRSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SEVONPRSR { bits }
    }
    #[doc = "Bits 1:4 - SEVONPRS PRS Channel Select"]
    #[inline]
    pub fn sevonprssel(&self) -> SEVONPRSSELR {
        SEVONPRSSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Set Event on PRS"]
    #[inline]
    pub fn sevonprs(&mut self) -> _SEVONPRSW {
        _SEVONPRSW { w: self }
    }
    #[doc = "Bits 1:4 - SEVONPRS PRS Channel Select"]
    #[inline]
    pub fn sevonprssel(&mut self) -> _SEVONPRSSELW {
        _SEVONPRSSELW { w: self }
    }
}
