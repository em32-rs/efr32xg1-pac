#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HFCLKSEL {
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
}
#[doc = "Values that can be written to the field `HF`"]
pub enum HFW {
    #[doc = "Select HFRCO as HFCLK"]
    HFRCO,
    #[doc = "Select HFXO as HFCLK"]
    HFXO,
    #[doc = "Select LFRCO as HFCLK"]
    LFRCO,
    #[doc = "Select LFXO as HFCLK"]
    LFXO,
}
impl HFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HFW::HFRCO => 1,
            HFW::HFXO => 2,
            HFW::LFRCO => 3,
            HFW::LFXO => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HFW<'a> {
    w: &'a mut W,
}
impl<'a> _HFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select HFRCO as HFCLK"]
    #[inline]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(HFW::HFRCO)
    }
    #[doc = "Select HFXO as HFCLK"]
    #[inline]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(HFW::HFXO)
    }
    #[doc = "Select LFRCO as HFCLK"]
    #[inline]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(HFW::LFRCO)
    }
    #[doc = "Select LFXO as HFCLK"]
    #[inline]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(HFW::LFXO)
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
    #[doc = "Bits 0:2 - HFCLK Select"]
    #[inline]
    pub fn hf(&mut self) -> _HFW {
        _HFW { w: self }
    }
}
