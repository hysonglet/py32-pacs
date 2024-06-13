#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc ACR"]
    pub acr: ACR,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - desc KEYR"]
    pub keyr: KEYR,
    #[doc = "0x0c - desc OPTKEYR"]
    pub optkeyr: OPTKEYR,
    #[doc = "0x10 - desc SR"]
    pub sr: SR,
    #[doc = "0x14 - desc CR"]
    pub cr: CR,
    _reserved5: [u8; 0x08],
    #[doc = "0x20 - desc OPTR"]
    pub optr: OPTR,
    #[doc = "0x24 - desc SDKR"]
    pub sdkr: SDKR,
    _reserved7: [u8; 0x04],
    #[doc = "0x2c - desc WRPR"]
    pub wrpr: WRPR,
    _reserved8: [u8; 0x60],
    #[doc = "0x90 - desc STCR"]
    pub stcr: STCR,
    _reserved9: [u8; 0x6c],
    #[doc = "0x100 - desc TS0"]
    pub ts0: TS0,
    #[doc = "0x104 - desc TS1"]
    pub ts1: TS1,
    #[doc = "0x108 - desc TS2P"]
    pub ts2p: TS2P,
    #[doc = "0x10c - desc TPS3"]
    pub tps3: TPS3,
    #[doc = "0x110 - desc TS3"]
    pub ts3: TS3,
    #[doc = "0x114 - desc PERTPE"]
    pub pertpe: PERTPE,
    #[doc = "0x118 - desc SMERTPE"]
    pub smertpe: SMERTPE,
    #[doc = "0x11c - desc PRGTPE"]
    pub prgtpe: PRGTPE,
    #[doc = "0x120 - desc PRETPE"]
    pub pretpe: PRETPE,
}
#[doc = "ACR (rw) register accessor: an alias for `Reg<ACR_SPEC>`"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "desc ACR"]
pub mod acr;
#[doc = "KEYR (w) register accessor: an alias for `Reg<KEYR_SPEC>`"]
pub type KEYR = crate::Reg<keyr::KEYR_SPEC>;
#[doc = "desc KEYR"]
pub mod keyr;
#[doc = "OPTKEYR (w) register accessor: an alias for `Reg<OPTKEYR_SPEC>`"]
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYR_SPEC>;
#[doc = "desc OPTKEYR"]
pub mod optkeyr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "desc SR"]
pub mod sr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "OPTR (r) register accessor: an alias for `Reg<OPTR_SPEC>`"]
pub type OPTR = crate::Reg<optr::OPTR_SPEC>;
#[doc = "desc OPTR"]
pub mod optr;
#[doc = "SDKR (r) register accessor: an alias for `Reg<SDKR_SPEC>`"]
pub type SDKR = crate::Reg<sdkr::SDKR_SPEC>;
#[doc = "desc SDKR"]
pub mod sdkr;
#[doc = "WRPR (rw) register accessor: an alias for `Reg<WRPR_SPEC>`"]
pub type WRPR = crate::Reg<wrpr::WRPR_SPEC>;
#[doc = "desc WRPR"]
pub mod wrpr;
#[doc = "STCR (rw) register accessor: an alias for `Reg<STCR_SPEC>`"]
pub type STCR = crate::Reg<stcr::STCR_SPEC>;
#[doc = "desc STCR"]
pub mod stcr;
#[doc = "TS0 (rw) register accessor: an alias for `Reg<TS0_SPEC>`"]
pub type TS0 = crate::Reg<ts0::TS0_SPEC>;
#[doc = "desc TS0"]
pub mod ts0;
#[doc = "TS1 (rw) register accessor: an alias for `Reg<TS1_SPEC>`"]
pub type TS1 = crate::Reg<ts1::TS1_SPEC>;
#[doc = "desc TS1"]
pub mod ts1;
#[doc = "TS2P (rw) register accessor: an alias for `Reg<TS2P_SPEC>`"]
pub type TS2P = crate::Reg<ts2p::TS2P_SPEC>;
#[doc = "desc TS2P"]
pub mod ts2p;
#[doc = "TPS3 (rw) register accessor: an alias for `Reg<TPS3_SPEC>`"]
pub type TPS3 = crate::Reg<tps3::TPS3_SPEC>;
#[doc = "desc TPS3"]
pub mod tps3;
#[doc = "TS3 (rw) register accessor: an alias for `Reg<TS3_SPEC>`"]
pub type TS3 = crate::Reg<ts3::TS3_SPEC>;
#[doc = "desc TS3"]
pub mod ts3;
#[doc = "PERTPE (rw) register accessor: an alias for `Reg<PERTPE_SPEC>`"]
pub type PERTPE = crate::Reg<pertpe::PERTPE_SPEC>;
#[doc = "desc PERTPE"]
pub mod pertpe;
#[doc = "SMERTPE (rw) register accessor: an alias for `Reg<SMERTPE_SPEC>`"]
pub type SMERTPE = crate::Reg<smertpe::SMERTPE_SPEC>;
#[doc = "desc SMERTPE"]
pub mod smertpe;
#[doc = "PRGTPE (rw) register accessor: an alias for `Reg<PRGTPE_SPEC>`"]
pub type PRGTPE = crate::Reg<prgtpe::PRGTPE_SPEC>;
#[doc = "desc PRGTPE"]
pub mod prgtpe;
#[doc = "PRETPE (rw) register accessor: an alias for `Reg<PRETPE_SPEC>`"]
pub type PRETPE = crate::Reg<pretpe::PRETPE_SPEC>;
#[doc = "desc PRETPE"]
pub mod pretpe;
