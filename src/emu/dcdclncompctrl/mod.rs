#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCDCLNCOMPCTRL {
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
pub struct COMPENR1R {
    bits: u8,
}
impl COMPENR1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct COMPENR2R {
    bits: u8,
}
impl COMPENR2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct COMPENR3R {
    bits: u8,
}
impl COMPENR3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct COMPENC1R {
    bits: u8,
}
impl COMPENC1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct COMPENC2R {
    bits: u8,
}
impl COMPENC2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct COMPENC3R {
    bits: u8,
}
impl COMPENC3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _COMPENR1W<'a> {
    w: &'a mut W,
}
impl<'a> _COMPENR1W<'a> {
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
#[doc = r" Proxy"]
pub struct _COMPENR2W<'a> {
    w: &'a mut W,
}
impl<'a> _COMPENR2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COMPENR3W<'a> {
    w: &'a mut W,
}
impl<'a> _COMPENR3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COMPENC1W<'a> {
    w: &'a mut W,
}
impl<'a> _COMPENC1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COMPENC2W<'a> {
    w: &'a mut W,
}
impl<'a> _COMPENC2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COMPENC3W<'a> {
    w: &'a mut W,
}
impl<'a> _COMPENC3W<'a> {
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
    #[doc = "Bits 0:2 - Low Noise Mode Compensator R1 Trim Value"]
    #[inline]
    pub fn compenr1(&self) -> COMPENR1R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        COMPENR1R { bits }
    }
    #[doc = "Bits 4:8 - Low Noise Mode Compensator R2 Trim Value"]
    #[inline]
    pub fn compenr2(&self) -> COMPENR2R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        COMPENR2R { bits }
    }
    #[doc = "Bits 12:15 - Low Noise Mode Compensator R3 Trim Value"]
    #[inline]
    pub fn compenr3(&self) -> COMPENR3R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        COMPENR3R { bits }
    }
    #[doc = "Bits 20:21 - Low Noise Mode Compensator C1 Trim Value"]
    #[inline]
    pub fn compenc1(&self) -> COMPENC1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        COMPENC1R { bits }
    }
    #[doc = "Bits 24:26 - Low Noise Mode Compensator C2 Trim Value"]
    #[inline]
    pub fn compenc2(&self) -> COMPENC2R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        COMPENC2R { bits }
    }
    #[doc = "Bits 28:31 - Low Noise Mode Compensator C3 Trim Value"]
    #[inline]
    pub fn compenc3(&self) -> COMPENC3R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        COMPENC3R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1461731447 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Low Noise Mode Compensator R1 Trim Value"]
    #[inline]
    pub fn compenr1(&mut self) -> _COMPENR1W {
        _COMPENR1W { w: self }
    }
    #[doc = "Bits 4:8 - Low Noise Mode Compensator R2 Trim Value"]
    #[inline]
    pub fn compenr2(&mut self) -> _COMPENR2W {
        _COMPENR2W { w: self }
    }
    #[doc = "Bits 12:15 - Low Noise Mode Compensator R3 Trim Value"]
    #[inline]
    pub fn compenr3(&mut self) -> _COMPENR3W {
        _COMPENR3W { w: self }
    }
    #[doc = "Bits 20:21 - Low Noise Mode Compensator C1 Trim Value"]
    #[inline]
    pub fn compenc1(&mut self) -> _COMPENC1W {
        _COMPENC1W { w: self }
    }
    #[doc = "Bits 24:26 - Low Noise Mode Compensator C2 Trim Value"]
    #[inline]
    pub fn compenc2(&mut self) -> _COMPENC2W {
        _COMPENC2W { w: self }
    }
    #[doc = "Bits 28:31 - Low Noise Mode Compensator C3 Trim Value"]
    #[inline]
    pub fn compenc3(&mut self) -> _COMPENC3W {
        _COMPENC3W { w: self }
    }
}
