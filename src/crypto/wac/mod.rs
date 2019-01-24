#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WAC {
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
#[doc = "Possible values of the field `MODULUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODULUSR {
    #[doc = "Generic modulus. p = 2^256"]
    BIN256,
    #[doc = "Generic modulus. p = 2^128"]
    BIN128,
    #[doc = "Modulus for B-233 and K-233 ECC curves. p(t) = t^233 + t^74 + 1"]
    ECCBIN233P,
    #[doc = "Modulus for B-163 and K-163 ECC curves. p(t) = t^163 + t^7 + t^6 + t^3 + 1"]
    ECCBIN163P,
    #[doc = "Modulus for GCM. P(t) = t^128 + t^7 + t^2 + t + 1"]
    GCMBIN128,
    #[doc = "Modulus for P-256 ECC curve. p = 2^256 - 2^224 + 2^192 + 2^96 - 1"]
    ECCPRIME256P,
    #[doc = "Modulus for P-224 ECC curve. p = 2^224 - 2^96 - 1"]
    ECCPRIME224P,
    #[doc = "Modulus for P-192 ECC curve. p = 2^192 - 2^64 - 1"]
    ECCPRIME192P,
    #[doc = "P modulus for B-233 ECC curve"]
    ECCBIN233N,
    #[doc = "P modulus for K-233 ECC curve"]
    ECCBIN233KN,
    #[doc = "P modulus for B-163 ECC curve"]
    ECCBIN163N,
    #[doc = "P modulus for K-163 ECC curve"]
    ECCBIN163KN,
    #[doc = "P modulus for P-256 ECC curve"]
    ECCPRIME256N,
    #[doc = "P modulus for P-224 ECC curve"]
    ECCPRIME224N,
    #[doc = "P modulus for P-192 ECC curve"]
    ECCPRIME192N,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODULUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODULUSR::BIN256 => 0,
            MODULUSR::BIN128 => 1,
            MODULUSR::ECCBIN233P => 2,
            MODULUSR::ECCBIN163P => 3,
            MODULUSR::GCMBIN128 => 4,
            MODULUSR::ECCPRIME256P => 5,
            MODULUSR::ECCPRIME224P => 6,
            MODULUSR::ECCPRIME192P => 7,
            MODULUSR::ECCBIN233N => 8,
            MODULUSR::ECCBIN233KN => 9,
            MODULUSR::ECCBIN163N => 10,
            MODULUSR::ECCBIN163KN => 11,
            MODULUSR::ECCPRIME256N => 12,
            MODULUSR::ECCPRIME224N => 13,
            MODULUSR::ECCPRIME192N => 14,
            MODULUSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODULUSR {
        match value {
            0 => MODULUSR::BIN256,
            1 => MODULUSR::BIN128,
            2 => MODULUSR::ECCBIN233P,
            3 => MODULUSR::ECCBIN163P,
            4 => MODULUSR::GCMBIN128,
            5 => MODULUSR::ECCPRIME256P,
            6 => MODULUSR::ECCPRIME224P,
            7 => MODULUSR::ECCPRIME192P,
            8 => MODULUSR::ECCBIN233N,
            9 => MODULUSR::ECCBIN233KN,
            10 => MODULUSR::ECCBIN163N,
            11 => MODULUSR::ECCBIN163KN,
            12 => MODULUSR::ECCPRIME256N,
            13 => MODULUSR::ECCPRIME224N,
            14 => MODULUSR::ECCPRIME192N,
            i => MODULUSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIN256`"]
    #[inline]
    pub fn is_bin256(&self) -> bool {
        *self == MODULUSR::BIN256
    }
    #[doc = "Checks if the value of the field is `BIN128`"]
    #[inline]
    pub fn is_bin128(&self) -> bool {
        *self == MODULUSR::BIN128
    }
    #[doc = "Checks if the value of the field is `ECCBIN233P`"]
    #[inline]
    pub fn is_eccbin233p(&self) -> bool {
        *self == MODULUSR::ECCBIN233P
    }
    #[doc = "Checks if the value of the field is `ECCBIN163P`"]
    #[inline]
    pub fn is_eccbin163p(&self) -> bool {
        *self == MODULUSR::ECCBIN163P
    }
    #[doc = "Checks if the value of the field is `GCMBIN128`"]
    #[inline]
    pub fn is_gcmbin128(&self) -> bool {
        *self == MODULUSR::GCMBIN128
    }
    #[doc = "Checks if the value of the field is `ECCPRIME256P`"]
    #[inline]
    pub fn is_eccprime256p(&self) -> bool {
        *self == MODULUSR::ECCPRIME256P
    }
    #[doc = "Checks if the value of the field is `ECCPRIME224P`"]
    #[inline]
    pub fn is_eccprime224p(&self) -> bool {
        *self == MODULUSR::ECCPRIME224P
    }
    #[doc = "Checks if the value of the field is `ECCPRIME192P`"]
    #[inline]
    pub fn is_eccprime192p(&self) -> bool {
        *self == MODULUSR::ECCPRIME192P
    }
    #[doc = "Checks if the value of the field is `ECCBIN233N`"]
    #[inline]
    pub fn is_eccbin233n(&self) -> bool {
        *self == MODULUSR::ECCBIN233N
    }
    #[doc = "Checks if the value of the field is `ECCBIN233KN`"]
    #[inline]
    pub fn is_eccbin233kn(&self) -> bool {
        *self == MODULUSR::ECCBIN233KN
    }
    #[doc = "Checks if the value of the field is `ECCBIN163N`"]
    #[inline]
    pub fn is_eccbin163n(&self) -> bool {
        *self == MODULUSR::ECCBIN163N
    }
    #[doc = "Checks if the value of the field is `ECCBIN163KN`"]
    #[inline]
    pub fn is_eccbin163kn(&self) -> bool {
        *self == MODULUSR::ECCBIN163KN
    }
    #[doc = "Checks if the value of the field is `ECCPRIME256N`"]
    #[inline]
    pub fn is_eccprime256n(&self) -> bool {
        *self == MODULUSR::ECCPRIME256N
    }
    #[doc = "Checks if the value of the field is `ECCPRIME224N`"]
    #[inline]
    pub fn is_eccprime224n(&self) -> bool {
        *self == MODULUSR::ECCPRIME224N
    }
    #[doc = "Checks if the value of the field is `ECCPRIME192N`"]
    #[inline]
    pub fn is_eccprime192n(&self) -> bool {
        *self == MODULUSR::ECCPRIME192N
    }
}
#[doc = r" Value of the field"]
pub struct MODOPR {
    bits: bool,
}
impl MODOPR {
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
#[doc = "Possible values of the field `MULWIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MULWIDTHR {
    #[doc = "Multiply 256 bits"]
    MUL256,
    #[doc = "Multiply 128 bits"]
    MUL128,
    #[doc = "Same number of bits as specified by MODULUS"]
    MULMOD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MULWIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MULWIDTHR::MUL256 => 0,
            MULWIDTHR::MUL128 => 1,
            MULWIDTHR::MULMOD => 2,
            MULWIDTHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MULWIDTHR {
        match value {
            0 => MULWIDTHR::MUL256,
            1 => MULWIDTHR::MUL128,
            2 => MULWIDTHR::MULMOD,
            i => MULWIDTHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MUL256`"]
    #[inline]
    pub fn is_mul256(&self) -> bool {
        *self == MULWIDTHR::MUL256
    }
    #[doc = "Checks if the value of the field is `MUL128`"]
    #[inline]
    pub fn is_mul128(&self) -> bool {
        *self == MULWIDTHR::MUL128
    }
    #[doc = "Checks if the value of the field is `MULMOD`"]
    #[inline]
    pub fn is_mulmod(&self) -> bool {
        *self == MULWIDTHR::MULMOD
    }
}
#[doc = "Possible values of the field `RESULTWIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESULTWIDTHR {
    #[doc = "Results have 256 bits"]
    _256BIT,
    #[doc = "Results have 128 bits"]
    _128BIT,
    #[doc = "Results have 260 bits. Upper bits of result can be read through DDATA0MSBS in CRYPTO_STATUS"]
    _260BIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RESULTWIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RESULTWIDTHR::_256BIT => 0,
            RESULTWIDTHR::_128BIT => 1,
            RESULTWIDTHR::_260BIT => 2,
            RESULTWIDTHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RESULTWIDTHR {
        match value {
            0 => RESULTWIDTHR::_256BIT,
            1 => RESULTWIDTHR::_128BIT,
            2 => RESULTWIDTHR::_260BIT,
            i => RESULTWIDTHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_256BIT`"]
    #[inline]
    pub fn is_256bit(&self) -> bool {
        *self == RESULTWIDTHR::_256BIT
    }
    #[doc = "Checks if the value of the field is `_128BIT`"]
    #[inline]
    pub fn is_128bit(&self) -> bool {
        *self == RESULTWIDTHR::_128BIT
    }
    #[doc = "Checks if the value of the field is `_260BIT`"]
    #[inline]
    pub fn is_260bit(&self) -> bool {
        *self == RESULTWIDTHR::_260BIT
    }
}
#[doc = "Values that can be written to the field `MODULUS`"]
pub enum MODULUSW {
    #[doc = "Generic modulus. p = 2^256"]
    BIN256,
    #[doc = "Generic modulus. p = 2^128"]
    BIN128,
    #[doc = "Modulus for B-233 and K-233 ECC curves. p(t) = t^233 + t^74 + 1"]
    ECCBIN233P,
    #[doc = "Modulus for B-163 and K-163 ECC curves. p(t) = t^163 + t^7 + t^6 + t^3 + 1"]
    ECCBIN163P,
    #[doc = "Modulus for GCM. P(t) = t^128 + t^7 + t^2 + t + 1"]
    GCMBIN128,
    #[doc = "Modulus for P-256 ECC curve. p = 2^256 - 2^224 + 2^192 + 2^96 - 1"]
    ECCPRIME256P,
    #[doc = "Modulus for P-224 ECC curve. p = 2^224 - 2^96 - 1"]
    ECCPRIME224P,
    #[doc = "Modulus for P-192 ECC curve. p = 2^192 - 2^64 - 1"]
    ECCPRIME192P,
    #[doc = "P modulus for B-233 ECC curve"]
    ECCBIN233N,
    #[doc = "P modulus for K-233 ECC curve"]
    ECCBIN233KN,
    #[doc = "P modulus for B-163 ECC curve"]
    ECCBIN163N,
    #[doc = "P modulus for K-163 ECC curve"]
    ECCBIN163KN,
    #[doc = "P modulus for P-256 ECC curve"]
    ECCPRIME256N,
    #[doc = "P modulus for P-224 ECC curve"]
    ECCPRIME224N,
    #[doc = "P modulus for P-192 ECC curve"]
    ECCPRIME192N,
}
impl MODULUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODULUSW::BIN256 => 0,
            MODULUSW::BIN128 => 1,
            MODULUSW::ECCBIN233P => 2,
            MODULUSW::ECCBIN163P => 3,
            MODULUSW::GCMBIN128 => 4,
            MODULUSW::ECCPRIME256P => 5,
            MODULUSW::ECCPRIME224P => 6,
            MODULUSW::ECCPRIME192P => 7,
            MODULUSW::ECCBIN233N => 8,
            MODULUSW::ECCBIN233KN => 9,
            MODULUSW::ECCBIN163N => 10,
            MODULUSW::ECCBIN163KN => 11,
            MODULUSW::ECCPRIME256N => 12,
            MODULUSW::ECCPRIME224N => 13,
            MODULUSW::ECCPRIME192N => 14,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODULUSW<'a> {
    w: &'a mut W,
}
impl<'a> _MODULUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODULUSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Generic modulus. p = 2^256"]
    #[inline]
    pub fn bin256(self) -> &'a mut W {
        self.variant(MODULUSW::BIN256)
    }
    #[doc = "Generic modulus. p = 2^128"]
    #[inline]
    pub fn bin128(self) -> &'a mut W {
        self.variant(MODULUSW::BIN128)
    }
    #[doc = "Modulus for B-233 and K-233 ECC curves. p(t) = t^233 + t^74 + 1"]
    #[inline]
    pub fn eccbin233p(self) -> &'a mut W {
        self.variant(MODULUSW::ECCBIN233P)
    }
    #[doc = "Modulus for B-163 and K-163 ECC curves. p(t) = t^163 + t^7 + t^6 + t^3 + 1"]
    #[inline]
    pub fn eccbin163p(self) -> &'a mut W {
        self.variant(MODULUSW::ECCBIN163P)
    }
    #[doc = "Modulus for GCM. P(t) = t^128 + t^7 + t^2 + t + 1"]
    #[inline]
    pub fn gcmbin128(self) -> &'a mut W {
        self.variant(MODULUSW::GCMBIN128)
    }
    #[doc = "Modulus for P-256 ECC curve. p = 2^256 - 2^224 + 2^192 + 2^96 - 1"]
    #[inline]
    pub fn eccprime256p(self) -> &'a mut W {
        self.variant(MODULUSW::ECCPRIME256P)
    }
    #[doc = "Modulus for P-224 ECC curve. p = 2^224 - 2^96 - 1"]
    #[inline]
    pub fn eccprime224p(self) -> &'a mut W {
        self.variant(MODULUSW::ECCPRIME224P)
    }
    #[doc = "Modulus for P-192 ECC curve. p = 2^192 - 2^64 - 1"]
    #[inline]
    pub fn eccprime192p(self) -> &'a mut W {
        self.variant(MODULUSW::ECCPRIME192P)
    }
    #[doc = "P modulus for B-233 ECC curve"]
    #[inline]
    pub fn eccbin233n(self) -> &'a mut W {
        self.variant(MODULUSW::ECCBIN233N)
    }
    #[doc = "P modulus for K-233 ECC curve"]
    #[inline]
    pub fn eccbin233kn(self) -> &'a mut W {
        self.variant(MODULUSW::ECCBIN233KN)
    }
    #[doc = "P modulus for B-163 ECC curve"]
    #[inline]
    pub fn eccbin163n(self) -> &'a mut W {
        self.variant(MODULUSW::ECCBIN163N)
    }
    #[doc = "P modulus for K-163 ECC curve"]
    #[inline]
    pub fn eccbin163kn(self) -> &'a mut W {
        self.variant(MODULUSW::ECCBIN163KN)
    }
    #[doc = "P modulus for P-256 ECC curve"]
    #[inline]
    pub fn eccprime256n(self) -> &'a mut W {
        self.variant(MODULUSW::ECCPRIME256N)
    }
    #[doc = "P modulus for P-224 ECC curve"]
    #[inline]
    pub fn eccprime224n(self) -> &'a mut W {
        self.variant(MODULUSW::ECCPRIME224N)
    }
    #[doc = "P modulus for P-192 ECC curve"]
    #[inline]
    pub fn eccprime192n(self) -> &'a mut W {
        self.variant(MODULUSW::ECCPRIME192N)
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
pub struct _MODOPW<'a> {
    w: &'a mut W,
}
impl<'a> _MODOPW<'a> {
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
#[doc = "Values that can be written to the field `MULWIDTH`"]
pub enum MULWIDTHW {
    #[doc = "Multiply 256 bits"]
    MUL256,
    #[doc = "Multiply 128 bits"]
    MUL128,
    #[doc = "Same number of bits as specified by MODULUS"]
    MULMOD,
}
impl MULWIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MULWIDTHW::MUL256 => 0,
            MULWIDTHW::MUL128 => 1,
            MULWIDTHW::MULMOD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MULWIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _MULWIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MULWIDTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Multiply 256 bits"]
    #[inline]
    pub fn mul256(self) -> &'a mut W {
        self.variant(MULWIDTHW::MUL256)
    }
    #[doc = "Multiply 128 bits"]
    #[inline]
    pub fn mul128(self) -> &'a mut W {
        self.variant(MULWIDTHW::MUL128)
    }
    #[doc = "Same number of bits as specified by MODULUS"]
    #[inline]
    pub fn mulmod(self) -> &'a mut W {
        self.variant(MULWIDTHW::MULMOD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RESULTWIDTH`"]
pub enum RESULTWIDTHW {
    #[doc = "Results have 256 bits"]
    _256BIT,
    #[doc = "Results have 128 bits"]
    _128BIT,
    #[doc = "Results have 260 bits. Upper bits of result can be read through DDATA0MSBS in CRYPTO_STATUS"]
    _260BIT,
}
impl RESULTWIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RESULTWIDTHW::_256BIT => 0,
            RESULTWIDTHW::_128BIT => 1,
            RESULTWIDTHW::_260BIT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESULTWIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _RESULTWIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESULTWIDTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Results have 256 bits"]
    #[inline]
    pub fn _256bit(self) -> &'a mut W {
        self.variant(RESULTWIDTHW::_256BIT)
    }
    #[doc = "Results have 128 bits"]
    #[inline]
    pub fn _128bit(self) -> &'a mut W {
        self.variant(RESULTWIDTHW::_128BIT)
    }
    #[doc = "Results have 260 bits. Upper bits of result can be read through DDATA0MSBS in CRYPTO_STATUS"]
    #[inline]
    pub fn _260bit(self) -> &'a mut W {
        self.variant(RESULTWIDTHW::_260BIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
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
    #[doc = "Bits 0:3 - Modular Operation Modulus"]
    #[inline]
    pub fn modulus(&self) -> MODULUSR {
        MODULUSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Modular Operation Field Type"]
    #[inline]
    pub fn modop(&self) -> MODOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MODOPR { bits }
    }
    #[doc = "Bits 8:9 - Multiply Width"]
    #[inline]
    pub fn mulwidth(&self) -> MULWIDTHR {
        MULWIDTHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Result Width"]
    #[inline]
    pub fn resultwidth(&self) -> RESULTWIDTHR {
        RESULTWIDTHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
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
    #[doc = "Bits 0:3 - Modular Operation Modulus"]
    #[inline]
    pub fn modulus(&mut self) -> _MODULUSW {
        _MODULUSW { w: self }
    }
    #[doc = "Bit 4 - Modular Operation Field Type"]
    #[inline]
    pub fn modop(&mut self) -> _MODOPW {
        _MODOPW { w: self }
    }
    #[doc = "Bits 8:9 - Multiply Width"]
    #[inline]
    pub fn mulwidth(&mut self) -> _MULWIDTHW {
        _MULWIDTHW { w: self }
    }
    #[doc = "Bits 10:11 - Result Width"]
    #[inline]
    pub fn resultwidth(&mut self) -> _RESULTWIDTHW {
        _RESULTWIDTHW { w: self }
    }
}
