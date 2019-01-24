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
#[doc = "Possible values of the field `WARMUPMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WARMUPMODER {
    #[doc = "ADC is shut down after each conversion. 5us warmup time is used before each conversion."]
    NORMAL,
    #[doc = "ADC is kept in standby mode between conversions. 1us warmup time is used before each conversion."]
    KEEPINSTANDBY,
    #[doc = "ADC is kept in slow acquisition mode between conversions. 1us warmup time is used before each conversion."]
    KEEPINSLOWACC,
    #[doc = "ADC is kept on after conversions, allowing for continuous conversion."]
    KEEPADCWARM,
}
impl WARMUPMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WARMUPMODER::NORMAL => 0,
            WARMUPMODER::KEEPINSTANDBY => 1,
            WARMUPMODER::KEEPINSLOWACC => 2,
            WARMUPMODER::KEEPADCWARM => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WARMUPMODER {
        match value {
            0 => WARMUPMODER::NORMAL,
            1 => WARMUPMODER::KEEPINSTANDBY,
            2 => WARMUPMODER::KEEPINSLOWACC,
            3 => WARMUPMODER::KEEPADCWARM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == WARMUPMODER::NORMAL
    }
    #[doc = "Checks if the value of the field is `KEEPINSTANDBY`"]
    #[inline]
    pub fn is_keepinstandby(&self) -> bool {
        *self == WARMUPMODER::KEEPINSTANDBY
    }
    #[doc = "Checks if the value of the field is `KEEPINSLOWACC`"]
    #[inline]
    pub fn is_keepinslowacc(&self) -> bool {
        *self == WARMUPMODER::KEEPINSLOWACC
    }
    #[doc = "Checks if the value of the field is `KEEPADCWARM`"]
    #[inline]
    pub fn is_keepadcwarm(&self) -> bool {
        *self == WARMUPMODER::KEEPADCWARM
    }
}
#[doc = r" Value of the field"]
pub struct SINGLEDMAWUR {
    bits: bool,
}
impl SINGLEDMAWUR {
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
pub struct SCANDMAWUR {
    bits: bool,
}
impl SCANDMAWUR {
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
pub struct TAILGATER {
    bits: bool,
}
impl TAILGATER {
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
pub struct ASYNCCLKENR {
    bits: bool,
}
impl ASYNCCLKENR {
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
pub struct ADCCLKMODER {
    bits: bool,
}
impl ADCCLKMODER {
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
#[doc = "Possible values of the field `PRESC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCR {
    #[doc = "undocumented"]
    NODIVISION,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRESCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCR::NODIVISION => 0,
            PRESCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCR {
        match value {
            0 => PRESCR::NODIVISION,
            i => PRESCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NODIVISION`"]
    #[inline]
    pub fn is_nodivision(&self) -> bool {
        *self == PRESCR::NODIVISION
    }
}
#[doc = r" Value of the field"]
pub struct TIMEBASER {
    bits: u8,
}
impl TIMEBASER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `OVSRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVSRSELR {
    #[doc = "2 samples for each conversion result"]
    X2,
    #[doc = "4 samples for each conversion result"]
    X4,
    #[doc = "8 samples for each conversion result"]
    X8,
    #[doc = "16 samples for each conversion result"]
    X16,
    #[doc = "32 samples for each conversion result"]
    X32,
    #[doc = "64 samples for each conversion result"]
    X64,
    #[doc = "128 samples for each conversion result"]
    X128,
    #[doc = "256 samples for each conversion result"]
    X256,
    #[doc = "512 samples for each conversion result"]
    X512,
    #[doc = "1024 samples for each conversion result"]
    X1024,
    #[doc = "2048 samples for each conversion result"]
    X2048,
    #[doc = "4096 samples for each conversion result"]
    X4096,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OVSRSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OVSRSELR::X2 => 0,
            OVSRSELR::X4 => 1,
            OVSRSELR::X8 => 2,
            OVSRSELR::X16 => 3,
            OVSRSELR::X32 => 4,
            OVSRSELR::X64 => 5,
            OVSRSELR::X128 => 6,
            OVSRSELR::X256 => 7,
            OVSRSELR::X512 => 8,
            OVSRSELR::X1024 => 9,
            OVSRSELR::X2048 => 10,
            OVSRSELR::X4096 => 11,
            OVSRSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OVSRSELR {
        match value {
            0 => OVSRSELR::X2,
            1 => OVSRSELR::X4,
            2 => OVSRSELR::X8,
            3 => OVSRSELR::X16,
            4 => OVSRSELR::X32,
            5 => OVSRSELR::X64,
            6 => OVSRSELR::X128,
            7 => OVSRSELR::X256,
            8 => OVSRSELR::X512,
            9 => OVSRSELR::X1024,
            10 => OVSRSELR::X2048,
            11 => OVSRSELR::X4096,
            i => OVSRSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `X2`"]
    #[inline]
    pub fn is_x2(&self) -> bool {
        *self == OVSRSELR::X2
    }
    #[doc = "Checks if the value of the field is `X4`"]
    #[inline]
    pub fn is_x4(&self) -> bool {
        *self == OVSRSELR::X4
    }
    #[doc = "Checks if the value of the field is `X8`"]
    #[inline]
    pub fn is_x8(&self) -> bool {
        *self == OVSRSELR::X8
    }
    #[doc = "Checks if the value of the field is `X16`"]
    #[inline]
    pub fn is_x16(&self) -> bool {
        *self == OVSRSELR::X16
    }
    #[doc = "Checks if the value of the field is `X32`"]
    #[inline]
    pub fn is_x32(&self) -> bool {
        *self == OVSRSELR::X32
    }
    #[doc = "Checks if the value of the field is `X64`"]
    #[inline]
    pub fn is_x64(&self) -> bool {
        *self == OVSRSELR::X64
    }
    #[doc = "Checks if the value of the field is `X128`"]
    #[inline]
    pub fn is_x128(&self) -> bool {
        *self == OVSRSELR::X128
    }
    #[doc = "Checks if the value of the field is `X256`"]
    #[inline]
    pub fn is_x256(&self) -> bool {
        *self == OVSRSELR::X256
    }
    #[doc = "Checks if the value of the field is `X512`"]
    #[inline]
    pub fn is_x512(&self) -> bool {
        *self == OVSRSELR::X512
    }
    #[doc = "Checks if the value of the field is `X1024`"]
    #[inline]
    pub fn is_x1024(&self) -> bool {
        *self == OVSRSELR::X1024
    }
    #[doc = "Checks if the value of the field is `X2048`"]
    #[inline]
    pub fn is_x2048(&self) -> bool {
        *self == OVSRSELR::X2048
    }
    #[doc = "Checks if the value of the field is `X4096`"]
    #[inline]
    pub fn is_x4096(&self) -> bool {
        *self == OVSRSELR::X4096
    }
}
#[doc = r" Value of the field"]
pub struct CHCONMODER {
    bits: bool,
}
impl CHCONMODER {
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
#[doc = "Values that can be written to the field `WARMUPMODE`"]
pub enum WARMUPMODEW {
    #[doc = "ADC is shut down after each conversion. 5us warmup time is used before each conversion."]
    NORMAL,
    #[doc = "ADC is kept in standby mode between conversions. 1us warmup time is used before each conversion."]
    KEEPINSTANDBY,
    #[doc = "ADC is kept in slow acquisition mode between conversions. 1us warmup time is used before each conversion."]
    KEEPINSLOWACC,
    #[doc = "ADC is kept on after conversions, allowing for continuous conversion."]
    KEEPADCWARM,
}
impl WARMUPMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WARMUPMODEW::NORMAL => 0,
            WARMUPMODEW::KEEPINSTANDBY => 1,
            WARMUPMODEW::KEEPINSLOWACC => 2,
            WARMUPMODEW::KEEPADCWARM => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WARMUPMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _WARMUPMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WARMUPMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ADC is shut down after each conversion. 5us warmup time is used before each conversion."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(WARMUPMODEW::NORMAL)
    }
    #[doc = "ADC is kept in standby mode between conversions. 1us warmup time is used before each conversion."]
    #[inline]
    pub fn keepinstandby(self) -> &'a mut W {
        self.variant(WARMUPMODEW::KEEPINSTANDBY)
    }
    #[doc = "ADC is kept in slow acquisition mode between conversions. 1us warmup time is used before each conversion."]
    #[inline]
    pub fn keepinslowacc(self) -> &'a mut W {
        self.variant(WARMUPMODEW::KEEPINSLOWACC)
    }
    #[doc = "ADC is kept on after conversions, allowing for continuous conversion."]
    #[inline]
    pub fn keepadcwarm(self) -> &'a mut W {
        self.variant(WARMUPMODEW::KEEPADCWARM)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SINGLEDMAWUW<'a> {
    w: &'a mut W,
}
impl<'a> _SINGLEDMAWUW<'a> {
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
pub struct _SCANDMAWUW<'a> {
    w: &'a mut W,
}
impl<'a> _SCANDMAWUW<'a> {
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
pub struct _TAILGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _TAILGATEW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ASYNCCLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _ASYNCCLKENW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADCCLKMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCCLKMODEW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRESC`"]
pub enum PRESCW {
    #[doc = "`0`"]
    NODIVISION,
}
impl PRESCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCW::NODIVISION => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(PRESCW::NODIVISION)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TIMEBASEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMEBASEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OVSRSEL`"]
pub enum OVSRSELW {
    #[doc = "2 samples for each conversion result"]
    X2,
    #[doc = "4 samples for each conversion result"]
    X4,
    #[doc = "8 samples for each conversion result"]
    X8,
    #[doc = "16 samples for each conversion result"]
    X16,
    #[doc = "32 samples for each conversion result"]
    X32,
    #[doc = "64 samples for each conversion result"]
    X64,
    #[doc = "128 samples for each conversion result"]
    X128,
    #[doc = "256 samples for each conversion result"]
    X256,
    #[doc = "512 samples for each conversion result"]
    X512,
    #[doc = "1024 samples for each conversion result"]
    X1024,
    #[doc = "2048 samples for each conversion result"]
    X2048,
    #[doc = "4096 samples for each conversion result"]
    X4096,
}
impl OVSRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OVSRSELW::X2 => 0,
            OVSRSELW::X4 => 1,
            OVSRSELW::X8 => 2,
            OVSRSELW::X16 => 3,
            OVSRSELW::X32 => 4,
            OVSRSELW::X64 => 5,
            OVSRSELW::X128 => 6,
            OVSRSELW::X256 => 7,
            OVSRSELW::X512 => 8,
            OVSRSELW::X1024 => 9,
            OVSRSELW::X2048 => 10,
            OVSRSELW::X4096 => 11,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVSRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _OVSRSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVSRSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "2 samples for each conversion result"]
    #[inline]
    pub fn x2(self) -> &'a mut W {
        self.variant(OVSRSELW::X2)
    }
    #[doc = "4 samples for each conversion result"]
    #[inline]
    pub fn x4(self) -> &'a mut W {
        self.variant(OVSRSELW::X4)
    }
    #[doc = "8 samples for each conversion result"]
    #[inline]
    pub fn x8(self) -> &'a mut W {
        self.variant(OVSRSELW::X8)
    }
    #[doc = "16 samples for each conversion result"]
    #[inline]
    pub fn x16(self) -> &'a mut W {
        self.variant(OVSRSELW::X16)
    }
    #[doc = "32 samples for each conversion result"]
    #[inline]
    pub fn x32(self) -> &'a mut W {
        self.variant(OVSRSELW::X32)
    }
    #[doc = "64 samples for each conversion result"]
    #[inline]
    pub fn x64(self) -> &'a mut W {
        self.variant(OVSRSELW::X64)
    }
    #[doc = "128 samples for each conversion result"]
    #[inline]
    pub fn x128(self) -> &'a mut W {
        self.variant(OVSRSELW::X128)
    }
    #[doc = "256 samples for each conversion result"]
    #[inline]
    pub fn x256(self) -> &'a mut W {
        self.variant(OVSRSELW::X256)
    }
    #[doc = "512 samples for each conversion result"]
    #[inline]
    pub fn x512(self) -> &'a mut W {
        self.variant(OVSRSELW::X512)
    }
    #[doc = "1024 samples for each conversion result"]
    #[inline]
    pub fn x1024(self) -> &'a mut W {
        self.variant(OVSRSELW::X1024)
    }
    #[doc = "2048 samples for each conversion result"]
    #[inline]
    pub fn x2048(self) -> &'a mut W {
        self.variant(OVSRSELW::X2048)
    }
    #[doc = "4096 samples for each conversion result"]
    #[inline]
    pub fn x4096(self) -> &'a mut W {
        self.variant(OVSRSELW::X4096)
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
pub struct _CHCONMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CHCONMODEW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Warm-up Mode"]
    #[inline]
    pub fn warmupmode(&self) -> WARMUPMODER {
        WARMUPMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - SINGLEFIFO DMA Wakeup"]
    #[inline]
    pub fn singledmawu(&self) -> SINGLEDMAWUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SINGLEDMAWUR { bits }
    }
    #[doc = "Bit 3 - SCANFIFO DMA Wakeup"]
    #[inline]
    pub fn scandmawu(&self) -> SCANDMAWUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCANDMAWUR { bits }
    }
    #[doc = "Bit 4 - Conversion Tailgating"]
    #[inline]
    pub fn tailgate(&self) -> TAILGATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TAILGATER { bits }
    }
    #[doc = "Bit 6 - Selects ASYNC CLK Enable Mode When ADCCLKMODE=1"]
    #[inline]
    pub fn asyncclken(&self) -> ASYNCCLKENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ASYNCCLKENR { bits }
    }
    #[doc = "Bit 7 - ADC Clock Mode"]
    #[inline]
    pub fn adcclkmode(&self) -> ADCCLKMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADCCLKMODER { bits }
    }
    #[doc = "Bits 8:14 - Prescalar Setting for ADC Sample and Conversion Clock"]
    #[inline]
    pub fn presc(&self) -> PRESCR {
        PRESCR::_from({
            const MASK: u8 = 127;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:22 - 1us Time Base"]
    #[inline]
    pub fn timebase(&self) -> TIMEBASER {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TIMEBASER { bits }
    }
    #[doc = "Bits 24:27 - Oversample Rate Select"]
    #[inline]
    pub fn ovsrsel(&self) -> OVSRSELR {
        OVSRSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 29 - Channel Connect"]
    #[inline]
    pub fn chconmode(&self) -> CHCONMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHCONMODER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2031616 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Warm-up Mode"]
    #[inline]
    pub fn warmupmode(&mut self) -> _WARMUPMODEW {
        _WARMUPMODEW { w: self }
    }
    #[doc = "Bit 2 - SINGLEFIFO DMA Wakeup"]
    #[inline]
    pub fn singledmawu(&mut self) -> _SINGLEDMAWUW {
        _SINGLEDMAWUW { w: self }
    }
    #[doc = "Bit 3 - SCANFIFO DMA Wakeup"]
    #[inline]
    pub fn scandmawu(&mut self) -> _SCANDMAWUW {
        _SCANDMAWUW { w: self }
    }
    #[doc = "Bit 4 - Conversion Tailgating"]
    #[inline]
    pub fn tailgate(&mut self) -> _TAILGATEW {
        _TAILGATEW { w: self }
    }
    #[doc = "Bit 6 - Selects ASYNC CLK Enable Mode When ADCCLKMODE=1"]
    #[inline]
    pub fn asyncclken(&mut self) -> _ASYNCCLKENW {
        _ASYNCCLKENW { w: self }
    }
    #[doc = "Bit 7 - ADC Clock Mode"]
    #[inline]
    pub fn adcclkmode(&mut self) -> _ADCCLKMODEW {
        _ADCCLKMODEW { w: self }
    }
    #[doc = "Bits 8:14 - Prescalar Setting for ADC Sample and Conversion Clock"]
    #[inline]
    pub fn presc(&mut self) -> _PRESCW {
        _PRESCW { w: self }
    }
    #[doc = "Bits 16:22 - 1us Time Base"]
    #[inline]
    pub fn timebase(&mut self) -> _TIMEBASEW {
        _TIMEBASEW { w: self }
    }
    #[doc = "Bits 24:27 - Oversample Rate Select"]
    #[inline]
    pub fn ovsrsel(&mut self) -> _OVSRSELW {
        _OVSRSELW { w: self }
    }
    #[doc = "Bit 29 - Channel Connect"]
    #[inline]
    pub fn chconmode(&mut self) -> _CHCONMODEW {
        _CHCONMODEW { w: self }
    }
}
