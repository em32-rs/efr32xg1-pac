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
pub struct AESR {
    bits: bool,
}
impl AESR {
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
pub struct KEYBUFDISR {
    bits: bool,
}
impl KEYBUFDISR {
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
pub struct SHAR {
    bits: bool,
}
impl SHAR {
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
pub struct NOBUSYSTALLR {
    bits: bool,
}
impl NOBUSYSTALLR {
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
#[doc = "Possible values of the field `INCWIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INCWIDTHR {
    #[doc = "Byte 15 in DATA1 is used for the increment function."]
    INCWIDTH1,
    #[doc = "Bytes 14 and 15 in DATA1 are used for the increment function."]
    INCWIDTH2,
    #[doc = "Bytes 13 to 15 in DATA1 are used for the increment function."]
    INCWIDTH3,
    #[doc = "Bytes 12 to 15 in DATA1 are used for the increment function."]
    INCWIDTH4,
}
impl INCWIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INCWIDTHR::INCWIDTH1 => 0,
            INCWIDTHR::INCWIDTH2 => 1,
            INCWIDTHR::INCWIDTH3 => 2,
            INCWIDTHR::INCWIDTH4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INCWIDTHR {
        match value {
            0 => INCWIDTHR::INCWIDTH1,
            1 => INCWIDTHR::INCWIDTH2,
            2 => INCWIDTHR::INCWIDTH3,
            3 => INCWIDTHR::INCWIDTH4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INCWIDTH1`"]
    #[inline]
    pub fn is_incwidth1(&self) -> bool {
        *self == INCWIDTHR::INCWIDTH1
    }
    #[doc = "Checks if the value of the field is `INCWIDTH2`"]
    #[inline]
    pub fn is_incwidth2(&self) -> bool {
        *self == INCWIDTHR::INCWIDTH2
    }
    #[doc = "Checks if the value of the field is `INCWIDTH3`"]
    #[inline]
    pub fn is_incwidth3(&self) -> bool {
        *self == INCWIDTHR::INCWIDTH3
    }
    #[doc = "Checks if the value of the field is `INCWIDTH4`"]
    #[inline]
    pub fn is_incwidth4(&self) -> bool {
        *self == INCWIDTHR::INCWIDTH4
    }
}
#[doc = "Possible values of the field `DMA0MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA0MODER {
    #[doc = "Target register is fully read/written during every DMA transaction"]
    FULL,
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Zero padding is automatically added when writing."]
    LENLIMIT,
    #[doc = "Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    FULLBYTE,
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    LENLIMITBYTE,
}
impl DMA0MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMA0MODER::FULL => 0,
            DMA0MODER::LENLIMIT => 1,
            DMA0MODER::FULLBYTE => 2,
            DMA0MODER::LENLIMITBYTE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMA0MODER {
        match value {
            0 => DMA0MODER::FULL,
            1 => DMA0MODER::LENLIMIT,
            2 => DMA0MODER::FULLBYTE,
            3 => DMA0MODER::LENLIMITBYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline]
    pub fn is_full(&self) -> bool {
        *self == DMA0MODER::FULL
    }
    #[doc = "Checks if the value of the field is `LENLIMIT`"]
    #[inline]
    pub fn is_lenlimit(&self) -> bool {
        *self == DMA0MODER::LENLIMIT
    }
    #[doc = "Checks if the value of the field is `FULLBYTE`"]
    #[inline]
    pub fn is_fullbyte(&self) -> bool {
        *self == DMA0MODER::FULLBYTE
    }
    #[doc = "Checks if the value of the field is `LENLIMITBYTE`"]
    #[inline]
    pub fn is_lenlimitbyte(&self) -> bool {
        *self == DMA0MODER::LENLIMITBYTE
    }
}
#[doc = "Possible values of the field `DMA0RSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA0RSELR {
    #[doc = "undocumented"]
    DATA0,
    #[doc = "undocumented"]
    DDATA0,
    #[doc = "undocumented"]
    DDATA0BIG,
    #[doc = "undocumented"]
    QDATA0,
}
impl DMA0RSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMA0RSELR::DATA0 => 0,
            DMA0RSELR::DDATA0 => 1,
            DMA0RSELR::DDATA0BIG => 2,
            DMA0RSELR::QDATA0 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMA0RSELR {
        match value {
            0 => DMA0RSELR::DATA0,
            1 => DMA0RSELR::DDATA0,
            2 => DMA0RSELR::DDATA0BIG,
            3 => DMA0RSELR::QDATA0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline]
    pub fn is_data0(&self) -> bool {
        *self == DMA0RSELR::DATA0
    }
    #[doc = "Checks if the value of the field is `DDATA0`"]
    #[inline]
    pub fn is_ddata0(&self) -> bool {
        *self == DMA0RSELR::DDATA0
    }
    #[doc = "Checks if the value of the field is `DDATA0BIG`"]
    #[inline]
    pub fn is_ddata0big(&self) -> bool {
        *self == DMA0RSELR::DDATA0BIG
    }
    #[doc = "Checks if the value of the field is `QDATA0`"]
    #[inline]
    pub fn is_qdata0(&self) -> bool {
        *self == DMA0RSELR::QDATA0
    }
}
#[doc = "Possible values of the field `DMA1MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1MODER {
    #[doc = "Target register is fully read/written during every DMA transaction"]
    FULL,
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Zero padding is automatically added when writing."]
    LENLIMIT,
    #[doc = "Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    FULLBYTE,
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    LENLIMITBYTE,
}
impl DMA1MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMA1MODER::FULL => 0,
            DMA1MODER::LENLIMIT => 1,
            DMA1MODER::FULLBYTE => 2,
            DMA1MODER::LENLIMITBYTE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMA1MODER {
        match value {
            0 => DMA1MODER::FULL,
            1 => DMA1MODER::LENLIMIT,
            2 => DMA1MODER::FULLBYTE,
            3 => DMA1MODER::LENLIMITBYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline]
    pub fn is_full(&self) -> bool {
        *self == DMA1MODER::FULL
    }
    #[doc = "Checks if the value of the field is `LENLIMIT`"]
    #[inline]
    pub fn is_lenlimit(&self) -> bool {
        *self == DMA1MODER::LENLIMIT
    }
    #[doc = "Checks if the value of the field is `FULLBYTE`"]
    #[inline]
    pub fn is_fullbyte(&self) -> bool {
        *self == DMA1MODER::FULLBYTE
    }
    #[doc = "Checks if the value of the field is `LENLIMITBYTE`"]
    #[inline]
    pub fn is_lenlimitbyte(&self) -> bool {
        *self == DMA1MODER::LENLIMITBYTE
    }
}
#[doc = "Possible values of the field `DMA1RSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1RSELR {
    #[doc = "undocumented"]
    DATA1,
    #[doc = "undocumented"]
    DDATA1,
    #[doc = "undocumented"]
    QDATA1,
    #[doc = "undocumented"]
    QDATA1BIG,
}
impl DMA1RSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMA1RSELR::DATA1 => 0,
            DMA1RSELR::DDATA1 => 1,
            DMA1RSELR::QDATA1 => 2,
            DMA1RSELR::QDATA1BIG => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMA1RSELR {
        match value {
            0 => DMA1RSELR::DATA1,
            1 => DMA1RSELR::DDATA1,
            2 => DMA1RSELR::QDATA1,
            3 => DMA1RSELR::QDATA1BIG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline]
    pub fn is_data1(&self) -> bool {
        *self == DMA1RSELR::DATA1
    }
    #[doc = "Checks if the value of the field is `DDATA1`"]
    #[inline]
    pub fn is_ddata1(&self) -> bool {
        *self == DMA1RSELR::DDATA1
    }
    #[doc = "Checks if the value of the field is `QDATA1`"]
    #[inline]
    pub fn is_qdata1(&self) -> bool {
        *self == DMA1RSELR::QDATA1
    }
    #[doc = "Checks if the value of the field is `QDATA1BIG`"]
    #[inline]
    pub fn is_qdata1big(&self) -> bool {
        *self == DMA1RSELR::QDATA1BIG
    }
}
#[doc = r" Value of the field"]
pub struct COMBDMA0WEREQR {
    bits: bool,
}
impl COMBDMA0WEREQR {
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
pub struct _AESW<'a> {
    w: &'a mut W,
}
impl<'a> _AESW<'a> {
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
pub struct _KEYBUFDISW<'a> {
    w: &'a mut W,
}
impl<'a> _KEYBUFDISW<'a> {
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
pub struct _SHAW<'a> {
    w: &'a mut W,
}
impl<'a> _SHAW<'a> {
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
pub struct _NOBUSYSTALLW<'a> {
    w: &'a mut W,
}
impl<'a> _NOBUSYSTALLW<'a> {
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
#[doc = "Values that can be written to the field `INCWIDTH`"]
pub enum INCWIDTHW {
    #[doc = "Byte 15 in DATA1 is used for the increment function."]
    INCWIDTH1,
    #[doc = "Bytes 14 and 15 in DATA1 are used for the increment function."]
    INCWIDTH2,
    #[doc = "Bytes 13 to 15 in DATA1 are used for the increment function."]
    INCWIDTH3,
    #[doc = "Bytes 12 to 15 in DATA1 are used for the increment function."]
    INCWIDTH4,
}
impl INCWIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INCWIDTHW::INCWIDTH1 => 0,
            INCWIDTHW::INCWIDTH2 => 1,
            INCWIDTHW::INCWIDTH3 => 2,
            INCWIDTHW::INCWIDTH4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INCWIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _INCWIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INCWIDTHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Byte 15 in DATA1 is used for the increment function."]
    #[inline]
    pub fn incwidth1(self) -> &'a mut W {
        self.variant(INCWIDTHW::INCWIDTH1)
    }
    #[doc = "Bytes 14 and 15 in DATA1 are used for the increment function."]
    #[inline]
    pub fn incwidth2(self) -> &'a mut W {
        self.variant(INCWIDTHW::INCWIDTH2)
    }
    #[doc = "Bytes 13 to 15 in DATA1 are used for the increment function."]
    #[inline]
    pub fn incwidth3(self) -> &'a mut W {
        self.variant(INCWIDTHW::INCWIDTH3)
    }
    #[doc = "Bytes 12 to 15 in DATA1 are used for the increment function."]
    #[inline]
    pub fn incwidth4(self) -> &'a mut W {
        self.variant(INCWIDTHW::INCWIDTH4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA0MODE`"]
pub enum DMA0MODEW {
    #[doc = "Target register is fully read/written during every DMA transaction"]
    FULL,
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Zero padding is automatically added when writing."]
    LENLIMIT,
    #[doc = "Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    FULLBYTE,
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    LENLIMITBYTE,
}
impl DMA0MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMA0MODEW::FULL => 0,
            DMA0MODEW::LENLIMIT => 1,
            DMA0MODEW::FULLBYTE => 2,
            DMA0MODEW::LENLIMITBYTE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA0MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA0MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA0MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Target register is fully read/written during every DMA transaction"]
    #[inline]
    pub fn full(self) -> &'a mut W {
        self.variant(DMA0MODEW::FULL)
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Zero padding is automatically added when writing."]
    #[inline]
    pub fn lenlimit(self) -> &'a mut W {
        self.variant(DMA0MODEW::LENLIMIT)
    }
    #[doc = "Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    #[inline]
    pub fn fullbyte(self) -> &'a mut W {
        self.variant(DMA0MODEW::FULLBYTE)
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    #[inline]
    pub fn lenlimitbyte(self) -> &'a mut W {
        self.variant(DMA0MODEW::LENLIMITBYTE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA0RSEL`"]
pub enum DMA0RSELW {
    #[doc = "`0`"]
    DATA0,
    #[doc = "`1`"]
    DDATA0,
    #[doc = "`10`"]
    DDATA0BIG,
    #[doc = "`11`"]
    QDATA0,
}
impl DMA0RSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMA0RSELW::DATA0 => 0,
            DMA0RSELW::DDATA0 => 1,
            DMA0RSELW::DDATA0BIG => 2,
            DMA0RSELW::QDATA0 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA0RSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA0RSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA0RSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn data0(self) -> &'a mut W {
        self.variant(DMA0RSELW::DATA0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn ddata0(self) -> &'a mut W {
        self.variant(DMA0RSELW::DDATA0)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn ddata0big(self) -> &'a mut W {
        self.variant(DMA0RSELW::DDATA0BIG)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn qdata0(self) -> &'a mut W {
        self.variant(DMA0RSELW::QDATA0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA1MODE`"]
pub enum DMA1MODEW {
    #[doc = "Target register is fully read/written during every DMA transaction"]
    FULL,
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Zero padding is automatically added when writing."]
    LENLIMIT,
    #[doc = "Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    FULLBYTE,
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    LENLIMITBYTE,
}
impl DMA1MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMA1MODEW::FULL => 0,
            DMA1MODEW::LENLIMIT => 1,
            DMA1MODEW::FULLBYTE => 2,
            DMA1MODEW::LENLIMITBYTE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA1MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA1MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA1MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Target register is fully read/written during every DMA transaction"]
    #[inline]
    pub fn full(self) -> &'a mut W {
        self.variant(DMA1MODEW::FULL)
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Zero padding is automatically added when writing."]
    #[inline]
    pub fn lenlimit(self) -> &'a mut W {
        self.variant(DMA1MODEW::LENLIMIT)
    }
    #[doc = "Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    #[inline]
    pub fn fullbyte(self) -> &'a mut W {
        self.variant(DMA1MODEW::FULLBYTE)
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    #[inline]
    pub fn lenlimitbyte(self) -> &'a mut W {
        self.variant(DMA1MODEW::LENLIMITBYTE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA1RSEL`"]
pub enum DMA1RSELW {
    #[doc = "`0`"]
    DATA1,
    #[doc = "`1`"]
    DDATA1,
    #[doc = "`10`"]
    QDATA1,
    #[doc = "`11`"]
    QDATA1BIG,
}
impl DMA1RSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMA1RSELW::DATA1 => 0,
            DMA1RSELW::DDATA1 => 1,
            DMA1RSELW::QDATA1 => 2,
            DMA1RSELW::QDATA1BIG => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA1RSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA1RSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA1RSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn data1(self) -> &'a mut W {
        self.variant(DMA1RSELW::DATA1)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn ddata1(self) -> &'a mut W {
        self.variant(DMA1RSELW::DDATA1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn qdata1(self) -> &'a mut W {
        self.variant(DMA1RSELW::QDATA1)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn qdata1big(self) -> &'a mut W {
        self.variant(DMA1RSELW::QDATA1BIG)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COMBDMA0WEREQW<'a> {
    w: &'a mut W,
}
impl<'a> _COMBDMA0WEREQW<'a> {
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
    #[doc = "Bit 0 - AES Mode"]
    #[inline]
    pub fn aes(&self) -> AESR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AESR { bits }
    }
    #[doc = "Bit 1 - Key Buffer Disable"]
    #[inline]
    pub fn keybufdis(&self) -> KEYBUFDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        KEYBUFDISR { bits }
    }
    #[doc = "Bit 2 - SHA Mode"]
    #[inline]
    pub fn sha(&self) -> SHAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SHAR { bits }
    }
    #[doc = "Bit 10 - No Stalling of Bus When Busy"]
    #[inline]
    pub fn nobusystall(&self) -> NOBUSYSTALLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NOBUSYSTALLR { bits }
    }
    #[doc = "Bits 14:15 - Increment Width"]
    #[inline]
    pub fn incwidth(&self) -> INCWIDTHR {
        INCWIDTHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - DMA0 Read Mode"]
    #[inline]
    pub fn dma0mode(&self) -> DMA0MODER {
        DMA0MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - DMA0 Read Register Select"]
    #[inline]
    pub fn dma0rsel(&self) -> DMA0RSELR {
        DMA0RSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - DMA1 Read Mode"]
    #[inline]
    pub fn dma1mode(&self) -> DMA1MODER {
        DMA1MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - DATA0 DMA Unaligned Read Register Select"]
    #[inline]
    pub fn dma1rsel(&self) -> DMA1RSELR {
        DMA1RSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 31 - Combined Data0 Write DMA Request"]
    #[inline]
    pub fn combdma0wereq(&self) -> COMBDMA0WEREQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COMBDMA0WEREQR { bits }
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
    #[doc = "Bit 0 - AES Mode"]
    #[inline]
    pub fn aes(&mut self) -> _AESW {
        _AESW { w: self }
    }
    #[doc = "Bit 1 - Key Buffer Disable"]
    #[inline]
    pub fn keybufdis(&mut self) -> _KEYBUFDISW {
        _KEYBUFDISW { w: self }
    }
    #[doc = "Bit 2 - SHA Mode"]
    #[inline]
    pub fn sha(&mut self) -> _SHAW {
        _SHAW { w: self }
    }
    #[doc = "Bit 10 - No Stalling of Bus When Busy"]
    #[inline]
    pub fn nobusystall(&mut self) -> _NOBUSYSTALLW {
        _NOBUSYSTALLW { w: self }
    }
    #[doc = "Bits 14:15 - Increment Width"]
    #[inline]
    pub fn incwidth(&mut self) -> _INCWIDTHW {
        _INCWIDTHW { w: self }
    }
    #[doc = "Bits 16:17 - DMA0 Read Mode"]
    #[inline]
    pub fn dma0mode(&mut self) -> _DMA0MODEW {
        _DMA0MODEW { w: self }
    }
    #[doc = "Bits 20:21 - DMA0 Read Register Select"]
    #[inline]
    pub fn dma0rsel(&mut self) -> _DMA0RSELW {
        _DMA0RSELW { w: self }
    }
    #[doc = "Bits 24:25 - DMA1 Read Mode"]
    #[inline]
    pub fn dma1mode(&mut self) -> _DMA1MODEW {
        _DMA1MODEW { w: self }
    }
    #[doc = "Bits 28:29 - DATA0 DMA Unaligned Read Register Select"]
    #[inline]
    pub fn dma1rsel(&mut self) -> _DMA1RSELW {
        _DMA1RSELW { w: self }
    }
    #[doc = "Bit 31 - Combined Data0 Write DMA Request"]
    #[inline]
    pub fn combdma0wereq(&mut self) -> _COMBDMA0WEREQW {
        _COMBDMA0WEREQW { w: self }
    }
}
