#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x30],
    #[doc = "0x30 - OPA output enable register"]
    pub oenr: OENR,
    #[doc = "0x34 - OPA control register"]
    pub cr: CR,
}
#[doc = "OENR (rw) register accessor: an alias for `Reg<OENR_SPEC>`"]
pub type OENR = crate::Reg<oenr::OENR_SPEC>;
#[doc = "OPA output enable register"]
pub mod oenr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "OPA control register"]
pub mod cr;
