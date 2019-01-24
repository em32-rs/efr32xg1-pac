#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RAM0CTRL {
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
#[doc = "Possible values of the field `RAMPOWERDOWN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMPOWERDOWNR {
    #[doc = "None of the RAM blocks powered down"]
    NONE,
    #[doc = "Power down RAM blocks 4 and above"]
    BLK4,
    #[doc = "Power down RAM blocks 3 and above"]
    BLK3TO4,
    #[doc = "Power down RAM blocks 2 and above"]
    BLK2TO4,
    #[doc = "Power down RAM blocks 1 and above"]
    BLK1TO4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RAMPOWERDOWNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RAMPOWERDOWNR::NONE => 0,
            RAMPOWERDOWNR::BLK4 => 8,
            RAMPOWERDOWNR::BLK3TO4 => 12,
            RAMPOWERDOWNR::BLK2TO4 => 14,
            RAMPOWERDOWNR::BLK1TO4 => 15,
            RAMPOWERDOWNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RAMPOWERDOWNR {
        match value {
            0 => RAMPOWERDOWNR::NONE,
            8 => RAMPOWERDOWNR::BLK4,
            12 => RAMPOWERDOWNR::BLK3TO4,
            14 => RAMPOWERDOWNR::BLK2TO4,
            15 => RAMPOWERDOWNR::BLK1TO4,
            i => RAMPOWERDOWNR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == RAMPOWERDOWNR::NONE
    }
    #[doc = "Checks if the value of the field is `BLK4`"]
    #[inline]
    pub fn is_blk4(&self) -> bool {
        *self == RAMPOWERDOWNR::BLK4
    }
    #[doc = "Checks if the value of the field is `BLK3TO4`"]
    #[inline]
    pub fn is_blk3to4(&self) -> bool {
        *self == RAMPOWERDOWNR::BLK3TO4
    }
    #[doc = "Checks if the value of the field is `BLK2TO4`"]
    #[inline]
    pub fn is_blk2to4(&self) -> bool {
        *self == RAMPOWERDOWNR::BLK2TO4
    }
    #[doc = "Checks if the value of the field is `BLK1TO4`"]
    #[inline]
    pub fn is_blk1to4(&self) -> bool {
        *self == RAMPOWERDOWNR::BLK1TO4
    }
}
#[doc = "Values that can be written to the field `RAMPOWERDOWN`"]
pub enum RAMPOWERDOWNW {
    #[doc = "None of the RAM blocks powered down"]
    NONE,
    #[doc = "Power down RAM blocks 4 and above"]
    BLK4,
    #[doc = "Power down RAM blocks 3 and above"]
    BLK3TO4,
    #[doc = "Power down RAM blocks 2 and above"]
    BLK2TO4,
    #[doc = "Power down RAM blocks 1 and above"]
    BLK1TO4,
}
impl RAMPOWERDOWNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RAMPOWERDOWNW::NONE => 0,
            RAMPOWERDOWNW::BLK4 => 8,
            RAMPOWERDOWNW::BLK3TO4 => 12,
            RAMPOWERDOWNW::BLK2TO4 => 14,
            RAMPOWERDOWNW::BLK1TO4 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAMPOWERDOWNW<'a> {
    w: &'a mut W,
}
impl<'a> _RAMPOWERDOWNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAMPOWERDOWNW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "None of the RAM blocks powered down"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(RAMPOWERDOWNW::NONE)
    }
    #[doc = "Power down RAM blocks 4 and above"]
    #[inline]
    pub fn blk4(self) -> &'a mut W {
        self.variant(RAMPOWERDOWNW::BLK4)
    }
    #[doc = "Power down RAM blocks 3 and above"]
    #[inline]
    pub fn blk3to4(self) -> &'a mut W {
        self.variant(RAMPOWERDOWNW::BLK3TO4)
    }
    #[doc = "Power down RAM blocks 2 and above"]
    #[inline]
    pub fn blk2to4(self) -> &'a mut W {
        self.variant(RAMPOWERDOWNW::BLK2TO4)
    }
    #[doc = "Power down RAM blocks 1 and above"]
    #[inline]
    pub fn blk1to4(self) -> &'a mut W {
        self.variant(RAMPOWERDOWNW::BLK1TO4)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - RAM0 Blockset Power-down"]
    #[inline]
    pub fn rampowerdown(&self) -> RAMPOWERDOWNR {
        RAMPOWERDOWNR::_from({
            const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - RAM0 Blockset Power-down"]
    #[inline]
    pub fn rampowerdown(&mut self) -> _RAMPOWERDOWNW {
        _RAMPOWERDOWNW { w: self }
    }
}
