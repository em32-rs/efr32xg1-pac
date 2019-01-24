#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AUXHFRCOCTRL {
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
#[doc = r" Value of the field"]
pub struct FINETUNINGR {
    bits: u8,
}
impl FINETUNINGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FREQRANGER {
    bits: u8,
}
impl FREQRANGER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CMPBIASR {
    bits: u8,
}
impl CMPBIASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LDOHPR {
    bits: bool,
}
impl LDOHPR {
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
#[doc = "Possible values of the field `CLKDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKDIVR {
    #[doc = "Divide by 1."]
    DIV1,
    #[doc = "Divide by 2."]
    DIV2,
    #[doc = "Divide by 4."]
    DIV4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKDIVR::DIV1 => 0,
            CLKDIVR::DIV2 => 1,
            CLKDIVR::DIV4 => 2,
            CLKDIVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKDIVR {
        match value {
            0 => CLKDIVR::DIV1,
            1 => CLKDIVR::DIV2,
            2 => CLKDIVR::DIV4,
            i => CLKDIVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == CLKDIVR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == CLKDIVR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == CLKDIVR::DIV4
    }
}
#[doc = r" Value of the field"]
pub struct FINETUNINGENR {
    bits: bool,
}
impl FINETUNINGENR {
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
pub struct VREFTCR {
    bits: u8,
}
impl VREFTCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
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
#[doc = r" Proxy"]
pub struct _FINETUNINGW<'a> {
    w: &'a mut W,
}
impl<'a> _FINETUNINGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FREQRANGEW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQRANGEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMPBIASW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPBIASW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LDOHPW<'a> {
    w: &'a mut W,
}
impl<'a> _LDOHPW<'a> {
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
#[doc = "Values that can be written to the field `CLKDIV`"]
pub enum CLKDIVW {
    #[doc = "Divide by 1."]
    DIV1,
    #[doc = "Divide by 2."]
    DIV2,
    #[doc = "Divide by 4."]
    DIV4,
}
impl CLKDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKDIVW::DIV1 => 0,
            CLKDIVW::DIV2 => 1,
            CLKDIVW::DIV4 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKDIVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Divide by 1."]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(CLKDIVW::DIV1)
    }
    #[doc = "Divide by 2."]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(CLKDIVW::DIV2)
    }
    #[doc = "Divide by 4."]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(CLKDIVW::DIV4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FINETUNINGENW<'a> {
    w: &'a mut W,
}
impl<'a> _FINETUNINGENW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VREFTCW<'a> {
    w: &'a mut W,
}
impl<'a> _VREFTCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:6 - AUXHFRCO Tuning Value"]
    #[inline]
    pub fn tuning(&self) -> TUNINGR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TUNINGR { bits }
    }
    #[doc = "Bits 8:13 - AUXHFRCO Fine Tuning Value"]
    #[inline]
    pub fn finetuning(&self) -> FINETUNINGR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FINETUNINGR { bits }
    }
    #[doc = "Bits 16:20 - AUXHFRCO Frequency Range"]
    #[inline]
    pub fn freqrange(&self) -> FREQRANGER {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FREQRANGER { bits }
    }
    #[doc = "Bits 21:23 - AUXHFRCO Comparator Bias Current"]
    #[inline]
    pub fn cmpbias(&self) -> CMPBIASR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMPBIASR { bits }
    }
    #[doc = "Bit 24 - AUXHFRCO LDO High Power Mode"]
    #[inline]
    pub fn ldohp(&self) -> LDOHPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LDOHPR { bits }
    }
    #[doc = "Bits 25:26 - Locally Divide AUXHFRCO Clock Output"]
    #[inline]
    pub fn clkdiv(&self) -> CLKDIVR {
        CLKDIVR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 27 - Enable Reference for Fine Tuning"]
    #[inline]
    pub fn finetuningen(&self) -> FINETUNINGENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FINETUNINGENR { bits }
    }
    #[doc = "Bits 28:31 - AUXHFRCO Temperature Coefficient Trim on Comparator Reference"]
    #[inline]
    pub fn vreftc(&self) -> VREFTCR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VREFTCR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2974293820 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - AUXHFRCO Tuning Value"]
    #[inline]
    pub fn tuning(&mut self) -> _TUNINGW {
        _TUNINGW { w: self }
    }
    #[doc = "Bits 8:13 - AUXHFRCO Fine Tuning Value"]
    #[inline]
    pub fn finetuning(&mut self) -> _FINETUNINGW {
        _FINETUNINGW { w: self }
    }
    #[doc = "Bits 16:20 - AUXHFRCO Frequency Range"]
    #[inline]
    pub fn freqrange(&mut self) -> _FREQRANGEW {
        _FREQRANGEW { w: self }
    }
    #[doc = "Bits 21:23 - AUXHFRCO Comparator Bias Current"]
    #[inline]
    pub fn cmpbias(&mut self) -> _CMPBIASW {
        _CMPBIASW { w: self }
    }
    #[doc = "Bit 24 - AUXHFRCO LDO High Power Mode"]
    #[inline]
    pub fn ldohp(&mut self) -> _LDOHPW {
        _LDOHPW { w: self }
    }
    #[doc = "Bits 25:26 - Locally Divide AUXHFRCO Clock Output"]
    #[inline]
    pub fn clkdiv(&mut self) -> _CLKDIVW {
        _CLKDIVW { w: self }
    }
    #[doc = "Bit 27 - Enable Reference for Fine Tuning"]
    #[inline]
    pub fn finetuningen(&mut self) -> _FINETUNINGENW {
        _FINETUNINGENW { w: self }
    }
    #[doc = "Bits 28:31 - AUXHFRCO Temperature Coefficient Trim on Comparator Reference"]
    #[inline]
    pub fn vreftc(&mut self) -> _VREFTCW {
        _VREFTCW { w: self }
    }
}
