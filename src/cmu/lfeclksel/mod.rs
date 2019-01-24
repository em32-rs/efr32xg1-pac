#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LFECLKSEL {
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
#[doc = "Possible values of the field `LFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFER {
    #[doc = "LFECLK is disabled"]
    DISABLED,
    #[doc = "LFRCO selected as LFECLK"]
    LFRCO,
    #[doc = "LFXO selected as LFECLK"]
    LFXO,
    #[doc = "ULFRCO selected as LFECLK"]
    ULFRCO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LFER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LFER::DISABLED => 0,
            LFER::LFRCO => 1,
            LFER::LFXO => 2,
            LFER::ULFRCO => 4,
            LFER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LFER {
        match value {
            0 => LFER::DISABLED,
            1 => LFER::LFRCO,
            2 => LFER::LFXO,
            4 => LFER::ULFRCO,
            i => LFER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LFER::DISABLED
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline]
    pub fn is_lfrco(&self) -> bool {
        *self == LFER::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline]
    pub fn is_lfxo(&self) -> bool {
        *self == LFER::LFXO
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline]
    pub fn is_ulfrco(&self) -> bool {
        *self == LFER::ULFRCO
    }
}
#[doc = "Values that can be written to the field `LFE`"]
pub enum LFEW {
    #[doc = "LFECLK is disabled"]
    DISABLED,
    #[doc = "LFRCO selected as LFECLK"]
    LFRCO,
    #[doc = "LFXO selected as LFECLK"]
    LFXO,
    #[doc = "ULFRCO selected as LFECLK"]
    ULFRCO,
}
impl LFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LFEW::DISABLED => 0,
            LFEW::LFRCO => 1,
            LFEW::LFXO => 2,
            LFEW::ULFRCO => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LFEW<'a> {
    w: &'a mut W,
}
impl<'a> _LFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LFEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LFECLK is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFEW::DISABLED)
    }
    #[doc = "LFRCO selected as LFECLK"]
    #[inline]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(LFEW::LFRCO)
    }
    #[doc = "LFXO selected as LFECLK"]
    #[inline]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(LFEW::LFXO)
    }
    #[doc = "ULFRCO selected as LFECLK"]
    #[inline]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(LFEW::ULFRCO)
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
    #[doc = "Bits 0:2 - Clock Select for LFE"]
    #[inline]
    pub fn lfe(&self) -> LFER {
        LFER::_from({
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
    #[doc = "Bits 0:2 - Clock Select for LFE"]
    #[inline]
    pub fn lfe(&mut self) -> _LFEW {
        _LFEW { w: self }
    }
}
