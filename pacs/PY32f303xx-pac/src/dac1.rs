#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc CR"]
    pub cr: CR,
    #[doc = "0x04 - desc SWTRIGR"]
    pub swtrigr: SWTRIGR,
    #[doc = "0x08 - desc DHR12R1"]
    pub dhr12r1: DHR12R1,
    #[doc = "0x0c - desc DHR12L1"]
    pub dhr12l1: DHR12L1,
    #[doc = "0x10 - desc DHR8R1"]
    pub dhr8r1: DHR8R1,
    #[doc = "0x14 - desc DHR12R2"]
    pub dhr12r2: DHR12R2,
    #[doc = "0x18 - desc DHR12L2"]
    pub dhr12l2: DHR12L2,
    #[doc = "0x1c - desc DHR8R2"]
    pub dhr8r2: DHR8R2,
    #[doc = "0x20 - desc DHR12RD"]
    pub dhr12rd: DHR12RD,
    #[doc = "0x24 - desc DHR12LD"]
    pub dhr12ld: DHR12LD,
    #[doc = "0x28 - desc DHR8RD"]
    pub dhr8rd: DHR8RD,
    #[doc = "0x2c - desc DOR1"]
    pub dor1: DOR1,
    #[doc = "0x30 - desc DOR2"]
    pub dor2: DOR2,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "SWTRIGR (w) register accessor: an alias for `Reg<SWTRIGR_SPEC>`"]
pub type SWTRIGR = crate::Reg<swtrigr::SWTRIGR_SPEC>;
#[doc = "desc SWTRIGR"]
pub mod swtrigr;
#[doc = "DHR12R1 (rw) register accessor: an alias for `Reg<DHR12R1_SPEC>`"]
pub type DHR12R1 = crate::Reg<dhr12r1::DHR12R1_SPEC>;
#[doc = "desc DHR12R1"]
pub mod dhr12r1;
#[doc = "DHR12L1 (rw) register accessor: an alias for `Reg<DHR12L1_SPEC>`"]
pub type DHR12L1 = crate::Reg<dhr12l1::DHR12L1_SPEC>;
#[doc = "desc DHR12L1"]
pub mod dhr12l1;
#[doc = "DHR8R1 (rw) register accessor: an alias for `Reg<DHR8R1_SPEC>`"]
pub type DHR8R1 = crate::Reg<dhr8r1::DHR8R1_SPEC>;
#[doc = "desc DHR8R1"]
pub mod dhr8r1;
#[doc = "DHR12R2 (rw) register accessor: an alias for `Reg<DHR12R2_SPEC>`"]
pub type DHR12R2 = crate::Reg<dhr12r2::DHR12R2_SPEC>;
#[doc = "desc DHR12R2"]
pub mod dhr12r2;
#[doc = "DHR12L2 (rw) register accessor: an alias for `Reg<DHR12L2_SPEC>`"]
pub type DHR12L2 = crate::Reg<dhr12l2::DHR12L2_SPEC>;
#[doc = "desc DHR12L2"]
pub mod dhr12l2;
#[doc = "DHR8R2 (rw) register accessor: an alias for `Reg<DHR8R2_SPEC>`"]
pub type DHR8R2 = crate::Reg<dhr8r2::DHR8R2_SPEC>;
#[doc = "desc DHR8R2"]
pub mod dhr8r2;
#[doc = "DHR12RD (rw) register accessor: an alias for `Reg<DHR12RD_SPEC>`"]
pub type DHR12RD = crate::Reg<dhr12rd::DHR12RD_SPEC>;
#[doc = "desc DHR12RD"]
pub mod dhr12rd;
#[doc = "DHR12LD (rw) register accessor: an alias for `Reg<DHR12LD_SPEC>`"]
pub type DHR12LD = crate::Reg<dhr12ld::DHR12LD_SPEC>;
#[doc = "desc DHR12LD"]
pub mod dhr12ld;
#[doc = "DHR8RD (rw) register accessor: an alias for `Reg<DHR8RD_SPEC>`"]
pub type DHR8RD = crate::Reg<dhr8rd::DHR8RD_SPEC>;
#[doc = "desc DHR8RD"]
pub mod dhr8rd;
#[doc = "DOR1 (r) register accessor: an alias for `Reg<DOR1_SPEC>`"]
pub type DOR1 = crate::Reg<dor1::DOR1_SPEC>;
#[doc = "desc DOR1"]
pub mod dor1;
#[doc = "DOR2 (r) register accessor: an alias for `Reg<DOR2_SPEC>`"]
pub type DOR2 = crate::Reg<dor2::DOR2_SPEC>;
#[doc = "desc DOR2"]
pub mod dor2;
