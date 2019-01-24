#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `WDOGRMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOGRMODER {
    #[doc = "Reset request is blocked. This disable bit is redundant with enable/disable bit in WDOG"]
    DISABLED,
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    LIMITED,
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset. "]
    EXTENDED,
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    FULL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WDOGRMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WDOGRMODER::DISABLED => 0,
            WDOGRMODER::LIMITED => 1,
            WDOGRMODER::EXTENDED => 2,
            WDOGRMODER::FULL => 4,
            WDOGRMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WDOGRMODER {
        match value {
            0 => WDOGRMODER::DISABLED,
            1 => WDOGRMODER::LIMITED,
            2 => WDOGRMODER::EXTENDED,
            4 => WDOGRMODER::FULL,
            i => WDOGRMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == WDOGRMODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `LIMITED`"]
    #[inline]
    pub fn is_limited(&self) -> bool {
        *self == WDOGRMODER::LIMITED
    }
    #[doc = "Checks if the value of the field is `EXTENDED`"]
    #[inline]
    pub fn is_extended(&self) -> bool {
        *self == WDOGRMODER::EXTENDED
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline]
    pub fn is_full(&self) -> bool {
        *self == WDOGRMODER::FULL
    }
}
#[doc = "Possible values of the field `LOCKUPRMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKUPRMODER {
    #[doc = "Reset request is blocked."]
    DISABLED,
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    LIMITED,
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset. "]
    EXTENDED,
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    FULL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LOCKUPRMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LOCKUPRMODER::DISABLED => 0,
            LOCKUPRMODER::LIMITED => 1,
            LOCKUPRMODER::EXTENDED => 2,
            LOCKUPRMODER::FULL => 4,
            LOCKUPRMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LOCKUPRMODER {
        match value {
            0 => LOCKUPRMODER::DISABLED,
            1 => LOCKUPRMODER::LIMITED,
            2 => LOCKUPRMODER::EXTENDED,
            4 => LOCKUPRMODER::FULL,
            i => LOCKUPRMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LOCKUPRMODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `LIMITED`"]
    #[inline]
    pub fn is_limited(&self) -> bool {
        *self == LOCKUPRMODER::LIMITED
    }
    #[doc = "Checks if the value of the field is `EXTENDED`"]
    #[inline]
    pub fn is_extended(&self) -> bool {
        *self == LOCKUPRMODER::EXTENDED
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline]
    pub fn is_full(&self) -> bool {
        *self == LOCKUPRMODER::FULL
    }
}
#[doc = "Possible values of the field `SYSRMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRMODER {
    #[doc = "Reset request is blocked. "]
    DISABLED,
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    LIMITED,
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset. "]
    EXTENDED,
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    FULL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SYSRMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSRMODER::DISABLED => 0,
            SYSRMODER::LIMITED => 1,
            SYSRMODER::EXTENDED => 2,
            SYSRMODER::FULL => 4,
            SYSRMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYSRMODER {
        match value {
            0 => SYSRMODER::DISABLED,
            1 => SYSRMODER::LIMITED,
            2 => SYSRMODER::EXTENDED,
            4 => SYSRMODER::FULL,
            i => SYSRMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SYSRMODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `LIMITED`"]
    #[inline]
    pub fn is_limited(&self) -> bool {
        *self == SYSRMODER::LIMITED
    }
    #[doc = "Checks if the value of the field is `EXTENDED`"]
    #[inline]
    pub fn is_extended(&self) -> bool {
        *self == SYSRMODER::EXTENDED
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline]
    pub fn is_full(&self) -> bool {
        *self == SYSRMODER::FULL
    }
}
#[doc = "Possible values of the field `PINRMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINRMODER {
    #[doc = "Reset request is blocked. "]
    DISABLED,
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    LIMITED,
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset. "]
    EXTENDED,
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    FULL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PINRMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PINRMODER::DISABLED => 0,
            PINRMODER::LIMITED => 1,
            PINRMODER::EXTENDED => 2,
            PINRMODER::FULL => 4,
            PINRMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PINRMODER {
        match value {
            0 => PINRMODER::DISABLED,
            1 => PINRMODER::LIMITED,
            2 => PINRMODER::EXTENDED,
            4 => PINRMODER::FULL,
            i => PINRMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PINRMODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `LIMITED`"]
    #[inline]
    pub fn is_limited(&self) -> bool {
        *self == PINRMODER::LIMITED
    }
    #[doc = "Checks if the value of the field is `EXTENDED`"]
    #[inline]
    pub fn is_extended(&self) -> bool {
        *self == PINRMODER::EXTENDED
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline]
    pub fn is_full(&self) -> bool {
        *self == PINRMODER::FULL
    }
}
#[doc = r" Value of the field"]
pub struct RESETSTATER {
    bits: u8,
}
impl RESETSTATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `WDOGRMODE`"]
pub enum WDOGRMODEW {
    #[doc = "Reset request is blocked. This disable bit is redundant with enable/disable bit in WDOG"]
    DISABLED,
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    LIMITED,
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset. "]
    EXTENDED,
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    FULL,
}
impl WDOGRMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WDOGRMODEW::DISABLED => 0,
            WDOGRMODEW::LIMITED => 1,
            WDOGRMODEW::EXTENDED => 2,
            WDOGRMODEW::FULL => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDOGRMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _WDOGRMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDOGRMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reset request is blocked. This disable bit is redundant with enable/disable bit in WDOG"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WDOGRMODEW::DISABLED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline]
    pub fn limited(self) -> &'a mut W {
        self.variant(WDOGRMODEW::LIMITED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline]
    pub fn extended(self) -> &'a mut W {
        self.variant(WDOGRMODEW::EXTENDED)
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline]
    pub fn full(self) -> &'a mut W {
        self.variant(WDOGRMODEW::FULL)
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
#[doc = "Values that can be written to the field `LOCKUPRMODE`"]
pub enum LOCKUPRMODEW {
    #[doc = "Reset request is blocked."]
    DISABLED,
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    LIMITED,
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset. "]
    EXTENDED,
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    FULL,
}
impl LOCKUPRMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LOCKUPRMODEW::DISABLED => 0,
            LOCKUPRMODEW::LIMITED => 1,
            LOCKUPRMODEW::EXTENDED => 2,
            LOCKUPRMODEW::FULL => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCKUPRMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKUPRMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCKUPRMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reset request is blocked."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOCKUPRMODEW::DISABLED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline]
    pub fn limited(self) -> &'a mut W {
        self.variant(LOCKUPRMODEW::LIMITED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline]
    pub fn extended(self) -> &'a mut W {
        self.variant(LOCKUPRMODEW::EXTENDED)
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline]
    pub fn full(self) -> &'a mut W {
        self.variant(LOCKUPRMODEW::FULL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYSRMODE`"]
pub enum SYSRMODEW {
    #[doc = "Reset request is blocked. "]
    DISABLED,
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    LIMITED,
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset. "]
    EXTENDED,
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    FULL,
}
impl SYSRMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSRMODEW::DISABLED => 0,
            SYSRMODEW::LIMITED => 1,
            SYSRMODEW::EXTENDED => 2,
            SYSRMODEW::FULL => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYSRMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSRMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSRMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reset request is blocked."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYSRMODEW::DISABLED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline]
    pub fn limited(self) -> &'a mut W {
        self.variant(SYSRMODEW::LIMITED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline]
    pub fn extended(self) -> &'a mut W {
        self.variant(SYSRMODEW::EXTENDED)
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline]
    pub fn full(self) -> &'a mut W {
        self.variant(SYSRMODEW::FULL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PINRMODE`"]
pub enum PINRMODEW {
    #[doc = "Reset request is blocked. "]
    DISABLED,
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    LIMITED,
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset. "]
    EXTENDED,
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    FULL,
}
impl PINRMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PINRMODEW::DISABLED => 0,
            PINRMODEW::LIMITED => 1,
            PINRMODEW::EXTENDED => 2,
            PINRMODEW::FULL => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PINRMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PINRMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PINRMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reset request is blocked."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINRMODEW::DISABLED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline]
    pub fn limited(self) -> &'a mut W {
        self.variant(PINRMODEW::LIMITED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline]
    pub fn extended(self) -> &'a mut W {
        self.variant(PINRMODEW::EXTENDED)
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline]
    pub fn full(self) -> &'a mut W {
        self.variant(PINRMODEW::FULL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESETSTATEW<'a> {
    w: &'a mut W,
}
impl<'a> _RESETSTATEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:2 - WDOG Reset Mode"]
    #[inline]
    pub fn wdogrmode(&self) -> WDOGRMODER {
        WDOGRMODER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Core LOCKUP Reset Mode"]
    #[inline]
    pub fn lockuprmode(&self) -> LOCKUPRMODER {
        LOCKUPRMODER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - Core Sysreset Reset Mode"]
    #[inline]
    pub fn sysrmode(&self) -> SYSRMODER {
        SYSRMODER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - PIN Reset Mode"]
    #[inline]
    pub fn pinrmode(&self) -> PINRMODER {
        PINRMODER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - System Software Reset State"]
    #[inline]
    pub fn resetstate(&self) -> RESETSTATER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESETSTATER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16932 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - WDOG Reset Mode"]
    #[inline]
    pub fn wdogrmode(&mut self) -> _WDOGRMODEW {
        _WDOGRMODEW { w: self }
    }
    #[doc = "Bits 4:6 - Core LOCKUP Reset Mode"]
    #[inline]
    pub fn lockuprmode(&mut self) -> _LOCKUPRMODEW {
        _LOCKUPRMODEW { w: self }
    }
    #[doc = "Bits 8:10 - Core Sysreset Reset Mode"]
    #[inline]
    pub fn sysrmode(&mut self) -> _SYSRMODEW {
        _SYSRMODEW { w: self }
    }
    #[doc = "Bits 12:14 - PIN Reset Mode"]
    #[inline]
    pub fn pinrmode(&mut self) -> _PINRMODEW {
        _PINRMODEW { w: self }
    }
    #[doc = "Bits 24:25 - System Software Reset State"]
    #[inline]
    pub fn resetstate(&mut self) -> _RESETSTATEW {
        _RESETSTATEW { w: self }
    }
}
