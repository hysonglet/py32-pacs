#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc TSNCR"]
    pub tsncr: TSNCR,
    #[doc = "0x04 - desc ACBTR"]
    pub acbtr: ACBTR,
    #[doc = "0x08 - desc FDBTR"]
    pub fdbtr: FDBTR,
    #[doc = "0x0c - desc XLBTR"]
    pub xlbtr: XLBTR,
    #[doc = "0x10 - desc RLSSP"]
    pub rlssp: RLSSP,
    #[doc = "0x14 - desc IFR"]
    pub ifr: IFR,
    #[doc = "0x18 - desc IER"]
    pub ier: IER,
    #[doc = "0x1c - desc TSR"]
    pub tsr: TSR,
    #[doc = "0x20 - desc TTSL"]
    pub ttsl: TTSL,
    #[doc = "0x24 - desc TTSH"]
    pub ttsh: TTSH,
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
    #[doc = "0x40 - desc MESR"]
    pub mesr: MESR,
    #[doc = "0x44 - desc ACFCR"]
    pub acfcr: ACFCR,
    _reserved18: [u8; 0x1064],
    #[doc = "0x10ac - desc PWMCR"]
    pub pwmcr: PWMCR,
}
#[doc = "TSNCR (rw) register accessor: an alias for `Reg<TSNCR_SPEC>`"]
pub type TSNCR = crate::Reg<tsncr::TSNCR_SPEC>;
#[doc = "desc TSNCR"]
pub mod tsncr;
#[doc = "ACBTR (rw) register accessor: an alias for `Reg<ACBTR_SPEC>`"]
pub type ACBTR = crate::Reg<acbtr::ACBTR_SPEC>;
#[doc = "desc ACBTR"]
pub mod acbtr;
#[doc = "FDBTR (rw) register accessor: an alias for `Reg<FDBTR_SPEC>`"]
pub type FDBTR = crate::Reg<fdbtr::FDBTR_SPEC>;
#[doc = "desc FDBTR"]
pub mod fdbtr;
#[doc = "XLBTR (rw) register accessor: an alias for `Reg<XLBTR_SPEC>`"]
pub type XLBTR = crate::Reg<xlbtr::XLBTR_SPEC>;
#[doc = "desc XLBTR"]
pub mod xlbtr;
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
#[doc = "TTSL (r) register accessor: an alias for `Reg<TTSL_SPEC>`"]
pub type TTSL = crate::Reg<ttsl::TTSL_SPEC>;
#[doc = "desc TTSL"]
pub mod ttsl;
#[doc = "TTSH (r) register accessor: an alias for `Reg<TTSH_SPEC>`"]
pub type TTSH = crate::Reg<ttsh::TTSH_SPEC>;
#[doc = "desc TTSH"]
pub mod ttsh;
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
#[doc = "MESR (rw) register accessor: an alias for `Reg<MESR_SPEC>`"]
pub type MESR = crate::Reg<mesr::MESR_SPEC>;
#[doc = "desc MESR"]
pub mod mesr;
#[doc = "ACFCR (rw) register accessor: an alias for `Reg<ACFCR_SPEC>`"]
pub type ACFCR = crate::Reg<acfcr::ACFCR_SPEC>;
#[doc = "desc ACFCR"]
pub mod acfcr;
#[doc = "PWMCR (rw) register accessor: an alias for `Reg<PWMCR_SPEC>`"]
pub type PWMCR = crate::Reg<pwmcr::PWMCR_SPEC>;
#[doc = "desc PWMCR"]
pub mod pwmcr;
