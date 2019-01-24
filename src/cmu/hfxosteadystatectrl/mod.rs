#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HFXOSTEADYSTATECTRL {
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
pub struct IBTRIMXOCORER {
    bits: u8,
}
impl IBTRIMXOCORER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct REGISHR {
    bits: u8,
}
impl REGISHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTUNER {
    bits: u16,
}
impl CTUNER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct REGSELILOWR {
    bits: u8,
}
impl REGSELILOWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PEAKDETENR {
    bits: bool,
}
impl PEAKDETENR {
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
pub struct REGISHUPPERR {
    bits: u8,
}
impl REGISHUPPERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _IBTRIMXOCOREW<'a> {
    w: &'a mut W,
}
impl<'a> _IBTRIMXOCOREW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _REGISHW<'a> {
    w: &'a mut W,
}
impl<'a> _REGISHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTUNEW<'a> {
    w: &'a mut W,
}
impl<'a> _CTUNEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _REGSELILOWW<'a> {
    w: &'a mut W,
}
impl<'a> _REGSELILOWW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PEAKDETENW<'a> {
    w: &'a mut W,
}
impl<'a> _PEAKDETENW<'a> {
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
#[doc = r" Proxy"]
pub struct _REGISHUPPERW<'a> {
    w: &'a mut W,
}
impl<'a> _REGISHUPPERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:6 - Sets the Steady State Oscillator Core Bias Current."]
    #[inline]
    pub fn ibtrimxocore(&self) -> IBTRIMXOCORER {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IBTRIMXOCORER { bits }
    }
    #[doc = "Bits 7:10 - Sets the Steady State Regulator Output Current Level (shunt Regulator)"]
    #[inline]
    pub fn regish(&self) -> REGISHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REGISHR { bits }
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline]
    pub fn ctune(&self) -> CTUNER {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CTUNER { bits }
    }
    #[doc = "Bits 24:25 - Controls Regulator Minimum Shunt Current Detection Relative to Nominal"]
    #[inline]
    pub fn regselilow(&self) -> REGSELILOWR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REGSELILOWR { bits }
    }
    #[doc = "Bit 26 - Enables Oscillator Peak Detectors"]
    #[inline]
    pub fn peakdeten(&self) -> PEAKDETENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PEAKDETENR { bits }
    }
    #[doc = "Bits 28:31 - Set Regulator Output Current Level (shunt Regulator). Ish = 120uA + REGISHUPPER X 120uA"]
    #[inline]
    pub fn regishupper(&self) -> REGISHUPPERR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REGISHUPPERR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2735385865 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - Sets the Steady State Oscillator Core Bias Current."]
    #[inline]
    pub fn ibtrimxocore(&mut self) -> _IBTRIMXOCOREW {
        _IBTRIMXOCOREW { w: self }
    }
    #[doc = "Bits 7:10 - Sets the Steady State Regulator Output Current Level (shunt Regulator)"]
    #[inline]
    pub fn regish(&mut self) -> _REGISHW {
        _REGISHW { w: self }
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline]
    pub fn ctune(&mut self) -> _CTUNEW {
        _CTUNEW { w: self }
    }
    #[doc = "Bits 24:25 - Controls Regulator Minimum Shunt Current Detection Relative to Nominal"]
    #[inline]
    pub fn regselilow(&mut self) -> _REGSELILOWW {
        _REGSELILOWW { w: self }
    }
    #[doc = "Bit 26 - Enables Oscillator Peak Detectors"]
    #[inline]
    pub fn peakdeten(&mut self) -> _PEAKDETENW {
        _PEAKDETENW { w: self }
    }
    #[doc = "Bits 28:31 - Set Regulator Output Current Level (shunt Regulator). Ish = 120uA + REGISHUPPER X 120uA"]
    #[inline]
    pub fn regishupper(&mut self) -> _REGISHUPPERW {
        _REGISHUPPERW { w: self }
    }
}
