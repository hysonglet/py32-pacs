#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc CR"]
    pub cr: CR,
    #[doc = "0x04 - INTR"]
    pub intr: INTR,
    #[doc = "0x08 - desc INTRE"]
    pub intre: INTRE,
    #[doc = "0x0c - desc FRAME"]
    pub frame: FRAME,
    #[doc = "0x10 - desc EP0CSR"]
    pub ep0csr: EP0CSR,
    #[doc = "0x14 - desc INEPxCSR"]
    pub inepx_csr: INEPX_CSR,
    #[doc = "0x18 - desc OUTEPxCSR"]
    pub outepx_csr: OUTEPX_CSR,
    #[doc = "0x1c - desc OUTCOUNT"]
    pub outcount: OUTCOUNT,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "INTR (r) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "INTR"]
pub mod intr;
#[doc = "INTRE (rw) register accessor: an alias for `Reg<INTRE_SPEC>`"]
pub type INTRE = crate::Reg<intre::INTRE_SPEC>;
#[doc = "desc INTRE"]
pub mod intre;
#[doc = "FRAME (r) register accessor: an alias for `Reg<FRAME_SPEC>`"]
pub type FRAME = crate::Reg<frame::FRAME_SPEC>;
#[doc = "desc FRAME"]
pub mod frame;
#[doc = "EP0CSR (w) register accessor: an alias for `Reg<EP0CSR_SPEC>`"]
pub type EP0CSR = crate::Reg<ep0csr::EP0CSR_SPEC>;
#[doc = "desc EP0CSR"]
pub mod ep0csr;
#[doc = "INEPxCSR (rw) register accessor: an alias for `Reg<INEPX_CSR_SPEC>`"]
pub type INEPX_CSR = crate::Reg<inepx_csr::INEPX_CSR_SPEC>;
#[doc = "desc INEPxCSR"]
pub mod inepx_csr;
#[doc = "OUTEPxCSR (rw) register accessor: an alias for `Reg<OUTEPX_CSR_SPEC>`"]
pub type OUTEPX_CSR = crate::Reg<outepx_csr::OUTEPX_CSR_SPEC>;
#[doc = "desc OUTEPxCSR"]
pub mod outepx_csr;
#[doc = "OUTCOUNT (r) register accessor: an alias for `Reg<OUTCOUNT_SPEC>`"]
pub type OUTCOUNT = crate::Reg<outcount::OUTCOUNT_SPEC>;
#[doc = "desc OUTCOUNT"]
pub mod outcount;
