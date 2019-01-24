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
#[doc = r" Value of the field"]
pub struct ENR {
    bits: bool,
}
impl ENR {
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
pub struct INACTVALR {
    bits: bool,
}
impl INACTVALR {
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
pub struct GPIOINVR {
    bits: bool,
}
impl GPIOINVR {
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
pub struct APORTXMASTERDISR {
    bits: bool,
}
impl APORTXMASTERDISR {
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
pub struct APORTYMASTERDISR {
    bits: bool,
}
impl APORTYMASTERDISR {
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
pub struct APORTVMASTERDISR {
    bits: bool,
}
impl APORTVMASTERDISR {
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
#[doc = "Possible values of the field `PWRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRSELR {
    #[doc = "AVDD supply"]
    AVDD,
    #[doc = "DVDD supply"]
    DVDD,
    #[doc = "IOVDD/IOVDD0 supply"]
    IOVDD0,
    #[doc = "IOVDD1 supply (if part has two I/O voltages)"]
    IOVDD1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWRSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWRSELR::AVDD => 0,
            PWRSELR::DVDD => 1,
            PWRSELR::IOVDD0 => 2,
            PWRSELR::IOVDD1 => 4,
            PWRSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWRSELR {
        match value {
            0 => PWRSELR::AVDD,
            1 => PWRSELR::DVDD,
            2 => PWRSELR::IOVDD0,
            4 => PWRSELR::IOVDD1,
            i => PWRSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AVDD`"]
    #[inline]
    pub fn is_avdd(&self) -> bool {
        *self == PWRSELR::AVDD
    }
    #[doc = "Checks if the value of the field is `DVDD`"]
    #[inline]
    pub fn is_dvdd(&self) -> bool {
        *self == PWRSELR::DVDD
    }
    #[doc = "Checks if the value of the field is `IOVDD0`"]
    #[inline]
    pub fn is_iovdd0(&self) -> bool {
        *self == PWRSELR::IOVDD0
    }
    #[doc = "Checks if the value of the field is `IOVDD1`"]
    #[inline]
    pub fn is_iovdd1(&self) -> bool {
        *self == PWRSELR::IOVDD1
    }
}
#[doc = r" Value of the field"]
pub struct ACCURACYR {
    bits: bool,
}
impl ACCURACYR {
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
#[doc = "Possible values of the field `INPUTRANGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUTRANGER {
    #[doc = "Setting when the input can be from 0 to ACMPVDD."]
    FULL,
    #[doc = "Setting when the input will always be greater than ACMPVDD/2."]
    GTVDDDIV2,
    #[doc = "Setting when the input will always be less than ACMPVDD/2."]
    LTVDDDIV2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INPUTRANGER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPUTRANGER::FULL => 0,
            INPUTRANGER::GTVDDDIV2 => 1,
            INPUTRANGER::LTVDDDIV2 => 2,
            INPUTRANGER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPUTRANGER {
        match value {
            0 => INPUTRANGER::FULL,
            1 => INPUTRANGER::GTVDDDIV2,
            2 => INPUTRANGER::LTVDDDIV2,
            i => INPUTRANGER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline]
    pub fn is_full(&self) -> bool {
        *self == INPUTRANGER::FULL
    }
    #[doc = "Checks if the value of the field is `GTVDDDIV2`"]
    #[inline]
    pub fn is_gtvdddiv2(&self) -> bool {
        *self == INPUTRANGER::GTVDDDIV2
    }
    #[doc = "Checks if the value of the field is `LTVDDDIV2`"]
    #[inline]
    pub fn is_ltvdddiv2(&self) -> bool {
        *self == INPUTRANGER::LTVDDDIV2
    }
}
#[doc = r" Value of the field"]
pub struct IRISER {
    bits: bool,
}
impl IRISER {
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
pub struct IFALLR {
    bits: bool,
}
impl IFALLR {
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
pub struct BIASPROGR {
    bits: u8,
}
impl BIASPROGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FULLBIASR {
    bits: bool,
}
impl FULLBIASR {
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
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
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
pub struct _INACTVALW<'a> {
    w: &'a mut W,
}
impl<'a> _INACTVALW<'a> {
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
#[doc = r" Proxy"]
pub struct _GPIOINVW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOINVW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _APORTXMASTERDISW<'a> {
    w: &'a mut W,
}
impl<'a> _APORTXMASTERDISW<'a> {
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
#[doc = r" Proxy"]
pub struct _APORTYMASTERDISW<'a> {
    w: &'a mut W,
}
impl<'a> _APORTYMASTERDISW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _APORTVMASTERDISW<'a> {
    w: &'a mut W,
}
impl<'a> _APORTVMASTERDISW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWRSEL`"]
pub enum PWRSELW {
    #[doc = "AVDD supply"]
    AVDD,
    #[doc = "DVDD supply"]
    DVDD,
    #[doc = "IOVDD/IOVDD0 supply"]
    IOVDD0,
    #[doc = "IOVDD1 supply (if part has two I/O voltages)"]
    IOVDD1,
}
impl PWRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWRSELW::AVDD => 0,
            PWRSELW::DVDD => 1,
            PWRSELW::IOVDD0 => 2,
            PWRSELW::IOVDD1 => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "AVDD supply"]
    #[inline]
    pub fn avdd(self) -> &'a mut W {
        self.variant(PWRSELW::AVDD)
    }
    #[doc = "DVDD supply"]
    #[inline]
    pub fn dvdd(self) -> &'a mut W {
        self.variant(PWRSELW::DVDD)
    }
    #[doc = "IOVDD/IOVDD0 supply"]
    #[inline]
    pub fn iovdd0(self) -> &'a mut W {
        self.variant(PWRSELW::IOVDD0)
    }
    #[doc = "IOVDD1 supply (if part has two I/O voltages)"]
    #[inline]
    pub fn iovdd1(self) -> &'a mut W {
        self.variant(PWRSELW::IOVDD1)
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
pub struct _ACCURACYW<'a> {
    w: &'a mut W,
}
impl<'a> _ACCURACYW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INPUTRANGE`"]
pub enum INPUTRANGEW {
    #[doc = "Setting when the input can be from 0 to ACMPVDD."]
    FULL,
    #[doc = "Setting when the input will always be greater than ACMPVDD/2."]
    GTVDDDIV2,
    #[doc = "Setting when the input will always be less than ACMPVDD/2."]
    LTVDDDIV2,
}
impl INPUTRANGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPUTRANGEW::FULL => 0,
            INPUTRANGEW::GTVDDDIV2 => 1,
            INPUTRANGEW::LTVDDDIV2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUTRANGEW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUTRANGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUTRANGEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Setting when the input can be from 0 to ACMPVDD."]
    #[inline]
    pub fn full(self) -> &'a mut W {
        self.variant(INPUTRANGEW::FULL)
    }
    #[doc = "Setting when the input will always be greater than ACMPVDD/2."]
    #[inline]
    pub fn gtvdddiv2(self) -> &'a mut W {
        self.variant(INPUTRANGEW::GTVDDDIV2)
    }
    #[doc = "Setting when the input will always be less than ACMPVDD/2."]
    #[inline]
    pub fn ltvdddiv2(self) -> &'a mut W {
        self.variant(INPUTRANGEW::LTVDDDIV2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IRISEW<'a> {
    w: &'a mut W,
}
impl<'a> _IRISEW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IFALLW<'a> {
    w: &'a mut W,
}
impl<'a> _IFALLW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BIASPROGW<'a> {
    w: &'a mut W,
}
impl<'a> _BIASPROGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FULLBIASW<'a> {
    w: &'a mut W,
}
impl<'a> _FULLBIASW<'a> {
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
    #[doc = "Bit 0 - Analog Comparator Enable"]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENR { bits }
    }
    #[doc = "Bit 2 - Inactive Value"]
    #[inline]
    pub fn inactval(&self) -> INACTVALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INACTVALR { bits }
    }
    #[doc = "Bit 3 - Comparator GPIO Output Invert"]
    #[inline]
    pub fn gpioinv(&self) -> GPIOINVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPIOINVR { bits }
    }
    #[doc = "Bit 8 - APORT Bus X Master Disable"]
    #[inline]
    pub fn aportxmasterdis(&self) -> APORTXMASTERDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APORTXMASTERDISR { bits }
    }
    #[doc = "Bit 9 - APORT Bus Y Master Disable"]
    #[inline]
    pub fn aportymasterdis(&self) -> APORTYMASTERDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APORTYMASTERDISR { bits }
    }
    #[doc = "Bit 10 - APORT Bus Master Disable for Bus Selected By VASEL"]
    #[inline]
    pub fn aportvmasterdis(&self) -> APORTVMASTERDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APORTVMASTERDISR { bits }
    }
    #[doc = "Bits 12:14 - Power Select"]
    #[inline]
    pub fn pwrsel(&self) -> PWRSELR {
        PWRSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - ACMP Accuracy Mode"]
    #[inline]
    pub fn accuracy(&self) -> ACCURACYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACCURACYR { bits }
    }
    #[doc = "Bits 18:19 - Input Range"]
    #[inline]
    pub fn inputrange(&self) -> INPUTRANGER {
        INPUTRANGER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Rising Edge Interrupt Sense"]
    #[inline]
    pub fn irise(&self) -> IRISER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IRISER { bits }
    }
    #[doc = "Bit 21 - Falling Edge Interrupt Sense"]
    #[inline]
    pub fn ifall(&self) -> IFALLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IFALLR { bits }
    }
    #[doc = "Bits 24:29 - Bias Configuration"]
    #[inline]
    pub fn biasprog(&self) -> BIASPROGR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BIASPROGR { bits }
    }
    #[doc = "Bit 31 - Full Bias Current"]
    #[inline]
    pub fn fullbias(&self) -> FULLBIASR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FULLBIASR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 117440512 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Analog Comparator Enable"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bit 2 - Inactive Value"]
    #[inline]
    pub fn inactval(&mut self) -> _INACTVALW {
        _INACTVALW { w: self }
    }
    #[doc = "Bit 3 - Comparator GPIO Output Invert"]
    #[inline]
    pub fn gpioinv(&mut self) -> _GPIOINVW {
        _GPIOINVW { w: self }
    }
    #[doc = "Bit 8 - APORT Bus X Master Disable"]
    #[inline]
    pub fn aportxmasterdis(&mut self) -> _APORTXMASTERDISW {
        _APORTXMASTERDISW { w: self }
    }
    #[doc = "Bit 9 - APORT Bus Y Master Disable"]
    #[inline]
    pub fn aportymasterdis(&mut self) -> _APORTYMASTERDISW {
        _APORTYMASTERDISW { w: self }
    }
    #[doc = "Bit 10 - APORT Bus Master Disable for Bus Selected By VASEL"]
    #[inline]
    pub fn aportvmasterdis(&mut self) -> _APORTVMASTERDISW {
        _APORTVMASTERDISW { w: self }
    }
    #[doc = "Bits 12:14 - Power Select"]
    #[inline]
    pub fn pwrsel(&mut self) -> _PWRSELW {
        _PWRSELW { w: self }
    }
    #[doc = "Bit 15 - ACMP Accuracy Mode"]
    #[inline]
    pub fn accuracy(&mut self) -> _ACCURACYW {
        _ACCURACYW { w: self }
    }
    #[doc = "Bits 18:19 - Input Range"]
    #[inline]
    pub fn inputrange(&mut self) -> _INPUTRANGEW {
        _INPUTRANGEW { w: self }
    }
    #[doc = "Bit 20 - Rising Edge Interrupt Sense"]
    #[inline]
    pub fn irise(&mut self) -> _IRISEW {
        _IRISEW { w: self }
    }
    #[doc = "Bit 21 - Falling Edge Interrupt Sense"]
    #[inline]
    pub fn ifall(&mut self) -> _IFALLW {
        _IFALLW { w: self }
    }
    #[doc = "Bits 24:29 - Bias Configuration"]
    #[inline]
    pub fn biasprog(&mut self) -> _BIASPROGW {
        _BIASPROGW { w: self }
    }
    #[doc = "Bit 31 - Full Bias Current"]
    #[inline]
    pub fn fullbias(&mut self) -> _FULLBIASW {
        _FULLBIASW { w: self }
    }
}
