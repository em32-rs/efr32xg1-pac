#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HFXOTRIMSTATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct IBTRIMXOCORER {
    bits: u8,
}
impl IBTRIMXOCORER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct REGISHR {
    bits: u8,
}
impl REGISHR {
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
    #[doc = "Bits 0:6 - Value of IBTRIMXOCORE Found By Automatic HFXO Peak Detection Algorithm"]
    #[inline]
    pub fn ibtrimxocore(&self) -> IBTRIMXOCORER {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IBTRIMXOCORER { bits }
    }
    #[doc = "Bits 7:10 - Value of REGISH Found By Automatic HFXO Shunt Current Optimization Algorithm"]
    #[inline]
    pub fn regish(&self) -> REGISHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REGISHR { bits }
    }
}
