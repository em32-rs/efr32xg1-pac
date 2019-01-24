#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BIASPROG {
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
#[doc = "Possible values of the field `ADCBIASPROG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCBIASPROGR {
    #[doc = "Normal power (use for 1Msps operation)"]
    NORMAL,
    #[doc = "Scaling bias to 1/2"]
    SCALE2,
    #[doc = "Scaling bias to 1/4"]
    SCALE4,
    #[doc = "Scaling bias to 1/8"]
    SCALE8,
    #[doc = "Scaling bias to 1/16"]
    SCALE16,
    #[doc = "Scaling bias to 1/32"]
    SCALE32,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ADCBIASPROGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCBIASPROGR::NORMAL => 0,
            ADCBIASPROGR::SCALE2 => 4,
            ADCBIASPROGR::SCALE4 => 8,
            ADCBIASPROGR::SCALE8 => 12,
            ADCBIASPROGR::SCALE16 => 14,
            ADCBIASPROGR::SCALE32 => 15,
            ADCBIASPROGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCBIASPROGR {
        match value {
            0 => ADCBIASPROGR::NORMAL,
            4 => ADCBIASPROGR::SCALE2,
            8 => ADCBIASPROGR::SCALE4,
            12 => ADCBIASPROGR::SCALE8,
            14 => ADCBIASPROGR::SCALE16,
            15 => ADCBIASPROGR::SCALE32,
            i => ADCBIASPROGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == ADCBIASPROGR::NORMAL
    }
    #[doc = "Checks if the value of the field is `SCALE2`"]
    #[inline]
    pub fn is_scale2(&self) -> bool {
        *self == ADCBIASPROGR::SCALE2
    }
    #[doc = "Checks if the value of the field is `SCALE4`"]
    #[inline]
    pub fn is_scale4(&self) -> bool {
        *self == ADCBIASPROGR::SCALE4
    }
    #[doc = "Checks if the value of the field is `SCALE8`"]
    #[inline]
    pub fn is_scale8(&self) -> bool {
        *self == ADCBIASPROGR::SCALE8
    }
    #[doc = "Checks if the value of the field is `SCALE16`"]
    #[inline]
    pub fn is_scale16(&self) -> bool {
        *self == ADCBIASPROGR::SCALE16
    }
    #[doc = "Checks if the value of the field is `SCALE32`"]
    #[inline]
    pub fn is_scale32(&self) -> bool {
        *self == ADCBIASPROGR::SCALE32
    }
}
#[doc = r" Value of the field"]
pub struct VFAULTCLRR {
    bits: bool,
}
impl VFAULTCLRR {
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
pub struct GPBIASACCR {
    bits: bool,
}
impl GPBIASACCR {
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
#[doc = "Values that can be written to the field `ADCBIASPROG`"]
pub enum ADCBIASPROGW {
    #[doc = "Normal power (use for 1Msps operation)"]
    NORMAL,
    #[doc = "Scaling bias to 1/2"]
    SCALE2,
    #[doc = "Scaling bias to 1/4"]
    SCALE4,
    #[doc = "Scaling bias to 1/8"]
    SCALE8,
    #[doc = "Scaling bias to 1/16"]
    SCALE16,
    #[doc = "Scaling bias to 1/32"]
    SCALE32,
}
impl ADCBIASPROGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCBIASPROGW::NORMAL => 0,
            ADCBIASPROGW::SCALE2 => 4,
            ADCBIASPROGW::SCALE4 => 8,
            ADCBIASPROGW::SCALE8 => 12,
            ADCBIASPROGW::SCALE16 => 14,
            ADCBIASPROGW::SCALE32 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCBIASPROGW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCBIASPROGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCBIASPROGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Normal power (use for 1Msps operation)"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(ADCBIASPROGW::NORMAL)
    }
    #[doc = "Scaling bias to 1/2"]
    #[inline]
    pub fn scale2(self) -> &'a mut W {
        self.variant(ADCBIASPROGW::SCALE2)
    }
    #[doc = "Scaling bias to 1/4"]
    #[inline]
    pub fn scale4(self) -> &'a mut W {
        self.variant(ADCBIASPROGW::SCALE4)
    }
    #[doc = "Scaling bias to 1/8"]
    #[inline]
    pub fn scale8(self) -> &'a mut W {
        self.variant(ADCBIASPROGW::SCALE8)
    }
    #[doc = "Scaling bias to 1/16"]
    #[inline]
    pub fn scale16(self) -> &'a mut W {
        self.variant(ADCBIASPROGW::SCALE16)
    }
    #[doc = "Scaling bias to 1/32"]
    #[inline]
    pub fn scale32(self) -> &'a mut W {
        self.variant(ADCBIASPROGW::SCALE32)
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
#[doc = r" Proxy"]
pub struct _VFAULTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _VFAULTCLRW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPBIASACCW<'a> {
    w: &'a mut W,
}
impl<'a> _GPBIASACCW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Bias Programming Value of Analog ADC Block"]
    #[inline]
    pub fn adcbiasprog(&self) -> ADCBIASPROGR {
        ADCBIASPROGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Clear VREFOF Flag"]
    #[inline]
    pub fn vfaultclr(&self) -> VFAULTCLRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VFAULTCLRR { bits }
    }
    #[doc = "Bit 16 - Accuracy Setting for the System Bias During ADC Operation"]
    #[inline]
    pub fn gpbiasacc(&self) -> GPBIASACCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPBIASACCR { bits }
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
    #[doc = "Bits 0:3 - Bias Programming Value of Analog ADC Block"]
    #[inline]
    pub fn adcbiasprog(&mut self) -> _ADCBIASPROGW {
        _ADCBIASPROGW { w: self }
    }
    #[doc = "Bit 12 - Clear VREFOF Flag"]
    #[inline]
    pub fn vfaultclr(&mut self) -> _VFAULTCLRW {
        _VFAULTCLRW { w: self }
    }
    #[doc = "Bit 16 - Accuracy Setting for the System Bias During ADC Operation"]
    #[inline]
    pub fn gpbiasacc(&mut self) -> _GPBIASACCW {
        _GPBIASACCW { w: self }
    }
}
