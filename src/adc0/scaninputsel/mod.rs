#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCANINPUTSEL {
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
#[doc = "Possible values of the field `INPUT0TO7SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT0TO7SELR {
    #[doc = "undocumented"]
    APORT0CH0TO7,
    #[doc = "undocumented"]
    APORT0CH8TO15,
    #[doc = "undocumented"]
    APORT1CH0TO7,
    #[doc = "undocumented"]
    APORT1CH8TO15,
    #[doc = "undocumented"]
    APORT1CH16TO23,
    #[doc = "undocumented"]
    APORT1CH24TO31,
    #[doc = "undocumented"]
    APORT2CH0TO7,
    #[doc = "undocumented"]
    APORT2CH8TO15,
    #[doc = "undocumented"]
    APORT2CH16TO23,
    #[doc = "undocumented"]
    APORT2CH24TO31,
    #[doc = "undocumented"]
    APORT3CH0TO7,
    #[doc = "undocumented"]
    APORT3CH8TO15,
    #[doc = "undocumented"]
    APORT3CH16TO23,
    #[doc = "undocumented"]
    APORT3CH24TO31,
    #[doc = "undocumented"]
    APORT4CH0TO7,
    #[doc = "undocumented"]
    APORT4CH8TO15,
    #[doc = "undocumented"]
    APORT4CH16TO23,
    #[doc = "undocumented"]
    APORT4CH24TO31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INPUT0TO7SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPUT0TO7SELR::APORT0CH0TO7 => 0,
            INPUT0TO7SELR::APORT0CH8TO15 => 1,
            INPUT0TO7SELR::APORT1CH0TO7 => 4,
            INPUT0TO7SELR::APORT1CH8TO15 => 5,
            INPUT0TO7SELR::APORT1CH16TO23 => 6,
            INPUT0TO7SELR::APORT1CH24TO31 => 7,
            INPUT0TO7SELR::APORT2CH0TO7 => 8,
            INPUT0TO7SELR::APORT2CH8TO15 => 9,
            INPUT0TO7SELR::APORT2CH16TO23 => 10,
            INPUT0TO7SELR::APORT2CH24TO31 => 11,
            INPUT0TO7SELR::APORT3CH0TO7 => 12,
            INPUT0TO7SELR::APORT3CH8TO15 => 13,
            INPUT0TO7SELR::APORT3CH16TO23 => 14,
            INPUT0TO7SELR::APORT3CH24TO31 => 15,
            INPUT0TO7SELR::APORT4CH0TO7 => 16,
            INPUT0TO7SELR::APORT4CH8TO15 => 17,
            INPUT0TO7SELR::APORT4CH16TO23 => 18,
            INPUT0TO7SELR::APORT4CH24TO31 => 19,
            INPUT0TO7SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPUT0TO7SELR {
        match value {
            0 => INPUT0TO7SELR::APORT0CH0TO7,
            1 => INPUT0TO7SELR::APORT0CH8TO15,
            4 => INPUT0TO7SELR::APORT1CH0TO7,
            5 => INPUT0TO7SELR::APORT1CH8TO15,
            6 => INPUT0TO7SELR::APORT1CH16TO23,
            7 => INPUT0TO7SELR::APORT1CH24TO31,
            8 => INPUT0TO7SELR::APORT2CH0TO7,
            9 => INPUT0TO7SELR::APORT2CH8TO15,
            10 => INPUT0TO7SELR::APORT2CH16TO23,
            11 => INPUT0TO7SELR::APORT2CH24TO31,
            12 => INPUT0TO7SELR::APORT3CH0TO7,
            13 => INPUT0TO7SELR::APORT3CH8TO15,
            14 => INPUT0TO7SELR::APORT3CH16TO23,
            15 => INPUT0TO7SELR::APORT3CH24TO31,
            16 => INPUT0TO7SELR::APORT4CH0TO7,
            17 => INPUT0TO7SELR::APORT4CH8TO15,
            18 => INPUT0TO7SELR::APORT4CH16TO23,
            19 => INPUT0TO7SELR::APORT4CH24TO31,
            i => INPUT0TO7SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT0CH0TO7`"]
    #[inline]
    pub fn is_aport0ch0to7(&self) -> bool {
        *self == INPUT0TO7SELR::APORT0CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT0CH8TO15`"]
    #[inline]
    pub fn is_aport0ch8to15(&self) -> bool {
        *self == INPUT0TO7SELR::APORT0CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT0TO7SELR::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT0TO7SELR::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT0TO7SELR::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT0TO7SELR::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT2CH0TO7`"]
    #[inline]
    pub fn is_aport2ch0to7(&self) -> bool {
        *self == INPUT0TO7SELR::APORT2CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT2CH8TO15`"]
    #[inline]
    pub fn is_aport2ch8to15(&self) -> bool {
        *self == INPUT0TO7SELR::APORT2CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT2CH16TO23`"]
    #[inline]
    pub fn is_aport2ch16to23(&self) -> bool {
        *self == INPUT0TO7SELR::APORT2CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT2CH24TO31`"]
    #[inline]
    pub fn is_aport2ch24to31(&self) -> bool {
        *self == INPUT0TO7SELR::APORT2CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT0TO7SELR::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT0TO7SELR::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT0TO7SELR::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT0TO7SELR::APORT3CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT4CH0TO7`"]
    #[inline]
    pub fn is_aport4ch0to7(&self) -> bool {
        *self == INPUT0TO7SELR::APORT4CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT4CH8TO15`"]
    #[inline]
    pub fn is_aport4ch8to15(&self) -> bool {
        *self == INPUT0TO7SELR::APORT4CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT4CH16TO23`"]
    #[inline]
    pub fn is_aport4ch16to23(&self) -> bool {
        *self == INPUT0TO7SELR::APORT4CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT4CH24TO31`"]
    #[inline]
    pub fn is_aport4ch24to31(&self) -> bool {
        *self == INPUT0TO7SELR::APORT4CH24TO31
    }
}
#[doc = "Possible values of the field `INPUT8TO15SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT8TO15SELR {
    #[doc = "undocumented"]
    APORT0CH0TO7,
    #[doc = "undocumented"]
    APORT0CH8TO15,
    #[doc = "undocumented"]
    APORT1CH0TO7,
    #[doc = "undocumented"]
    APORT1CH8TO15,
    #[doc = "undocumented"]
    APORT1CH16TO23,
    #[doc = "undocumented"]
    APORT1CH24TO31,
    #[doc = "undocumented"]
    APORT2CH0TO7,
    #[doc = "undocumented"]
    APORT2CH8TO15,
    #[doc = "undocumented"]
    APORT2CH16TO23,
    #[doc = "undocumented"]
    APORT2CH24TO31,
    #[doc = "undocumented"]
    APORT3CH0TO7,
    #[doc = "undocumented"]
    APORT3CH8TO15,
    #[doc = "undocumented"]
    APORT3CH16TO23,
    #[doc = "undocumented"]
    APORT3CH24TO31,
    #[doc = "undocumented"]
    APORT4CH0TO7,
    #[doc = "undocumented"]
    APORT4CH8TO15,
    #[doc = "undocumented"]
    APORT4CH16TO23,
    #[doc = "undocumented"]
    APORT4CH24TO31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INPUT8TO15SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPUT8TO15SELR::APORT0CH0TO7 => 0,
            INPUT8TO15SELR::APORT0CH8TO15 => 1,
            INPUT8TO15SELR::APORT1CH0TO7 => 4,
            INPUT8TO15SELR::APORT1CH8TO15 => 5,
            INPUT8TO15SELR::APORT1CH16TO23 => 6,
            INPUT8TO15SELR::APORT1CH24TO31 => 7,
            INPUT8TO15SELR::APORT2CH0TO7 => 8,
            INPUT8TO15SELR::APORT2CH8TO15 => 9,
            INPUT8TO15SELR::APORT2CH16TO23 => 10,
            INPUT8TO15SELR::APORT2CH24TO31 => 11,
            INPUT8TO15SELR::APORT3CH0TO7 => 12,
            INPUT8TO15SELR::APORT3CH8TO15 => 13,
            INPUT8TO15SELR::APORT3CH16TO23 => 14,
            INPUT8TO15SELR::APORT3CH24TO31 => 15,
            INPUT8TO15SELR::APORT4CH0TO7 => 16,
            INPUT8TO15SELR::APORT4CH8TO15 => 17,
            INPUT8TO15SELR::APORT4CH16TO23 => 18,
            INPUT8TO15SELR::APORT4CH24TO31 => 19,
            INPUT8TO15SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPUT8TO15SELR {
        match value {
            0 => INPUT8TO15SELR::APORT0CH0TO7,
            1 => INPUT8TO15SELR::APORT0CH8TO15,
            4 => INPUT8TO15SELR::APORT1CH0TO7,
            5 => INPUT8TO15SELR::APORT1CH8TO15,
            6 => INPUT8TO15SELR::APORT1CH16TO23,
            7 => INPUT8TO15SELR::APORT1CH24TO31,
            8 => INPUT8TO15SELR::APORT2CH0TO7,
            9 => INPUT8TO15SELR::APORT2CH8TO15,
            10 => INPUT8TO15SELR::APORT2CH16TO23,
            11 => INPUT8TO15SELR::APORT2CH24TO31,
            12 => INPUT8TO15SELR::APORT3CH0TO7,
            13 => INPUT8TO15SELR::APORT3CH8TO15,
            14 => INPUT8TO15SELR::APORT3CH16TO23,
            15 => INPUT8TO15SELR::APORT3CH24TO31,
            16 => INPUT8TO15SELR::APORT4CH0TO7,
            17 => INPUT8TO15SELR::APORT4CH8TO15,
            18 => INPUT8TO15SELR::APORT4CH16TO23,
            19 => INPUT8TO15SELR::APORT4CH24TO31,
            i => INPUT8TO15SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT0CH0TO7`"]
    #[inline]
    pub fn is_aport0ch0to7(&self) -> bool {
        *self == INPUT8TO15SELR::APORT0CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT0CH8TO15`"]
    #[inline]
    pub fn is_aport0ch8to15(&self) -> bool {
        *self == INPUT8TO15SELR::APORT0CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT8TO15SELR::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT8TO15SELR::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT8TO15SELR::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT8TO15SELR::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT2CH0TO7`"]
    #[inline]
    pub fn is_aport2ch0to7(&self) -> bool {
        *self == INPUT8TO15SELR::APORT2CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT2CH8TO15`"]
    #[inline]
    pub fn is_aport2ch8to15(&self) -> bool {
        *self == INPUT8TO15SELR::APORT2CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT2CH16TO23`"]
    #[inline]
    pub fn is_aport2ch16to23(&self) -> bool {
        *self == INPUT8TO15SELR::APORT2CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT2CH24TO31`"]
    #[inline]
    pub fn is_aport2ch24to31(&self) -> bool {
        *self == INPUT8TO15SELR::APORT2CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT8TO15SELR::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT8TO15SELR::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT8TO15SELR::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT8TO15SELR::APORT3CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT4CH0TO7`"]
    #[inline]
    pub fn is_aport4ch0to7(&self) -> bool {
        *self == INPUT8TO15SELR::APORT4CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT4CH8TO15`"]
    #[inline]
    pub fn is_aport4ch8to15(&self) -> bool {
        *self == INPUT8TO15SELR::APORT4CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT4CH16TO23`"]
    #[inline]
    pub fn is_aport4ch16to23(&self) -> bool {
        *self == INPUT8TO15SELR::APORT4CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT4CH24TO31`"]
    #[inline]
    pub fn is_aport4ch24to31(&self) -> bool {
        *self == INPUT8TO15SELR::APORT4CH24TO31
    }
}
#[doc = "Possible values of the field `INPUT16TO23SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT16TO23SELR {
    #[doc = "undocumented"]
    APORT0CH0TO7,
    #[doc = "undocumented"]
    APORT0CH8TO15,
    #[doc = "undocumented"]
    APORT1CH0TO7,
    #[doc = "undocumented"]
    APORT1CH8TO15,
    #[doc = "undocumented"]
    APORT1CH16TO23,
    #[doc = "undocumented"]
    APORT1CH24TO31,
    #[doc = "undocumented"]
    APORT2CH0TO7,
    #[doc = "undocumented"]
    APORT2CH8TO15,
    #[doc = "undocumented"]
    APORT2CH16TO23,
    #[doc = "undocumented"]
    APORT2CH24TO31,
    #[doc = "undocumented"]
    APORT3CH0TO7,
    #[doc = "undocumented"]
    APORT3CH8TO15,
    #[doc = "undocumented"]
    APORT3CH16TO23,
    #[doc = "undocumented"]
    APORT3CH24TO31,
    #[doc = "undocumented"]
    APORT4CH0TO7,
    #[doc = "undocumented"]
    APORT4CH8TO15,
    #[doc = "undocumented"]
    APORT4CH16TO23,
    #[doc = "undocumented"]
    APORT4CH24TO31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INPUT16TO23SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPUT16TO23SELR::APORT0CH0TO7 => 0,
            INPUT16TO23SELR::APORT0CH8TO15 => 1,
            INPUT16TO23SELR::APORT1CH0TO7 => 4,
            INPUT16TO23SELR::APORT1CH8TO15 => 5,
            INPUT16TO23SELR::APORT1CH16TO23 => 6,
            INPUT16TO23SELR::APORT1CH24TO31 => 7,
            INPUT16TO23SELR::APORT2CH0TO7 => 8,
            INPUT16TO23SELR::APORT2CH8TO15 => 9,
            INPUT16TO23SELR::APORT2CH16TO23 => 10,
            INPUT16TO23SELR::APORT2CH24TO31 => 11,
            INPUT16TO23SELR::APORT3CH0TO7 => 12,
            INPUT16TO23SELR::APORT3CH8TO15 => 13,
            INPUT16TO23SELR::APORT3CH16TO23 => 14,
            INPUT16TO23SELR::APORT3CH24TO31 => 15,
            INPUT16TO23SELR::APORT4CH0TO7 => 16,
            INPUT16TO23SELR::APORT4CH8TO15 => 17,
            INPUT16TO23SELR::APORT4CH16TO23 => 18,
            INPUT16TO23SELR::APORT4CH24TO31 => 19,
            INPUT16TO23SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPUT16TO23SELR {
        match value {
            0 => INPUT16TO23SELR::APORT0CH0TO7,
            1 => INPUT16TO23SELR::APORT0CH8TO15,
            4 => INPUT16TO23SELR::APORT1CH0TO7,
            5 => INPUT16TO23SELR::APORT1CH8TO15,
            6 => INPUT16TO23SELR::APORT1CH16TO23,
            7 => INPUT16TO23SELR::APORT1CH24TO31,
            8 => INPUT16TO23SELR::APORT2CH0TO7,
            9 => INPUT16TO23SELR::APORT2CH8TO15,
            10 => INPUT16TO23SELR::APORT2CH16TO23,
            11 => INPUT16TO23SELR::APORT2CH24TO31,
            12 => INPUT16TO23SELR::APORT3CH0TO7,
            13 => INPUT16TO23SELR::APORT3CH8TO15,
            14 => INPUT16TO23SELR::APORT3CH16TO23,
            15 => INPUT16TO23SELR::APORT3CH24TO31,
            16 => INPUT16TO23SELR::APORT4CH0TO7,
            17 => INPUT16TO23SELR::APORT4CH8TO15,
            18 => INPUT16TO23SELR::APORT4CH16TO23,
            19 => INPUT16TO23SELR::APORT4CH24TO31,
            i => INPUT16TO23SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT0CH0TO7`"]
    #[inline]
    pub fn is_aport0ch0to7(&self) -> bool {
        *self == INPUT16TO23SELR::APORT0CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT0CH8TO15`"]
    #[inline]
    pub fn is_aport0ch8to15(&self) -> bool {
        *self == INPUT16TO23SELR::APORT0CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT16TO23SELR::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT16TO23SELR::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT16TO23SELR::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT16TO23SELR::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT2CH0TO7`"]
    #[inline]
    pub fn is_aport2ch0to7(&self) -> bool {
        *self == INPUT16TO23SELR::APORT2CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT2CH8TO15`"]
    #[inline]
    pub fn is_aport2ch8to15(&self) -> bool {
        *self == INPUT16TO23SELR::APORT2CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT2CH16TO23`"]
    #[inline]
    pub fn is_aport2ch16to23(&self) -> bool {
        *self == INPUT16TO23SELR::APORT2CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT2CH24TO31`"]
    #[inline]
    pub fn is_aport2ch24to31(&self) -> bool {
        *self == INPUT16TO23SELR::APORT2CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT16TO23SELR::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT16TO23SELR::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT16TO23SELR::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT16TO23SELR::APORT3CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT4CH0TO7`"]
    #[inline]
    pub fn is_aport4ch0to7(&self) -> bool {
        *self == INPUT16TO23SELR::APORT4CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT4CH8TO15`"]
    #[inline]
    pub fn is_aport4ch8to15(&self) -> bool {
        *self == INPUT16TO23SELR::APORT4CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT4CH16TO23`"]
    #[inline]
    pub fn is_aport4ch16to23(&self) -> bool {
        *self == INPUT16TO23SELR::APORT4CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT4CH24TO31`"]
    #[inline]
    pub fn is_aport4ch24to31(&self) -> bool {
        *self == INPUT16TO23SELR::APORT4CH24TO31
    }
}
#[doc = "Possible values of the field `INPUT24TO31SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT24TO31SELR {
    #[doc = "undocumented"]
    APORT0CH0TO7,
    #[doc = "undocumented"]
    APORT0CH8TO15,
    #[doc = "undocumented"]
    APORT1CH0TO7,
    #[doc = "undocumented"]
    APORT1CH8TO15,
    #[doc = "undocumented"]
    APORT1CH16TO23,
    #[doc = "undocumented"]
    APORT1CH24TO31,
    #[doc = "undocumented"]
    APORT2CH0TO7,
    #[doc = "undocumented"]
    APORT2CH8TO15,
    #[doc = "undocumented"]
    APORT2CH16TO23,
    #[doc = "undocumented"]
    APORT2CH24TO31,
    #[doc = "undocumented"]
    APORT3CH0TO7,
    #[doc = "undocumented"]
    APORT3CH8TO15,
    #[doc = "undocumented"]
    APORT3CH16TO23,
    #[doc = "undocumented"]
    APORT3CH24TO31,
    #[doc = "undocumented"]
    APORT4CH0TO7,
    #[doc = "undocumented"]
    APORT4CH8TO15,
    #[doc = "undocumented"]
    APORT4CH16TO23,
    #[doc = "undocumented"]
    APORT4CH24TO31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INPUT24TO31SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPUT24TO31SELR::APORT0CH0TO7 => 0,
            INPUT24TO31SELR::APORT0CH8TO15 => 1,
            INPUT24TO31SELR::APORT1CH0TO7 => 4,
            INPUT24TO31SELR::APORT1CH8TO15 => 5,
            INPUT24TO31SELR::APORT1CH16TO23 => 6,
            INPUT24TO31SELR::APORT1CH24TO31 => 7,
            INPUT24TO31SELR::APORT2CH0TO7 => 8,
            INPUT24TO31SELR::APORT2CH8TO15 => 9,
            INPUT24TO31SELR::APORT2CH16TO23 => 10,
            INPUT24TO31SELR::APORT2CH24TO31 => 11,
            INPUT24TO31SELR::APORT3CH0TO7 => 12,
            INPUT24TO31SELR::APORT3CH8TO15 => 13,
            INPUT24TO31SELR::APORT3CH16TO23 => 14,
            INPUT24TO31SELR::APORT3CH24TO31 => 15,
            INPUT24TO31SELR::APORT4CH0TO7 => 16,
            INPUT24TO31SELR::APORT4CH8TO15 => 17,
            INPUT24TO31SELR::APORT4CH16TO23 => 18,
            INPUT24TO31SELR::APORT4CH24TO31 => 19,
            INPUT24TO31SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPUT24TO31SELR {
        match value {
            0 => INPUT24TO31SELR::APORT0CH0TO7,
            1 => INPUT24TO31SELR::APORT0CH8TO15,
            4 => INPUT24TO31SELR::APORT1CH0TO7,
            5 => INPUT24TO31SELR::APORT1CH8TO15,
            6 => INPUT24TO31SELR::APORT1CH16TO23,
            7 => INPUT24TO31SELR::APORT1CH24TO31,
            8 => INPUT24TO31SELR::APORT2CH0TO7,
            9 => INPUT24TO31SELR::APORT2CH8TO15,
            10 => INPUT24TO31SELR::APORT2CH16TO23,
            11 => INPUT24TO31SELR::APORT2CH24TO31,
            12 => INPUT24TO31SELR::APORT3CH0TO7,
            13 => INPUT24TO31SELR::APORT3CH8TO15,
            14 => INPUT24TO31SELR::APORT3CH16TO23,
            15 => INPUT24TO31SELR::APORT3CH24TO31,
            16 => INPUT24TO31SELR::APORT4CH0TO7,
            17 => INPUT24TO31SELR::APORT4CH8TO15,
            18 => INPUT24TO31SELR::APORT4CH16TO23,
            19 => INPUT24TO31SELR::APORT4CH24TO31,
            i => INPUT24TO31SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT0CH0TO7`"]
    #[inline]
    pub fn is_aport0ch0to7(&self) -> bool {
        *self == INPUT24TO31SELR::APORT0CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT0CH8TO15`"]
    #[inline]
    pub fn is_aport0ch8to15(&self) -> bool {
        *self == INPUT24TO31SELR::APORT0CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT24TO31SELR::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT24TO31SELR::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT24TO31SELR::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT24TO31SELR::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT2CH0TO7`"]
    #[inline]
    pub fn is_aport2ch0to7(&self) -> bool {
        *self == INPUT24TO31SELR::APORT2CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT2CH8TO15`"]
    #[inline]
    pub fn is_aport2ch8to15(&self) -> bool {
        *self == INPUT24TO31SELR::APORT2CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT2CH16TO23`"]
    #[inline]
    pub fn is_aport2ch16to23(&self) -> bool {
        *self == INPUT24TO31SELR::APORT2CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT2CH24TO31`"]
    #[inline]
    pub fn is_aport2ch24to31(&self) -> bool {
        *self == INPUT24TO31SELR::APORT2CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT24TO31SELR::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT24TO31SELR::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT24TO31SELR::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT24TO31SELR::APORT3CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT4CH0TO7`"]
    #[inline]
    pub fn is_aport4ch0to7(&self) -> bool {
        *self == INPUT24TO31SELR::APORT4CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT4CH8TO15`"]
    #[inline]
    pub fn is_aport4ch8to15(&self) -> bool {
        *self == INPUT24TO31SELR::APORT4CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT4CH16TO23`"]
    #[inline]
    pub fn is_aport4ch16to23(&self) -> bool {
        *self == INPUT24TO31SELR::APORT4CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT4CH24TO31`"]
    #[inline]
    pub fn is_aport4ch24to31(&self) -> bool {
        *self == INPUT24TO31SELR::APORT4CH24TO31
    }
}
#[doc = "Values that can be written to the field `INPUT0TO7SEL`"]
pub enum INPUT0TO7SELW {
    #[doc = "`0`"]
    APORT0CH0TO7,
    #[doc = "`1`"]
    APORT0CH8TO15,
    #[doc = "`100`"]
    APORT1CH0TO7,
    #[doc = "`101`"]
    APORT1CH8TO15,
    #[doc = "`110`"]
    APORT1CH16TO23,
    #[doc = "`111`"]
    APORT1CH24TO31,
    #[doc = "`1000`"]
    APORT2CH0TO7,
    #[doc = "`1001`"]
    APORT2CH8TO15,
    #[doc = "`1010`"]
    APORT2CH16TO23,
    #[doc = "`1011`"]
    APORT2CH24TO31,
    #[doc = "`1100`"]
    APORT3CH0TO7,
    #[doc = "`1101`"]
    APORT3CH8TO15,
    #[doc = "`1110`"]
    APORT3CH16TO23,
    #[doc = "`1111`"]
    APORT3CH24TO31,
    #[doc = "`10000`"]
    APORT4CH0TO7,
    #[doc = "`10001`"]
    APORT4CH8TO15,
    #[doc = "`10010`"]
    APORT4CH16TO23,
    #[doc = "`10011`"]
    APORT4CH24TO31,
}
impl INPUT0TO7SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPUT0TO7SELW::APORT0CH0TO7 => 0,
            INPUT0TO7SELW::APORT0CH8TO15 => 1,
            INPUT0TO7SELW::APORT1CH0TO7 => 4,
            INPUT0TO7SELW::APORT1CH8TO15 => 5,
            INPUT0TO7SELW::APORT1CH16TO23 => 6,
            INPUT0TO7SELW::APORT1CH24TO31 => 7,
            INPUT0TO7SELW::APORT2CH0TO7 => 8,
            INPUT0TO7SELW::APORT2CH8TO15 => 9,
            INPUT0TO7SELW::APORT2CH16TO23 => 10,
            INPUT0TO7SELW::APORT2CH24TO31 => 11,
            INPUT0TO7SELW::APORT3CH0TO7 => 12,
            INPUT0TO7SELW::APORT3CH8TO15 => 13,
            INPUT0TO7SELW::APORT3CH16TO23 => 14,
            INPUT0TO7SELW::APORT3CH24TO31 => 15,
            INPUT0TO7SELW::APORT4CH0TO7 => 16,
            INPUT0TO7SELW::APORT4CH8TO15 => 17,
            INPUT0TO7SELW::APORT4CH16TO23 => 18,
            INPUT0TO7SELW::APORT4CH24TO31 => 19,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUT0TO7SELW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT0TO7SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUT0TO7SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn aport0ch0to7(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT0CH0TO7)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn aport0ch8to15(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT0CH8TO15)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT1CH0TO7)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT1CH8TO15)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT1CH16TO23)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT1CH24TO31)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn aport2ch0to7(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT2CH0TO7)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn aport2ch8to15(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT2CH8TO15)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn aport2ch16to23(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT2CH16TO23)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn aport2ch24to31(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT2CH24TO31)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT3CH0TO7)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT3CH8TO15)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT3CH16TO23)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT3CH24TO31)
    }
    #[doc = "`10000`"]
    #[inline]
    pub fn aport4ch0to7(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT4CH0TO7)
    }
    #[doc = "`10001`"]
    #[inline]
    pub fn aport4ch8to15(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT4CH8TO15)
    }
    #[doc = "`10010`"]
    #[inline]
    pub fn aport4ch16to23(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT4CH16TO23)
    }
    #[doc = "`10011`"]
    #[inline]
    pub fn aport4ch24to31(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT4CH24TO31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INPUT8TO15SEL`"]
pub enum INPUT8TO15SELW {
    #[doc = "`0`"]
    APORT0CH0TO7,
    #[doc = "`1`"]
    APORT0CH8TO15,
    #[doc = "`100`"]
    APORT1CH0TO7,
    #[doc = "`101`"]
    APORT1CH8TO15,
    #[doc = "`110`"]
    APORT1CH16TO23,
    #[doc = "`111`"]
    APORT1CH24TO31,
    #[doc = "`1000`"]
    APORT2CH0TO7,
    #[doc = "`1001`"]
    APORT2CH8TO15,
    #[doc = "`1010`"]
    APORT2CH16TO23,
    #[doc = "`1011`"]
    APORT2CH24TO31,
    #[doc = "`1100`"]
    APORT3CH0TO7,
    #[doc = "`1101`"]
    APORT3CH8TO15,
    #[doc = "`1110`"]
    APORT3CH16TO23,
    #[doc = "`1111`"]
    APORT3CH24TO31,
    #[doc = "`10000`"]
    APORT4CH0TO7,
    #[doc = "`10001`"]
    APORT4CH8TO15,
    #[doc = "`10010`"]
    APORT4CH16TO23,
    #[doc = "`10011`"]
    APORT4CH24TO31,
}
impl INPUT8TO15SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPUT8TO15SELW::APORT0CH0TO7 => 0,
            INPUT8TO15SELW::APORT0CH8TO15 => 1,
            INPUT8TO15SELW::APORT1CH0TO7 => 4,
            INPUT8TO15SELW::APORT1CH8TO15 => 5,
            INPUT8TO15SELW::APORT1CH16TO23 => 6,
            INPUT8TO15SELW::APORT1CH24TO31 => 7,
            INPUT8TO15SELW::APORT2CH0TO7 => 8,
            INPUT8TO15SELW::APORT2CH8TO15 => 9,
            INPUT8TO15SELW::APORT2CH16TO23 => 10,
            INPUT8TO15SELW::APORT2CH24TO31 => 11,
            INPUT8TO15SELW::APORT3CH0TO7 => 12,
            INPUT8TO15SELW::APORT3CH8TO15 => 13,
            INPUT8TO15SELW::APORT3CH16TO23 => 14,
            INPUT8TO15SELW::APORT3CH24TO31 => 15,
            INPUT8TO15SELW::APORT4CH0TO7 => 16,
            INPUT8TO15SELW::APORT4CH8TO15 => 17,
            INPUT8TO15SELW::APORT4CH16TO23 => 18,
            INPUT8TO15SELW::APORT4CH24TO31 => 19,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUT8TO15SELW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT8TO15SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUT8TO15SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn aport0ch0to7(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT0CH0TO7)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn aport0ch8to15(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT0CH8TO15)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT1CH0TO7)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT1CH8TO15)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT1CH16TO23)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT1CH24TO31)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn aport2ch0to7(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT2CH0TO7)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn aport2ch8to15(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT2CH8TO15)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn aport2ch16to23(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT2CH16TO23)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn aport2ch24to31(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT2CH24TO31)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT3CH0TO7)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT3CH8TO15)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT3CH16TO23)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT3CH24TO31)
    }
    #[doc = "`10000`"]
    #[inline]
    pub fn aport4ch0to7(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT4CH0TO7)
    }
    #[doc = "`10001`"]
    #[inline]
    pub fn aport4ch8to15(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT4CH8TO15)
    }
    #[doc = "`10010`"]
    #[inline]
    pub fn aport4ch16to23(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT4CH16TO23)
    }
    #[doc = "`10011`"]
    #[inline]
    pub fn aport4ch24to31(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT4CH24TO31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INPUT16TO23SEL`"]
pub enum INPUT16TO23SELW {
    #[doc = "`0`"]
    APORT0CH0TO7,
    #[doc = "`1`"]
    APORT0CH8TO15,
    #[doc = "`100`"]
    APORT1CH0TO7,
    #[doc = "`101`"]
    APORT1CH8TO15,
    #[doc = "`110`"]
    APORT1CH16TO23,
    #[doc = "`111`"]
    APORT1CH24TO31,
    #[doc = "`1000`"]
    APORT2CH0TO7,
    #[doc = "`1001`"]
    APORT2CH8TO15,
    #[doc = "`1010`"]
    APORT2CH16TO23,
    #[doc = "`1011`"]
    APORT2CH24TO31,
    #[doc = "`1100`"]
    APORT3CH0TO7,
    #[doc = "`1101`"]
    APORT3CH8TO15,
    #[doc = "`1110`"]
    APORT3CH16TO23,
    #[doc = "`1111`"]
    APORT3CH24TO31,
    #[doc = "`10000`"]
    APORT4CH0TO7,
    #[doc = "`10001`"]
    APORT4CH8TO15,
    #[doc = "`10010`"]
    APORT4CH16TO23,
    #[doc = "`10011`"]
    APORT4CH24TO31,
}
impl INPUT16TO23SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPUT16TO23SELW::APORT0CH0TO7 => 0,
            INPUT16TO23SELW::APORT0CH8TO15 => 1,
            INPUT16TO23SELW::APORT1CH0TO7 => 4,
            INPUT16TO23SELW::APORT1CH8TO15 => 5,
            INPUT16TO23SELW::APORT1CH16TO23 => 6,
            INPUT16TO23SELW::APORT1CH24TO31 => 7,
            INPUT16TO23SELW::APORT2CH0TO7 => 8,
            INPUT16TO23SELW::APORT2CH8TO15 => 9,
            INPUT16TO23SELW::APORT2CH16TO23 => 10,
            INPUT16TO23SELW::APORT2CH24TO31 => 11,
            INPUT16TO23SELW::APORT3CH0TO7 => 12,
            INPUT16TO23SELW::APORT3CH8TO15 => 13,
            INPUT16TO23SELW::APORT3CH16TO23 => 14,
            INPUT16TO23SELW::APORT3CH24TO31 => 15,
            INPUT16TO23SELW::APORT4CH0TO7 => 16,
            INPUT16TO23SELW::APORT4CH8TO15 => 17,
            INPUT16TO23SELW::APORT4CH16TO23 => 18,
            INPUT16TO23SELW::APORT4CH24TO31 => 19,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUT16TO23SELW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT16TO23SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUT16TO23SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn aport0ch0to7(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT0CH0TO7)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn aport0ch8to15(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT0CH8TO15)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT1CH0TO7)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT1CH8TO15)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT1CH16TO23)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT1CH24TO31)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn aport2ch0to7(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT2CH0TO7)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn aport2ch8to15(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT2CH8TO15)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn aport2ch16to23(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT2CH16TO23)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn aport2ch24to31(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT2CH24TO31)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT3CH0TO7)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT3CH8TO15)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT3CH16TO23)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT3CH24TO31)
    }
    #[doc = "`10000`"]
    #[inline]
    pub fn aport4ch0to7(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT4CH0TO7)
    }
    #[doc = "`10001`"]
    #[inline]
    pub fn aport4ch8to15(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT4CH8TO15)
    }
    #[doc = "`10010`"]
    #[inline]
    pub fn aport4ch16to23(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT4CH16TO23)
    }
    #[doc = "`10011`"]
    #[inline]
    pub fn aport4ch24to31(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT4CH24TO31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INPUT24TO31SEL`"]
pub enum INPUT24TO31SELW {
    #[doc = "`0`"]
    APORT0CH0TO7,
    #[doc = "`1`"]
    APORT0CH8TO15,
    #[doc = "`100`"]
    APORT1CH0TO7,
    #[doc = "`101`"]
    APORT1CH8TO15,
    #[doc = "`110`"]
    APORT1CH16TO23,
    #[doc = "`111`"]
    APORT1CH24TO31,
    #[doc = "`1000`"]
    APORT2CH0TO7,
    #[doc = "`1001`"]
    APORT2CH8TO15,
    #[doc = "`1010`"]
    APORT2CH16TO23,
    #[doc = "`1011`"]
    APORT2CH24TO31,
    #[doc = "`1100`"]
    APORT3CH0TO7,
    #[doc = "`1101`"]
    APORT3CH8TO15,
    #[doc = "`1110`"]
    APORT3CH16TO23,
    #[doc = "`1111`"]
    APORT3CH24TO31,
    #[doc = "`10000`"]
    APORT4CH0TO7,
    #[doc = "`10001`"]
    APORT4CH8TO15,
    #[doc = "`10010`"]
    APORT4CH16TO23,
    #[doc = "`10011`"]
    APORT4CH24TO31,
}
impl INPUT24TO31SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPUT24TO31SELW::APORT0CH0TO7 => 0,
            INPUT24TO31SELW::APORT0CH8TO15 => 1,
            INPUT24TO31SELW::APORT1CH0TO7 => 4,
            INPUT24TO31SELW::APORT1CH8TO15 => 5,
            INPUT24TO31SELW::APORT1CH16TO23 => 6,
            INPUT24TO31SELW::APORT1CH24TO31 => 7,
            INPUT24TO31SELW::APORT2CH0TO7 => 8,
            INPUT24TO31SELW::APORT2CH8TO15 => 9,
            INPUT24TO31SELW::APORT2CH16TO23 => 10,
            INPUT24TO31SELW::APORT2CH24TO31 => 11,
            INPUT24TO31SELW::APORT3CH0TO7 => 12,
            INPUT24TO31SELW::APORT3CH8TO15 => 13,
            INPUT24TO31SELW::APORT3CH16TO23 => 14,
            INPUT24TO31SELW::APORT3CH24TO31 => 15,
            INPUT24TO31SELW::APORT4CH0TO7 => 16,
            INPUT24TO31SELW::APORT4CH8TO15 => 17,
            INPUT24TO31SELW::APORT4CH16TO23 => 18,
            INPUT24TO31SELW::APORT4CH24TO31 => 19,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUT24TO31SELW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT24TO31SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUT24TO31SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn aport0ch0to7(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT0CH0TO7)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn aport0ch8to15(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT0CH8TO15)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT1CH0TO7)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT1CH8TO15)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT1CH16TO23)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT1CH24TO31)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn aport2ch0to7(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT2CH0TO7)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn aport2ch8to15(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT2CH8TO15)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn aport2ch16to23(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT2CH16TO23)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn aport2ch24to31(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT2CH24TO31)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT3CH0TO7)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT3CH8TO15)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT3CH16TO23)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT3CH24TO31)
    }
    #[doc = "`10000`"]
    #[inline]
    pub fn aport4ch0to7(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT4CH0TO7)
    }
    #[doc = "`10001`"]
    #[inline]
    pub fn aport4ch8to15(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT4CH8TO15)
    }
    #[doc = "`10010`"]
    #[inline]
    pub fn aport4ch16to23(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT4CH16TO23)
    }
    #[doc = "`10011`"]
    #[inline]
    pub fn aport4ch24to31(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT4CH24TO31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
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
    #[doc = "Bits 0:4 - Inputs Chosen for ADCn_INPUT7-ADCn_INPUT0 as Referred in SCANMASK"]
    #[inline]
    pub fn input0to7sel(&self) -> INPUT0TO7SELR {
        INPUT0TO7SELR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:12 - Inputs Chosen for ADCn_INPUT8-ADCn_INPUT15 as Referred in SCANMASK"]
    #[inline]
    pub fn input8to15sel(&self) -> INPUT8TO15SELR {
        INPUT8TO15SELR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:20 - Inputs Chosen for ADCn_INPUT16-ADCn_INPUT23 as Referred in SCANMASK"]
    #[inline]
    pub fn input16to23sel(&self) -> INPUT16TO23SELR {
        INPUT16TO23SELR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:28 - Inputs Chosen for ADCn_INPUT24-ADCn_INPUT31 as Referred in SCANMASK"]
    #[inline]
    pub fn input24to31sel(&self) -> INPUT24TO31SELR {
        INPUT24TO31SELR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:4 - Inputs Chosen for ADCn_INPUT7-ADCn_INPUT0 as Referred in SCANMASK"]
    #[inline]
    pub fn input0to7sel(&mut self) -> _INPUT0TO7SELW {
        _INPUT0TO7SELW { w: self }
    }
    #[doc = "Bits 8:12 - Inputs Chosen for ADCn_INPUT8-ADCn_INPUT15 as Referred in SCANMASK"]
    #[inline]
    pub fn input8to15sel(&mut self) -> _INPUT8TO15SELW {
        _INPUT8TO15SELW { w: self }
    }
    #[doc = "Bits 16:20 - Inputs Chosen for ADCn_INPUT16-ADCn_INPUT23 as Referred in SCANMASK"]
    #[inline]
    pub fn input16to23sel(&mut self) -> _INPUT16TO23SELW {
        _INPUT16TO23SELW { w: self }
    }
    #[doc = "Bits 24:28 - Inputs Chosen for ADCn_INPUT24-ADCn_INPUT31 as Referred in SCANMASK"]
    #[inline]
    pub fn input24to31sel(&mut self) -> _INPUT24TO31SELW {
        _INPUT24TO31SELW { w: self }
    }
}
