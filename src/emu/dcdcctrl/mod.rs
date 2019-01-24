#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCDCCTRL {
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
#[doc = "Possible values of the field `DCDCMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDCMODER {
    #[doc = "DCDC regulator is operating in bypass mode. Prior to configuring DCDCMODE=BYPASS, software must set EMU_DCDCCLIMCTRL.BYPLIMEN=1 to prevent excessive current between VREGVDD and DVDD supplies."]
    BYPASS,
    #[doc = "DCDC regulator is operating in low noise mode."]
    LOWNOISE,
    #[doc = "DCDC regulator is operating in low power mode."]
    LOWPOWER,
    #[doc = "DCDC regulator is off and the bypass switch is off. Note: DVDD must be supplied externally"]
    OFF,
}
impl DCDCMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DCDCMODER::BYPASS => 0,
            DCDCMODER::LOWNOISE => 1,
            DCDCMODER::LOWPOWER => 2,
            DCDCMODER::OFF => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DCDCMODER {
        match value {
            0 => DCDCMODER::BYPASS,
            1 => DCDCMODER::LOWNOISE,
            2 => DCDCMODER::LOWPOWER,
            3 => DCDCMODER::OFF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline]
    pub fn is_bypass(&self) -> bool {
        *self == DCDCMODER::BYPASS
    }
    #[doc = "Checks if the value of the field is `LOWNOISE`"]
    #[inline]
    pub fn is_lownoise(&self) -> bool {
        *self == DCDCMODER::LOWNOISE
    }
    #[doc = "Checks if the value of the field is `LOWPOWER`"]
    #[inline]
    pub fn is_lowpower(&self) -> bool {
        *self == DCDCMODER::LOWPOWER
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == DCDCMODER::OFF
    }
}
#[doc = r" Value of the field"]
pub struct DCDCMODEEM23R {
    bits: bool,
}
impl DCDCMODEEM23R {
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
pub struct DCDCMODEEM4R {
    bits: bool,
}
impl DCDCMODEEM4R {
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
#[doc = "Values that can be written to the field `DCDCMODE`"]
pub enum DCDCMODEW {
    #[doc = "DCDC regulator is operating in bypass mode. Prior to configuring DCDCMODE=BYPASS, software must set EMU_DCDCCLIMCTRL.BYPLIMEN=1 to prevent excessive current between VREGVDD and DVDD supplies."]
    BYPASS,
    #[doc = "DCDC regulator is operating in low noise mode."]
    LOWNOISE,
    #[doc = "DCDC regulator is operating in low power mode."]
    LOWPOWER,
    #[doc = "DCDC regulator is off and the bypass switch is off. Note: DVDD must be supplied externally"]
    OFF,
}
impl DCDCMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DCDCMODEW::BYPASS => 0,
            DCDCMODEW::LOWNOISE => 1,
            DCDCMODEW::LOWPOWER => 2,
            DCDCMODEW::OFF => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCDCMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDCMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCDCMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "DCDC regulator is operating in bypass mode. Prior to configuring DCDCMODE=BYPASS, software must set EMU_DCDCCLIMCTRL.BYPLIMEN=1 to prevent excessive current between VREGVDD and DVDD supplies."]
    #[inline]
    pub fn bypass(self) -> &'a mut W {
        self.variant(DCDCMODEW::BYPASS)
    }
    #[doc = "DCDC regulator is operating in low noise mode."]
    #[inline]
    pub fn lownoise(self) -> &'a mut W {
        self.variant(DCDCMODEW::LOWNOISE)
    }
    #[doc = "DCDC regulator is operating in low power mode."]
    #[inline]
    pub fn lowpower(self) -> &'a mut W {
        self.variant(DCDCMODEW::LOWPOWER)
    }
    #[doc = "DCDC regulator is off and the bypass switch is off. Note: DVDD must be supplied externally"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(DCDCMODEW::OFF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCDCMODEEM23W<'a> {
    w: &'a mut W,
}
impl<'a> _DCDCMODEEM23W<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCDCMODEEM4W<'a> {
    w: &'a mut W,
}
impl<'a> _DCDCMODEEM4W<'a> {
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
        const OFFSET: u8 = 5;
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
    #[doc = "Bits 0:1 - Regulator Mode"]
    #[inline]
    pub fn dcdcmode(&self) -> DCDCMODER {
        DCDCMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - DCDC Mode EM23"]
    #[inline]
    pub fn dcdcmodeem23(&self) -> DCDCMODEEM23R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDCMODEEM23R { bits }
    }
    #[doc = "Bit 5 - DCDC Mode EM4H"]
    #[inline]
    pub fn dcdcmodeem4(&self) -> DCDCMODEEM4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDCMODEEM4R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 48 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Regulator Mode"]
    #[inline]
    pub fn dcdcmode(&mut self) -> _DCDCMODEW {
        _DCDCMODEW { w: self }
    }
    #[doc = "Bit 4 - DCDC Mode EM23"]
    #[inline]
    pub fn dcdcmodeem23(&mut self) -> _DCDCMODEEM23W {
        _DCDCMODEEM23W { w: self }
    }
    #[doc = "Bit 5 - DCDC Mode EM4H"]
    #[inline]
    pub fn dcdcmodeem4(&mut self) -> _DCDCMODEEM4W {
        _DCDCMODEEM4W { w: self }
    }
}
