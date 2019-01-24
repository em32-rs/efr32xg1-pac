#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LFACLKSEL {
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
#[doc = "Possible values of the field `LFA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFAR {
    #[doc = "LFACLK is disabled"]
    DISABLED,
    #[doc = "LFRCO selected as LFACLK"]
    LFRCO,
    #[doc = "LFXO selected as LFACLK"]
    LFXO,
    #[doc = "ULFRCO selected as LFACLK"]
    ULFRCO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LFAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LFAR::DISABLED => 0,
            LFAR::LFRCO => 1,
            LFAR::LFXO => 2,
            LFAR::ULFRCO => 4,
            LFAR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LFAR {
        match value {
            0 => LFAR::DISABLED,
            1 => LFAR::LFRCO,
            2 => LFAR::LFXO,
            4 => LFAR::ULFRCO,
            i => LFAR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LFAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline]
    pub fn is_lfrco(&self) -> bool {
        *self == LFAR::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline]
    pub fn is_lfxo(&self) -> bool {
        *self == LFAR::LFXO
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline]
    pub fn is_ulfrco(&self) -> bool {
        *self == LFAR::ULFRCO
    }
}
#[doc = "Values that can be written to the field `LFA`"]
pub enum LFAW {
    #[doc = "LFACLK is disabled"]
    DISABLED,
    #[doc = "LFRCO selected as LFACLK"]
    LFRCO,
    #[doc = "LFXO selected as LFACLK"]
    LFXO,
    #[doc = "ULFRCO selected as LFACLK"]
    ULFRCO,
}
impl LFAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LFAW::DISABLED => 0,
            LFAW::LFRCO => 1,
            LFAW::LFXO => 2,
            LFAW::ULFRCO => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LFAW<'a> {
    w: &'a mut W,
}
impl<'a> _LFAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LFAW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LFACLK is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFAW::DISABLED)
    }
    #[doc = "LFRCO selected as LFACLK"]
    #[inline]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(LFAW::LFRCO)
    }
    #[doc = "LFXO selected as LFACLK"]
    #[inline]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(LFAW::LFXO)
    }
    #[doc = "ULFRCO selected as LFACLK"]
    #[inline]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(LFAW::ULFRCO)
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
    #[doc = "Bits 0:2 - Clock Select for LFA"]
    #[inline]
    pub fn lfa(&self) -> LFAR {
        LFAR::_from({
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
    #[doc = "Bits 0:2 - Clock Select for LFA"]
    #[inline]
    pub fn lfa(&mut self) -> _LFAW {
        _LFAW { w: self }
    }
}
