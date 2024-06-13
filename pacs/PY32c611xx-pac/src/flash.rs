#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access control register"]
    pub acr: ACR,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Flash key register"]
    pub keyr: KEYR,
    #[doc = "0x0c - Option byte key register"]
    pub optkeyr: OPTKEYR,
    #[doc = "0x10 - Status register"]
    pub sr: SR,
    #[doc = "0x14 - Flash control register"]
    pub cr: CR,
    _reserved5: [u8; 0x08],
    #[doc = "0x20 - Flash option register"]
    pub optr: OPTR,
    #[doc = "0x24 - Flash SDK address register"]
    pub sdkr: SDKR,
    _reserved7: [u8; 0x04],
    #[doc = "0x2c - Flash WRP address register"]
    pub wrpr: WRPR,
    _reserved8: [u8; 0x60],
    #[doc = "0x90 - Flash sleep time config register"]
    pub stcr: STCR,
    _reserved9: [u8; 0x6c],
    #[doc = "0x100 - Flash TS0 register"]
    pub ts0: TS0,
    #[doc = "0x104 - Flash TS1 register"]
    pub ts1: TS1,
    #[doc = "0x108 - Flash TS2P register"]
    pub ts2p: TS2P,
    #[doc = "0x10c - Flash TPS3 register"]
    pub tps3: TPS3,
    #[doc = "0x110 - Flash TS3 register"]
    pub ts3: TS3,
    #[doc = "0x114 - Flash PERTPE register"]
    pub pertpe: PERTPE,
    #[doc = "0x118 - Flash SMERTPE register"]
    pub smertpe: SMERTPE,
    #[doc = "0x11c - Flash PRGTPE register"]
    pub prgtpe: PRGTPE,
    #[doc = "0x120 - Flash PRETPE register"]
    pub pretpe: PRETPE,
}
#[doc = "ACR (rw) register accessor: an alias for `Reg<ACR_SPEC>`"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "Access control register"]
pub mod acr;
#[doc = "KEYR (w) register accessor: an alias for `Reg<KEYR_SPEC>`"]
pub type KEYR = crate::Reg<keyr::KEYR_SPEC>;
#[doc = "Flash key register"]
pub mod keyr;
#[doc = "OPTKEYR (w) register accessor: an alias for `Reg<OPTKEYR_SPEC>`"]
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYR_SPEC>;
#[doc = "Option byte key register"]
pub mod optkeyr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Flash control register"]
pub mod cr;
#[doc = "OPTR (rw) register accessor: an alias for `Reg<OPTR_SPEC>`"]
pub type OPTR = crate::Reg<optr::OPTR_SPEC>;
#[doc = "Flash option register"]
pub mod optr;
#[doc = "SDKR (rw) register accessor: an alias for `Reg<SDKR_SPEC>`"]
pub type SDKR = crate::Reg<sdkr::SDKR_SPEC>;
#[doc = "Flash SDK address register"]
pub mod sdkr;
#[doc = "WRPR (rw) register accessor: an alias for `Reg<WRPR_SPEC>`"]
pub type WRPR = crate::Reg<wrpr::WRPR_SPEC>;
#[doc = "Flash WRP address register"]
pub mod wrpr;
#[doc = "STCR (rw) register accessor: an alias for `Reg<STCR_SPEC>`"]
pub type STCR = crate::Reg<stcr::STCR_SPEC>;
#[doc = "Flash sleep time config register"]
pub mod stcr;
#[doc = "TS0 (rw) register accessor: an alias for `Reg<TS0_SPEC>`"]
pub type TS0 = crate::Reg<ts0::TS0_SPEC>;
#[doc = "Flash TS0 register"]
pub mod ts0;
#[doc = "TS1 (rw) register accessor: an alias for `Reg<TS1_SPEC>`"]
pub type TS1 = crate::Reg<ts1::TS1_SPEC>;
#[doc = "Flash TS1 register"]
pub mod ts1;
#[doc = "TS2P (rw) register accessor: an alias for `Reg<TS2P_SPEC>`"]
pub type TS2P = crate::Reg<ts2p::TS2P_SPEC>;
#[doc = "Flash TS2P register"]
pub mod ts2p;
#[doc = "TPS3 (rw) register accessor: an alias for `Reg<TPS3_SPEC>`"]
pub type TPS3 = crate::Reg<tps3::TPS3_SPEC>;
#[doc = "Flash TPS3 register"]
pub mod tps3;
#[doc = "TS3 (rw) register accessor: an alias for `Reg<TS3_SPEC>`"]
pub type TS3 = crate::Reg<ts3::TS3_SPEC>;
#[doc = "Flash TS3 register"]
pub mod ts3;
#[doc = "PERTPE (rw) register accessor: an alias for `Reg<PERTPE_SPEC>`"]
pub type PERTPE = crate::Reg<pertpe::PERTPE_SPEC>;
#[doc = "Flash PERTPE register"]
pub mod pertpe;
#[doc = "SMERTPE (rw) register accessor: an alias for `Reg<SMERTPE_SPEC>`"]
pub type SMERTPE = crate::Reg<smertpe::SMERTPE_SPEC>;
#[doc = "Flash SMERTPE register"]
pub mod smertpe;
#[doc = "PRGTPE (rw) register accessor: an alias for `Reg<PRGTPE_SPEC>`"]
pub type PRGTPE = crate::Reg<prgtpe::PRGTPE_SPEC>;
#[doc = "Flash PRGTPE register"]
pub mod prgtpe;
#[doc = "PRETPE (rw) register accessor: an alias for `Reg<PRETPE_SPEC>`"]
pub type PRETPE = crate::Reg<pretpe::PRETPE_SPEC>;
#[doc = "Flash PRETPE register"]
pub mod pretpe;
