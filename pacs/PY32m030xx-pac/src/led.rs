#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub cr: CR,
    #[doc = "0x04 - Prescaler register"]
    pub pr: PR,
    #[doc = "0x08 - Time register"]
    pub tr: TR,
    #[doc = "0x0c - Data0 register"]
    pub dr0: DR0,
    #[doc = "0x10 - Data1 register"]
    pub dr1: DR1,
    #[doc = "0x14 - Data2 register"]
    pub dr2: DR2,
    #[doc = "0x18 - Data3 register"]
    pub dr3: DR3,
    #[doc = "0x1c - Interrupt register 1"]
    pub ir: IR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control register"]
pub mod cr;
#[doc = "PR (rw) register accessor: an alias for `Reg<PR_SPEC>`"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "Prescaler register"]
pub mod pr;
#[doc = "TR (rw) register accessor: an alias for `Reg<TR_SPEC>`"]
pub type TR = crate::Reg<tr::TR_SPEC>;
#[doc = "Time register"]
pub mod tr;
#[doc = "DR0 (rw) register accessor: an alias for `Reg<DR0_SPEC>`"]
pub type DR0 = crate::Reg<dr0::DR0_SPEC>;
#[doc = "Data0 register"]
pub mod dr0;
#[doc = "DR1 (rw) register accessor: an alias for `Reg<DR1_SPEC>`"]
pub type DR1 = crate::Reg<dr1::DR1_SPEC>;
#[doc = "Data1 register"]
pub mod dr1;
#[doc = "DR2 (rw) register accessor: an alias for `Reg<DR2_SPEC>`"]
pub type DR2 = crate::Reg<dr2::DR2_SPEC>;
#[doc = "Data2 register"]
pub mod dr2;
#[doc = "DR3 (rw) register accessor: an alias for `Reg<DR3_SPEC>`"]
pub type DR3 = crate::Reg<dr3::DR3_SPEC>;
#[doc = "Data3 register"]
pub mod dr3;
#[doc = "IR (rw) register accessor: an alias for `Reg<IR_SPEC>`"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "Interrupt register 1"]
pub mod ir;
