#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - Control register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - IDAC Control register"]
    pub idacr: IDACR,
    #[doc = "0x0c - Key enable register"]
    pub kr: KR,
    #[doc = "0x10 - Channel Select register"]
    pub csr: CSR,
    #[doc = "0x14 - Unsel Control register"]
    pub ucr: UCR,
    #[doc = "0x18 - Stable Time register 1"]
    pub str1: STR1,
    #[doc = "0x1c - Stable Time register 2"]
    pub str2: STR2,
    #[doc = "0x20 - Window register"]
    pub wr: WR,
    #[doc = "0x24 - Switch Divide register"]
    pub sdr: SDR,
    #[doc = "0x28 - Pseudo Random Sequence register"]
    pub prsr: PRSR,
    #[doc = "0x2c - Data register"]
    pub dr: DR,
    #[doc = "0x30 - Status register"]
    pub sr: SR,
    #[doc = "0x34 - Interrupt enable register"]
    pub ier: IER,
    #[doc = "0x38 - Interrupt clear register"]
    pub icr: ICR,
    #[doc = "0x3c - Compare Data register"]
    pub cdr: CDR,
    #[doc = "0x40 - Lowpower Delta register"]
    pub lpdr: LPDR,
    #[doc = "0x44 - Lowpower Time register"]
    pub lptr: LPTR,
}
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "Control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "Control register 2"]
pub mod cr2;
#[doc = "IDACR (rw) register accessor: an alias for `Reg<IDACR_SPEC>`"]
pub type IDACR = crate::Reg<idacr::IDACR_SPEC>;
#[doc = "IDAC Control register"]
pub mod idacr;
#[doc = "KR (rw) register accessor: an alias for `Reg<KR_SPEC>`"]
pub type KR = crate::Reg<kr::KR_SPEC>;
#[doc = "Key enable register"]
pub mod kr;
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Channel Select register"]
pub mod csr;
#[doc = "UCR (rw) register accessor: an alias for `Reg<UCR_SPEC>`"]
pub type UCR = crate::Reg<ucr::UCR_SPEC>;
#[doc = "Unsel Control register"]
pub mod ucr;
#[doc = "STR1 (rw) register accessor: an alias for `Reg<STR1_SPEC>`"]
pub type STR1 = crate::Reg<str1::STR1_SPEC>;
#[doc = "Stable Time register 1"]
pub mod str1;
#[doc = "STR2 (rw) register accessor: an alias for `Reg<STR2_SPEC>`"]
pub type STR2 = crate::Reg<str2::STR2_SPEC>;
#[doc = "Stable Time register 2"]
pub mod str2;
#[doc = "WR (rw) register accessor: an alias for `Reg<WR_SPEC>`"]
pub type WR = crate::Reg<wr::WR_SPEC>;
#[doc = "Window register"]
pub mod wr;
#[doc = "SDR (rw) register accessor: an alias for `Reg<SDR_SPEC>`"]
pub type SDR = crate::Reg<sdr::SDR_SPEC>;
#[doc = "Switch Divide register"]
pub mod sdr;
#[doc = "PRSR (rw) register accessor: an alias for `Reg<PRSR_SPEC>`"]
pub type PRSR = crate::Reg<prsr::PRSR_SPEC>;
#[doc = "Pseudo Random Sequence register"]
pub mod prsr;
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Data register"]
pub mod dr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt enable register"]
pub mod ier;
#[doc = "ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt clear register"]
pub mod icr;
#[doc = "CDR (rw) register accessor: an alias for `Reg<CDR_SPEC>`"]
pub type CDR = crate::Reg<cdr::CDR_SPEC>;
#[doc = "Compare Data register"]
pub mod cdr;
#[doc = "LPDR (rw) register accessor: an alias for `Reg<LPDR_SPEC>`"]
pub type LPDR = crate::Reg<lpdr::LPDR_SPEC>;
#[doc = "Lowpower Delta register"]
pub mod lpdr;
#[doc = "LPTR (rw) register accessor: an alias for `Reg<LPTR_SPEC>`"]
pub type LPTR = crate::Reg<lptr::LPTR_SPEC>;
#[doc = "Lowpower Time register"]
pub mod lptr;
