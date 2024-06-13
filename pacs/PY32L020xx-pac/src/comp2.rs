#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - COMP control and status register"]
    pub csr: CSR,
    #[doc = "0x04 - Comparator Filter register"]
    pub fr: FR,
}
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "COMP control and status register"]
pub mod csr;
#[doc = "FR (rw) register accessor: an alias for `Reg<FR_SPEC>`"]
pub type FR = crate::Reg<fr::FR_SPEC>;
#[doc = "Comparator Filter register"]
pub mod fr;
