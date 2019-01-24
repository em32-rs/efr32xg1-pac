#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCDCMISCCTRL {
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
pub struct LNFORCECCMR {
    bits: bool,
}
impl LNFORCECCMR {
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
pub struct PFETCNTR {
    bits: u8,
}
impl PFETCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NFETCNTR {
    bits: u8,
}
impl NFETCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BYPLIMSELR {
    bits: u8,
}
impl BYPLIMSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LPCLIMILIMSELR {
    bits: u8,
}
impl LPCLIMILIMSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNCLIMILIMSELR {
    bits: u8,
}
impl LNCLIMILIMSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `LPCMPBIAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPCMPBIASR {
    #[doc = "Maximum load current less than 75uA."]
    BIAS0,
    #[doc = "Maximum load current less than 500uA."]
    BIAS1,
    #[doc = "Maximum load current less than 2.5mA."]
    BIAS2,
    #[doc = "Maximum load current less than 10mA."]
    BIAS3,
}
impl LPCMPBIASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPCMPBIASR::BIAS0 => 0,
            LPCMPBIASR::BIAS1 => 1,
            LPCMPBIASR::BIAS2 => 2,
            LPCMPBIASR::BIAS3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPCMPBIASR {
        match value {
            0 => LPCMPBIASR::BIAS0,
            1 => LPCMPBIASR::BIAS1,
            2 => LPCMPBIASR::BIAS2,
            3 => LPCMPBIASR::BIAS3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS0`"]
    #[inline]
    pub fn is_bias0(&self) -> bool {
        *self == LPCMPBIASR::BIAS0
    }
    #[doc = "Checks if the value of the field is `BIAS1`"]
    #[inline]
    pub fn is_bias1(&self) -> bool {
        *self == LPCMPBIASR::BIAS1
    }
    #[doc = "Checks if the value of the field is `BIAS2`"]
    #[inline]
    pub fn is_bias2(&self) -> bool {
        *self == LPCMPBIASR::BIAS2
    }
    #[doc = "Checks if the value of the field is `BIAS3`"]
    #[inline]
    pub fn is_bias3(&self) -> bool {
        *self == LPCMPBIASR::BIAS3
    }
}
#[doc = r" Proxy"]
pub struct _LNFORCECCMW<'a> {
    w: &'a mut W,
}
impl<'a> _LNFORCECCMW<'a> {
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
pub struct _PFETCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _PFETCNTW<'a> {
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
pub struct _NFETCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _NFETCNTW<'a> {
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
#[doc = r" Proxy"]
pub struct _BYPLIMSELW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPLIMSELW<'a> {
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
#[doc = r" Proxy"]
pub struct _LPCLIMILIMSELW<'a> {
    w: &'a mut W,
}
impl<'a> _LPCLIMILIMSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LNCLIMILIMSELW<'a> {
    w: &'a mut W,
}
impl<'a> _LNCLIMILIMSELW<'a> {
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
#[doc = "Values that can be written to the field `LPCMPBIAS`"]
pub enum LPCMPBIASW {
    #[doc = "Maximum load current less than 75uA."]
    BIAS0,
    #[doc = "Maximum load current less than 500uA."]
    BIAS1,
    #[doc = "Maximum load current less than 2.5mA."]
    BIAS2,
    #[doc = "Maximum load current less than 10mA."]
    BIAS3,
}
impl LPCMPBIASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPCMPBIASW::BIAS0 => 0,
            LPCMPBIASW::BIAS1 => 1,
            LPCMPBIASW::BIAS2 => 2,
            LPCMPBIASW::BIAS3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPCMPBIASW<'a> {
    w: &'a mut W,
}
impl<'a> _LPCMPBIASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPCMPBIASW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Maximum load current less than 75uA."]
    #[inline]
    pub fn bias0(self) -> &'a mut W {
        self.variant(LPCMPBIASW::BIAS0)
    }
    #[doc = "Maximum load current less than 500uA."]
    #[inline]
    pub fn bias1(self) -> &'a mut W {
        self.variant(LPCMPBIASW::BIAS1)
    }
    #[doc = "Maximum load current less than 2.5mA."]
    #[inline]
    pub fn bias2(self) -> &'a mut W {
        self.variant(LPCMPBIASW::BIAS2)
    }
    #[doc = "Maximum load current less than 10mA."]
    #[inline]
    pub fn bias3(self) -> &'a mut W {
        self.variant(LPCMPBIASW::BIAS3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bit 0 - Force DCDC Into CCM Mode in Low Noise Operation"]
    #[inline]
    pub fn lnforceccm(&self) -> LNFORCECCMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LNFORCECCMR { bits }
    }
    #[doc = "Bits 8:11 - PFET Switch Number Selection"]
    #[inline]
    pub fn pfetcnt(&self) -> PFETCNTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PFETCNTR { bits }
    }
    #[doc = "Bits 12:15 - NFET Switch Number Selection"]
    #[inline]
    pub fn nfetcnt(&self) -> NFETCNTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NFETCNTR { bits }
    }
    #[doc = "Bits 16:19 - Current Limit in Bypass Mode"]
    #[inline]
    pub fn byplimsel(&self) -> BYPLIMSELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BYPLIMSELR { bits }
    }
    #[doc = "Bits 20:22 - Current Limit Level Selection for Current Limiter in LP Mode"]
    #[inline]
    pub fn lpclimilimsel(&self) -> LPCLIMILIMSELR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPCLIMILIMSELR { bits }
    }
    #[doc = "Bits 24:26 - Current Limit Level Selection for Current Limiter in LN Mode"]
    #[inline]
    pub fn lnclimilimsel(&self) -> LNCLIMILIMSELR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNCLIMILIMSELR { bits }
    }
    #[doc = "Bits 28:29 - LP Mode Comparator Bias Selection"]
    #[inline]
    pub fn lpcmpbias(&self) -> LPCMPBIASR {
        LPCMPBIASR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 858814208 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Force DCDC Into CCM Mode in Low Noise Operation"]
    #[inline]
    pub fn lnforceccm(&mut self) -> _LNFORCECCMW {
        _LNFORCECCMW { w: self }
    }
    #[doc = "Bits 8:11 - PFET Switch Number Selection"]
    #[inline]
    pub fn pfetcnt(&mut self) -> _PFETCNTW {
        _PFETCNTW { w: self }
    }
    #[doc = "Bits 12:15 - NFET Switch Number Selection"]
    #[inline]
    pub fn nfetcnt(&mut self) -> _NFETCNTW {
        _NFETCNTW { w: self }
    }
    #[doc = "Bits 16:19 - Current Limit in Bypass Mode"]
    #[inline]
    pub fn byplimsel(&mut self) -> _BYPLIMSELW {
        _BYPLIMSELW { w: self }
    }
    #[doc = "Bits 20:22 - Current Limit Level Selection for Current Limiter in LP Mode"]
    #[inline]
    pub fn lpclimilimsel(&mut self) -> _LPCLIMILIMSELW {
        _LPCLIMILIMSELW { w: self }
    }
    #[doc = "Bits 24:26 - Current Limit Level Selection for Current Limiter in LN Mode"]
    #[inline]
    pub fn lnclimilimsel(&mut self) -> _LNCLIMILIMSELW {
        _LNCLIMILIMSELW { w: self }
    }
    #[doc = "Bits 28:29 - LP Mode Comparator Bias Selection"]
    #[inline]
    pub fn lpcmpbias(&mut self) -> _LPCMPBIASW {
        _LPCMPBIASW { w: self }
    }
}
