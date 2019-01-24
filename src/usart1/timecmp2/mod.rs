#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIMECMP2 {
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
pub struct TCMPVALR {
    bits: u8,
}
impl TCMPVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TSTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTARTR {
    #[doc = "Comparator 2 is disabled"]
    DISABLE,
    #[doc = "Comparator 2 and timer are started at TX end of frame"]
    TXEOF,
    #[doc = "Comparator 2 and timer are started at TX Complete"]
    TXC,
    #[doc = "Comparator 2 and timer are started at RX going going Active (default: low)"]
    RXACT,
    #[doc = "Comparator 2 and timer are started at RX end of frame"]
    RXEOF,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TSTARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSTARTR::DISABLE => 0,
            TSTARTR::TXEOF => 1,
            TSTARTR::TXC => 2,
            TSTARTR::RXACT => 3,
            TSTARTR::RXEOF => 4,
            TSTARTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSTARTR {
        match value {
            0 => TSTARTR::DISABLE,
            1 => TSTARTR::TXEOF,
            2 => TSTARTR::TXC,
            3 => TSTARTR::RXACT,
            4 => TSTARTR::RXEOF,
            i => TSTARTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TSTARTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `TXEOF`"]
    #[inline]
    pub fn is_txeof(&self) -> bool {
        *self == TSTARTR::TXEOF
    }
    #[doc = "Checks if the value of the field is `TXC`"]
    #[inline]
    pub fn is_txc(&self) -> bool {
        *self == TSTARTR::TXC
    }
    #[doc = "Checks if the value of the field is `RXACT`"]
    #[inline]
    pub fn is_rxact(&self) -> bool {
        *self == TSTARTR::RXACT
    }
    #[doc = "Checks if the value of the field is `RXEOF`"]
    #[inline]
    pub fn is_rxeof(&self) -> bool {
        *self == TSTARTR::RXEOF
    }
}
#[doc = "Possible values of the field `TSTOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTOPR {
    #[doc = "Comparator 2 is disabled when the counter equals TCMPVAL and triggers a TCMP2 event"]
    TCMP2,
    #[doc = "Comparator 2 is disabled at TX start TX Engine"]
    TXST,
    #[doc = "Comparator 2 is disabled on RX going going Active (default: low)"]
    RXACT,
    #[doc = "Comparator 2 is disabled on RX going Inactive"]
    RXACTN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TSTOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSTOPR::TCMP2 => 0,
            TSTOPR::TXST => 1,
            TSTOPR::RXACT => 2,
            TSTOPR::RXACTN => 3,
            TSTOPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSTOPR {
        match value {
            0 => TSTOPR::TCMP2,
            1 => TSTOPR::TXST,
            2 => TSTOPR::RXACT,
            3 => TSTOPR::RXACTN,
            i => TSTOPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TCMP2`"]
    #[inline]
    pub fn is_tcmp2(&self) -> bool {
        *self == TSTOPR::TCMP2
    }
    #[doc = "Checks if the value of the field is `TXST`"]
    #[inline]
    pub fn is_txst(&self) -> bool {
        *self == TSTOPR::TXST
    }
    #[doc = "Checks if the value of the field is `RXACT`"]
    #[inline]
    pub fn is_rxact(&self) -> bool {
        *self == TSTOPR::RXACT
    }
    #[doc = "Checks if the value of the field is `RXACTN`"]
    #[inline]
    pub fn is_rxactn(&self) -> bool {
        *self == TSTOPR::RXACTN
    }
}
#[doc = r" Value of the field"]
pub struct RESTARTENR {
    bits: bool,
}
impl RESTARTENR {
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
pub struct _TCMPVALW<'a> {
    w: &'a mut W,
}
impl<'a> _TCMPVALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TSTART`"]
pub enum TSTARTW {
    #[doc = "Comparator 2 is disabled"]
    DISABLE,
    #[doc = "Comparator 2 and timer are started at TX end of frame"]
    TXEOF,
    #[doc = "Comparator 2 and timer are started at TX Complete"]
    TXC,
    #[doc = "Comparator 2 and timer are started at RX going going Active (default: low)"]
    RXACT,
    #[doc = "Comparator 2 and timer are started at RX end of frame"]
    RXEOF,
}
impl TSTARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSTARTW::DISABLE => 0,
            TSTARTW::TXEOF => 1,
            TSTARTW::TXC => 2,
            TSTARTW::RXACT => 3,
            TSTARTW::RXEOF => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSTARTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Comparator 2 is disabled"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TSTARTW::DISABLE)
    }
    #[doc = "Comparator 2 and timer are started at TX end of frame"]
    #[inline]
    pub fn txeof(self) -> &'a mut W {
        self.variant(TSTARTW::TXEOF)
    }
    #[doc = "Comparator 2 and timer are started at TX Complete"]
    #[inline]
    pub fn txc(self) -> &'a mut W {
        self.variant(TSTARTW::TXC)
    }
    #[doc = "Comparator 2 and timer are started at RX going going Active (default: low)"]
    #[inline]
    pub fn rxact(self) -> &'a mut W {
        self.variant(TSTARTW::RXACT)
    }
    #[doc = "Comparator 2 and timer are started at RX end of frame"]
    #[inline]
    pub fn rxeof(self) -> &'a mut W {
        self.variant(TSTARTW::RXEOF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TSTOP`"]
pub enum TSTOPW {
    #[doc = "Comparator 2 is disabled when the counter equals TCMPVAL and triggers a TCMP2 event"]
    TCMP2,
    #[doc = "Comparator 2 is disabled at TX start TX Engine"]
    TXST,
    #[doc = "Comparator 2 is disabled on RX going going Active (default: low)"]
    RXACT,
    #[doc = "Comparator 2 is disabled on RX going Inactive"]
    RXACTN,
}
impl TSTOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSTOPW::TCMP2 => 0,
            TSTOPW::TXST => 1,
            TSTOPW::RXACT => 2,
            TSTOPW::RXACTN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSTOPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Comparator 2 is disabled when the counter equals TCMPVAL and triggers a TCMP2 event"]
    #[inline]
    pub fn tcmp2(self) -> &'a mut W {
        self.variant(TSTOPW::TCMP2)
    }
    #[doc = "Comparator 2 is disabled at TX start TX Engine"]
    #[inline]
    pub fn txst(self) -> &'a mut W {
        self.variant(TSTOPW::TXST)
    }
    #[doc = "Comparator 2 is disabled on RX going going Active (default: low)"]
    #[inline]
    pub fn rxact(self) -> &'a mut W {
        self.variant(TSTOPW::RXACT)
    }
    #[doc = "Comparator 2 is disabled on RX going Inactive"]
    #[inline]
    pub fn rxactn(self) -> &'a mut W {
        self.variant(TSTOPW::RXACTN)
    }
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
pub struct _RESTARTENW<'a> {
    w: &'a mut W,
}
impl<'a> _RESTARTENW<'a> {
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
    #[doc = "Bits 0:7 - Timer Comparator 2"]
    #[inline]
    pub fn tcmpval(&self) -> TCMPVALR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TCMPVALR { bits }
    }
    #[doc = "Bits 16:18 - Timer Start Source"]
    #[inline]
    pub fn tstart(&self) -> TSTARTR {
        TSTARTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:22 - Source Used to Disable Comparator 2"]
    #[inline]
    pub fn tstop(&self) -> TSTOPR {
        TSTOPR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - Restart Timer on TCMP2"]
    #[inline]
    pub fn restarten(&self) -> RESTARTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESTARTENR { bits }
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
    #[doc = "Bits 0:7 - Timer Comparator 2"]
    #[inline]
    pub fn tcmpval(&mut self) -> _TCMPVALW {
        _TCMPVALW { w: self }
    }
    #[doc = "Bits 16:18 - Timer Start Source"]
    #[inline]
    pub fn tstart(&mut self) -> _TSTARTW {
        _TSTARTW { w: self }
    }
    #[doc = "Bits 20:22 - Source Used to Disable Comparator 2"]
    #[inline]
    pub fn tstop(&mut self) -> _TSTOPW {
        _TSTOPW { w: self }
    }
    #[doc = "Bit 24 - Restart Timer on TCMP2"]
    #[inline]
    pub fn restarten(&mut self) -> _RESTARTENW {
        _RESTARTENW { w: self }
    }
}
