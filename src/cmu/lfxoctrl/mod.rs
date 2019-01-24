#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LFXOCTRL {
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
pub struct TUNINGR {
    bits: u8,
}
impl TUNINGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "32768 Hz crystal oscillator"]
    XTAL,
    #[doc = "An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32768 Hz)."]
    BUFEXTCLK,
    #[doc = "Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed."]
    DIGEXTCLK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::XTAL => 0,
            MODER::BUFEXTCLK => 1,
            MODER::DIGEXTCLK => 2,
            MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::XTAL,
            1 => MODER::BUFEXTCLK,
            2 => MODER::DIGEXTCLK,
            i => MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline]
    pub fn is_xtal(&self) -> bool {
        *self == MODER::XTAL
    }
    #[doc = "Checks if the value of the field is `BUFEXTCLK`"]
    #[inline]
    pub fn is_bufextclk(&self) -> bool {
        *self == MODER::BUFEXTCLK
    }
    #[doc = "Checks if the value of the field is `DIGEXTCLK`"]
    #[inline]
    pub fn is_digextclk(&self) -> bool {
        *self == MODER::DIGEXTCLK
    }
}
#[doc = r" Value of the field"]
pub struct GAINR {
    bits: u8,
}
impl GAINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HIGHAMPLR {
    bits: bool,
}
impl HIGHAMPLR {
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
pub struct AGCR {
    bits: bool,
}
impl AGCR {
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
pub struct CURR {
    bits: u8,
}
impl CURR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BUFCURR {
    bits: bool,
}
impl BUFCURR {
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
#[doc = "Possible values of the field `TIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUTR {
    #[doc = "Timeout period of 2 cycles"]
    _2CYCLES,
    #[doc = "Timeout period of 256 cycles"]
    _256CYCLES,
    #[doc = "Timeout period of 1024 cycles"]
    _1KCYCLES,
    #[doc = "Timeout period of 2048 cycles"]
    _2KCYCLES,
    #[doc = "Timeout period of 4096 cycles"]
    _4KCYCLES,
    #[doc = "Timeout period of 8192 cycles"]
    _8KCYCLES,
    #[doc = "Timeout period of 16384 cycles"]
    _16KCYCLES,
    #[doc = "Timeout period of 32768 cycles"]
    _32KCYCLES,
}
impl TIMEOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMEOUTR::_2CYCLES => 0,
            TIMEOUTR::_256CYCLES => 1,
            TIMEOUTR::_1KCYCLES => 2,
            TIMEOUTR::_2KCYCLES => 3,
            TIMEOUTR::_4KCYCLES => 4,
            TIMEOUTR::_8KCYCLES => 5,
            TIMEOUTR::_16KCYCLES => 6,
            TIMEOUTR::_32KCYCLES => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMEOUTR {
        match value {
            0 => TIMEOUTR::_2CYCLES,
            1 => TIMEOUTR::_256CYCLES,
            2 => TIMEOUTR::_1KCYCLES,
            3 => TIMEOUTR::_2KCYCLES,
            4 => TIMEOUTR::_4KCYCLES,
            5 => TIMEOUTR::_8KCYCLES,
            6 => TIMEOUTR::_16KCYCLES,
            7 => TIMEOUTR::_32KCYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline]
    pub fn is_2cycles(&self) -> bool {
        *self == TIMEOUTR::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline]
    pub fn is_256cycles(&self) -> bool {
        *self == TIMEOUTR::_256CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline]
    pub fn is_1kcycles(&self) -> bool {
        *self == TIMEOUTR::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_2KCYCLES`"]
    #[inline]
    pub fn is_2kcycles(&self) -> bool {
        *self == TIMEOUTR::_2KCYCLES
    }
    #[doc = "Checks if the value of the field is `_4KCYCLES`"]
    #[inline]
    pub fn is_4kcycles(&self) -> bool {
        *self == TIMEOUTR::_4KCYCLES
    }
    #[doc = "Checks if the value of the field is `_8KCYCLES`"]
    #[inline]
    pub fn is_8kcycles(&self) -> bool {
        *self == TIMEOUTR::_8KCYCLES
    }
    #[doc = "Checks if the value of the field is `_16KCYCLES`"]
    #[inline]
    pub fn is_16kcycles(&self) -> bool {
        *self == TIMEOUTR::_16KCYCLES
    }
    #[doc = "Checks if the value of the field is `_32KCYCLES`"]
    #[inline]
    pub fn is_32kcycles(&self) -> bool {
        *self == TIMEOUTR::_32KCYCLES
    }
}
#[doc = r" Proxy"]
pub struct _TUNINGW<'a> {
    w: &'a mut W,
}
impl<'a> _TUNINGW<'a> {
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
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "32768 Hz crystal oscillator"]
    XTAL,
    #[doc = "An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32768 Hz)."]
    BUFEXTCLK,
    #[doc = "Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed."]
    DIGEXTCLK,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::XTAL => 0,
            MODEW::BUFEXTCLK => 1,
            MODEW::DIGEXTCLK => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "32768 Hz crystal oscillator"]
    #[inline]
    pub fn xtal(self) -> &'a mut W {
        self.variant(MODEW::XTAL)
    }
    #[doc = "An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32768 Hz)."]
    #[inline]
    pub fn bufextclk(self) -> &'a mut W {
        self.variant(MODEW::BUFEXTCLK)
    }
    #[doc = "Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed."]
    #[inline]
    pub fn digextclk(self) -> &'a mut W {
        self.variant(MODEW::DIGEXTCLK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GAINW<'a> {
    w: &'a mut W,
}
impl<'a> _GAINW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HIGHAMPLW<'a> {
    w: &'a mut W,
}
impl<'a> _HIGHAMPLW<'a> {
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
pub struct _AGCW<'a> {
    w: &'a mut W,
}
impl<'a> _AGCW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CURW<'a> {
    w: &'a mut W,
}
impl<'a> _CURW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BUFCURW<'a> {
    w: &'a mut W,
}
impl<'a> _BUFCURW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIMEOUT`"]
pub enum TIMEOUTW {
    #[doc = "Timeout period of 2 cycles"]
    _2CYCLES,
    #[doc = "Timeout period of 256 cycles"]
    _256CYCLES,
    #[doc = "Timeout period of 1024 cycles"]
    _1KCYCLES,
    #[doc = "Timeout period of 2048 cycles"]
    _2KCYCLES,
    #[doc = "Timeout period of 4096 cycles"]
    _4KCYCLES,
    #[doc = "Timeout period of 8192 cycles"]
    _8KCYCLES,
    #[doc = "Timeout period of 16384 cycles"]
    _16KCYCLES,
    #[doc = "Timeout period of 32768 cycles"]
    _32KCYCLES,
}
impl TIMEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMEOUTW::_2CYCLES => 0,
            TIMEOUTW::_256CYCLES => 1,
            TIMEOUTW::_1KCYCLES => 2,
            TIMEOUTW::_2KCYCLES => 3,
            TIMEOUTW::_4KCYCLES => 4,
            TIMEOUTW::_8KCYCLES => 5,
            TIMEOUTW::_16KCYCLES => 6,
            TIMEOUTW::_32KCYCLES => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMEOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMEOUTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(TIMEOUTW::_2CYCLES)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(TIMEOUTW::_256CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(TIMEOUTW::_1KCYCLES)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline]
    pub fn _2kcycles(self) -> &'a mut W {
        self.variant(TIMEOUTW::_2KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline]
    pub fn _4kcycles(self) -> &'a mut W {
        self.variant(TIMEOUTW::_4KCYCLES)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline]
    pub fn _8kcycles(self) -> &'a mut W {
        self.variant(TIMEOUTW::_8KCYCLES)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline]
    pub fn _16kcycles(self) -> &'a mut W {
        self.variant(TIMEOUTW::_16KCYCLES)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline]
    pub fn _32kcycles(self) -> &'a mut W {
        self.variant(TIMEOUTW::_32KCYCLES)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - LFXO Internal Capacitor Array Tuning Value"]
    #[inline]
    pub fn tuning(&self) -> TUNINGR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TUNINGR { bits }
    }
    #[doc = "Bits 8:9 - LFXO Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:12 - LFXO Startup Gain"]
    #[inline]
    pub fn gain(&self) -> GAINR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GAINR { bits }
    }
    #[doc = "Bit 14 - LFXO High XTAL Oscillation Amplitude Enable"]
    #[inline]
    pub fn highampl(&self) -> HIGHAMPLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HIGHAMPLR { bits }
    }
    #[doc = "Bit 15 - LFXO AGC Enable"]
    #[inline]
    pub fn agc(&self) -> AGCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AGCR { bits }
    }
    #[doc = "Bits 16:17 - LFXO Current Trim"]
    #[inline]
    pub fn cur(&self) -> CURR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CURR { bits }
    }
    #[doc = "Bit 20 - LFXO Buffer Bias Current"]
    #[inline]
    pub fn bufcur(&self) -> BUFCURR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BUFCURR { bits }
    }
    #[doc = "Bits 24:26 - LFXO Timeout"]
    #[inline]
    pub fn timeout(&self) -> TIMEOUTR {
        TIMEOUTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 117477376 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - LFXO Internal Capacitor Array Tuning Value"]
    #[inline]
    pub fn tuning(&mut self) -> _TUNINGW {
        _TUNINGW { w: self }
    }
    #[doc = "Bits 8:9 - LFXO Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bits 11:12 - LFXO Startup Gain"]
    #[inline]
    pub fn gain(&mut self) -> _GAINW {
        _GAINW { w: self }
    }
    #[doc = "Bit 14 - LFXO High XTAL Oscillation Amplitude Enable"]
    #[inline]
    pub fn highampl(&mut self) -> _HIGHAMPLW {
        _HIGHAMPLW { w: self }
    }
    #[doc = "Bit 15 - LFXO AGC Enable"]
    #[inline]
    pub fn agc(&mut self) -> _AGCW {
        _AGCW { w: self }
    }
    #[doc = "Bits 16:17 - LFXO Current Trim"]
    #[inline]
    pub fn cur(&mut self) -> _CURW {
        _CURW { w: self }
    }
    #[doc = "Bit 20 - LFXO Buffer Bias Current"]
    #[inline]
    pub fn bufcur(&mut self) -> _BUFCURW {
        _BUFCURW { w: self }
    }
    #[doc = "Bits 24:26 - LFXO Timeout"]
    #[inline]
    pub fn timeout(&mut self) -> _TIMEOUTW {
        _TIMEOUTW { w: self }
    }
}
