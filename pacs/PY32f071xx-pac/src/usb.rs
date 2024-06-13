#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR"]
    pub cr: CR,
    #[doc = "0x04 - INTR"]
    pub intr: INTR,
    #[doc = "0x08 - INTRE"]
    pub intre: INTRE,
    #[doc = "0x0c - FRAME"]
    pub frame: FRAME,
    #[doc = "0x10 - EP0CSR"]
    pub ep0csr: EP0CSR,
    #[doc = "0x14 - INEPxCSR"]
    pub inepx_csr: INEPX_CSR,
    #[doc = "0x18 - OUTEPxCSR"]
    pub outepx_csr: OUTEPX_CSR,
    #[doc = "0x1c - OUTCOUNT"]
    pub outcount: OUTCOUNT,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR"]
pub mod cr;
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "INTR"]
pub mod intr;
#[doc = "INTRE (rw) register accessor: an alias for `Reg<INTRE_SPEC>`"]
pub type INTRE = crate::Reg<intre::INTRE_SPEC>;
#[doc = "INTRE"]
pub mod intre;
#[doc = "FRAME (w) register accessor: an alias for `Reg<FRAME_SPEC>`"]
pub type FRAME = crate::Reg<frame::FRAME_SPEC>;
#[doc = "FRAME"]
pub mod frame;
#[doc = "EP0CSR (w) register accessor: an alias for `Reg<EP0CSR_SPEC>`"]
pub type EP0CSR = crate::Reg<ep0csr::EP0CSR_SPEC>;
#[doc = "EP0CSR"]
pub mod ep0csr;
#[doc = "INEPxCSR (w) register accessor: an alias for `Reg<INEPX_CSR_SPEC>`"]
pub type INEPX_CSR = crate::Reg<inepx_csr::INEPX_CSR_SPEC>;
#[doc = "INEPxCSR"]
pub mod inepx_csr;
#[doc = "OUTEPxCSR (w) register accessor: an alias for `Reg<OUTEPX_CSR_SPEC>`"]
pub type OUTEPX_CSR = crate::Reg<outepx_csr::OUTEPX_CSR_SPEC>;
#[doc = "OUTEPxCSR"]
pub mod outepx_csr;
#[doc = "OUTCOUNT (w) register accessor: an alias for `Reg<OUTCOUNT_SPEC>`"]
pub type OUTCOUNT = crate::Reg<outcount::OUTCOUNT_SPEC>;
#[doc = "OUTCOUNT"]
pub mod outcount;
