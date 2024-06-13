#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc TSNCR"]
    pub tsncr: TSNCR,
    #[doc = "0x04 - desc ACBTR"]
    pub acbtr: ACBTR,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - desc RLSSP"]
    pub rlssp: RLSSP,
    #[doc = "0x14 - desc IFR"]
    pub ifr: IFR,
    #[doc = "0x18 - desc IER"]
    pub ier: IER,
    #[doc = "0x1c - desc TSR"]
    pub tsr: TSR,
    _reserved6: [u8; 0x08],
    #[doc = "0x28 - desc MCR"]
    pub mcr: MCR,
    #[doc = "0x2c - desc WECR"]
    pub wecr: WECR,
    #[doc = "0x30 - desc REFMSG"]
    pub refmsg: REFMSG,
    #[doc = "0x34 - desc TTCR"]
    pub ttcr: TTCR,
    #[doc = "0x38 - desc TTTR"]
    pub tttr: TTTR,
    #[doc = "0x3c - desc SCMS"]
    pub scms: SCMS,
    _reserved12: [u8; 0x04],
    #[doc = "0x44 - desc ACFCR"]
    pub acfcr: ACFCR,
}
#[doc = "TSNCR (rw) register accessor: an alias for `Reg<TSNCR_SPEC>`"]
pub type TSNCR = crate::Reg<tsncr::TSNCR_SPEC>;
#[doc = "desc TSNCR"]
pub mod tsncr;
#[doc = "ACBTR (rw) register accessor: an alias for `Reg<ACBTR_SPEC>`"]
pub type ACBTR = crate::Reg<acbtr::ACBTR_SPEC>;
#[doc = "desc ACBTR"]
pub mod acbtr;
#[doc = "RLSSP (rw) register accessor: an alias for `Reg<RLSSP_SPEC>`"]
pub type RLSSP = crate::Reg<rlssp::RLSSP_SPEC>;
#[doc = "desc RLSSP"]
pub mod rlssp;
#[doc = "IFR (rw) register accessor: an alias for `Reg<IFR_SPEC>`"]
pub type IFR = crate::Reg<ifr::IFR_SPEC>;
#[doc = "desc IFR"]
pub mod ifr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "desc IER"]
pub mod ier;
#[doc = "TSR (r) register accessor: an alias for `Reg<TSR_SPEC>`"]
pub type TSR = crate::Reg<tsr::TSR_SPEC>;
#[doc = "desc TSR"]
pub mod tsr;
#[doc = "MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "desc MCR"]
pub mod mcr;
#[doc = "WECR (rw) register accessor: an alias for `Reg<WECR_SPEC>`"]
pub type WECR = crate::Reg<wecr::WECR_SPEC>;
#[doc = "desc WECR"]
pub mod wecr;
#[doc = "REFMSG (rw) register accessor: an alias for `Reg<REFMSG_SPEC>`"]
pub type REFMSG = crate::Reg<refmsg::REFMSG_SPEC>;
#[doc = "desc REFMSG"]
pub mod refmsg;
#[doc = "TTCR (rw) register accessor: an alias for `Reg<TTCR_SPEC>`"]
pub type TTCR = crate::Reg<ttcr::TTCR_SPEC>;
#[doc = "desc TTCR"]
pub mod ttcr;
#[doc = "TTTR (rw) register accessor: an alias for `Reg<TTTR_SPEC>`"]
pub type TTTR = crate::Reg<tttr::TTTR_SPEC>;
#[doc = "desc TTTR"]
pub mod tttr;
#[doc = "SCMS (rw) register accessor: an alias for `Reg<SCMS_SPEC>`"]
pub type SCMS = crate::Reg<scms::SCMS_SPEC>;
#[doc = "desc SCMS"]
pub mod scms;
#[doc = "ACFCR (rw) register accessor: an alias for `Reg<ACFCR_SPEC>`"]
pub type ACFCR = crate::Reg<acfcr::ACFCR_SPEC>;
#[doc = "desc ACFCR"]
pub mod acfcr;
