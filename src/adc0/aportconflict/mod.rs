#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::APORTCONFLICT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct APORT0XCONFLICTR {
    bits: bool,
}
impl APORT0XCONFLICTR {
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
pub struct APORT0YCONFLICTR {
    bits: bool,
}
impl APORT0YCONFLICTR {
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
pub struct APORT1XCONFLICTR {
    bits: bool,
}
impl APORT1XCONFLICTR {
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
pub struct APORT1YCONFLICTR {
    bits: bool,
}
impl APORT1YCONFLICTR {
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
pub struct APORT2XCONFLICTR {
    bits: bool,
}
impl APORT2XCONFLICTR {
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
pub struct APORT2YCONFLICTR {
    bits: bool,
}
impl APORT2YCONFLICTR {
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
pub struct APORT3XCONFLICTR {
    bits: bool,
}
impl APORT3XCONFLICTR {
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
pub struct APORT3YCONFLICTR {
    bits: bool,
}
impl APORT3YCONFLICTR {
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
pub struct APORT4XCONFLICTR {
    bits: bool,
}
impl APORT4XCONFLICTR {
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
pub struct APORT4YCONFLICTR {
    bits: bool,
}
impl APORT4YCONFLICTR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - 1 If the Bus Connected to APORT0X is in Conflict With Another Peripheral"]
    #[inline]
    pub fn aport0xconflict(&self) -> APORT0XCONFLICTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APORT0XCONFLICTR { bits }
    }
    #[doc = "Bit 1 - 1 If the Bus Connected to APORT0Y is in Conflict With Another Peripheral"]
    #[inline]
    pub fn aport0yconflict(&self) -> APORT0YCONFLICTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APORT0YCONFLICTR { bits }
    }
    #[doc = "Bit 2 - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral"]
    #[inline]
    pub fn aport1xconflict(&self) -> APORT1XCONFLICTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APORT1XCONFLICTR { bits }
    }
    #[doc = "Bit 3 - 1 If the Bus Connected to APORT1Y is in Conflict With Another Peripheral"]
    #[inline]
    pub fn aport1yconflict(&self) -> APORT1YCONFLICTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APORT1YCONFLICTR { bits }
    }
    #[doc = "Bit 4 - 1 If the Bus Connected to APORT2X is in Conflict With Another Peripheral"]
    #[inline]
    pub fn aport2xconflict(&self) -> APORT2XCONFLICTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APORT2XCONFLICTR { bits }
    }
    #[doc = "Bit 5 - 1 If the Bus Connected to APORT2Y is in Conflict With Another Peripheral"]
    #[inline]
    pub fn aport2yconflict(&self) -> APORT2YCONFLICTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APORT2YCONFLICTR { bits }
    }
    #[doc = "Bit 6 - 1 If the Bus Connected to APORT3X is in Conflict With Another Peripheral"]
    #[inline]
    pub fn aport3xconflict(&self) -> APORT3XCONFLICTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APORT3XCONFLICTR { bits }
    }
    #[doc = "Bit 7 - 1 If the Bus Connected to APORT3Y is in Conflict With Another Peripheral"]
    #[inline]
    pub fn aport3yconflict(&self) -> APORT3YCONFLICTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APORT3YCONFLICTR { bits }
    }
    #[doc = "Bit 8 - 1 If the Bus Connected to APORT4X is in Conflict With Another Peripheral"]
    #[inline]
    pub fn aport4xconflict(&self) -> APORT4XCONFLICTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APORT4XCONFLICTR { bits }
    }
    #[doc = "Bit 9 - 1 If the Bus Connected to APORT4Y is in Conflict With Another Peripheral"]
    #[inline]
    pub fn aport4yconflict(&self) -> APORT4YCONFLICTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APORT4YCONFLICTR { bits }
    }
}
