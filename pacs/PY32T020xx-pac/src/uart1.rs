#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data register"]
    pub dr: DR,
    #[doc = "0x04 - Baud rate register"]
    pub brr: BRR,
    #[doc = "0x08 - "]
    pub iidr: IIDR,
    #[doc = "0x0c - "]
    pub busy: BUSY,
    #[doc = "0x10 - Status register"]
    pub sr: SR,
    #[doc = "0x14 - Control register 1"]
    pub cr1: CR1,
    #[doc = "0x18 - Control register 2"]
    pub cr2: CR2,
    #[doc = "0x1c - Control register 3"]
    pub cr3: CR3,
    #[doc = "0x20 - "]
    pub rar: RAR,
    #[doc = "0x24 - "]
    pub tar: TAR,
}
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Data register"]
pub mod dr;
#[doc = "BRR (rw) register accessor: an alias for `Reg<BRR_SPEC>`"]
pub type BRR = crate::Reg<brr::BRR_SPEC>;
#[doc = "Baud rate register"]
pub mod brr;
#[doc = "IIDR (rw) register accessor: an alias for `Reg<IIDR_SPEC>`"]
pub type IIDR = crate::Reg<iidr::IIDR_SPEC>;
#[doc = ""]
pub mod iidr;
#[doc = "BUSY (rw) register accessor: an alias for `Reg<BUSY_SPEC>`"]
pub type BUSY = crate::Reg<busy::BUSY_SPEC>;
#[doc = ""]
pub mod busy;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "Control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "Control register 2"]
pub mod cr2;
#[doc = "CR3 (rw) register accessor: an alias for `Reg<CR3_SPEC>`"]
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
#[doc = "Control register 3"]
pub mod cr3;
#[doc = "RAR (rw) register accessor: an alias for `Reg<RAR_SPEC>`"]
pub type RAR = crate::Reg<rar::RAR_SPEC>;
#[doc = ""]
pub mod rar;
#[doc = "TAR (rw) register accessor: an alias for `Reg<TAR_SPEC>`"]
pub type TAR = crate::Reg<tar::TAR_SPEC>;
#[doc = ""]
pub mod tar;
