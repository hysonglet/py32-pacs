#![doc = "Peripheral access API for PY32C6XX_DFP microcontrollers (generated using svd2rust v0.25.1 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.25.1/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[cfg(feature = "rt")]
extern "C" {
    fn FLASH();
    fn RCC();
    fn EXTI0_1();
    fn EXTI2_3();
    fn EXTI4_15();
    fn ADC_COMP();
    fn TIM1_BRK_UP_TRG_COM();
    fn TIM1_CC();
    fn LPTIM1_DAC();
    fn TIM14();
    fn I2C1();
    fn SPI1();
    fn USART1();
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
pub static __INTERRUPTS: [Vector; 28] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0_1 },
    Vector { _handler: EXTI2_3 },
    Vector { _handler: EXTI4_15 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: ADC_COMP },
    Vector {
        _handler: TIM1_BRK_UP_TRG_COM,
    },
    Vector { _handler: TIM1_CC },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: LPTIM1_DAC,
    },
    Vector { _reserved: 0 },
    Vector { _handler: TIM14 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C1 },
    Vector { _reserved: 0 },
    Vector { _handler: SPI1 },
    Vector { _reserved: 0 },
    Vector { _handler: USART1 },
];
#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "3 - FLASH global Interrupt"]
    FLASH = 3,
    #[doc = "4 - RCC global Interrupt"]
    RCC = 4,
    #[doc = "5 - EXTI Line 0 and 1 Interrupt"]
    EXTI0_1 = 5,
    #[doc = "6 - EXTI Line 2 and 3 Interrupt"]
    EXTI2_3 = 6,
    #[doc = "7 - EXTI Line 4 to 15 Interrupt"]
    EXTI4_15 = 7,
    #[doc = "12 - ADC and COMP Interrupt"]
    ADC_COMP = 12,
    #[doc = "13 - TIM1 Break, Update, Trigger and Commutation Interrupt"]
    TIM1_BRK_UP_TRG_COM = 13,
    #[doc = "14 - TIM1 Capture Compare Interrupt"]
    TIM1_CC = 14,
    #[doc = "17 - LPTIM1, DAC global Interrupts"]
    LPTIM1_DAC = 17,
    #[doc = "19 - TIM14 global Interrupt"]
    TIM14 = 19,
    #[doc = "23 - I2C1 global Interrupt"]
    I2C1 = 23,
    #[doc = "25 - SPI1 global Interrupt"]
    SPI1 = 25,
    #[doc = "27 - USART1 global Interrupt"]
    USART1 = 27,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[doc = "Analog to Digital Converter"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const adc::RegisterBlock = 0x4001_2400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ADC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC").finish()
    }
}
#[doc = "Analog to Digital Converter"]
pub mod adc;
#[doc = "Comparator 1"]
pub struct COMP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMP1 {}
impl COMP1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const comp1::RegisterBlock = 0x4001_0200 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const comp1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for COMP1 {
    type Target = comp1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for COMP1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP1").finish()
    }
}
#[doc = "Comparator 1"]
pub mod comp1;
#[doc = "Comparator2"]
pub struct COMP2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMP2 {}
impl COMP2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const comp2::RegisterBlock = 0x4001_0210 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const comp2::RegisterBlock {
        Self::PTR
    }
}
impl Deref for COMP2 {
    type Target = comp2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for COMP2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP2").finish()
    }
}
#[doc = "Comparator2"]
pub mod comp2;
#[doc = "Reset and clock control"]
pub struct RCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCC {}
impl RCC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rcc::RegisterBlock = 0x4002_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rcc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RCC {
    type Target = rcc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RCC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC").finish()
    }
}
#[doc = "Reset and clock control"]
pub mod rcc;
#[doc = "Power control"]
pub struct PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWR {}
impl PWR {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pwr::RegisterBlock = 0x4000_7000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwr::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PWR {
    type Target = pwr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PWR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR").finish()
    }
}
#[doc = "Power control"]
pub mod pwr;
#[doc = "General-purpose I/Os"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpioa::RegisterBlock = 0x5000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIOA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOA").finish()
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioa;
#[doc = "General-purpose I/Os"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpiob::RegisterBlock = 0x5000_0400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiob::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIOB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOB").finish()
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpiob;
#[doc = "General-purpose I/Os"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpiob::RegisterBlock = 0x5000_0800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiob::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIOC {
    type Target = gpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIOC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOC").finish()
    }
}
#[doc = "General-purpose I/Os"]
pub use gpiob as gpioc;
#[doc = "External interrupt/event controller"]
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const exti::RegisterBlock = 0x4002_1800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const exti::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EXTI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTI").finish()
    }
}
#[doc = "External interrupt/event controller"]
pub mod exti;
#[doc = "Low power timer"]
pub struct LPTIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM1 {}
impl LPTIM1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const lptim1::RegisterBlock = 0x4000_7c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptim1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LPTIM1 {
    type Target = lptim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LPTIM1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM1").finish()
    }
}
#[doc = "Low power timer"]
pub mod lptim1;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usart1::RegisterBlock = 0x4001_3800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USART1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART1").finish()
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod usart1;
#[doc = "Independent watchdog"]
pub struct IWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IWDG {}
impl IWDG {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const iwdg::RegisterBlock = 0x4000_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const iwdg::RegisterBlock {
        Self::PTR
    }
}
impl Deref for IWDG {
    type Target = iwdg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for IWDG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IWDG").finish()
    }
}
#[doc = "Independent watchdog"]
pub mod iwdg;
#[doc = "Advanced timer"]
pub struct TIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM1 {}
impl TIM1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tim1::RegisterBlock = 0x4001_2c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIM1 {
    type Target = tim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIM1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM1").finish()
    }
}
#[doc = "Advanced timer"]
pub mod tim1;
#[doc = "General purpose timer"]
pub struct TIM14 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM14 {}
impl TIM14 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tim14::RegisterBlock = 0x4000_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim14::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIM14 {
    type Target = tim14::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIM14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM14").finish()
    }
}
#[doc = "General purpose timer"]
pub mod tim14;
#[doc = "System configuration controller"]
pub struct SYSCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCFG {}
impl SYSCFG {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const syscfg::RegisterBlock = 0x4001_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const syscfg::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SYSCFG {
    type Target = syscfg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SYSCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG").finish()
    }
}
#[doc = "System configuration controller"]
pub mod syscfg;
#[doc = "Flash"]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const flash::RegisterBlock = 0x4002_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash::RegisterBlock {
        Self::PTR
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for FLASH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH").finish()
    }
}
#[doc = "Flash"]
pub mod flash;
#[doc = "CRC calculation unit"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const crc::RegisterBlock = 0x4002_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CRC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRC").finish()
    }
}
#[doc = "CRC calculation unit"]
pub mod crc;
#[doc = "Serial peripheral interface"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi1::RegisterBlock = 0x4001_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI1").finish()
    }
}
#[doc = "Serial peripheral interface"]
pub mod spi1;
#[doc = "Inter integrated circuit"]
pub struct I2C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C {}
impl I2C {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2c::RegisterBlock = 0x4000_5400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2C {
    type Target = i2c::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C").finish()
    }
}
#[doc = "Inter integrated circuit"]
pub mod i2c;
#[doc = "Debug support"]
pub struct DBGMCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DBGMCU {}
impl DBGMCU {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dbgmcu::RegisterBlock = 0x4001_5800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dbgmcu::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DBGMCU {
    type Target = dbgmcu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for DBGMCU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGMCU").finish()
    }
}
#[doc = "Debug support"]
pub mod dbgmcu;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "COMP1"]
    pub COMP1: COMP1,
    #[doc = "COMP2"]
    pub COMP2: COMP2,
    #[doc = "RCC"]
    pub RCC: RCC,
    #[doc = "PWR"]
    pub PWR: PWR,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "EXTI"]
    pub EXTI: EXTI,
    #[doc = "LPTIM1"]
    pub LPTIM1: LPTIM1,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "IWDG"]
    pub IWDG: IWDG,
    #[doc = "TIM1"]
    pub TIM1: TIM1,
    #[doc = "TIM14"]
    pub TIM14: TIM14,
    #[doc = "SYSCFG"]
    pub SYSCFG: SYSCFG,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "I2C"]
    pub I2C: I2C,
    #[doc = "DBGMCU"]
    pub DBGMCU: DBGMCU,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
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
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            ADC: ADC {
                _marker: PhantomData,
            },
            COMP1: COMP1 {
                _marker: PhantomData,
            },
            COMP2: COMP2 {
                _marker: PhantomData,
            },
            RCC: RCC {
                _marker: PhantomData,
            },
            PWR: PWR {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            EXTI: EXTI {
                _marker: PhantomData,
            },
            LPTIM1: LPTIM1 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            IWDG: IWDG {
                _marker: PhantomData,
            },
            TIM1: TIM1 {
                _marker: PhantomData,
            },
            TIM14: TIM14 {
                _marker: PhantomData,
            },
            SYSCFG: SYSCFG {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            I2C: I2C {
                _marker: PhantomData,
            },
            DBGMCU: DBGMCU {
                _marker: PhantomData,
            },
        }
    }
}
