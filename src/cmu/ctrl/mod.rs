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
#[doc = "Possible values of the field `CLKOUTSEL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTSEL0R {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "ULFRCO (directly from oscillator)"]
    ULFRCO,
    #[doc = "LFRCO (directly from oscillator)"]
    LFRCO,
    #[doc = "LFXO (directly from oscillator)"]
    LFXO,
    #[doc = "HFXO (directly from oscillator)"]
    HFXO,
    #[doc = "HFEXPCLK"]
    HFEXPCLK,
    #[doc = "ULFRCO (qualified)"]
    ULFRCOQ,
    #[doc = "LFRCO (qualified)"]
    LFRCOQ,
    #[doc = "LFXO (qualified)"]
    LFXOQ,
    #[doc = "HFRCO (qualified)"]
    HFRCOQ,
    #[doc = "AUXHFRCO (qualified)"]
    AUXHFRCOQ,
    #[doc = "HFXO (qualified)"]
    HFXOQ,
    #[doc = "HFSRCCLK"]
    HFSRCCLK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKOUTSEL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKOUTSEL0R::DISABLED => 0,
            CLKOUTSEL0R::ULFRCO => 1,
            CLKOUTSEL0R::LFRCO => 2,
            CLKOUTSEL0R::LFXO => 3,
            CLKOUTSEL0R::HFXO => 6,
            CLKOUTSEL0R::HFEXPCLK => 7,
            CLKOUTSEL0R::ULFRCOQ => 9,
            CLKOUTSEL0R::LFRCOQ => 10,
            CLKOUTSEL0R::LFXOQ => 11,
            CLKOUTSEL0R::HFRCOQ => 12,
            CLKOUTSEL0R::AUXHFRCOQ => 13,
            CLKOUTSEL0R::HFXOQ => 14,
            CLKOUTSEL0R::HFSRCCLK => 15,
            CLKOUTSEL0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKOUTSEL0R {
        match value {
            0 => CLKOUTSEL0R::DISABLED,
            1 => CLKOUTSEL0R::ULFRCO,
            2 => CLKOUTSEL0R::LFRCO,
            3 => CLKOUTSEL0R::LFXO,
            6 => CLKOUTSEL0R::HFXO,
            7 => CLKOUTSEL0R::HFEXPCLK,
            9 => CLKOUTSEL0R::ULFRCOQ,
            10 => CLKOUTSEL0R::LFRCOQ,
            11 => CLKOUTSEL0R::LFXOQ,
            12 => CLKOUTSEL0R::HFRCOQ,
            13 => CLKOUTSEL0R::AUXHFRCOQ,
            14 => CLKOUTSEL0R::HFXOQ,
            15 => CLKOUTSEL0R::HFSRCCLK,
            i => CLKOUTSEL0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CLKOUTSEL0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKOUTSEL0R::ULFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKOUTSEL0R::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKOUTSEL0R::LFXO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline]
    pub fn is_hfxo(&self) -> bool {
        *self == CLKOUTSEL0R::HFXO
    }
    #[doc = "Checks if the value of the field is `HFEXPCLK`"]
    #[inline]
    pub fn is_hfexpclk(&self) -> bool {
        *self == CLKOUTSEL0R::HFEXPCLK
    }
    #[doc = "Checks if the value of the field is `ULFRCOQ`"]
    #[inline]
    pub fn is_ulfrcoq(&self) -> bool {
        *self == CLKOUTSEL0R::ULFRCOQ
    }
    #[doc = "Checks if the value of the field is `LFRCOQ`"]
    #[inline]
    pub fn is_lfrcoq(&self) -> bool {
        *self == CLKOUTSEL0R::LFRCOQ
    }
    #[doc = "Checks if the value of the field is `LFXOQ`"]
    #[inline]
    pub fn is_lfxoq(&self) -> bool {
        *self == CLKOUTSEL0R::LFXOQ
    }
    #[doc = "Checks if the value of the field is `HFRCOQ`"]
    #[inline]
    pub fn is_hfrcoq(&self) -> bool {
        *self == CLKOUTSEL0R::HFRCOQ
    }
    #[doc = "Checks if the value of the field is `AUXHFRCOQ`"]
    #[inline]
    pub fn is_auxhfrcoq(&self) -> bool {
        *self == CLKOUTSEL0R::AUXHFRCOQ
    }
    #[doc = "Checks if the value of the field is `HFXOQ`"]
    #[inline]
    pub fn is_hfxoq(&self) -> bool {
        *self == CLKOUTSEL0R::HFXOQ
    }
    #[doc = "Checks if the value of the field is `HFSRCCLK`"]
    #[inline]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == CLKOUTSEL0R::HFSRCCLK
    }
}
#[doc = "Possible values of the field `CLKOUTSEL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTSEL1R {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "ULFRCO (directly from oscillator)"]
    ULFRCO,
    #[doc = "LFRCO (directly from oscillator)"]
    LFRCO,
    #[doc = "LFXO (directly from oscillator)"]
    LFXO,
    #[doc = "HFXO (directly from oscillator)"]
    HFXO,
    #[doc = "HFEXPCLK"]
    HFEXPCLK,
    #[doc = "ULFRCO (qualified)"]
    ULFRCOQ,
    #[doc = "LFRCO (qualified)"]
    LFRCOQ,
    #[doc = "LFXO (qualified)"]
    LFXOQ,
    #[doc = "HFRCO (qualified)"]
    HFRCOQ,
    #[doc = "AUXHFRCO (qualified)"]
    AUXHFRCOQ,
    #[doc = "HFXO (qualified)"]
    HFXOQ,
    #[doc = "HFSRCCLK"]
    HFSRCCLK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKOUTSEL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKOUTSEL1R::DISABLED => 0,
            CLKOUTSEL1R::ULFRCO => 1,
            CLKOUTSEL1R::LFRCO => 2,
            CLKOUTSEL1R::LFXO => 3,
            CLKOUTSEL1R::HFXO => 6,
            CLKOUTSEL1R::HFEXPCLK => 7,
            CLKOUTSEL1R::ULFRCOQ => 9,
            CLKOUTSEL1R::LFRCOQ => 10,
            CLKOUTSEL1R::LFXOQ => 11,
            CLKOUTSEL1R::HFRCOQ => 12,
            CLKOUTSEL1R::AUXHFRCOQ => 13,
            CLKOUTSEL1R::HFXOQ => 14,
            CLKOUTSEL1R::HFSRCCLK => 15,
            CLKOUTSEL1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKOUTSEL1R {
        match value {
            0 => CLKOUTSEL1R::DISABLED,
            1 => CLKOUTSEL1R::ULFRCO,
            2 => CLKOUTSEL1R::LFRCO,
            3 => CLKOUTSEL1R::LFXO,
            6 => CLKOUTSEL1R::HFXO,
            7 => CLKOUTSEL1R::HFEXPCLK,
            9 => CLKOUTSEL1R::ULFRCOQ,
            10 => CLKOUTSEL1R::LFRCOQ,
            11 => CLKOUTSEL1R::LFXOQ,
            12 => CLKOUTSEL1R::HFRCOQ,
            13 => CLKOUTSEL1R::AUXHFRCOQ,
            14 => CLKOUTSEL1R::HFXOQ,
            15 => CLKOUTSEL1R::HFSRCCLK,
            i => CLKOUTSEL1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CLKOUTSEL1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKOUTSEL1R::ULFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKOUTSEL1R::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKOUTSEL1R::LFXO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline]
    pub fn is_hfxo(&self) -> bool {
        *self == CLKOUTSEL1R::HFXO
    }
    #[doc = "Checks if the value of the field is `HFEXPCLK`"]
    #[inline]
    pub fn is_hfexpclk(&self) -> bool {
        *self == CLKOUTSEL1R::HFEXPCLK
    }
    #[doc = "Checks if the value of the field is `ULFRCOQ`"]
    #[inline]
    pub fn is_ulfrcoq(&self) -> bool {
        *self == CLKOUTSEL1R::ULFRCOQ
    }
    #[doc = "Checks if the value of the field is `LFRCOQ`"]
    #[inline]
    pub fn is_lfrcoq(&self) -> bool {
        *self == CLKOUTSEL1R::LFRCOQ
    }
    #[doc = "Checks if the value of the field is `LFXOQ`"]
    #[inline]
    pub fn is_lfxoq(&self) -> bool {
        *self == CLKOUTSEL1R::LFXOQ
    }
    #[doc = "Checks if the value of the field is `HFRCOQ`"]
    #[inline]
    pub fn is_hfrcoq(&self) -> bool {
        *self == CLKOUTSEL1R::HFRCOQ
    }
    #[doc = "Checks if the value of the field is `AUXHFRCOQ`"]
    #[inline]
    pub fn is_auxhfrcoq(&self) -> bool {
        *self == CLKOUTSEL1R::AUXHFRCOQ
    }
    #[doc = "Checks if the value of the field is `HFXOQ`"]
    #[inline]
    pub fn is_hfxoq(&self) -> bool {
        *self == CLKOUTSEL1R::HFXOQ
    }
    #[doc = "Checks if the value of the field is `HFSRCCLK`"]
    #[inline]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == CLKOUTSEL1R::HFSRCCLK
    }
}
#[doc = r" Value of the field"]
pub struct WSHFLER {
    bits: bool,
}
impl WSHFLER {
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
pub struct HFPERCLKENR {
    bits: bool,
}
impl HFPERCLKENR {
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
#[doc = "Values that can be written to the field `CLKOUTSEL0`"]
pub enum CLKOUTSEL0W {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "ULFRCO (directly from oscillator)"]
    ULFRCO,
    #[doc = "LFRCO (directly from oscillator)"]
    LFRCO,
    #[doc = "LFXO (directly from oscillator)"]
    LFXO,
    #[doc = "HFXO (directly from oscillator)"]
    HFXO,
    #[doc = "HFEXPCLK"]
    HFEXPCLK,
    #[doc = "ULFRCO (qualified)"]
    ULFRCOQ,
    #[doc = "LFRCO (qualified)"]
    LFRCOQ,
    #[doc = "LFXO (qualified)"]
    LFXOQ,
    #[doc = "HFRCO (qualified)"]
    HFRCOQ,
    #[doc = "AUXHFRCO (qualified)"]
    AUXHFRCOQ,
    #[doc = "HFXO (qualified)"]
    HFXOQ,
    #[doc = "HFSRCCLK"]
    HFSRCCLK,
}
impl CLKOUTSEL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKOUTSEL0W::DISABLED => 0,
            CLKOUTSEL0W::ULFRCO => 1,
            CLKOUTSEL0W::LFRCO => 2,
            CLKOUTSEL0W::LFXO => 3,
            CLKOUTSEL0W::HFXO => 6,
            CLKOUTSEL0W::HFEXPCLK => 7,
            CLKOUTSEL0W::ULFRCOQ => 9,
            CLKOUTSEL0W::LFRCOQ => 10,
            CLKOUTSEL0W::LFXOQ => 11,
            CLKOUTSEL0W::HFRCOQ => 12,
            CLKOUTSEL0W::AUXHFRCOQ => 13,
            CLKOUTSEL0W::HFXOQ => 14,
            CLKOUTSEL0W::HFSRCCLK => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKOUTSEL0W<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUTSEL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKOUTSEL0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKOUTSEL0W::DISABLED)
    }
    #[doc = "ULFRCO (directly from oscillator)"]
    #[inline]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL0W::ULFRCO)
    }
    #[doc = "LFRCO (directly from oscillator)"]
    #[inline]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL0W::LFRCO)
    }
    #[doc = "LFXO (directly from oscillator)"]
    #[inline]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL0W::LFXO)
    }
    #[doc = "HFXO (directly from oscillator)"]
    #[inline]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL0W::HFXO)
    }
    #[doc = "HFEXPCLK"]
    #[inline]
    pub fn hfexpclk(self) -> &'a mut W {
        self.variant(CLKOUTSEL0W::HFEXPCLK)
    }
    #[doc = "ULFRCO (qualified)"]
    #[inline]
    pub fn ulfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL0W::ULFRCOQ)
    }
    #[doc = "LFRCO (qualified)"]
    #[inline]
    pub fn lfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL0W::LFRCOQ)
    }
    #[doc = "LFXO (qualified)"]
    #[inline]
    pub fn lfxoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL0W::LFXOQ)
    }
    #[doc = "HFRCO (qualified)"]
    #[inline]
    pub fn hfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL0W::HFRCOQ)
    }
    #[doc = "AUXHFRCO (qualified)"]
    #[inline]
    pub fn auxhfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL0W::AUXHFRCOQ)
    }
    #[doc = "HFXO (qualified)"]
    #[inline]
    pub fn hfxoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL0W::HFXOQ)
    }
    #[doc = "HFSRCCLK"]
    #[inline]
    pub fn hfsrcclk(self) -> &'a mut W {
        self.variant(CLKOUTSEL0W::HFSRCCLK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKOUTSEL1`"]
pub enum CLKOUTSEL1W {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "ULFRCO (directly from oscillator)"]
    ULFRCO,
    #[doc = "LFRCO (directly from oscillator)"]
    LFRCO,
    #[doc = "LFXO (directly from oscillator)"]
    LFXO,
    #[doc = "HFXO (directly from oscillator)"]
    HFXO,
    #[doc = "HFEXPCLK"]
    HFEXPCLK,
    #[doc = "ULFRCO (qualified)"]
    ULFRCOQ,
    #[doc = "LFRCO (qualified)"]
    LFRCOQ,
    #[doc = "LFXO (qualified)"]
    LFXOQ,
    #[doc = "HFRCO (qualified)"]
    HFRCOQ,
    #[doc = "AUXHFRCO (qualified)"]
    AUXHFRCOQ,
    #[doc = "HFXO (qualified)"]
    HFXOQ,
    #[doc = "HFSRCCLK"]
    HFSRCCLK,
}
impl CLKOUTSEL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKOUTSEL1W::DISABLED => 0,
            CLKOUTSEL1W::ULFRCO => 1,
            CLKOUTSEL1W::LFRCO => 2,
            CLKOUTSEL1W::LFXO => 3,
            CLKOUTSEL1W::HFXO => 6,
            CLKOUTSEL1W::HFEXPCLK => 7,
            CLKOUTSEL1W::ULFRCOQ => 9,
            CLKOUTSEL1W::LFRCOQ => 10,
            CLKOUTSEL1W::LFXOQ => 11,
            CLKOUTSEL1W::HFRCOQ => 12,
            CLKOUTSEL1W::AUXHFRCOQ => 13,
            CLKOUTSEL1W::HFXOQ => 14,
            CLKOUTSEL1W::HFSRCCLK => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKOUTSEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUTSEL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKOUTSEL1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKOUTSEL1W::DISABLED)
    }
    #[doc = "ULFRCO (directly from oscillator)"]
    #[inline]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL1W::ULFRCO)
    }
    #[doc = "LFRCO (directly from oscillator)"]
    #[inline]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL1W::LFRCO)
    }
    #[doc = "LFXO (directly from oscillator)"]
    #[inline]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL1W::LFXO)
    }
    #[doc = "HFXO (directly from oscillator)"]
    #[inline]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL1W::HFXO)
    }
    #[doc = "HFEXPCLK"]
    #[inline]
    pub fn hfexpclk(self) -> &'a mut W {
        self.variant(CLKOUTSEL1W::HFEXPCLK)
    }
    #[doc = "ULFRCO (qualified)"]
    #[inline]
    pub fn ulfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1W::ULFRCOQ)
    }
    #[doc = "LFRCO (qualified)"]
    #[inline]
    pub fn lfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1W::LFRCOQ)
    }
    #[doc = "LFXO (qualified)"]
    #[inline]
    pub fn lfxoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1W::LFXOQ)
    }
    #[doc = "HFRCO (qualified)"]
    #[inline]
    pub fn hfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1W::HFRCOQ)
    }
    #[doc = "AUXHFRCO (qualified)"]
    #[inline]
    pub fn auxhfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1W::AUXHFRCOQ)
    }
    #[doc = "HFXO (qualified)"]
    #[inline]
    pub fn hfxoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1W::HFXOQ)
    }
    #[doc = "HFSRCCLK"]
    #[inline]
    pub fn hfsrcclk(self) -> &'a mut W {
        self.variant(CLKOUTSEL1W::HFSRCCLK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WSHFLEW<'a> {
    w: &'a mut W,
}
impl<'a> _WSHFLEW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HFPERCLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _HFPERCLKENW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Clock Output Select 0"]
    #[inline]
    pub fn clkoutsel0(&self) -> CLKOUTSEL0R {
        CLKOUTSEL0R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 5:8 - Clock Output Select 1"]
    #[inline]
    pub fn clkoutsel1(&self) -> CLKOUTSEL1R {
        CLKOUTSEL1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Wait State for High-Frequency LE Interface"]
    #[inline]
    pub fn wshfle(&self) -> WSHFLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WSHFLER { bits }
    }
    #[doc = "Bit 20 - HFPERCLK Enable"]
    #[inline]
    pub fn hfperclken(&self) -> HFPERCLKENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HFPERCLKENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3145728 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Clock Output Select 0"]
    #[inline]
    pub fn clkoutsel0(&mut self) -> _CLKOUTSEL0W {
        _CLKOUTSEL0W { w: self }
    }
    #[doc = "Bits 5:8 - Clock Output Select 1"]
    #[inline]
    pub fn clkoutsel1(&mut self) -> _CLKOUTSEL1W {
        _CLKOUTSEL1W { w: self }
    }
    #[doc = "Bit 16 - Wait State for High-Frequency LE Interface"]
    #[inline]
    pub fn wshfle(&mut self) -> _WSHFLEW {
        _WSHFLEW { w: self }
    }
    #[doc = "Bit 20 - HFPERCLK Enable"]
    #[inline]
    pub fn hfperclken(&mut self) -> _HFPERCLKENW {
        _HFPERCLKENW { w: self }
    }
}
