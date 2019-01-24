#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCANCTRL {
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
pub struct REPR {
    bits: bool,
}
impl REPR {
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
pub struct DIFFR {
    bits: bool,
}
impl DIFFR {
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
pub struct ADJR {
    bits: bool,
}
impl ADJR {
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
#[doc = "Possible values of the field `RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESR {
    #[doc = "12-bit resolution"]
    _12BIT,
    #[doc = "8-bit resolution"]
    _8BIT,
    #[doc = "6-bit resolution"]
    _6BIT,
    #[doc = "Oversampling enabled. Oversampling rate is set in OVSRSEL"]
    OVS,
}
impl RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RESR::_12BIT => 0,
            RESR::_8BIT => 1,
            RESR::_6BIT => 2,
            RESR::OVS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RESR {
        match value {
            0 => RESR::_12BIT,
            1 => RESR::_8BIT,
            2 => RESR::_6BIT,
            3 => RESR::OVS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_12BIT`"]
    #[inline]
    pub fn is_12bit(&self) -> bool {
        *self == RESR::_12BIT
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline]
    pub fn is_8bit(&self) -> bool {
        *self == RESR::_8BIT
    }
    #[doc = "Checks if the value of the field is `_6BIT`"]
    #[inline]
    pub fn is_6bit(&self) -> bool {
        *self == RESR::_6BIT
    }
    #[doc = "Checks if the value of the field is `OVS`"]
    #[inline]
    pub fn is_ovs(&self) -> bool {
        *self == RESR::OVS
    }
}
#[doc = "Possible values of the field `REF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFR {
    #[doc = "VFS = 1.25V with internal VBGR reference"]
    _1V25,
    #[doc = "VFS = 2.5V with internal VBGR reference"]
    _2V5,
    #[doc = "VFS = AVDD with AVDD as reference source"]
    VDD,
    #[doc = "VFS = 5V with internal VBGR reference"]
    _5V,
    #[doc = "Single ended external reference"]
    EXTSINGLE,
    #[doc = "Differential external reference, 2x"]
    _2XEXTDIFF,
    #[doc = "VFS=2xAVDD with AVDD as the reference source"]
    _2XVDD,
    #[doc = "Use SCANCTRLX to configure reference"]
    CONF,
}
impl REFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFR::_1V25 => 0,
            REFR::_2V5 => 1,
            REFR::VDD => 2,
            REFR::_5V => 3,
            REFR::EXTSINGLE => 4,
            REFR::_2XEXTDIFF => 5,
            REFR::_2XVDD => 6,
            REFR::CONF => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFR {
        match value {
            0 => REFR::_1V25,
            1 => REFR::_2V5,
            2 => REFR::VDD,
            3 => REFR::_5V,
            4 => REFR::EXTSINGLE,
            5 => REFR::_2XEXTDIFF,
            6 => REFR::_2XVDD,
            7 => REFR::CONF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1V25`"]
    #[inline]
    pub fn is_1v25(&self) -> bool {
        *self == REFR::_1V25
    }
    #[doc = "Checks if the value of the field is `_2V5`"]
    #[inline]
    pub fn is_2v5(&self) -> bool {
        *self == REFR::_2V5
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline]
    pub fn is_vdd(&self) -> bool {
        *self == REFR::VDD
    }
    #[doc = "Checks if the value of the field is `_5V`"]
    #[inline]
    pub fn is_5v(&self) -> bool {
        *self == REFR::_5V
    }
    #[doc = "Checks if the value of the field is `EXTSINGLE`"]
    #[inline]
    pub fn is_extsingle(&self) -> bool {
        *self == REFR::EXTSINGLE
    }
    #[doc = "Checks if the value of the field is `_2XEXTDIFF`"]
    #[inline]
    pub fn is_2xextdiff(&self) -> bool {
        *self == REFR::_2XEXTDIFF
    }
    #[doc = "Checks if the value of the field is `_2XVDD`"]
    #[inline]
    pub fn is_2xvdd(&self) -> bool {
        *self == REFR::_2XVDD
    }
    #[doc = "Checks if the value of the field is `CONF`"]
    #[inline]
    pub fn is_conf(&self) -> bool {
        *self == REFR::CONF
    }
}
#[doc = "Possible values of the field `AT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATR {
    #[doc = "1 conversion clock cycle acquisition time for scan"]
    _1CYCLE,
    #[doc = "2 conversion clock cycles acquisition time for scan"]
    _2CYCLES,
    #[doc = "3 conversion clock cycles acquisition time for scan"]
    _3CYCLES,
    #[doc = "4 conversion clock cycles acquisition time for scan"]
    _4CYCLES,
    #[doc = "8 conversion clock cycles acquisition time for scan"]
    _8CYCLES,
    #[doc = "16 conversion clock cycles acquisition time for scan"]
    _16CYCLES,
    #[doc = "32 conversion clock cycles acquisition time for scan"]
    _32CYCLES,
    #[doc = "64 conversion clock cycles acquisition time for scan"]
    _64CYCLES,
    #[doc = "128 conversion clock cycles acquisition time for scan"]
    _128CYCLES,
    #[doc = "256 conversion clock cycles acquisition time for scan"]
    _256CYCLES,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ATR::_1CYCLE => 0,
            ATR::_2CYCLES => 1,
            ATR::_3CYCLES => 2,
            ATR::_4CYCLES => 3,
            ATR::_8CYCLES => 4,
            ATR::_16CYCLES => 5,
            ATR::_32CYCLES => 6,
            ATR::_64CYCLES => 7,
            ATR::_128CYCLES => 8,
            ATR::_256CYCLES => 9,
            ATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ATR {
        match value {
            0 => ATR::_1CYCLE,
            1 => ATR::_2CYCLES,
            2 => ATR::_3CYCLES,
            3 => ATR::_4CYCLES,
            4 => ATR::_8CYCLES,
            5 => ATR::_16CYCLES,
            6 => ATR::_32CYCLES,
            7 => ATR::_64CYCLES,
            8 => ATR::_128CYCLES,
            9 => ATR::_256CYCLES,
            i => ATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1CYCLE`"]
    #[inline]
    pub fn is_1cycle(&self) -> bool {
        *self == ATR::_1CYCLE
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline]
    pub fn is_2cycles(&self) -> bool {
        *self == ATR::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_3CYCLES`"]
    #[inline]
    pub fn is_3cycles(&self) -> bool {
        *self == ATR::_3CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline]
    pub fn is_4cycles(&self) -> bool {
        *self == ATR::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_8CYCLES`"]
    #[inline]
    pub fn is_8cycles(&self) -> bool {
        *self == ATR::_8CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline]
    pub fn is_16cycles(&self) -> bool {
        *self == ATR::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline]
    pub fn is_32cycles(&self) -> bool {
        *self == ATR::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline]
    pub fn is_64cycles(&self) -> bool {
        *self == ATR::_64CYCLES
    }
    #[doc = "Checks if the value of the field is `_128CYCLES`"]
    #[inline]
    pub fn is_128cycles(&self) -> bool {
        *self == ATR::_128CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline]
    pub fn is_256cycles(&self) -> bool {
        *self == ATR::_256CYCLES
    }
}
#[doc = r" Value of the field"]
pub struct PRSENR {
    bits: bool,
}
impl PRSENR {
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
pub struct CMPENR {
    bits: bool,
}
impl CMPENR {
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
#[doc = r" Proxy"]
pub struct _REPW<'a> {
    w: &'a mut W,
}
impl<'a> _REPW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIFFW<'a> {
    w: &'a mut W,
}
impl<'a> _DIFFW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADJW<'a> {
    w: &'a mut W,
}
impl<'a> _ADJW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RES`"]
pub enum RESW {
    #[doc = "12-bit resolution"]
    _12BIT,
    #[doc = "8-bit resolution"]
    _8BIT,
    #[doc = "6-bit resolution"]
    _6BIT,
    #[doc = "Oversampling enabled. Oversampling rate is set in OVSRSEL"]
    OVS,
}
impl RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RESW::_12BIT => 0,
            RESW::_8BIT => 1,
            RESW::_6BIT => 2,
            RESW::OVS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESW<'a> {
    w: &'a mut W,
}
impl<'a> _RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "12-bit resolution"]
    #[inline]
    pub fn _12bit(self) -> &'a mut W {
        self.variant(RESW::_12BIT)
    }
    #[doc = "8-bit resolution"]
    #[inline]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(RESW::_8BIT)
    }
    #[doc = "6-bit resolution"]
    #[inline]
    pub fn _6bit(self) -> &'a mut W {
        self.variant(RESW::_6BIT)
    }
    #[doc = "Oversampling enabled. Oversampling rate is set in OVSRSEL"]
    #[inline]
    pub fn ovs(self) -> &'a mut W {
        self.variant(RESW::OVS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REF`"]
pub enum REFW {
    #[doc = "VFS = 1.25V with internal VBGR reference"]
    _1V25,
    #[doc = "VFS = 2.5V with internal VBGR reference"]
    _2V5,
    #[doc = "VFS = AVDD with AVDD as reference source"]
    VDD,
    #[doc = "VFS = 5V with internal VBGR reference"]
    _5V,
    #[doc = "Single ended external reference"]
    EXTSINGLE,
    #[doc = "Differential external reference, 2x"]
    _2XEXTDIFF,
    #[doc = "VFS=2xAVDD with AVDD as the reference source"]
    _2XVDD,
    #[doc = "Use SCANCTRLX to configure reference"]
    CONF,
}
impl REFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFW::_1V25 => 0,
            REFW::_2V5 => 1,
            REFW::VDD => 2,
            REFW::_5V => 3,
            REFW::EXTSINGLE => 4,
            REFW::_2XEXTDIFF => 5,
            REFW::_2XVDD => 6,
            REFW::CONF => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFW<'a> {
    w: &'a mut W,
}
impl<'a> _REFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "VFS = 1.25V with internal VBGR reference"]
    #[inline]
    pub fn _1v25(self) -> &'a mut W {
        self.variant(REFW::_1V25)
    }
    #[doc = "VFS = 2.5V with internal VBGR reference"]
    #[inline]
    pub fn _2v5(self) -> &'a mut W {
        self.variant(REFW::_2V5)
    }
    #[doc = "VFS = AVDD with AVDD as reference source"]
    #[inline]
    pub fn vdd(self) -> &'a mut W {
        self.variant(REFW::VDD)
    }
    #[doc = "VFS = 5V with internal VBGR reference"]
    #[inline]
    pub fn _5v(self) -> &'a mut W {
        self.variant(REFW::_5V)
    }
    #[doc = "Single ended external reference"]
    #[inline]
    pub fn extsingle(self) -> &'a mut W {
        self.variant(REFW::EXTSINGLE)
    }
    #[doc = "Differential external reference, 2x"]
    #[inline]
    pub fn _2xextdiff(self) -> &'a mut W {
        self.variant(REFW::_2XEXTDIFF)
    }
    #[doc = "VFS=2xAVDD with AVDD as the reference source"]
    #[inline]
    pub fn _2xvdd(self) -> &'a mut W {
        self.variant(REFW::_2XVDD)
    }
    #[doc = "Use SCANCTRLX to configure reference"]
    #[inline]
    pub fn conf(self) -> &'a mut W {
        self.variant(REFW::CONF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AT`"]
pub enum ATW {
    #[doc = "1 conversion clock cycle acquisition time for scan"]
    _1CYCLE,
    #[doc = "2 conversion clock cycles acquisition time for scan"]
    _2CYCLES,
    #[doc = "3 conversion clock cycles acquisition time for scan"]
    _3CYCLES,
    #[doc = "4 conversion clock cycles acquisition time for scan"]
    _4CYCLES,
    #[doc = "8 conversion clock cycles acquisition time for scan"]
    _8CYCLES,
    #[doc = "16 conversion clock cycles acquisition time for scan"]
    _16CYCLES,
    #[doc = "32 conversion clock cycles acquisition time for scan"]
    _32CYCLES,
    #[doc = "64 conversion clock cycles acquisition time for scan"]
    _64CYCLES,
    #[doc = "128 conversion clock cycles acquisition time for scan"]
    _128CYCLES,
    #[doc = "256 conversion clock cycles acquisition time for scan"]
    _256CYCLES,
}
impl ATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ATW::_1CYCLE => 0,
            ATW::_2CYCLES => 1,
            ATW::_3CYCLES => 2,
            ATW::_4CYCLES => 3,
            ATW::_8CYCLES => 4,
            ATW::_16CYCLES => 5,
            ATW::_32CYCLES => 6,
            ATW::_64CYCLES => 7,
            ATW::_128CYCLES => 8,
            ATW::_256CYCLES => 9,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ATW<'a> {
    w: &'a mut W,
}
impl<'a> _ATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ATW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 conversion clock cycle acquisition time for scan"]
    #[inline]
    pub fn _1cycle(self) -> &'a mut W {
        self.variant(ATW::_1CYCLE)
    }
    #[doc = "2 conversion clock cycles acquisition time for scan"]
    #[inline]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(ATW::_2CYCLES)
    }
    #[doc = "3 conversion clock cycles acquisition time for scan"]
    #[inline]
    pub fn _3cycles(self) -> &'a mut W {
        self.variant(ATW::_3CYCLES)
    }
    #[doc = "4 conversion clock cycles acquisition time for scan"]
    #[inline]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(ATW::_4CYCLES)
    }
    #[doc = "8 conversion clock cycles acquisition time for scan"]
    #[inline]
    pub fn _8cycles(self) -> &'a mut W {
        self.variant(ATW::_8CYCLES)
    }
    #[doc = "16 conversion clock cycles acquisition time for scan"]
    #[inline]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(ATW::_16CYCLES)
    }
    #[doc = "32 conversion clock cycles acquisition time for scan"]
    #[inline]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(ATW::_32CYCLES)
    }
    #[doc = "64 conversion clock cycles acquisition time for scan"]
    #[inline]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(ATW::_64CYCLES)
    }
    #[doc = "128 conversion clock cycles acquisition time for scan"]
    #[inline]
    pub fn _128cycles(self) -> &'a mut W {
        self.variant(ATW::_128CYCLES)
    }
    #[doc = "256 conversion clock cycles acquisition time for scan"]
    #[inline]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(ATW::_256CYCLES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRSENW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSENW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMPENW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPENW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Scan Sequence Repetitive Mode"]
    #[inline]
    pub fn rep(&self) -> REPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        REPR { bits }
    }
    #[doc = "Bit 1 - Scan Sequence Differential Mode"]
    #[inline]
    pub fn diff(&self) -> DIFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIFFR { bits }
    }
    #[doc = "Bit 2 - Scan Sequence Result Adjustment"]
    #[inline]
    pub fn adj(&self) -> ADJR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADJR { bits }
    }
    #[doc = "Bits 3:4 - Scan Sequence Resolution Select"]
    #[inline]
    pub fn res(&self) -> RESR {
        RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 5:7 - Scan Sequence Reference Selection"]
    #[inline]
    pub fn ref_(&self) -> REFR {
        REFR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Scan Acquisition Time"]
    #[inline]
    pub fn at(&self) -> ATR {
        ATR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 29 - Scan Sequence PRS Trigger Enable"]
    #[inline]
    pub fn prsen(&self) -> PRSENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRSENR { bits }
    }
    #[doc = "Bit 31 - Compare Logic Enable for Scan"]
    #[inline]
    pub fn cmpen(&self) -> CMPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CMPENR { bits }
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
    #[doc = "Bit 0 - Scan Sequence Repetitive Mode"]
    #[inline]
    pub fn rep(&mut self) -> _REPW {
        _REPW { w: self }
    }
    #[doc = "Bit 1 - Scan Sequence Differential Mode"]
    #[inline]
    pub fn diff(&mut self) -> _DIFFW {
        _DIFFW { w: self }
    }
    #[doc = "Bit 2 - Scan Sequence Result Adjustment"]
    #[inline]
    pub fn adj(&mut self) -> _ADJW {
        _ADJW { w: self }
    }
    #[doc = "Bits 3:4 - Scan Sequence Resolution Select"]
    #[inline]
    pub fn res(&mut self) -> _RESW {
        _RESW { w: self }
    }
    #[doc = "Bits 5:7 - Scan Sequence Reference Selection"]
    #[inline]
    pub fn ref_(&mut self) -> _REFW {
        _REFW { w: self }
    }
    #[doc = "Bits 24:27 - Scan Acquisition Time"]
    #[inline]
    pub fn at(&mut self) -> _ATW {
        _ATW { w: self }
    }
    #[doc = "Bit 29 - Scan Sequence PRS Trigger Enable"]
    #[inline]
    pub fn prsen(&mut self) -> _PRSENW {
        _PRSENW { w: self }
    }
    #[doc = "Bit 31 - Compare Logic Enable for Scan"]
    #[inline]
    pub fn cmpen(&mut self) -> _CMPENW {
        _CMPENW { w: self }
    }
}
