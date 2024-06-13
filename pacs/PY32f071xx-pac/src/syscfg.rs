#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc CFGR1"]
    pub cfgr1: CFGR1,
    #[doc = "0x04 - desc CFGR2"]
    pub cfgr2: CFGR2,
    #[doc = "0x08 - desc CFGR3"]
    pub cfgr3: CFGR3,
    #[doc = "0x0c - desc CFGR4"]
    pub cfgr4: CFGR4,
    #[doc = "0x10 - desc PAENS"]
    pub paens: PAENS,
    #[doc = "0x14 - desc PBENS"]
    pub pbens: PBENS,
    #[doc = "0x18 - desc PCENS"]
    pub pcens: PCENS,
    #[doc = "0x1c - desc PFENS"]
    pub pfens: PFENS,
}
#[doc = "CFGR1 (rw) register accessor: an alias for `Reg<CFGR1_SPEC>`"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
#[doc = "desc CFGR1"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "desc CFGR2"]
pub mod cfgr2;
#[doc = "CFGR3 (rw) register accessor: an alias for `Reg<CFGR3_SPEC>`"]
pub type CFGR3 = crate::Reg<cfgr3::CFGR3_SPEC>;
#[doc = "desc CFGR3"]
pub mod cfgr3;
#[doc = "CFGR4 (rw) register accessor: an alias for `Reg<CFGR4_SPEC>`"]
pub type CFGR4 = crate::Reg<cfgr4::CFGR4_SPEC>;
#[doc = "desc CFGR4"]
pub mod cfgr4;
#[doc = "PAENS (rw) register accessor: an alias for `Reg<PAENS_SPEC>`"]
pub type PAENS = crate::Reg<paens::PAENS_SPEC>;
#[doc = "desc PAENS"]
pub mod paens;
#[doc = "PBENS (rw) register accessor: an alias for `Reg<PBENS_SPEC>`"]
pub type PBENS = crate::Reg<pbens::PBENS_SPEC>;
#[doc = "desc PBENS"]
pub mod pbens;
#[doc = "PCENS (rw) register accessor: an alias for `Reg<PCENS_SPEC>`"]
pub type PCENS = crate::Reg<pcens::PCENS_SPEC>;
#[doc = "desc PCENS"]
pub mod pcens;
#[doc = "PFENS (rw) register accessor: an alias for `Reg<PFENS_SPEC>`"]
pub type PFENS = crate::Reg<pfens::PFENS_SPEC>;
#[doc = "desc PFENS"]
pub mod pfens;
