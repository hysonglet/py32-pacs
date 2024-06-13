#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SYSCFG configuration register 1"]
    pub cfgr1: CFGR1,
    _reserved1: [u8; 0x14],
    #[doc = "0x18 - SYSCFG configuration register 2"]
    pub cfgr2: CFGR2,
    #[doc = "0x1c - SYSCFG configuration register 3"]
    pub cfgr3: CFGR3,
}
#[doc = "CFGR1 (rw) register accessor: an alias for `Reg<CFGR1_SPEC>`"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
#[doc = "SYSCFG configuration register 1"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "SYSCFG configuration register 2"]
pub mod cfgr2;
#[doc = "CFGR3 (rw) register accessor: an alias for `Reg<CFGR3_SPEC>`"]
pub type CFGR3 = crate::Reg<cfgr3::CFGR3_SPEC>;
#[doc = "SYSCFG configuration register 3"]
pub mod cfgr3;
