#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CH0_CTRL {
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
pub struct SIGSELR {
    bits: u8,
}
impl SIGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SOURCESEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOURCESELR {
    #[doc = "No source selected"]
    NONE,
    #[doc = "Peripheral Reflex System"]
    PRSL,
    #[doc = "Peripheral Reflex System"]
    PRSH,
    #[doc = "Analog Comparator 0"]
    ACMP0,
    #[doc = "Analog Comparator 1"]
    ACMP1,
    #[doc = "Analog to Digital Converter 0"]
    ADC0,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    USART0,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    USART1,
    #[doc = "Timer 0"]
    TIMER0,
    #[doc = "Timer 1"]
    TIMER1,
    #[doc = "Real-Time Counter and Calendar"]
    RTCC,
    #[doc = "General purpose Input/Output"]
    GPIOL,
    #[doc = "General purpose Input/Output"]
    GPIOH,
    #[doc = "Low Energy Timer 0"]
    LETIMER0,
    #[doc = "Pulse Counter 0"]
    PCNT0,
    #[doc = "CryoTimer"]
    CRYOTIMER,
    #[doc = "Clock Management Unit"]
    CMU,
    #[doc = "undocumented"]
    CM4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SOURCESELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SOURCESELR::NONE => 0,
            SOURCESELR::PRSL => 1,
            SOURCESELR::PRSH => 2,
            SOURCESELR::ACMP0 => 6,
            SOURCESELR::ACMP1 => 7,
            SOURCESELR::ADC0 => 8,
            SOURCESELR::USART0 => 16,
            SOURCESELR::USART1 => 17,
            SOURCESELR::TIMER0 => 28,
            SOURCESELR::TIMER1 => 29,
            SOURCESELR::RTCC => 41,
            SOURCESELR::GPIOL => 48,
            SOURCESELR::GPIOH => 49,
            SOURCESELR::LETIMER0 => 52,
            SOURCESELR::PCNT0 => 54,
            SOURCESELR::CRYOTIMER => 60,
            SOURCESELR::CMU => 61,
            SOURCESELR::CM4 => 67,
            SOURCESELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SOURCESELR {
        match value {
            0 => SOURCESELR::NONE,
            1 => SOURCESELR::PRSL,
            2 => SOURCESELR::PRSH,
            6 => SOURCESELR::ACMP0,
            7 => SOURCESELR::ACMP1,
            8 => SOURCESELR::ADC0,
            16 => SOURCESELR::USART0,
            17 => SOURCESELR::USART1,
            28 => SOURCESELR::TIMER0,
            29 => SOURCESELR::TIMER1,
            41 => SOURCESELR::RTCC,
            48 => SOURCESELR::GPIOL,
            49 => SOURCESELR::GPIOH,
            52 => SOURCESELR::LETIMER0,
            54 => SOURCESELR::PCNT0,
            60 => SOURCESELR::CRYOTIMER,
            61 => SOURCESELR::CMU,
            67 => SOURCESELR::CM4,
            i => SOURCESELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SOURCESELR::NONE
    }
    #[doc = "Checks if the value of the field is `PRSL`"]
    #[inline]
    pub fn is_prsl(&self) -> bool {
        *self == SOURCESELR::PRSL
    }
    #[doc = "Checks if the value of the field is `PRSH`"]
    #[inline]
    pub fn is_prsh(&self) -> bool {
        *self == SOURCESELR::PRSH
    }
    #[doc = "Checks if the value of the field is `ACMP0`"]
    #[inline]
    pub fn is_acmp0(&self) -> bool {
        *self == SOURCESELR::ACMP0
    }
    #[doc = "Checks if the value of the field is `ACMP1`"]
    #[inline]
    pub fn is_acmp1(&self) -> bool {
        *self == SOURCESELR::ACMP1
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline]
    pub fn is_adc0(&self) -> bool {
        *self == SOURCESELR::ADC0
    }
    #[doc = "Checks if the value of the field is `USART0`"]
    #[inline]
    pub fn is_usart0(&self) -> bool {
        *self == SOURCESELR::USART0
    }
    #[doc = "Checks if the value of the field is `USART1`"]
    #[inline]
    pub fn is_usart1(&self) -> bool {
        *self == SOURCESELR::USART1
    }
    #[doc = "Checks if the value of the field is `TIMER0`"]
    #[inline]
    pub fn is_timer0(&self) -> bool {
        *self == SOURCESELR::TIMER0
    }
    #[doc = "Checks if the value of the field is `TIMER1`"]
    #[inline]
    pub fn is_timer1(&self) -> bool {
        *self == SOURCESELR::TIMER1
    }
    #[doc = "Checks if the value of the field is `RTCC`"]
    #[inline]
    pub fn is_rtcc(&self) -> bool {
        *self == SOURCESELR::RTCC
    }
    #[doc = "Checks if the value of the field is `GPIOL`"]
    #[inline]
    pub fn is_gpiol(&self) -> bool {
        *self == SOURCESELR::GPIOL
    }
    #[doc = "Checks if the value of the field is `GPIOH`"]
    #[inline]
    pub fn is_gpioh(&self) -> bool {
        *self == SOURCESELR::GPIOH
    }
    #[doc = "Checks if the value of the field is `LETIMER0`"]
    #[inline]
    pub fn is_letimer0(&self) -> bool {
        *self == SOURCESELR::LETIMER0
    }
    #[doc = "Checks if the value of the field is `PCNT0`"]
    #[inline]
    pub fn is_pcnt0(&self) -> bool {
        *self == SOURCESELR::PCNT0
    }
    #[doc = "Checks if the value of the field is `CRYOTIMER`"]
    #[inline]
    pub fn is_cryotimer(&self) -> bool {
        *self == SOURCESELR::CRYOTIMER
    }
    #[doc = "Checks if the value of the field is `CMU`"]
    #[inline]
    pub fn is_cmu(&self) -> bool {
        *self == SOURCESELR::CMU
    }
    #[doc = "Checks if the value of the field is `CM4`"]
    #[inline]
    pub fn is_cm4(&self) -> bool {
        *self == SOURCESELR::CM4
    }
}
#[doc = "Possible values of the field `EDSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDSELR {
    #[doc = "Signal is left as it is"]
    OFF,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal"]
    POSEDGE,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    NEGEDGE,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal"]
    BOTHEDGES,
}
impl EDSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EDSELR::OFF => 0,
            EDSELR::POSEDGE => 1,
            EDSELR::NEGEDGE => 2,
            EDSELR::BOTHEDGES => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EDSELR {
        match value {
            0 => EDSELR::OFF,
            1 => EDSELR::POSEDGE,
            2 => EDSELR::NEGEDGE,
            3 => EDSELR::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == EDSELR::OFF
    }
    #[doc = "Checks if the value of the field is `POSEDGE`"]
    #[inline]
    pub fn is_posedge(&self) -> bool {
        *self == EDSELR::POSEDGE
    }
    #[doc = "Checks if the value of the field is `NEGEDGE`"]
    #[inline]
    pub fn is_negedge(&self) -> bool {
        *self == EDSELR::NEGEDGE
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline]
    pub fn is_bothedges(&self) -> bool {
        *self == EDSELR::BOTHEDGES
    }
}
#[doc = r" Value of the field"]
pub struct STRETCHR {
    bits: bool,
}
impl STRETCHR {
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
pub struct INVR {
    bits: bool,
}
impl INVR {
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
pub struct ORPREVR {
    bits: bool,
}
impl ORPREVR {
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
pub struct ANDNEXTR {
    bits: bool,
}
impl ANDNEXTR {
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
pub struct ASYNCR {
    bits: bool,
}
impl ASYNCR {
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
pub struct _SIGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SIGSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SOURCESEL`"]
pub enum SOURCESELW {
    #[doc = "No source selected"]
    NONE,
    #[doc = "Peripheral Reflex System"]
    PRSL,
    #[doc = "Peripheral Reflex System"]
    PRSH,
    #[doc = "Analog Comparator 0"]
    ACMP0,
    #[doc = "Analog Comparator 1"]
    ACMP1,
    #[doc = "Analog to Digital Converter 0"]
    ADC0,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    USART0,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    USART1,
    #[doc = "Timer 0"]
    TIMER0,
    #[doc = "Timer 1"]
    TIMER1,
    #[doc = "Real-Time Counter and Calendar"]
    RTCC,
    #[doc = "General purpose Input/Output"]
    GPIOL,
    #[doc = "General purpose Input/Output"]
    GPIOH,
    #[doc = "Low Energy Timer 0"]
    LETIMER0,
    #[doc = "Pulse Counter 0"]
    PCNT0,
    #[doc = "CryoTimer"]
    CRYOTIMER,
    #[doc = "Clock Management Unit"]
    CMU,
    #[doc = "`1000011`"]
    CM4,
}
impl SOURCESELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SOURCESELW::NONE => 0,
            SOURCESELW::PRSL => 1,
            SOURCESELW::PRSH => 2,
            SOURCESELW::ACMP0 => 6,
            SOURCESELW::ACMP1 => 7,
            SOURCESELW::ADC0 => 8,
            SOURCESELW::USART0 => 16,
            SOURCESELW::USART1 => 17,
            SOURCESELW::TIMER0 => 28,
            SOURCESELW::TIMER1 => 29,
            SOURCESELW::RTCC => 41,
            SOURCESELW::GPIOL => 48,
            SOURCESELW::GPIOH => 49,
            SOURCESELW::LETIMER0 => 52,
            SOURCESELW::PCNT0 => 54,
            SOURCESELW::CRYOTIMER => 60,
            SOURCESELW::CMU => 61,
            SOURCESELW::CM4 => 67,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOURCESELW<'a> {
    w: &'a mut W,
}
impl<'a> _SOURCESELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOURCESELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No source selected"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SOURCESELW::NONE)
    }
    #[doc = "Peripheral Reflex System"]
    #[inline]
    pub fn prsl(self) -> &'a mut W {
        self.variant(SOURCESELW::PRSL)
    }
    #[doc = "Peripheral Reflex System"]
    #[inline]
    pub fn prsh(self) -> &'a mut W {
        self.variant(SOURCESELW::PRSH)
    }
    #[doc = "Analog Comparator 0"]
    #[inline]
    pub fn acmp0(self) -> &'a mut W {
        self.variant(SOURCESELW::ACMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline]
    pub fn acmp1(self) -> &'a mut W {
        self.variant(SOURCESELW::ACMP1)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline]
    pub fn adc0(self) -> &'a mut W {
        self.variant(SOURCESELW::ADC0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    #[inline]
    pub fn usart0(self) -> &'a mut W {
        self.variant(SOURCESELW::USART0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    #[inline]
    pub fn usart1(self) -> &'a mut W {
        self.variant(SOURCESELW::USART1)
    }
    #[doc = "Timer 0"]
    #[inline]
    pub fn timer0(self) -> &'a mut W {
        self.variant(SOURCESELW::TIMER0)
    }
    #[doc = "Timer 1"]
    #[inline]
    pub fn timer1(self) -> &'a mut W {
        self.variant(SOURCESELW::TIMER1)
    }
    #[doc = "Real-Time Counter and Calendar"]
    #[inline]
    pub fn rtcc(self) -> &'a mut W {
        self.variant(SOURCESELW::RTCC)
    }
    #[doc = "General purpose Input/Output"]
    #[inline]
    pub fn gpiol(self) -> &'a mut W {
        self.variant(SOURCESELW::GPIOL)
    }
    #[doc = "General purpose Input/Output"]
    #[inline]
    pub fn gpioh(self) -> &'a mut W {
        self.variant(SOURCESELW::GPIOH)
    }
    #[doc = "Low Energy Timer 0"]
    #[inline]
    pub fn letimer0(self) -> &'a mut W {
        self.variant(SOURCESELW::LETIMER0)
    }
    #[doc = "Pulse Counter 0"]
    #[inline]
    pub fn pcnt0(self) -> &'a mut W {
        self.variant(SOURCESELW::PCNT0)
    }
    #[doc = "CryoTimer"]
    #[inline]
    pub fn cryotimer(self) -> &'a mut W {
        self.variant(SOURCESELW::CRYOTIMER)
    }
    #[doc = "Clock Management Unit"]
    #[inline]
    pub fn cmu(self) -> &'a mut W {
        self.variant(SOURCESELW::CMU)
    }
    #[doc = "`1000011`"]
    #[inline]
    pub fn cm4(self) -> &'a mut W {
        self.variant(SOURCESELW::CM4)
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
#[doc = "Values that can be written to the field `EDSEL`"]
pub enum EDSELW {
    #[doc = "Signal is left as it is"]
    OFF,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal"]
    POSEDGE,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    NEGEDGE,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal"]
    BOTHEDGES,
}
impl EDSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EDSELW::OFF => 0,
            EDSELW::POSEDGE => 1,
            EDSELW::NEGEDGE => 2,
            EDSELW::BOTHEDGES => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDSELW<'a> {
    w: &'a mut W,
}
impl<'a> _EDSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Signal is left as it is"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(EDSELW::OFF)
    }
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal"]
    #[inline]
    pub fn posedge(self) -> &'a mut W {
        self.variant(EDSELW::POSEDGE)
    }
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    #[inline]
    pub fn negedge(self) -> &'a mut W {
        self.variant(EDSELW::NEGEDGE)
    }
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal"]
    #[inline]
    pub fn bothedges(self) -> &'a mut W {
        self.variant(EDSELW::BOTHEDGES)
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
#[doc = r" Proxy"]
pub struct _STRETCHW<'a> {
    w: &'a mut W,
}
impl<'a> _STRETCHW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INVW<'a> {
    w: &'a mut W,
}
impl<'a> _INVW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ORPREVW<'a> {
    w: &'a mut W,
}
impl<'a> _ORPREVW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ANDNEXTW<'a> {
    w: &'a mut W,
}
impl<'a> _ANDNEXTW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ASYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _ASYNCW<'a> {
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
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline]
    pub fn sigsel(&self) -> SIGSELR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SIGSELR { bits }
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline]
    pub fn sourcesel(&self) -> SOURCESELR {
        SOURCESELR::_from({
            const MASK: u8 = 127;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Edge Detect Select"]
    #[inline]
    pub fn edsel(&self) -> EDSELR {
        EDSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 25 - Stretch Channel Output"]
    #[inline]
    pub fn stretch(&self) -> STRETCHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STRETCHR { bits }
    }
    #[doc = "Bit 26 - Invert Channel"]
    #[inline]
    pub fn inv(&self) -> INVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INVR { bits }
    }
    #[doc = "Bit 27 - Or Previous"]
    #[inline]
    pub fn orprev(&self) -> ORPREVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ORPREVR { bits }
    }
    #[doc = "Bit 28 - And Next"]
    #[inline]
    pub fn andnext(&self) -> ANDNEXTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ANDNEXTR { bits }
    }
    #[doc = "Bit 30 - Asynchronous Reflex"]
    #[inline]
    pub fn async_(&self) -> ASYNCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ASYNCR { bits }
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
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline]
    pub fn sigsel(&mut self) -> _SIGSELW {
        _SIGSELW { w: self }
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline]
    pub fn sourcesel(&mut self) -> _SOURCESELW {
        _SOURCESELW { w: self }
    }
    #[doc = "Bits 20:21 - Edge Detect Select"]
    #[inline]
    pub fn edsel(&mut self) -> _EDSELW {
        _EDSELW { w: self }
    }
    #[doc = "Bit 25 - Stretch Channel Output"]
    #[inline]
    pub fn stretch(&mut self) -> _STRETCHW {
        _STRETCHW { w: self }
    }
    #[doc = "Bit 26 - Invert Channel"]
    #[inline]
    pub fn inv(&mut self) -> _INVW {
        _INVW { w: self }
    }
    #[doc = "Bit 27 - Or Previous"]
    #[inline]
    pub fn orprev(&mut self) -> _ORPREVW {
        _ORPREVW { w: self }
    }
    #[doc = "Bit 28 - And Next"]
    #[inline]
    pub fn andnext(&mut self) -> _ANDNEXTW {
        _ANDNEXTW { w: self }
    }
    #[doc = "Bit 30 - Asynchronous Reflex"]
    #[inline]
    pub fn async_(&mut self) -> _ASYNCW {
        _ASYNCW { w: self }
    }
}
