#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWRCFG {
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
#[doc = "Possible values of the field `PWRCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRCFGR {
    #[doc = "Power up configuration. Works with any external configuration."]
    STARTUP,
    #[doc = "DCDC is enabled and routed to DVDD."]
    DCDCTODVDD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWRCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWRCFGR::STARTUP => 0,
            PWRCFGR::DCDCTODVDD => 2,
            PWRCFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWRCFGR {
        match value {
            0 => PWRCFGR::STARTUP,
            2 => PWRCFGR::DCDCTODVDD,
            i => PWRCFGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STARTUP`"]
    #[inline]
    pub fn is_startup(&self) -> bool {
        *self == PWRCFGR::STARTUP
    }
    #[doc = "Checks if the value of the field is `DCDCTODVDD`"]
    #[inline]
    pub fn is_dcdctodvdd(&self) -> bool {
        *self == PWRCFGR::DCDCTODVDD
    }
}
#[doc = "Values that can be written to the field `PWRCFG`"]
pub enum PWRCFGW {
    #[doc = "Power up configuration. Works with any external configuration."]
    STARTUP,
    #[doc = "DCDC is enabled and routed to DVDD."]
    DCDCTODVDD,
}
impl PWRCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWRCFGW::STARTUP => 0,
            PWRCFGW::DCDCTODVDD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRCFGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Power up configuration. Works with any external configuration."]
    #[inline]
    pub fn startup(self) -> &'a mut W {
        self.variant(PWRCFGW::STARTUP)
    }
    #[doc = "DCDC is enabled and routed to DVDD."]
    #[inline]
    pub fn dcdctodvdd(self) -> &'a mut W {
        self.variant(PWRCFGW::DCDCTODVDD)
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
    #[doc = "Bits 0:3 - Power Configuration"]
    #[inline]
    pub fn pwrcfg(&self) -> PWRCFGR {
        PWRCFGR::_from({
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
    #[doc = "Bits 0:3 - Power Configuration"]
    #[inline]
    pub fn pwrcfg(&mut self) -> _PWRCFGW {
        _PWRCFGW { w: self }
    }
}
