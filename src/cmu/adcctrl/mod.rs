#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ADCCTRL {
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
#[doc = "Possible values of the field `ADC0CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0CLKSELR {
    #[doc = "ADC0 is not clocked"]
    DISABLED,
    #[doc = "AUXHFRCO is clocking ADC0"]
    AUXHFRCO,
    #[doc = "HFXO is clocking ADC0"]
    HFXO,
    #[doc = "HFSRCCLK is clocking ADC0"]
    HFSRCCLK,
}
impl ADC0CLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC0CLKSELR::DISABLED => 0,
            ADC0CLKSELR::AUXHFRCO => 1,
            ADC0CLKSELR::HFXO => 2,
            ADC0CLKSELR::HFSRCCLK => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADC0CLKSELR {
        match value {
            0 => ADC0CLKSELR::DISABLED,
            1 => ADC0CLKSELR::AUXHFRCO,
            2 => ADC0CLKSELR::HFXO,
            3 => ADC0CLKSELR::HFSRCCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADC0CLKSELR::DISABLED
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline]
    pub fn is_auxhfrco(&self) -> bool {
        *self == ADC0CLKSELR::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline]
    pub fn is_hfxo(&self) -> bool {
        *self == ADC0CLKSELR::HFXO
    }
    #[doc = "Checks if the value of the field is `HFSRCCLK`"]
    #[inline]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == ADC0CLKSELR::HFSRCCLK
    }
}
#[doc = r" Value of the field"]
pub struct ADC0CLKINVR {
    bits: bool,
}
impl ADC0CLKINVR {
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
#[doc = "Values that can be written to the field `ADC0CLKSEL`"]
pub enum ADC0CLKSELW {
    #[doc = "ADC0 is not clocked"]
    DISABLED,
    #[doc = "AUXHFRCO is clocking ADC0"]
    AUXHFRCO,
    #[doc = "HFXO is clocking ADC0"]
    HFXO,
    #[doc = "HFSRCCLK is clocking ADC0"]
    HFSRCCLK,
}
impl ADC0CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC0CLKSELW::DISABLED => 0,
            ADC0CLKSELW::AUXHFRCO => 1,
            ADC0CLKSELW::HFXO => 2,
            ADC0CLKSELW::HFSRCCLK => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC0CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC0CLKSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ADC0 is not clocked"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC0CLKSELW::DISABLED)
    }
    #[doc = "AUXHFRCO is clocking ADC0"]
    #[inline]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(ADC0CLKSELW::AUXHFRCO)
    }
    #[doc = "HFXO is clocking ADC0"]
    #[inline]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(ADC0CLKSELW::HFXO)
    }
    #[doc = "HFSRCCLK is clocking ADC0"]
    #[inline]
    pub fn hfsrcclk(self) -> &'a mut W {
        self.variant(ADC0CLKSELW::HFSRCCLK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADC0CLKINVW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0CLKINVW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 4:5 - ADC0 Clock Select"]
    #[inline]
    pub fn adc0clksel(&self) -> ADC0CLKSELR {
        ADC0CLKSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Invert Clock Selected By ADC0CLKSEL"]
    #[inline]
    pub fn adc0clkinv(&self) -> ADC0CLKINVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADC0CLKINVR { bits }
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
    #[doc = "Bits 4:5 - ADC0 Clock Select"]
    #[inline]
    pub fn adc0clksel(&mut self) -> _ADC0CLKSELW {
        _ADC0CLKSELW { w: self }
    }
    #[doc = "Bit 8 - Invert Clock Selected By ADC0CLKSEL"]
    #[inline]
    pub fn adc0clkinv(&mut self) -> _ADC0CLKINVW {
        _ADC0CLKINVW { w: self }
    }
}
