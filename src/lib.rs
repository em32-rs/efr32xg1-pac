#![doc = "Peripheral access API for EFR32MG1P133F256GM48 microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn EMU();
    fn WDOG0();
    fn LDMA();
    fn GPIO_EVEN();
    fn TIMER0();
    fn USART0_RX();
    fn USART0_TX();
    fn ACMP0();
    fn ADC0();
    fn IDAC0();
    fn I2C0();
    fn GPIO_ODD();
    fn TIMER1();
    fn USART1_RX();
    fn USART1_TX();
    fn LEUART0();
    fn PCNT0();
    fn CMU();
    fn MSC();
    fn CRYPTO();
    fn LETIMER0();
    fn RTCC();
    fn CRYOTIMER();
    fn FPUEH();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 34] = [
    Vector { _handler: EMU },
    Vector { _reserved: 0 },
    Vector { _handler: WDOG0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: LDMA },
    Vector {
        _handler: GPIO_EVEN,
    },
    Vector { _handler: TIMER0 },
    Vector {
        _handler: USART0_RX,
    },
    Vector {
        _handler: USART0_TX,
    },
    Vector { _handler: ACMP0 },
    Vector { _handler: ADC0 },
    Vector { _handler: IDAC0 },
    Vector { _handler: I2C0 },
    Vector { _handler: GPIO_ODD },
    Vector { _handler: TIMER1 },
    Vector {
        _handler: USART1_RX,
    },
    Vector {
        _handler: USART1_TX,
    },
    Vector { _handler: LEUART0 },
    Vector { _handler: PCNT0 },
    Vector { _handler: CMU },
    Vector { _handler: MSC },
    Vector { _handler: CRYPTO },
    Vector { _handler: LETIMER0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: RTCC },
    Vector { _reserved: 0 },
    Vector {
        _handler: CRYOTIMER,
    },
    Vector { _reserved: 0 },
    Vector { _handler: FPUEH },
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - EMU"]
    EMU,
    #[doc = "2 - WDOG0"]
    WDOG0,
    #[doc = "8 - LDMA"]
    LDMA,
    #[doc = "9 - GPIO_EVEN"]
    GPIO_EVEN,
    #[doc = "10 - TIMER0"]
    TIMER0,
    #[doc = "11 - USART0_RX"]
    USART0_RX,
    #[doc = "12 - USART0_TX"]
    USART0_TX,
    #[doc = "13 - ACMP0"]
    ACMP0,
    #[doc = "14 - ADC0"]
    ADC0,
    #[doc = "15 - IDAC0"]
    IDAC0,
    #[doc = "16 - I2C0"]
    I2C0,
    #[doc = "17 - GPIO_ODD"]
    GPIO_ODD,
    #[doc = "18 - TIMER1"]
    TIMER1,
    #[doc = "19 - USART1_RX"]
    USART1_RX,
    #[doc = "20 - USART1_TX"]
    USART1_TX,
    #[doc = "21 - LEUART0"]
    LEUART0,
    #[doc = "22 - PCNT0"]
    PCNT0,
    #[doc = "23 - CMU"]
    CMU,
    #[doc = "24 - MSC"]
    MSC,
    #[doc = "25 - CRYPTO"]
    CRYPTO,
    #[doc = "26 - LETIMER0"]
    LETIMER0,
    #[doc = "29 - RTCC"]
    RTCC,
    #[doc = "31 - CRYOTIMER"]
    CRYOTIMER,
    #[doc = "33 - FPUEH"]
    FPUEH,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::EMU => 0,
            Interrupt::WDOG0 => 2,
            Interrupt::LDMA => 8,
            Interrupt::GPIO_EVEN => 9,
            Interrupt::TIMER0 => 10,
            Interrupt::USART0_RX => 11,
            Interrupt::USART0_TX => 12,
            Interrupt::ACMP0 => 13,
            Interrupt::ADC0 => 14,
            Interrupt::IDAC0 => 15,
            Interrupt::I2C0 => 16,
            Interrupt::GPIO_ODD => 17,
            Interrupt::TIMER1 => 18,
            Interrupt::USART1_RX => 19,
            Interrupt::USART1_TX => 20,
            Interrupt::LEUART0 => 21,
            Interrupt::PCNT0 => 22,
            Interrupt::CMU => 23,
            Interrupt::MSC => 24,
            Interrupt::CRYPTO => 25,
            Interrupt::LETIMER0 => 26,
            Interrupt::RTCC => 29,
            Interrupt::CRYOTIMER => 31,
            Interrupt::FPUEH => 33,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "MSC"]
pub struct MSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MSC {}
impl MSC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const msc::RegisterBlock {
        1074659328 as *const _
    }
}
impl Deref for MSC {
    type Target = msc::RegisterBlock;
    fn deref(&self) -> &msc::RegisterBlock {
        unsafe { &*MSC::ptr() }
    }
}
#[doc = "MSC"]
pub mod msc;
#[doc = "EMU"]
pub struct EMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EMU {}
impl EMU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const emu::RegisterBlock {
        1074671616 as *const _
    }
}
impl Deref for EMU {
    type Target = emu::RegisterBlock;
    fn deref(&self) -> &emu::RegisterBlock {
        unsafe { &*EMU::ptr() }
    }
}
#[doc = "EMU"]
pub mod emu;
#[doc = "RMU"]
pub struct RMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RMU {}
impl RMU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rmu::RegisterBlock {
        1074679808 as *const _
    }
}
impl Deref for RMU {
    type Target = rmu::RegisterBlock;
    fn deref(&self) -> &rmu::RegisterBlock {
        unsafe { &*RMU::ptr() }
    }
}
#[doc = "RMU"]
pub mod rmu;
#[doc = "CMU"]
pub struct CMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMU {}
impl CMU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cmu::RegisterBlock {
        1074675712 as *const _
    }
}
impl Deref for CMU {
    type Target = cmu::RegisterBlock;
    fn deref(&self) -> &cmu::RegisterBlock {
        unsafe { &*CMU::ptr() }
    }
}
#[doc = "CMU"]
pub mod cmu;
#[doc = "CRYPTO"]
pub struct CRYPTO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRYPTO {}
impl CRYPTO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crypto::RegisterBlock {
        1074724864 as *const _
    }
}
impl Deref for CRYPTO {
    type Target = crypto::RegisterBlock;
    fn deref(&self) -> &crypto::RegisterBlock {
        unsafe { &*CRYPTO::ptr() }
    }
}
#[doc = "CRYPTO"]
pub mod crypto;
#[doc = "GPIO"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio::RegisterBlock {
        1073782784 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    fn deref(&self) -> &gpio::RegisterBlock {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "GPIO"]
pub mod gpio;
#[doc = "PRS"]
pub struct PRS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PRS {}
impl PRS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const prs::RegisterBlock {
        1074683904 as *const _
    }
}
impl Deref for PRS {
    type Target = prs::RegisterBlock;
    fn deref(&self) -> &prs::RegisterBlock {
        unsafe { &*PRS::ptr() }
    }
}
#[doc = "PRS"]
pub mod prs;
#[doc = "LDMA"]
pub struct LDMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LDMA {}
impl LDMA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ldma::RegisterBlock {
        1074667520 as *const _
    }
}
impl Deref for LDMA {
    type Target = ldma::RegisterBlock;
    fn deref(&self) -> &ldma::RegisterBlock {
        unsafe { &*LDMA::ptr() }
    }
}
#[doc = "LDMA"]
pub mod ldma;
#[doc = "FPUEH"]
pub struct FPUEH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FPUEH {}
impl FPUEH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fpueh::RegisterBlock {
        1074663424 as *const _
    }
}
impl Deref for FPUEH {
    type Target = fpueh::RegisterBlock;
    fn deref(&self) -> &fpueh::RegisterBlock {
        unsafe { &*FPUEH::ptr() }
    }
}
#[doc = "FPUEH"]
pub mod fpueh;
#[doc = "GPCRC"]
pub struct GPCRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPCRC {}
impl GPCRC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpcrc::RegisterBlock {
        1073856512 as *const _
    }
}
impl Deref for GPCRC {
    type Target = gpcrc::RegisterBlock;
    fn deref(&self) -> &gpcrc::RegisterBlock {
        unsafe { &*GPCRC::ptr() }
    }
}
#[doc = "GPCRC"]
pub mod gpcrc;
#[doc = "TIMER0"]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1073840128 as *const _
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER0::ptr() }
    }
}
#[doc = "TIMER0"]
pub mod timer0;
#[doc = "TIMER1"]
pub struct TIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1 {}
impl TIMER1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer1::RegisterBlock {
        1073841152 as *const _
    }
}
impl Deref for TIMER1 {
    type Target = timer1::RegisterBlock;
    fn deref(&self) -> &timer1::RegisterBlock {
        unsafe { &*TIMER1::ptr() }
    }
}
#[doc = "TIMER1"]
pub mod timer1;
#[doc = "USART0"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart0::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &usart0::RegisterBlock {
        unsafe { &*USART0::ptr() }
    }
}
#[doc = "USART0"]
pub mod usart0;
#[doc = "USART1"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073808384 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "USART1"]
pub mod usart1;
#[doc = "LEUART0"]
pub struct LEUART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LEUART0 {}
impl LEUART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const leuart0::RegisterBlock {
        1074044928 as *const _
    }
}
impl Deref for LEUART0 {
    type Target = leuart0::RegisterBlock;
    fn deref(&self) -> &leuart0::RegisterBlock {
        unsafe { &*LEUART0::ptr() }
    }
}
#[doc = "LEUART0"]
pub mod leuart0;
#[doc = "LETIMER0"]
pub struct LETIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LETIMER0 {}
impl LETIMER0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const letimer0::RegisterBlock {
        1074028544 as *const _
    }
}
impl Deref for LETIMER0 {
    type Target = letimer0::RegisterBlock;
    fn deref(&self) -> &letimer0::RegisterBlock {
        unsafe { &*LETIMER0::ptr() }
    }
}
#[doc = "LETIMER0"]
pub mod letimer0;
#[doc = "CRYOTIMER"]
pub struct CRYOTIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRYOTIMER {}
impl CRYOTIMER {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cryotimer::RegisterBlock {
        1073864704 as *const _
    }
}
impl Deref for CRYOTIMER {
    type Target = cryotimer::RegisterBlock;
    fn deref(&self) -> &cryotimer::RegisterBlock {
        unsafe { &*CRYOTIMER::ptr() }
    }
}
#[doc = "CRYOTIMER"]
pub mod cryotimer;
#[doc = "PCNT0"]
pub struct PCNT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PCNT0 {}
impl PCNT0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pcnt0::RegisterBlock {
        1074061312 as *const _
    }
}
impl Deref for PCNT0 {
    type Target = pcnt0::RegisterBlock;
    fn deref(&self) -> &pcnt0::RegisterBlock {
        unsafe { &*PCNT0::ptr() }
    }
}
#[doc = "PCNT0"]
pub mod pcnt0;
#[doc = "I2C0"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1073790976 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "I2C0"]
pub mod i2c0;
#[doc = "ADC0"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc0::RegisterBlock {
        1073750016 as *const _
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    fn deref(&self) -> &adc0::RegisterBlock {
        unsafe { &*ADC0::ptr() }
    }
}
#[doc = "ADC0"]
pub mod adc0;
#[doc = "ACMP0"]
pub struct ACMP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACMP0 {}
impl ACMP0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const acmp0::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for ACMP0 {
    type Target = acmp0::RegisterBlock;
    fn deref(&self) -> &acmp0::RegisterBlock {
        unsafe { &*ACMP0::ptr() }
    }
}
#[doc = "ACMP0"]
pub mod acmp0;
#[doc = "ACMP1"]
pub struct ACMP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACMP1 {}
impl ACMP1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const acmp1::RegisterBlock {
        1073742848 as *const _
    }
}
impl Deref for ACMP1 {
    type Target = acmp1::RegisterBlock;
    fn deref(&self) -> &acmp1::RegisterBlock {
        unsafe { &*ACMP1::ptr() }
    }
}
#[doc = "ACMP1"]
pub mod acmp1;
#[doc = "IDAC0"]
pub struct IDAC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IDAC0 {}
impl IDAC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const idac0::RegisterBlock {
        1073766400 as *const _
    }
}
impl Deref for IDAC0 {
    type Target = idac0::RegisterBlock;
    fn deref(&self) -> &idac0::RegisterBlock {
        unsafe { &*IDAC0::ptr() }
    }
}
#[doc = "IDAC0"]
pub mod idac0;
#[doc = "RTCC"]
pub struct RTCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTCC {}
impl RTCC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtcc::RegisterBlock {
        1074012160 as *const _
    }
}
impl Deref for RTCC {
    type Target = rtcc::RegisterBlock;
    fn deref(&self) -> &rtcc::RegisterBlock {
        unsafe { &*RTCC::ptr() }
    }
}
#[doc = "RTCC"]
pub mod rtcc;
#[doc = "WDOG0"]
pub struct WDOG0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDOG0 {}
impl WDOG0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdog0::RegisterBlock {
        1074077696 as *const _
    }
}
impl Deref for WDOG0 {
    type Target = wdog0::RegisterBlock;
    fn deref(&self) -> &wdog0::RegisterBlock {
        unsafe { &*WDOG0::ptr() }
    }
}
#[doc = "WDOG0"]
pub mod wdog0;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "MSC"]
    pub MSC: MSC,
    #[doc = "EMU"]
    pub EMU: EMU,
    #[doc = "RMU"]
    pub RMU: RMU,
    #[doc = "CMU"]
    pub CMU: CMU,
    #[doc = "CRYPTO"]
    pub CRYPTO: CRYPTO,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "PRS"]
    pub PRS: PRS,
    #[doc = "LDMA"]
    pub LDMA: LDMA,
    #[doc = "FPUEH"]
    pub FPUEH: FPUEH,
    #[doc = "GPCRC"]
    pub GPCRC: GPCRC,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TIMER1"]
    pub TIMER1: TIMER1,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "LEUART0"]
    pub LEUART0: LEUART0,
    #[doc = "LETIMER0"]
    pub LETIMER0: LETIMER0,
    #[doc = "CRYOTIMER"]
    pub CRYOTIMER: CRYOTIMER,
    #[doc = "PCNT0"]
    pub PCNT0: PCNT0,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "ACMP0"]
    pub ACMP0: ACMP0,
    #[doc = "ACMP1"]
    pub ACMP1: ACMP1,
    #[doc = "IDAC0"]
    pub IDAC0: IDAC0,
    #[doc = "RTCC"]
    pub RTCC: RTCC,
    #[doc = "WDOG0"]
    pub WDOG0: WDOG0,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            MSC: MSC {
                _marker: PhantomData,
            },
            EMU: EMU {
                _marker: PhantomData,
            },
            RMU: RMU {
                _marker: PhantomData,
            },
            CMU: CMU {
                _marker: PhantomData,
            },
            CRYPTO: CRYPTO {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            PRS: PRS {
                _marker: PhantomData,
            },
            LDMA: LDMA {
                _marker: PhantomData,
            },
            FPUEH: FPUEH {
                _marker: PhantomData,
            },
            GPCRC: GPCRC {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            TIMER1: TIMER1 {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            LEUART0: LEUART0 {
                _marker: PhantomData,
            },
            LETIMER0: LETIMER0 {
                _marker: PhantomData,
            },
            CRYOTIMER: CRYOTIMER {
                _marker: PhantomData,
            },
            PCNT0: PCNT0 {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            ADC0: ADC0 {
                _marker: PhantomData,
            },
            ACMP0: ACMP0 {
                _marker: PhantomData,
            },
            ACMP1: ACMP1 {
                _marker: PhantomData,
            },
            IDAC0: IDAC0 {
                _marker: PhantomData,
            },
            RTCC: RTCC {
                _marker: PhantomData,
            },
            WDOG0: WDOG0 {
                _marker: PhantomData,
            },
        }
    }
}
