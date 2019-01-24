#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LFBCLKSEL {
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
#[doc = "Possible values of the field `LFB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFBR {
    #[doc = "LFBCLK is disabled"]
    DISABLED,
    #[doc = "LFRCO selected as LFBCLK"]
    LFRCO,
    #[doc = "LFXO selected as LFBCLK"]
    LFXO,
    #[doc = "HFCLK divided by two/four is selected as LFBCLK"]
    HFCLKLE,
    #[doc = "ULFRCO selected as LFBCLK"]
    ULFRCO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LFBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LFBR::DISABLED => 0,
            LFBR::LFRCO => 1,
            LFBR::LFXO => 2,
            LFBR::HFCLKLE => 3,
            LFBR::ULFRCO => 4,
            LFBR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LFBR {
        match value {
            0 => LFBR::DISABLED,
            1 => LFBR::LFRCO,
            2 => LFBR::LFXO,
            3 => LFBR::HFCLKLE,
            4 => LFBR::ULFRCO,
            i => LFBR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LFBR::DISABLED
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline]
    pub fn is_lfrco(&self) -> bool {
        *self == LFBR::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline]
    pub fn is_lfxo(&self) -> bool {
        *self == LFBR::LFXO
    }
    #[doc = "Checks if the value of the field is `HFCLKLE`"]
    #[inline]
    pub fn is_hfclkle(&self) -> bool {
        *self == LFBR::HFCLKLE
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline]
    pub fn is_ulfrco(&self) -> bool {
        *self == LFBR::ULFRCO
    }
}
#[doc = "Values that can be written to the field `LFB`"]
pub enum LFBW {
    #[doc = "LFBCLK is disabled"]
    DISABLED,
    #[doc = "LFRCO selected as LFBCLK"]
    LFRCO,
    #[doc = "LFXO selected as LFBCLK"]
    LFXO,
    #[doc = "HFCLK divided by two/four is selected as LFBCLK"]
    HFCLKLE,
    #[doc = "ULFRCO selected as LFBCLK"]
    ULFRCO,
}
impl LFBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LFBW::DISABLED => 0,
            LFBW::LFRCO => 1,
            LFBW::LFXO => 2,
            LFBW::HFCLKLE => 3,
            LFBW::ULFRCO => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LFBW<'a> {
    w: &'a mut W,
}
impl<'a> _LFBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LFBW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LFBCLK is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFBW::DISABLED)
    }
    #[doc = "LFRCO selected as LFBCLK"]
    #[inline]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(LFBW::LFRCO)
    }
    #[doc = "LFXO selected as LFBCLK"]
    #[inline]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(LFBW::LFXO)
    }
    #[doc = "HFCLK divided by two/four is selected as LFBCLK"]
    #[inline]
    pub fn hfclkle(self) -> &'a mut W {
        self.variant(LFBW::HFCLKLE)
    }
    #[doc = "ULFRCO selected as LFBCLK"]
    #[inline]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(LFBW::ULFRCO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:2 - Clock Select for LFB"]
    #[inline]
    pub fn lfb(&self) -> LFBR {
        LFBR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:2 - Clock Select for LFB"]
    #[inline]
    pub fn lfb(&mut self) -> _LFBW {
        _LFBW { w: self }
    }
}
