#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HFXOCTRL {
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
pub struct MODER {
    bits: bool,
}
impl MODER {
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
#[doc = "Possible values of the field `PEAKDETSHUNTOPTMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEAKDETSHUNTOPTMODER {
    #[doc = "Automatic control of HFXO peak detection and shunt optimization sequences. CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can also be used."]
    AUTOCMD,
    #[doc = "CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can be used to trigger peak detection and shunt optimization sequences."]
    CMD,
    #[doc = "CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE, REGISH, REGSELILOW, and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    MANUAL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PEAKDETSHUNTOPTMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PEAKDETSHUNTOPTMODER::AUTOCMD => 0,
            PEAKDETSHUNTOPTMODER::CMD => 1,
            PEAKDETSHUNTOPTMODER::MANUAL => 2,
            PEAKDETSHUNTOPTMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PEAKDETSHUNTOPTMODER {
        match value {
            0 => PEAKDETSHUNTOPTMODER::AUTOCMD,
            1 => PEAKDETSHUNTOPTMODER::CMD,
            2 => PEAKDETSHUNTOPTMODER::MANUAL,
            i => PEAKDETSHUNTOPTMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AUTOCMD`"]
    #[inline]
    pub fn is_autocmd(&self) -> bool {
        *self == PEAKDETSHUNTOPTMODER::AUTOCMD
    }
    #[doc = "Checks if the value of the field is `CMD`"]
    #[inline]
    pub fn is_cmd(&self) -> bool {
        *self == PEAKDETSHUNTOPTMODER::CMD
    }
    #[doc = "Checks if the value of the field is `MANUAL`"]
    #[inline]
    pub fn is_manual(&self) -> bool {
        *self == PEAKDETSHUNTOPTMODER::MANUAL
    }
}
#[doc = r" Value of the field"]
pub struct LOWPOWERR {
    bits: bool,
}
impl LOWPOWERR {
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
pub struct XTI2GNDR {
    bits: bool,
}
impl XTI2GNDR {
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
pub struct XTO2GNDR {
    bits: bool,
}
impl XTO2GNDR {
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
#[doc = "Possible values of the field `LFTIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFTIMEOUTR {
    #[doc = "Timeout period of 0 cycles (disabled)"]
    _0CYCLES,
    #[doc = "Timeout period of 2 cycles"]
    _2CYCLES,
    #[doc = "Timeout period of 4 cycles"]
    _4CYCLES,
    #[doc = "Timeout period of 16 cycles"]
    _16CYCLES,
    #[doc = "Timeout period of 32 cycles"]
    _32CYCLES,
    #[doc = "Timeout period of 64 cycles"]
    _64CYCLES,
    #[doc = "Timeout period of 1024 cycles"]
    _1KCYCLES,
    #[doc = "Timeout period of 4096 cycles"]
    _4KCYCLES,
}
impl LFTIMEOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LFTIMEOUTR::_0CYCLES => 0,
            LFTIMEOUTR::_2CYCLES => 1,
            LFTIMEOUTR::_4CYCLES => 2,
            LFTIMEOUTR::_16CYCLES => 3,
            LFTIMEOUTR::_32CYCLES => 4,
            LFTIMEOUTR::_64CYCLES => 5,
            LFTIMEOUTR::_1KCYCLES => 6,
            LFTIMEOUTR::_4KCYCLES => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LFTIMEOUTR {
        match value {
            0 => LFTIMEOUTR::_0CYCLES,
            1 => LFTIMEOUTR::_2CYCLES,
            2 => LFTIMEOUTR::_4CYCLES,
            3 => LFTIMEOUTR::_16CYCLES,
            4 => LFTIMEOUTR::_32CYCLES,
            5 => LFTIMEOUTR::_64CYCLES,
            6 => LFTIMEOUTR::_1KCYCLES,
            7 => LFTIMEOUTR::_4KCYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0CYCLES`"]
    #[inline]
    pub fn is_0cycles(&self) -> bool {
        *self == LFTIMEOUTR::_0CYCLES
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline]
    pub fn is_2cycles(&self) -> bool {
        *self == LFTIMEOUTR::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline]
    pub fn is_4cycles(&self) -> bool {
        *self == LFTIMEOUTR::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline]
    pub fn is_16cycles(&self) -> bool {
        *self == LFTIMEOUTR::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline]
    pub fn is_32cycles(&self) -> bool {
        *self == LFTIMEOUTR::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline]
    pub fn is_64cycles(&self) -> bool {
        *self == LFTIMEOUTR::_64CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline]
    pub fn is_1kcycles(&self) -> bool {
        *self == LFTIMEOUTR::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_4KCYCLES`"]
    #[inline]
    pub fn is_4kcycles(&self) -> bool {
        *self == LFTIMEOUTR::_4KCYCLES
    }
}
#[doc = r" Value of the field"]
pub struct AUTOSTARTEM0EM1R {
    bits: bool,
}
impl AUTOSTARTEM0EM1R {
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
pub struct AUTOSTARTSELEM0EM1R {
    bits: bool,
}
impl AUTOSTARTSELEM0EM1R {
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
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
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
#[doc = "Values that can be written to the field `PEAKDETSHUNTOPTMODE`"]
pub enum PEAKDETSHUNTOPTMODEW {
    #[doc = "Automatic control of HFXO peak detection and shunt optimization sequences. CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can also be used."]
    AUTOCMD,
    #[doc = "CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can be used to trigger peak detection and shunt optimization sequences."]
    CMD,
    #[doc = "CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE, REGISH, REGSELILOW, and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    MANUAL,
}
impl PEAKDETSHUNTOPTMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PEAKDETSHUNTOPTMODEW::AUTOCMD => 0,
            PEAKDETSHUNTOPTMODEW::CMD => 1,
            PEAKDETSHUNTOPTMODEW::MANUAL => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEAKDETSHUNTOPTMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEAKDETSHUNTOPTMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEAKDETSHUNTOPTMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Automatic control of HFXO peak detection and shunt optimization sequences. CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can also be used."]
    #[inline]
    pub fn autocmd(self) -> &'a mut W {
        self.variant(PEAKDETSHUNTOPTMODEW::AUTOCMD)
    }
    #[doc = "CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can be used to trigger peak detection and shunt optimization sequences."]
    #[inline]
    pub fn cmd(self) -> &'a mut W {
        self.variant(PEAKDETSHUNTOPTMODEW::CMD)
    }
    #[doc = "CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE, REGISH, REGSELILOW, and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    #[inline]
    pub fn manual(self) -> &'a mut W {
        self.variant(PEAKDETSHUNTOPTMODEW::MANUAL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LOWPOWERW<'a> {
    w: &'a mut W,
}
impl<'a> _LOWPOWERW<'a> {
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
#[doc = r" Proxy"]
pub struct _XTI2GNDW<'a> {
    w: &'a mut W,
}
impl<'a> _XTI2GNDW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XTO2GNDW<'a> {
    w: &'a mut W,
}
impl<'a> _XTO2GNDW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LFTIMEOUT`"]
pub enum LFTIMEOUTW {
    #[doc = "Timeout period of 0 cycles (disabled)"]
    _0CYCLES,
    #[doc = "Timeout period of 2 cycles"]
    _2CYCLES,
    #[doc = "Timeout period of 4 cycles"]
    _4CYCLES,
    #[doc = "Timeout period of 16 cycles"]
    _16CYCLES,
    #[doc = "Timeout period of 32 cycles"]
    _32CYCLES,
    #[doc = "Timeout period of 64 cycles"]
    _64CYCLES,
    #[doc = "Timeout period of 1024 cycles"]
    _1KCYCLES,
    #[doc = "Timeout period of 4096 cycles"]
    _4KCYCLES,
}
impl LFTIMEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LFTIMEOUTW::_0CYCLES => 0,
            LFTIMEOUTW::_2CYCLES => 1,
            LFTIMEOUTW::_4CYCLES => 2,
            LFTIMEOUTW::_16CYCLES => 3,
            LFTIMEOUTW::_32CYCLES => 4,
            LFTIMEOUTW::_64CYCLES => 5,
            LFTIMEOUTW::_1KCYCLES => 6,
            LFTIMEOUTW::_4KCYCLES => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LFTIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _LFTIMEOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LFTIMEOUTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timeout period of 0 cycles (disabled)"]
    #[inline]
    pub fn _0cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUTW::_0CYCLES)
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUTW::_2CYCLES)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUTW::_4CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUTW::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUTW::_32CYCLES)
    }
    #[doc = "Timeout period of 64 cycles"]
    #[inline]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUTW::_64CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(LFTIMEOUTW::_1KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline]
    pub fn _4kcycles(self) -> &'a mut W {
        self.variant(LFTIMEOUTW::_4KCYCLES)
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
#[doc = r" Proxy"]
pub struct _AUTOSTARTEM0EM1W<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOSTARTEM0EM1W<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AUTOSTARTSELEM0EM1W<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOSTARTSELEM0EM1W<'a> {
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
        const OFFSET: u8 = 29;
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
    #[doc = "Bit 0 - HFXO Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MODER { bits }
    }
    #[doc = "Bits 4:5 - HFXO Automatic Peak Detection and Shunt Current Optimization Mode"]
    #[inline]
    pub fn peakdetshuntoptmode(&self) -> PEAKDETSHUNTOPTMODER {
        PEAKDETSHUNTOPTMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Low Power Mode Control"]
    #[inline]
    pub fn lowpower(&self) -> LOWPOWERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LOWPOWERR { bits }
    }
    #[doc = "Bit 9 - Clamp HFXTAL_N Pin to Ground When HFXO Oscillator is Off"]
    #[inline]
    pub fn xti2gnd(&self) -> XTI2GNDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XTI2GNDR { bits }
    }
    #[doc = "Bit 10 - Clamp HFXTAL_P Pin to Ground When HFXO Oscillator is Off"]
    #[inline]
    pub fn xto2gnd(&self) -> XTO2GNDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XTO2GNDR { bits }
    }
    #[doc = "Bits 24:26 - HFXO Low Frequency Timeout"]
    #[inline]
    pub fn lftimeout(&self) -> LFTIMEOUTR {
        LFTIMEOUTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline]
    pub fn autostartem0em1(&self) -> AUTOSTARTEM0EM1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUTOSTARTEM0EM1R { bits }
    }
    #[doc = "Bit 29 - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline]
    pub fn autostartselem0em1(&self) -> AUTOSTARTSELEM0EM1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUTOSTARTSELEM0EM1R { bits }
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
    #[doc = "Bit 0 - HFXO Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bits 4:5 - HFXO Automatic Peak Detection and Shunt Current Optimization Mode"]
    #[inline]
    pub fn peakdetshuntoptmode(&mut self) -> _PEAKDETSHUNTOPTMODEW {
        _PEAKDETSHUNTOPTMODEW { w: self }
    }
    #[doc = "Bit 8 - Low Power Mode Control"]
    #[inline]
    pub fn lowpower(&mut self) -> _LOWPOWERW {
        _LOWPOWERW { w: self }
    }
    #[doc = "Bit 9 - Clamp HFXTAL_N Pin to Ground When HFXO Oscillator is Off"]
    #[inline]
    pub fn xti2gnd(&mut self) -> _XTI2GNDW {
        _XTI2GNDW { w: self }
    }
    #[doc = "Bit 10 - Clamp HFXTAL_P Pin to Ground When HFXO Oscillator is Off"]
    #[inline]
    pub fn xto2gnd(&mut self) -> _XTO2GNDW {
        _XTO2GNDW { w: self }
    }
    #[doc = "Bits 24:26 - HFXO Low Frequency Timeout"]
    #[inline]
    pub fn lftimeout(&mut self) -> _LFTIMEOUTW {
        _LFTIMEOUTW { w: self }
    }
    #[doc = "Bit 28 - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline]
    pub fn autostartem0em1(&mut self) -> _AUTOSTARTEM0EM1W {
        _AUTOSTARTEM0EM1W { w: self }
    }
    #[doc = "Bit 29 - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline]
    pub fn autostartselem0em1(&mut self) -> _AUTOSTARTSELEM0EM1W {
        _AUTOSTARTSELEM0EM1W { w: self }
    }
}
