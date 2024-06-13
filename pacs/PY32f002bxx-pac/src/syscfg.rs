#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SYSCFG configuration register 1"]
    pub cfgr1: CFGR1,
    _reserved1: [u8; 0x14],
    #[doc = "0x18 - SYSCFG configuration register 2"]
    pub cfgr2: CFGR2,
    #[doc = "0x1c - desc GPIO_ENS"]
    pub gpio_ens: GPIO_ENS,
}
#[doc = "CFGR1 (rw) register accessor: an alias for `Reg<CFGR1_SPEC>`"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
#[doc = "SYSCFG configuration register 1"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "SYSCFG configuration register 2"]
pub mod cfgr2;
#[doc = "GPIO_ENS (rw) register accessor: an alias for `Reg<GPIO_ENS_SPEC>`"]
pub type GPIO_ENS = crate::Reg<gpio_ens::GPIO_ENS_SPEC>;
#[doc = "desc GPIO_ENS"]
pub mod gpio_ens;
