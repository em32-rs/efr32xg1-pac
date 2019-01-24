#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CSTATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `V0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum V0R {
    #[doc = "undocumented"]
    DDATA0,
    #[doc = "undocumented"]
    DDATA1,
    #[doc = "undocumented"]
    DDATA2,
    #[doc = "undocumented"]
    DDATA3,
    #[doc = "undocumented"]
    DDATA4,
    #[doc = "undocumented"]
    DATA0,
    #[doc = "undocumented"]
    DATA1,
    #[doc = "undocumented"]
    DATA2,
}
impl V0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            V0R::DDATA0 => 0,
            V0R::DDATA1 => 1,
            V0R::DDATA2 => 2,
            V0R::DDATA3 => 3,
            V0R::DDATA4 => 4,
            V0R::DATA0 => 5,
            V0R::DATA1 => 6,
            V0R::DATA2 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> V0R {
        match value {
            0 => V0R::DDATA0,
            1 => V0R::DDATA1,
            2 => V0R::DDATA2,
            3 => V0R::DDATA3,
            4 => V0R::DDATA4,
            5 => V0R::DATA0,
            6 => V0R::DATA1,
            7 => V0R::DATA2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DDATA0`"]
    #[inline]
    pub fn is_ddata0(&self) -> bool {
        *self == V0R::DDATA0
    }
    #[doc = "Checks if the value of the field is `DDATA1`"]
    #[inline]
    pub fn is_ddata1(&self) -> bool {
        *self == V0R::DDATA1
    }
    #[doc = "Checks if the value of the field is `DDATA2`"]
    #[inline]
    pub fn is_ddata2(&self) -> bool {
        *self == V0R::DDATA2
    }
    #[doc = "Checks if the value of the field is `DDATA3`"]
    #[inline]
    pub fn is_ddata3(&self) -> bool {
        *self == V0R::DDATA3
    }
    #[doc = "Checks if the value of the field is `DDATA4`"]
    #[inline]
    pub fn is_ddata4(&self) -> bool {
        *self == V0R::DDATA4
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline]
    pub fn is_data0(&self) -> bool {
        *self == V0R::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline]
    pub fn is_data1(&self) -> bool {
        *self == V0R::DATA1
    }
    #[doc = "Checks if the value of the field is `DATA2`"]
    #[inline]
    pub fn is_data2(&self) -> bool {
        *self == V0R::DATA2
    }
}
#[doc = "Possible values of the field `V1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum V1R {
    #[doc = "undocumented"]
    DDATA0,
    #[doc = "undocumented"]
    DDATA1,
    #[doc = "undocumented"]
    DDATA2,
    #[doc = "undocumented"]
    DDATA3,
    #[doc = "undocumented"]
    DDATA4,
    #[doc = "undocumented"]
    DATA0,
    #[doc = "undocumented"]
    DATA1,
    #[doc = "undocumented"]
    DATA2,
}
impl V1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            V1R::DDATA0 => 0,
            V1R::DDATA1 => 1,
            V1R::DDATA2 => 2,
            V1R::DDATA3 => 3,
            V1R::DDATA4 => 4,
            V1R::DATA0 => 5,
            V1R::DATA1 => 6,
            V1R::DATA2 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> V1R {
        match value {
            0 => V1R::DDATA0,
            1 => V1R::DDATA1,
            2 => V1R::DDATA2,
            3 => V1R::DDATA3,
            4 => V1R::DDATA4,
            5 => V1R::DATA0,
            6 => V1R::DATA1,
            7 => V1R::DATA2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DDATA0`"]
    #[inline]
    pub fn is_ddata0(&self) -> bool {
        *self == V1R::DDATA0
    }
    #[doc = "Checks if the value of the field is `DDATA1`"]
    #[inline]
    pub fn is_ddata1(&self) -> bool {
        *self == V1R::DDATA1
    }
    #[doc = "Checks if the value of the field is `DDATA2`"]
    #[inline]
    pub fn is_ddata2(&self) -> bool {
        *self == V1R::DDATA2
    }
    #[doc = "Checks if the value of the field is `DDATA3`"]
    #[inline]
    pub fn is_ddata3(&self) -> bool {
        *self == V1R::DDATA3
    }
    #[doc = "Checks if the value of the field is `DDATA4`"]
    #[inline]
    pub fn is_ddata4(&self) -> bool {
        *self == V1R::DDATA4
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline]
    pub fn is_data0(&self) -> bool {
        *self == V1R::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline]
    pub fn is_data1(&self) -> bool {
        *self == V1R::DATA1
    }
    #[doc = "Checks if the value of the field is `DATA2`"]
    #[inline]
    pub fn is_data2(&self) -> bool {
        *self == V1R::DATA2
    }
}
#[doc = r" Value of the field"]
pub struct SEQPARTR {
    bits: bool,
}
impl SEQPARTR {
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
pub struct SEQSKIPR {
    bits: bool,
}
impl SEQSKIPR {
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
pub struct SEQIPR {
    bits: u8,
}
impl SEQIPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Selected ALU Operand 0"]
    #[inline]
    pub fn v0(&self) -> V0R {
        V0R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - Selected ALU Operand 1"]
    #[inline]
    pub fn v1(&self) -> V1R {
        V1R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Sequence Part"]
    #[inline]
    pub fn seqpart(&self) -> SEQPARTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SEQPARTR { bits }
    }
    #[doc = "Bit 17 - Sequence Skip Next Instruction"]
    #[inline]
    pub fn seqskip(&self) -> SEQSKIPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SEQSKIPR { bits }
    }
    #[doc = "Bits 20:24 - Sequence Next Instruction Pointer"]
    #[inline]
    pub fn seqip(&self) -> SEQIPR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SEQIPR { bits }
    }
}
