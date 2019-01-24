#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ROUTELOC0 {
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
#[doc = "Possible values of the field `OUTLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTLOCR {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
    #[doc = "Location 4"]
    LOC4,
    #[doc = "Location 5"]
    LOC5,
    #[doc = "Location 6"]
    LOC6,
    #[doc = "Location 7"]
    LOC7,
    #[doc = "Location 8"]
    LOC8,
    #[doc = "Location 9"]
    LOC9,
    #[doc = "Location 10"]
    LOC10,
    #[doc = "Location 11"]
    LOC11,
    #[doc = "Location 12"]
    LOC12,
    #[doc = "Location 13"]
    LOC13,
    #[doc = "Location 14"]
    LOC14,
    #[doc = "Location 15"]
    LOC15,
    #[doc = "Location 16"]
    LOC16,
    #[doc = "Location 17"]
    LOC17,
    #[doc = "Location 18"]
    LOC18,
    #[doc = "Location 19"]
    LOC19,
    #[doc = "Location 20"]
    LOC20,
    #[doc = "Location 21"]
    LOC21,
    #[doc = "Location 22"]
    LOC22,
    #[doc = "Location 23"]
    LOC23,
    #[doc = "Location 24"]
    LOC24,
    #[doc = "Location 25"]
    LOC25,
    #[doc = "Location 26"]
    LOC26,
    #[doc = "Location 27"]
    LOC27,
    #[doc = "Location 28"]
    LOC28,
    #[doc = "Location 29"]
    LOC29,
    #[doc = "Location 30"]
    LOC30,
    #[doc = "Location 31"]
    LOC31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OUTLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OUTLOCR::LOC0 => 0,
            OUTLOCR::LOC1 => 1,
            OUTLOCR::LOC2 => 2,
            OUTLOCR::LOC3 => 3,
            OUTLOCR::LOC4 => 4,
            OUTLOCR::LOC5 => 5,
            OUTLOCR::LOC6 => 6,
            OUTLOCR::LOC7 => 7,
            OUTLOCR::LOC8 => 8,
            OUTLOCR::LOC9 => 9,
            OUTLOCR::LOC10 => 10,
            OUTLOCR::LOC11 => 11,
            OUTLOCR::LOC12 => 12,
            OUTLOCR::LOC13 => 13,
            OUTLOCR::LOC14 => 14,
            OUTLOCR::LOC15 => 15,
            OUTLOCR::LOC16 => 16,
            OUTLOCR::LOC17 => 17,
            OUTLOCR::LOC18 => 18,
            OUTLOCR::LOC19 => 19,
            OUTLOCR::LOC20 => 20,
            OUTLOCR::LOC21 => 21,
            OUTLOCR::LOC22 => 22,
            OUTLOCR::LOC23 => 23,
            OUTLOCR::LOC24 => 24,
            OUTLOCR::LOC25 => 25,
            OUTLOCR::LOC26 => 26,
            OUTLOCR::LOC27 => 27,
            OUTLOCR::LOC28 => 28,
            OUTLOCR::LOC29 => 29,
            OUTLOCR::LOC30 => 30,
            OUTLOCR::LOC31 => 31,
            OUTLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OUTLOCR {
        match value {
            0 => OUTLOCR::LOC0,
            1 => OUTLOCR::LOC1,
            2 => OUTLOCR::LOC2,
            3 => OUTLOCR::LOC3,
            4 => OUTLOCR::LOC4,
            5 => OUTLOCR::LOC5,
            6 => OUTLOCR::LOC6,
            7 => OUTLOCR::LOC7,
            8 => OUTLOCR::LOC8,
            9 => OUTLOCR::LOC9,
            10 => OUTLOCR::LOC10,
            11 => OUTLOCR::LOC11,
            12 => OUTLOCR::LOC12,
            13 => OUTLOCR::LOC13,
            14 => OUTLOCR::LOC14,
            15 => OUTLOCR::LOC15,
            16 => OUTLOCR::LOC16,
            17 => OUTLOCR::LOC17,
            18 => OUTLOCR::LOC18,
            19 => OUTLOCR::LOC19,
            20 => OUTLOCR::LOC20,
            21 => OUTLOCR::LOC21,
            22 => OUTLOCR::LOC22,
            23 => OUTLOCR::LOC23,
            24 => OUTLOCR::LOC24,
            25 => OUTLOCR::LOC25,
            26 => OUTLOCR::LOC26,
            27 => OUTLOCR::LOC27,
            28 => OUTLOCR::LOC28,
            29 => OUTLOCR::LOC29,
            30 => OUTLOCR::LOC30,
            31 => OUTLOCR::LOC31,
            i => OUTLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == OUTLOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == OUTLOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == OUTLOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == OUTLOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == OUTLOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == OUTLOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == OUTLOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == OUTLOCR::LOC7
    }
    #[doc = "Checks if the value of the field is `LOC8`"]
    #[inline]
    pub fn is_loc8(&self) -> bool {
        *self == OUTLOCR::LOC8
    }
    #[doc = "Checks if the value of the field is `LOC9`"]
    #[inline]
    pub fn is_loc9(&self) -> bool {
        *self == OUTLOCR::LOC9
    }
    #[doc = "Checks if the value of the field is `LOC10`"]
    #[inline]
    pub fn is_loc10(&self) -> bool {
        *self == OUTLOCR::LOC10
    }
    #[doc = "Checks if the value of the field is `LOC11`"]
    #[inline]
    pub fn is_loc11(&self) -> bool {
        *self == OUTLOCR::LOC11
    }
    #[doc = "Checks if the value of the field is `LOC12`"]
    #[inline]
    pub fn is_loc12(&self) -> bool {
        *self == OUTLOCR::LOC12
    }
    #[doc = "Checks if the value of the field is `LOC13`"]
    #[inline]
    pub fn is_loc13(&self) -> bool {
        *self == OUTLOCR::LOC13
    }
    #[doc = "Checks if the value of the field is `LOC14`"]
    #[inline]
    pub fn is_loc14(&self) -> bool {
        *self == OUTLOCR::LOC14
    }
    #[doc = "Checks if the value of the field is `LOC15`"]
    #[inline]
    pub fn is_loc15(&self) -> bool {
        *self == OUTLOCR::LOC15
    }
    #[doc = "Checks if the value of the field is `LOC16`"]
    #[inline]
    pub fn is_loc16(&self) -> bool {
        *self == OUTLOCR::LOC16
    }
    #[doc = "Checks if the value of the field is `LOC17`"]
    #[inline]
    pub fn is_loc17(&self) -> bool {
        *self == OUTLOCR::LOC17
    }
    #[doc = "Checks if the value of the field is `LOC18`"]
    #[inline]
    pub fn is_loc18(&self) -> bool {
        *self == OUTLOCR::LOC18
    }
    #[doc = "Checks if the value of the field is `LOC19`"]
    #[inline]
    pub fn is_loc19(&self) -> bool {
        *self == OUTLOCR::LOC19
    }
    #[doc = "Checks if the value of the field is `LOC20`"]
    #[inline]
    pub fn is_loc20(&self) -> bool {
        *self == OUTLOCR::LOC20
    }
    #[doc = "Checks if the value of the field is `LOC21`"]
    #[inline]
    pub fn is_loc21(&self) -> bool {
        *self == OUTLOCR::LOC21
    }
    #[doc = "Checks if the value of the field is `LOC22`"]
    #[inline]
    pub fn is_loc22(&self) -> bool {
        *self == OUTLOCR::LOC22
    }
    #[doc = "Checks if the value of the field is `LOC23`"]
    #[inline]
    pub fn is_loc23(&self) -> bool {
        *self == OUTLOCR::LOC23
    }
    #[doc = "Checks if the value of the field is `LOC24`"]
    #[inline]
    pub fn is_loc24(&self) -> bool {
        *self == OUTLOCR::LOC24
    }
    #[doc = "Checks if the value of the field is `LOC25`"]
    #[inline]
    pub fn is_loc25(&self) -> bool {
        *self == OUTLOCR::LOC25
    }
    #[doc = "Checks if the value of the field is `LOC26`"]
    #[inline]
    pub fn is_loc26(&self) -> bool {
        *self == OUTLOCR::LOC26
    }
    #[doc = "Checks if the value of the field is `LOC27`"]
    #[inline]
    pub fn is_loc27(&self) -> bool {
        *self == OUTLOCR::LOC27
    }
    #[doc = "Checks if the value of the field is `LOC28`"]
    #[inline]
    pub fn is_loc28(&self) -> bool {
        *self == OUTLOCR::LOC28
    }
    #[doc = "Checks if the value of the field is `LOC29`"]
    #[inline]
    pub fn is_loc29(&self) -> bool {
        *self == OUTLOCR::LOC29
    }
    #[doc = "Checks if the value of the field is `LOC30`"]
    #[inline]
    pub fn is_loc30(&self) -> bool {
        *self == OUTLOCR::LOC30
    }
    #[doc = "Checks if the value of the field is `LOC31`"]
    #[inline]
    pub fn is_loc31(&self) -> bool {
        *self == OUTLOCR::LOC31
    }
}
#[doc = "Values that can be written to the field `OUTLOC`"]
pub enum OUTLOCW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
    #[doc = "Location 4"]
    LOC4,
    #[doc = "Location 5"]
    LOC5,
    #[doc = "Location 6"]
    LOC6,
    #[doc = "Location 7"]
    LOC7,
    #[doc = "Location 8"]
    LOC8,
    #[doc = "Location 9"]
    LOC9,
    #[doc = "Location 10"]
    LOC10,
    #[doc = "Location 11"]
    LOC11,
    #[doc = "Location 12"]
    LOC12,
    #[doc = "Location 13"]
    LOC13,
    #[doc = "Location 14"]
    LOC14,
    #[doc = "Location 15"]
    LOC15,
    #[doc = "Location 16"]
    LOC16,
    #[doc = "Location 17"]
    LOC17,
    #[doc = "Location 18"]
    LOC18,
    #[doc = "Location 19"]
    LOC19,
    #[doc = "Location 20"]
    LOC20,
    #[doc = "Location 21"]
    LOC21,
    #[doc = "Location 22"]
    LOC22,
    #[doc = "Location 23"]
    LOC23,
    #[doc = "Location 24"]
    LOC24,
    #[doc = "Location 25"]
    LOC25,
    #[doc = "Location 26"]
    LOC26,
    #[doc = "Location 27"]
    LOC27,
    #[doc = "Location 28"]
    LOC28,
    #[doc = "Location 29"]
    LOC29,
    #[doc = "Location 30"]
    LOC30,
    #[doc = "Location 31"]
    LOC31,
}
impl OUTLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OUTLOCW::LOC0 => 0,
            OUTLOCW::LOC1 => 1,
            OUTLOCW::LOC2 => 2,
            OUTLOCW::LOC3 => 3,
            OUTLOCW::LOC4 => 4,
            OUTLOCW::LOC5 => 5,
            OUTLOCW::LOC6 => 6,
            OUTLOCW::LOC7 => 7,
            OUTLOCW::LOC8 => 8,
            OUTLOCW::LOC9 => 9,
            OUTLOCW::LOC10 => 10,
            OUTLOCW::LOC11 => 11,
            OUTLOCW::LOC12 => 12,
            OUTLOCW::LOC13 => 13,
            OUTLOCW::LOC14 => 14,
            OUTLOCW::LOC15 => 15,
            OUTLOCW::LOC16 => 16,
            OUTLOCW::LOC17 => 17,
            OUTLOCW::LOC18 => 18,
            OUTLOCW::LOC19 => 19,
            OUTLOCW::LOC20 => 20,
            OUTLOCW::LOC21 => 21,
            OUTLOCW::LOC22 => 22,
            OUTLOCW::LOC23 => 23,
            OUTLOCW::LOC24 => 24,
            OUTLOCW::LOC25 => 25,
            OUTLOCW::LOC26 => 26,
            OUTLOCW::LOC27 => 27,
            OUTLOCW::LOC28 => 28,
            OUTLOCW::LOC29 => 29,
            OUTLOCW::LOC30 => 30,
            OUTLOCW::LOC31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUTLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC7)
    }
    #[doc = "Location 8"]
    #[inline]
    pub fn loc8(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC8)
    }
    #[doc = "Location 9"]
    #[inline]
    pub fn loc9(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC9)
    }
    #[doc = "Location 10"]
    #[inline]
    pub fn loc10(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC10)
    }
    #[doc = "Location 11"]
    #[inline]
    pub fn loc11(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC11)
    }
    #[doc = "Location 12"]
    #[inline]
    pub fn loc12(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC12)
    }
    #[doc = "Location 13"]
    #[inline]
    pub fn loc13(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC13)
    }
    #[doc = "Location 14"]
    #[inline]
    pub fn loc14(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC14)
    }
    #[doc = "Location 15"]
    #[inline]
    pub fn loc15(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC15)
    }
    #[doc = "Location 16"]
    #[inline]
    pub fn loc16(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC16)
    }
    #[doc = "Location 17"]
    #[inline]
    pub fn loc17(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC17)
    }
    #[doc = "Location 18"]
    #[inline]
    pub fn loc18(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC18)
    }
    #[doc = "Location 19"]
    #[inline]
    pub fn loc19(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC19)
    }
    #[doc = "Location 20"]
    #[inline]
    pub fn loc20(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC20)
    }
    #[doc = "Location 21"]
    #[inline]
    pub fn loc21(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC21)
    }
    #[doc = "Location 22"]
    #[inline]
    pub fn loc22(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC22)
    }
    #[doc = "Location 23"]
    #[inline]
    pub fn loc23(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC23)
    }
    #[doc = "Location 24"]
    #[inline]
    pub fn loc24(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC24)
    }
    #[doc = "Location 25"]
    #[inline]
    pub fn loc25(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC25)
    }
    #[doc = "Location 26"]
    #[inline]
    pub fn loc26(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC26)
    }
    #[doc = "Location 27"]
    #[inline]
    pub fn loc27(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC27)
    }
    #[doc = "Location 28"]
    #[inline]
    pub fn loc28(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC28)
    }
    #[doc = "Location 29"]
    #[inline]
    pub fn loc29(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC29)
    }
    #[doc = "Location 30"]
    #[inline]
    pub fn loc30(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC30)
    }
    #[doc = "Location 31"]
    #[inline]
    pub fn loc31(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
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
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline]
    pub fn outloc(&self) -> OUTLOCR {
        OUTLOCR::_from({
            const MASK: u8 = 63;
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
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline]
    pub fn outloc(&mut self) -> _OUTLOCW {
        _OUTLOCW { w: self }
    }
}
