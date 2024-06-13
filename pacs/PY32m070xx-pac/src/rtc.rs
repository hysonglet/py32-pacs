#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc CRH"]
    pub crh: CRH,
    #[doc = "0x04 - desc CRL"]
    pub crl: CRL,
    #[doc = "0x08 - desc PRLH"]
    pub prlh: PRLH,
    #[doc = "0x0c - desc PRLL"]
    pub prll: PRLL,
    #[doc = "0x10 - desc DIVH"]
    pub divh: DIVH,
    #[doc = "0x14 - desc DIVL"]
    pub divl: DIVL,
    #[doc = "0x18 - desc CNTH"]
    pub cnth: CNTH,
    #[doc = "0x1c - desc CNTL"]
    pub cntl: CNTL,
    #[doc = "0x20 - desc ALRH"]
    pub alrh: ALRH,
    #[doc = "0x24 - desc ALRL"]
    pub alrl: ALRL,
    _reserved10: [u8; 0x04],
    #[doc = "0x2c - desc BKP_RTCCR"]
    pub bkp_rtccr: BKP_RTCCR,
}
#[doc = "CRH (rw) register accessor: an alias for `Reg<CRH_SPEC>`"]
pub type CRH = crate::Reg<crh::CRH_SPEC>;
#[doc = "desc CRH"]
pub mod crh;
#[doc = "CRL (rw) register accessor: an alias for `Reg<CRL_SPEC>`"]
pub type CRL = crate::Reg<crl::CRL_SPEC>;
#[doc = "desc CRL"]
pub mod crl;
#[doc = "PRLH (w) register accessor: an alias for `Reg<PRLH_SPEC>`"]
pub type PRLH = crate::Reg<prlh::PRLH_SPEC>;
#[doc = "desc PRLH"]
pub mod prlh;
#[doc = "PRLL (w) register accessor: an alias for `Reg<PRLL_SPEC>`"]
pub type PRLL = crate::Reg<prll::PRLL_SPEC>;
#[doc = "desc PRLL"]
pub mod prll;
#[doc = "DIVH (r) register accessor: an alias for `Reg<DIVH_SPEC>`"]
pub type DIVH = crate::Reg<divh::DIVH_SPEC>;
#[doc = "desc DIVH"]
pub mod divh;
#[doc = "DIVL (r) register accessor: an alias for `Reg<DIVL_SPEC>`"]
pub type DIVL = crate::Reg<divl::DIVL_SPEC>;
#[doc = "desc DIVL"]
pub mod divl;
#[doc = "CNTH (rw) register accessor: an alias for `Reg<CNTH_SPEC>`"]
pub type CNTH = crate::Reg<cnth::CNTH_SPEC>;
#[doc = "desc CNTH"]
pub mod cnth;
#[doc = "CNTL (rw) register accessor: an alias for `Reg<CNTL_SPEC>`"]
pub type CNTL = crate::Reg<cntl::CNTL_SPEC>;
#[doc = "desc CNTL"]
pub mod cntl;
#[doc = "ALRH (rw) register accessor: an alias for `Reg<ALRH_SPEC>`"]
pub type ALRH = crate::Reg<alrh::ALRH_SPEC>;
#[doc = "desc ALRH"]
pub mod alrh;
#[doc = "ALRL (rw) register accessor: an alias for `Reg<ALRL_SPEC>`"]
pub type ALRL = crate::Reg<alrl::ALRL_SPEC>;
#[doc = "desc ALRL"]
pub mod alrl;
#[doc = "BKP_RTCCR (rw) register accessor: an alias for `Reg<BKP_RTCCR_SPEC>`"]
pub type BKP_RTCCR = crate::Reg<bkp_rtccr::BKP_RTCCR_SPEC>;
#[doc = "desc BKP_RTCCR"]
pub mod bkp_rtccr;
