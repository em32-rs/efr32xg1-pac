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
pub struct ENR {
    bits: bool,
}
impl ENR {
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
pub struct DEBUGRUNR {
    bits: bool,
}
impl DEBUGRUNR {
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
#[doc = "Possible values of the field `OSCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSELR {
    #[doc = "Select Low Frequency RC Oscillator"]
    LFRCO,
    #[doc = "Select Low Frequency Crystal Oscillator"]
    LFXO,
    #[doc = "Select Ultra Low Frequency RC Oscillator"]
    ULFRCO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OSCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OSCSELR::LFRCO => 0,
            OSCSELR::LFXO => 1,
            OSCSELR::ULFRCO => 2,
            OSCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OSCSELR {
        match value {
            0 => OSCSELR::LFRCO,
            1 => OSCSELR::LFXO,
            2 => OSCSELR::ULFRCO,
            i => OSCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline]
    pub fn is_lfrco(&self) -> bool {
        *self == OSCSELR::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline]
    pub fn is_lfxo(&self) -> bool {
        *self == OSCSELR::LFXO
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline]
    pub fn is_ulfrco(&self) -> bool {
        *self == OSCSELR::ULFRCO
    }
}
#[doc = "Possible values of the field `PRESC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCR {
    #[doc = "LF Oscillator frequency undivided"]
    DIV1,
    #[doc = "LF Oscillator frequency divided by 2"]
    DIV2,
    #[doc = "LF Oscillator frequency divided by 4"]
    DIV4,
    #[doc = "LF Oscillator frequency divided by 8"]
    DIV8,
    #[doc = "LF Oscillator frequency divided by 16"]
    DIV16,
    #[doc = "LF Oscillator frequency divided by 32"]
    DIV32,
    #[doc = "LF Oscillator frequency divided by 64"]
    DIV64,
    #[doc = "LF Oscillator frequency divided by 128"]
    DIV128,
}
impl PRESCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCR::DIV1 => 0,
            PRESCR::DIV2 => 1,
            PRESCR::DIV4 => 2,
            PRESCR::DIV8 => 3,
            PRESCR::DIV16 => 4,
            PRESCR::DIV32 => 5,
            PRESCR::DIV64 => 6,
            PRESCR::DIV128 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCR {
        match value {
            0 => PRESCR::DIV1,
            1 => PRESCR::DIV2,
            2 => PRESCR::DIV4,
            3 => PRESCR::DIV8,
            4 => PRESCR::DIV16,
            5 => PRESCR::DIV32,
            6 => PRESCR::DIV64,
            7 => PRESCR::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == PRESCR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == PRESCR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == PRESCR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == PRESCR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == PRESCR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == PRESCR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == PRESCR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == PRESCR::DIV128
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
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
pub struct _DEBUGRUNW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBUGRUNW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSCSEL`"]
pub enum OSCSELW {
    #[doc = "Select Low Frequency RC Oscillator"]
    LFRCO,
    #[doc = "Select Low Frequency Crystal Oscillator"]
    LFXO,
    #[doc = "Select Ultra Low Frequency RC Oscillator"]
    ULFRCO,
}
impl OSCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OSCSELW::LFRCO => 0,
            OSCSELW::LFXO => 1,
            OSCSELW::ULFRCO => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select Low Frequency RC Oscillator"]
    #[inline]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(OSCSELW::LFRCO)
    }
    #[doc = "Select Low Frequency Crystal Oscillator"]
    #[inline]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(OSCSELW::LFXO)
    }
    #[doc = "Select Ultra Low Frequency RC Oscillator"]
    #[inline]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(OSCSELW::ULFRCO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRESC`"]
pub enum PRESCW {
    #[doc = "LF Oscillator frequency undivided"]
    DIV1,
    #[doc = "LF Oscillator frequency divided by 2"]
    DIV2,
    #[doc = "LF Oscillator frequency divided by 4"]
    DIV4,
    #[doc = "LF Oscillator frequency divided by 8"]
    DIV8,
    #[doc = "LF Oscillator frequency divided by 16"]
    DIV16,
    #[doc = "LF Oscillator frequency divided by 32"]
    DIV32,
    #[doc = "LF Oscillator frequency divided by 64"]
    DIV64,
    #[doc = "LF Oscillator frequency divided by 128"]
    DIV128,
}
impl PRESCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCW::DIV1 => 0,
            PRESCW::DIV2 => 1,
            PRESCW::DIV4 => 2,
            PRESCW::DIV8 => 3,
            PRESCW::DIV16 => 4,
            PRESCW::DIV32 => 5,
            PRESCW::DIV64 => 6,
            PRESCW::DIV128 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LF Oscillator frequency undivided"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESCW::DIV1)
    }
    #[doc = "LF Oscillator frequency divided by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCW::DIV2)
    }
    #[doc = "LF Oscillator frequency divided by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCW::DIV4)
    }
    #[doc = "LF Oscillator frequency divided by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCW::DIV8)
    }
    #[doc = "LF Oscillator frequency divided by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCW::DIV16)
    }
    #[doc = "LF Oscillator frequency divided by 32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESCW::DIV32)
    }
    #[doc = "LF Oscillator frequency divided by 64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCW::DIV64)
    }
    #[doc = "LF Oscillator frequency divided by 128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESCW::DIV128)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bit 0 - Enable CRYOTIMER"]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENR { bits }
    }
    #[doc = "Bit 1 - Debug Mode Run Enable"]
    #[inline]
    pub fn debugrun(&self) -> DEBUGRUNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DEBUGRUNR { bits }
    }
    #[doc = "Bits 2:3 - Select Low Frequency Oscillator"]
    #[inline]
    pub fn oscsel(&self) -> OSCSELR {
        OSCSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 5:7 - Prescaler Setting"]
    #[inline]
    pub fn presc(&self) -> PRESCR {
        PRESCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - Enable CRYOTIMER"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bit 1 - Debug Mode Run Enable"]
    #[inline]
    pub fn debugrun(&mut self) -> _DEBUGRUNW {
        _DEBUGRUNW { w: self }
    }
    #[doc = "Bits 2:3 - Select Low Frequency Oscillator"]
    #[inline]
    pub fn oscsel(&mut self) -> _OSCSELW {
        _OSCSELW { w: self }
    }
    #[doc = "Bits 5:7 - Prescaler Setting"]
    #[inline]
    pub fn presc(&mut self) -> _PRESCW {
        _PRESCW { w: self }
    }
}
