#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PA_MODEL {
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
#[doc = "Possible values of the field `MODE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE0R {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull using alternate control"]
    PUSHPULLALT,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output using alternate control"]
    WIREDANDALT,
    #[doc = "Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER,
    #[doc = "Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP,
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER,
}
impl MODE0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODE0R::DISABLED => 0,
            MODE0R::INPUT => 1,
            MODE0R::INPUTPULL => 2,
            MODE0R::INPUTPULLFILTER => 3,
            MODE0R::PUSHPULL => 4,
            MODE0R::PUSHPULLALT => 5,
            MODE0R::WIREDOR => 6,
            MODE0R::WIREDORPULLDOWN => 7,
            MODE0R::WIREDAND => 8,
            MODE0R::WIREDANDFILTER => 9,
            MODE0R::WIREDANDPULLUP => 10,
            MODE0R::WIREDANDPULLUPFILTER => 11,
            MODE0R::WIREDANDALT => 12,
            MODE0R::WIREDANDALTFILTER => 13,
            MODE0R::WIREDANDALTPULLUP => 14,
            MODE0R::WIREDANDALTPULLUPFILTER => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODE0R {
        match value {
            0 => MODE0R::DISABLED,
            1 => MODE0R::INPUT,
            2 => MODE0R::INPUTPULL,
            3 => MODE0R::INPUTPULLFILTER,
            4 => MODE0R::PUSHPULL,
            5 => MODE0R::PUSHPULLALT,
            6 => MODE0R::WIREDOR,
            7 => MODE0R::WIREDORPULLDOWN,
            8 => MODE0R::WIREDAND,
            9 => MODE0R::WIREDANDFILTER,
            10 => MODE0R::WIREDANDPULLUP,
            11 => MODE0R::WIREDANDPULLUPFILTER,
            12 => MODE0R::WIREDANDALT,
            13 => MODE0R::WIREDANDALTFILTER,
            14 => MODE0R::WIREDANDALTPULLUP,
            15 => MODE0R::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MODE0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == MODE0R::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE0R::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE0R::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE0R::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE0R::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE0R::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE0R::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE0R::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE0R::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE0R::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE0R::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE0R::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE0R::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE0R::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE0R::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Possible values of the field `MODE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE1R {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull using alternate control"]
    PUSHPULLALT,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output using alternate control"]
    WIREDANDALT,
    #[doc = "Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER,
    #[doc = "Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP,
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER,
}
impl MODE1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODE1R::DISABLED => 0,
            MODE1R::INPUT => 1,
            MODE1R::INPUTPULL => 2,
            MODE1R::INPUTPULLFILTER => 3,
            MODE1R::PUSHPULL => 4,
            MODE1R::PUSHPULLALT => 5,
            MODE1R::WIREDOR => 6,
            MODE1R::WIREDORPULLDOWN => 7,
            MODE1R::WIREDAND => 8,
            MODE1R::WIREDANDFILTER => 9,
            MODE1R::WIREDANDPULLUP => 10,
            MODE1R::WIREDANDPULLUPFILTER => 11,
            MODE1R::WIREDANDALT => 12,
            MODE1R::WIREDANDALTFILTER => 13,
            MODE1R::WIREDANDALTPULLUP => 14,
            MODE1R::WIREDANDALTPULLUPFILTER => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODE1R {
        match value {
            0 => MODE1R::DISABLED,
            1 => MODE1R::INPUT,
            2 => MODE1R::INPUTPULL,
            3 => MODE1R::INPUTPULLFILTER,
            4 => MODE1R::PUSHPULL,
            5 => MODE1R::PUSHPULLALT,
            6 => MODE1R::WIREDOR,
            7 => MODE1R::WIREDORPULLDOWN,
            8 => MODE1R::WIREDAND,
            9 => MODE1R::WIREDANDFILTER,
            10 => MODE1R::WIREDANDPULLUP,
            11 => MODE1R::WIREDANDPULLUPFILTER,
            12 => MODE1R::WIREDANDALT,
            13 => MODE1R::WIREDANDALTFILTER,
            14 => MODE1R::WIREDANDALTPULLUP,
            15 => MODE1R::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MODE1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == MODE1R::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE1R::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE1R::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE1R::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE1R::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE1R::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE1R::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE1R::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE1R::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE1R::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE1R::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE1R::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE1R::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE1R::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE1R::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Possible values of the field `MODE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE2R {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull using alternate control"]
    PUSHPULLALT,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output using alternate control"]
    WIREDANDALT,
    #[doc = "Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER,
    #[doc = "Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP,
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER,
}
impl MODE2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODE2R::DISABLED => 0,
            MODE2R::INPUT => 1,
            MODE2R::INPUTPULL => 2,
            MODE2R::INPUTPULLFILTER => 3,
            MODE2R::PUSHPULL => 4,
            MODE2R::PUSHPULLALT => 5,
            MODE2R::WIREDOR => 6,
            MODE2R::WIREDORPULLDOWN => 7,
            MODE2R::WIREDAND => 8,
            MODE2R::WIREDANDFILTER => 9,
            MODE2R::WIREDANDPULLUP => 10,
            MODE2R::WIREDANDPULLUPFILTER => 11,
            MODE2R::WIREDANDALT => 12,
            MODE2R::WIREDANDALTFILTER => 13,
            MODE2R::WIREDANDALTPULLUP => 14,
            MODE2R::WIREDANDALTPULLUPFILTER => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODE2R {
        match value {
            0 => MODE2R::DISABLED,
            1 => MODE2R::INPUT,
            2 => MODE2R::INPUTPULL,
            3 => MODE2R::INPUTPULLFILTER,
            4 => MODE2R::PUSHPULL,
            5 => MODE2R::PUSHPULLALT,
            6 => MODE2R::WIREDOR,
            7 => MODE2R::WIREDORPULLDOWN,
            8 => MODE2R::WIREDAND,
            9 => MODE2R::WIREDANDFILTER,
            10 => MODE2R::WIREDANDPULLUP,
            11 => MODE2R::WIREDANDPULLUPFILTER,
            12 => MODE2R::WIREDANDALT,
            13 => MODE2R::WIREDANDALTFILTER,
            14 => MODE2R::WIREDANDALTPULLUP,
            15 => MODE2R::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MODE2R::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == MODE2R::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE2R::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE2R::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE2R::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE2R::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE2R::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE2R::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE2R::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE2R::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE2R::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE2R::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE2R::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE2R::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE2R::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE2R::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Possible values of the field `MODE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE3R {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull using alternate control"]
    PUSHPULLALT,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output using alternate control"]
    WIREDANDALT,
    #[doc = "Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER,
    #[doc = "Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP,
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER,
}
impl MODE3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODE3R::DISABLED => 0,
            MODE3R::INPUT => 1,
            MODE3R::INPUTPULL => 2,
            MODE3R::INPUTPULLFILTER => 3,
            MODE3R::PUSHPULL => 4,
            MODE3R::PUSHPULLALT => 5,
            MODE3R::WIREDOR => 6,
            MODE3R::WIREDORPULLDOWN => 7,
            MODE3R::WIREDAND => 8,
            MODE3R::WIREDANDFILTER => 9,
            MODE3R::WIREDANDPULLUP => 10,
            MODE3R::WIREDANDPULLUPFILTER => 11,
            MODE3R::WIREDANDALT => 12,
            MODE3R::WIREDANDALTFILTER => 13,
            MODE3R::WIREDANDALTPULLUP => 14,
            MODE3R::WIREDANDALTPULLUPFILTER => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODE3R {
        match value {
            0 => MODE3R::DISABLED,
            1 => MODE3R::INPUT,
            2 => MODE3R::INPUTPULL,
            3 => MODE3R::INPUTPULLFILTER,
            4 => MODE3R::PUSHPULL,
            5 => MODE3R::PUSHPULLALT,
            6 => MODE3R::WIREDOR,
            7 => MODE3R::WIREDORPULLDOWN,
            8 => MODE3R::WIREDAND,
            9 => MODE3R::WIREDANDFILTER,
            10 => MODE3R::WIREDANDPULLUP,
            11 => MODE3R::WIREDANDPULLUPFILTER,
            12 => MODE3R::WIREDANDALT,
            13 => MODE3R::WIREDANDALTFILTER,
            14 => MODE3R::WIREDANDALTPULLUP,
            15 => MODE3R::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MODE3R::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == MODE3R::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE3R::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE3R::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE3R::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE3R::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE3R::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE3R::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE3R::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE3R::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE3R::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE3R::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE3R::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE3R::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE3R::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE3R::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Possible values of the field `MODE4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE4R {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull using alternate control"]
    PUSHPULLALT,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output using alternate control"]
    WIREDANDALT,
    #[doc = "Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER,
    #[doc = "Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP,
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER,
}
impl MODE4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODE4R::DISABLED => 0,
            MODE4R::INPUT => 1,
            MODE4R::INPUTPULL => 2,
            MODE4R::INPUTPULLFILTER => 3,
            MODE4R::PUSHPULL => 4,
            MODE4R::PUSHPULLALT => 5,
            MODE4R::WIREDOR => 6,
            MODE4R::WIREDORPULLDOWN => 7,
            MODE4R::WIREDAND => 8,
            MODE4R::WIREDANDFILTER => 9,
            MODE4R::WIREDANDPULLUP => 10,
            MODE4R::WIREDANDPULLUPFILTER => 11,
            MODE4R::WIREDANDALT => 12,
            MODE4R::WIREDANDALTFILTER => 13,
            MODE4R::WIREDANDALTPULLUP => 14,
            MODE4R::WIREDANDALTPULLUPFILTER => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODE4R {
        match value {
            0 => MODE4R::DISABLED,
            1 => MODE4R::INPUT,
            2 => MODE4R::INPUTPULL,
            3 => MODE4R::INPUTPULLFILTER,
            4 => MODE4R::PUSHPULL,
            5 => MODE4R::PUSHPULLALT,
            6 => MODE4R::WIREDOR,
            7 => MODE4R::WIREDORPULLDOWN,
            8 => MODE4R::WIREDAND,
            9 => MODE4R::WIREDANDFILTER,
            10 => MODE4R::WIREDANDPULLUP,
            11 => MODE4R::WIREDANDPULLUPFILTER,
            12 => MODE4R::WIREDANDALT,
            13 => MODE4R::WIREDANDALTFILTER,
            14 => MODE4R::WIREDANDALTPULLUP,
            15 => MODE4R::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MODE4R::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == MODE4R::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE4R::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE4R::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE4R::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE4R::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE4R::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE4R::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE4R::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE4R::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE4R::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE4R::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE4R::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE4R::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE4R::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE4R::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Possible values of the field `MODE5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE5R {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull using alternate control"]
    PUSHPULLALT,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output using alternate control"]
    WIREDANDALT,
    #[doc = "Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER,
    #[doc = "Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP,
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER,
}
impl MODE5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODE5R::DISABLED => 0,
            MODE5R::INPUT => 1,
            MODE5R::INPUTPULL => 2,
            MODE5R::INPUTPULLFILTER => 3,
            MODE5R::PUSHPULL => 4,
            MODE5R::PUSHPULLALT => 5,
            MODE5R::WIREDOR => 6,
            MODE5R::WIREDORPULLDOWN => 7,
            MODE5R::WIREDAND => 8,
            MODE5R::WIREDANDFILTER => 9,
            MODE5R::WIREDANDPULLUP => 10,
            MODE5R::WIREDANDPULLUPFILTER => 11,
            MODE5R::WIREDANDALT => 12,
            MODE5R::WIREDANDALTFILTER => 13,
            MODE5R::WIREDANDALTPULLUP => 14,
            MODE5R::WIREDANDALTPULLUPFILTER => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODE5R {
        match value {
            0 => MODE5R::DISABLED,
            1 => MODE5R::INPUT,
            2 => MODE5R::INPUTPULL,
            3 => MODE5R::INPUTPULLFILTER,
            4 => MODE5R::PUSHPULL,
            5 => MODE5R::PUSHPULLALT,
            6 => MODE5R::WIREDOR,
            7 => MODE5R::WIREDORPULLDOWN,
            8 => MODE5R::WIREDAND,
            9 => MODE5R::WIREDANDFILTER,
            10 => MODE5R::WIREDANDPULLUP,
            11 => MODE5R::WIREDANDPULLUPFILTER,
            12 => MODE5R::WIREDANDALT,
            13 => MODE5R::WIREDANDALTFILTER,
            14 => MODE5R::WIREDANDALTPULLUP,
            15 => MODE5R::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MODE5R::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == MODE5R::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE5R::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE5R::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE5R::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE5R::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE5R::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE5R::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE5R::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE5R::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE5R::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE5R::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE5R::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE5R::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE5R::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE5R::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Possible values of the field `MODE6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE6R {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull using alternate control"]
    PUSHPULLALT,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output using alternate control"]
    WIREDANDALT,
    #[doc = "Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER,
    #[doc = "Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP,
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER,
}
impl MODE6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODE6R::DISABLED => 0,
            MODE6R::INPUT => 1,
            MODE6R::INPUTPULL => 2,
            MODE6R::INPUTPULLFILTER => 3,
            MODE6R::PUSHPULL => 4,
            MODE6R::PUSHPULLALT => 5,
            MODE6R::WIREDOR => 6,
            MODE6R::WIREDORPULLDOWN => 7,
            MODE6R::WIREDAND => 8,
            MODE6R::WIREDANDFILTER => 9,
            MODE6R::WIREDANDPULLUP => 10,
            MODE6R::WIREDANDPULLUPFILTER => 11,
            MODE6R::WIREDANDALT => 12,
            MODE6R::WIREDANDALTFILTER => 13,
            MODE6R::WIREDANDALTPULLUP => 14,
            MODE6R::WIREDANDALTPULLUPFILTER => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODE6R {
        match value {
            0 => MODE6R::DISABLED,
            1 => MODE6R::INPUT,
            2 => MODE6R::INPUTPULL,
            3 => MODE6R::INPUTPULLFILTER,
            4 => MODE6R::PUSHPULL,
            5 => MODE6R::PUSHPULLALT,
            6 => MODE6R::WIREDOR,
            7 => MODE6R::WIREDORPULLDOWN,
            8 => MODE6R::WIREDAND,
            9 => MODE6R::WIREDANDFILTER,
            10 => MODE6R::WIREDANDPULLUP,
            11 => MODE6R::WIREDANDPULLUPFILTER,
            12 => MODE6R::WIREDANDALT,
            13 => MODE6R::WIREDANDALTFILTER,
            14 => MODE6R::WIREDANDALTPULLUP,
            15 => MODE6R::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MODE6R::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == MODE6R::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE6R::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE6R::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE6R::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE6R::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE6R::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE6R::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE6R::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE6R::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE6R::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE6R::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE6R::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE6R::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE6R::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE6R::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Possible values of the field `MODE7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE7R {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull using alternate control"]
    PUSHPULLALT,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output using alternate control"]
    WIREDANDALT,
    #[doc = "Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER,
    #[doc = "Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP,
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER,
}
impl MODE7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODE7R::DISABLED => 0,
            MODE7R::INPUT => 1,
            MODE7R::INPUTPULL => 2,
            MODE7R::INPUTPULLFILTER => 3,
            MODE7R::PUSHPULL => 4,
            MODE7R::PUSHPULLALT => 5,
            MODE7R::WIREDOR => 6,
            MODE7R::WIREDORPULLDOWN => 7,
            MODE7R::WIREDAND => 8,
            MODE7R::WIREDANDFILTER => 9,
            MODE7R::WIREDANDPULLUP => 10,
            MODE7R::WIREDANDPULLUPFILTER => 11,
            MODE7R::WIREDANDALT => 12,
            MODE7R::WIREDANDALTFILTER => 13,
            MODE7R::WIREDANDALTPULLUP => 14,
            MODE7R::WIREDANDALTPULLUPFILTER => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODE7R {
        match value {
            0 => MODE7R::DISABLED,
            1 => MODE7R::INPUT,
            2 => MODE7R::INPUTPULL,
            3 => MODE7R::INPUTPULLFILTER,
            4 => MODE7R::PUSHPULL,
            5 => MODE7R::PUSHPULLALT,
            6 => MODE7R::WIREDOR,
            7 => MODE7R::WIREDORPULLDOWN,
            8 => MODE7R::WIREDAND,
            9 => MODE7R::WIREDANDFILTER,
            10 => MODE7R::WIREDANDPULLUP,
            11 => MODE7R::WIREDANDPULLUPFILTER,
            12 => MODE7R::WIREDANDALT,
            13 => MODE7R::WIREDANDALTFILTER,
            14 => MODE7R::WIREDANDALTPULLUP,
            15 => MODE7R::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MODE7R::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == MODE7R::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE7R::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE7R::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE7R::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE7R::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE7R::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE7R::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE7R::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE7R::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE7R::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE7R::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE7R::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE7R::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE7R::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE7R::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Values that can be written to the field `MODE0`"]
pub enum MODE0W {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull using alternate control"]
    PUSHPULLALT,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output using alternate control"]
    WIREDANDALT,
    #[doc = "Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER,
    #[doc = "Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP,
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER,
}
impl MODE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODE0W::DISABLED => 0,
            MODE0W::INPUT => 1,
            MODE0W::INPUTPULL => 2,
            MODE0W::INPUTPULLFILTER => 3,
            MODE0W::PUSHPULL => 4,
            MODE0W::PUSHPULLALT => 5,
            MODE0W::WIREDOR => 6,
            MODE0W::WIREDORPULLDOWN => 7,
            MODE0W::WIREDAND => 8,
            MODE0W::WIREDANDFILTER => 9,
            MODE0W::WIREDANDPULLUP => 10,
            MODE0W::WIREDANDPULLUPFILTER => 11,
            MODE0W::WIREDANDALT => 12,
            MODE0W::WIREDANDALTFILTER => 13,
            MODE0W::WIREDANDALTPULLUP => 14,
            MODE0W::WIREDANDALTPULLUPFILTER => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE0W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE0W::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE0W::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE0W::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE0W::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE0W::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE0W::PUSHPULLALT)
    }
    #[doc = "Wired-or output"]
    #[inline]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE0W::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE0W::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE0W::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE0W::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE0W::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE0W::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE0W::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE0W::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE0W::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE0W::WIREDANDALTPULLUPFILTER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE1`"]
pub enum MODE1W {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull using alternate control"]
    PUSHPULLALT,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output using alternate control"]
    WIREDANDALT,
    #[doc = "Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER,
    #[doc = "Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP,
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER,
}
impl MODE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODE1W::DISABLED => 0,
            MODE1W::INPUT => 1,
            MODE1W::INPUTPULL => 2,
            MODE1W::INPUTPULLFILTER => 3,
            MODE1W::PUSHPULL => 4,
            MODE1W::PUSHPULLALT => 5,
            MODE1W::WIREDOR => 6,
            MODE1W::WIREDORPULLDOWN => 7,
            MODE1W::WIREDAND => 8,
            MODE1W::WIREDANDFILTER => 9,
            MODE1W::WIREDANDPULLUP => 10,
            MODE1W::WIREDANDPULLUPFILTER => 11,
            MODE1W::WIREDANDALT => 12,
            MODE1W::WIREDANDALTFILTER => 13,
            MODE1W::WIREDANDALTPULLUP => 14,
            MODE1W::WIREDANDALTPULLUPFILTER => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE1W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE1W::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE1W::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE1W::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE1W::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE1W::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE1W::PUSHPULLALT)
    }
    #[doc = "Wired-or output"]
    #[inline]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE1W::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE1W::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE1W::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE1W::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE1W::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE1W::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE1W::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE1W::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE1W::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE1W::WIREDANDALTPULLUPFILTER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE2`"]
pub enum MODE2W {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull using alternate control"]
    PUSHPULLALT,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output using alternate control"]
    WIREDANDALT,
    #[doc = "Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER,
    #[doc = "Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP,
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER,
}
impl MODE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODE2W::DISABLED => 0,
            MODE2W::INPUT => 1,
            MODE2W::INPUTPULL => 2,
            MODE2W::INPUTPULLFILTER => 3,
            MODE2W::PUSHPULL => 4,
            MODE2W::PUSHPULLALT => 5,
            MODE2W::WIREDOR => 6,
            MODE2W::WIREDORPULLDOWN => 7,
            MODE2W::WIREDAND => 8,
            MODE2W::WIREDANDFILTER => 9,
            MODE2W::WIREDANDPULLUP => 10,
            MODE2W::WIREDANDPULLUPFILTER => 11,
            MODE2W::WIREDANDALT => 12,
            MODE2W::WIREDANDALTFILTER => 13,
            MODE2W::WIREDANDALTPULLUP => 14,
            MODE2W::WIREDANDALTPULLUPFILTER => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE2W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE2W::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE2W::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE2W::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE2W::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE2W::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE2W::PUSHPULLALT)
    }
    #[doc = "Wired-or output"]
    #[inline]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE2W::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE2W::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE2W::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE2W::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE2W::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE2W::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE2W::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE2W::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE2W::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE2W::WIREDANDALTPULLUPFILTER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE3`"]
pub enum MODE3W {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull using alternate control"]
    PUSHPULLALT,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output using alternate control"]
    WIREDANDALT,
    #[doc = "Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER,
    #[doc = "Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP,
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER,
}
impl MODE3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODE3W::DISABLED => 0,
            MODE3W::INPUT => 1,
            MODE3W::INPUTPULL => 2,
            MODE3W::INPUTPULLFILTER => 3,
            MODE3W::PUSHPULL => 4,
            MODE3W::PUSHPULLALT => 5,
            MODE3W::WIREDOR => 6,
            MODE3W::WIREDORPULLDOWN => 7,
            MODE3W::WIREDAND => 8,
            MODE3W::WIREDANDFILTER => 9,
            MODE3W::WIREDANDPULLUP => 10,
            MODE3W::WIREDANDPULLUPFILTER => 11,
            MODE3W::WIREDANDALT => 12,
            MODE3W::WIREDANDALTFILTER => 13,
            MODE3W::WIREDANDALTPULLUP => 14,
            MODE3W::WIREDANDALTPULLUPFILTER => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE3W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE3W::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE3W::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE3W::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE3W::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE3W::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE3W::PUSHPULLALT)
    }
    #[doc = "Wired-or output"]
    #[inline]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE3W::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE3W::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE3W::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE3W::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE3W::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE3W::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE3W::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE3W::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE3W::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE3W::WIREDANDALTPULLUPFILTER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE4`"]
pub enum MODE4W {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull using alternate control"]
    PUSHPULLALT,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output using alternate control"]
    WIREDANDALT,
    #[doc = "Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER,
    #[doc = "Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP,
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER,
}
impl MODE4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODE4W::DISABLED => 0,
            MODE4W::INPUT => 1,
            MODE4W::INPUTPULL => 2,
            MODE4W::INPUTPULLFILTER => 3,
            MODE4W::PUSHPULL => 4,
            MODE4W::PUSHPULLALT => 5,
            MODE4W::WIREDOR => 6,
            MODE4W::WIREDORPULLDOWN => 7,
            MODE4W::WIREDAND => 8,
            MODE4W::WIREDANDFILTER => 9,
            MODE4W::WIREDANDPULLUP => 10,
            MODE4W::WIREDANDPULLUPFILTER => 11,
            MODE4W::WIREDANDALT => 12,
            MODE4W::WIREDANDALTFILTER => 13,
            MODE4W::WIREDANDALTPULLUP => 14,
            MODE4W::WIREDANDALTPULLUPFILTER => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE4W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE4W::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE4W::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE4W::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE4W::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE4W::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE4W::PUSHPULLALT)
    }
    #[doc = "Wired-or output"]
    #[inline]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE4W::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE4W::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE4W::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE4W::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE4W::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE4W::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE4W::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE4W::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE4W::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE4W::WIREDANDALTPULLUPFILTER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE5`"]
pub enum MODE5W {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull using alternate control"]
    PUSHPULLALT,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output using alternate control"]
    WIREDANDALT,
    #[doc = "Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER,
    #[doc = "Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP,
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER,
}
impl MODE5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODE5W::DISABLED => 0,
            MODE5W::INPUT => 1,
            MODE5W::INPUTPULL => 2,
            MODE5W::INPUTPULLFILTER => 3,
            MODE5W::PUSHPULL => 4,
            MODE5W::PUSHPULLALT => 5,
            MODE5W::WIREDOR => 6,
            MODE5W::WIREDORPULLDOWN => 7,
            MODE5W::WIREDAND => 8,
            MODE5W::WIREDANDFILTER => 9,
            MODE5W::WIREDANDPULLUP => 10,
            MODE5W::WIREDANDPULLUPFILTER => 11,
            MODE5W::WIREDANDALT => 12,
            MODE5W::WIREDANDALTFILTER => 13,
            MODE5W::WIREDANDALTPULLUP => 14,
            MODE5W::WIREDANDALTPULLUPFILTER => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE5W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE5W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE5W::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE5W::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE5W::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE5W::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE5W::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE5W::PUSHPULLALT)
    }
    #[doc = "Wired-or output"]
    #[inline]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE5W::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE5W::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE5W::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE5W::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE5W::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE5W::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE5W::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE5W::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE5W::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE5W::WIREDANDALTPULLUPFILTER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE6`"]
pub enum MODE6W {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull using alternate control"]
    PUSHPULLALT,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output using alternate control"]
    WIREDANDALT,
    #[doc = "Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER,
    #[doc = "Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP,
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER,
}
impl MODE6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODE6W::DISABLED => 0,
            MODE6W::INPUT => 1,
            MODE6W::INPUTPULL => 2,
            MODE6W::INPUTPULLFILTER => 3,
            MODE6W::PUSHPULL => 4,
            MODE6W::PUSHPULLALT => 5,
            MODE6W::WIREDOR => 6,
            MODE6W::WIREDORPULLDOWN => 7,
            MODE6W::WIREDAND => 8,
            MODE6W::WIREDANDFILTER => 9,
            MODE6W::WIREDANDPULLUP => 10,
            MODE6W::WIREDANDPULLUPFILTER => 11,
            MODE6W::WIREDANDALT => 12,
            MODE6W::WIREDANDALTFILTER => 13,
            MODE6W::WIREDANDALTPULLUP => 14,
            MODE6W::WIREDANDALTPULLUPFILTER => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE6W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE6W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE6W::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE6W::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE6W::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE6W::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE6W::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE6W::PUSHPULLALT)
    }
    #[doc = "Wired-or output"]
    #[inline]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE6W::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE6W::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE6W::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE6W::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE6W::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE6W::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE6W::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE6W::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE6W::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE6W::WIREDANDALTPULLUPFILTER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE7`"]
pub enum MODE7W {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull using alternate control"]
    PUSHPULLALT,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output using alternate control"]
    WIREDANDALT,
    #[doc = "Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER,
    #[doc = "Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP,
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER,
}
impl MODE7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODE7W::DISABLED => 0,
            MODE7W::INPUT => 1,
            MODE7W::INPUTPULL => 2,
            MODE7W::INPUTPULLFILTER => 3,
            MODE7W::PUSHPULL => 4,
            MODE7W::PUSHPULLALT => 5,
            MODE7W::WIREDOR => 6,
            MODE7W::WIREDORPULLDOWN => 7,
            MODE7W::WIREDAND => 8,
            MODE7W::WIREDANDFILTER => 9,
            MODE7W::WIREDANDPULLUP => 10,
            MODE7W::WIREDANDPULLUPFILTER => 11,
            MODE7W::WIREDANDALT => 12,
            MODE7W::WIREDANDALTFILTER => 13,
            MODE7W::WIREDANDALTPULLUP => 14,
            MODE7W::WIREDANDALTPULLUPFILTER => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE7W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE7W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE7W::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE7W::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE7W::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE7W::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE7W::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE7W::PUSHPULLALT)
    }
    #[doc = "Wired-or output"]
    #[inline]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE7W::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE7W::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE7W::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE7W::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE7W::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE7W::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE7W::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE7W::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE7W::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE7W::WIREDANDALTPULLUPFILTER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:3 - Pin 0 Mode"]
    #[inline]
    pub fn mode0(&self) -> MODE0R {
        MODE0R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Pin 1 Mode"]
    #[inline]
    pub fn mode1(&self) -> MODE1R {
        MODE1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Pin 2 Mode"]
    #[inline]
    pub fn mode2(&self) -> MODE2R {
        MODE2R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Pin 3 Mode"]
    #[inline]
    pub fn mode3(&self) -> MODE3R {
        MODE3R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Pin 4 Mode"]
    #[inline]
    pub fn mode4(&self) -> MODE4R {
        MODE4R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Pin 5 Mode"]
    #[inline]
    pub fn mode5(&self) -> MODE5R {
        MODE5R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Pin 6 Mode"]
    #[inline]
    pub fn mode6(&self) -> MODE6R {
        MODE6R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Pin 7 Mode"]
    #[inline]
    pub fn mode7(&self) -> MODE7R {
        MODE7R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:3 - Pin 0 Mode"]
    #[inline]
    pub fn mode0(&mut self) -> _MODE0W {
        _MODE0W { w: self }
    }
    #[doc = "Bits 4:7 - Pin 1 Mode"]
    #[inline]
    pub fn mode1(&mut self) -> _MODE1W {
        _MODE1W { w: self }
    }
    #[doc = "Bits 8:11 - Pin 2 Mode"]
    #[inline]
    pub fn mode2(&mut self) -> _MODE2W {
        _MODE2W { w: self }
    }
    #[doc = "Bits 12:15 - Pin 3 Mode"]
    #[inline]
    pub fn mode3(&mut self) -> _MODE3W {
        _MODE3W { w: self }
    }
    #[doc = "Bits 16:19 - Pin 4 Mode"]
    #[inline]
    pub fn mode4(&mut self) -> _MODE4W {
        _MODE4W { w: self }
    }
    #[doc = "Bits 20:23 - Pin 5 Mode"]
    #[inline]
    pub fn mode5(&mut self) -> _MODE5W {
        _MODE5W { w: self }
    }
    #[doc = "Bits 24:27 - Pin 6 Mode"]
    #[inline]
    pub fn mode6(&mut self) -> _MODE6W {
        _MODE6W { w: self }
    }
    #[doc = "Bits 28:31 - Pin 7 Mode"]
    #[inline]
    pub fn mode7(&mut self) -> _MODE7W {
        _MODE7W { w: self }
    }
}
