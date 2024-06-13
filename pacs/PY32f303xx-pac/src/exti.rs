#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc IMR"]
    pub imr: IMR,
    #[doc = "0x04 - desc EMR"]
    pub emr: EMR,
    #[doc = "0x08 - desc RTSR"]
    pub rtsr: RTSR,
    #[doc = "0x0c - desc FTSR"]
    pub ftsr: FTSR,
    #[doc = "0x10 - desc SWIER"]
    pub swier: SWIER,
    #[doc = "0x14 - desc PR"]
    pub pr: PR,
}
#[doc = "IMR (rw) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "desc IMR"]
pub mod imr;
#[doc = "EMR (rw) register accessor: an alias for `Reg<EMR_SPEC>`"]
pub type EMR = crate::Reg<emr::EMR_SPEC>;
#[doc = "desc EMR"]
pub mod emr;
#[doc = "RTSR (rw) register accessor: an alias for `Reg<RTSR_SPEC>`"]
pub type RTSR = crate::Reg<rtsr::RTSR_SPEC>;
#[doc = "desc RTSR"]
pub mod rtsr;
#[doc = "FTSR (rw) register accessor: an alias for `Reg<FTSR_SPEC>`"]
pub type FTSR = crate::Reg<ftsr::FTSR_SPEC>;
#[doc = "desc FTSR"]
pub mod ftsr;
#[doc = "SWIER (rw) register accessor: an alias for `Reg<SWIER_SPEC>`"]
pub type SWIER = crate::Reg<swier::SWIER_SPEC>;
#[doc = "desc SWIER"]
pub mod swier;
#[doc = "PR (rw) register accessor: an alias for `Reg<PR_SPEC>`"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "desc PR"]
pub mod pr;
