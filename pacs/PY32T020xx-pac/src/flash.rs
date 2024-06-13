#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub acr: ACR,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - "]
    pub key: KEY,
    #[doc = "0x0c - "]
    pub optkey: OPTKEY,
    #[doc = "0x10 - "]
    pub sr: SR,
    #[doc = "0x14 - "]
    pub cr: CR,
    _reserved5: [u8; 0x08],
    #[doc = "0x20 - "]
    pub optr: OPTR,
    #[doc = "0x24 - "]
    pub sdkr: SDKR,
    #[doc = "0x28 - "]
    pub btcr: BTCR,
    #[doc = "0x2c - "]
    pub wrpr: WRPR,
    _reserved9: [u8; 0x60],
    #[doc = "0x90 - "]
    pub stcr: STCR,
    _reserved10: [u8; 0x8c],
    #[doc = "0x120 - "]
    pub pretpe: PRETPE,
}
#[doc = "ACR (rw) register accessor: an alias for `Reg<ACR_SPEC>`"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = ""]
pub mod acr;
#[doc = "KEY (rw) register accessor: an alias for `Reg<KEY_SPEC>`"]
pub type KEY = crate::Reg<key::KEY_SPEC>;
#[doc = ""]
pub mod key;
#[doc = "OPTKEY (rw) register accessor: an alias for `Reg<OPTKEY_SPEC>`"]
pub type OPTKEY = crate::Reg<optkey::OPTKEY_SPEC>;
#[doc = ""]
pub mod optkey;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = ""]
pub mod sr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = ""]
pub mod cr;
#[doc = "OPTR (rw) register accessor: an alias for `Reg<OPTR_SPEC>`"]
pub type OPTR = crate::Reg<optr::OPTR_SPEC>;
#[doc = ""]
pub mod optr;
#[doc = "SDKR (rw) register accessor: an alias for `Reg<SDKR_SPEC>`"]
pub type SDKR = crate::Reg<sdkr::SDKR_SPEC>;
#[doc = ""]
pub mod sdkr;
#[doc = "BTCR (rw) register accessor: an alias for `Reg<BTCR_SPEC>`"]
pub type BTCR = crate::Reg<btcr::BTCR_SPEC>;
#[doc = ""]
pub mod btcr;
#[doc = "WRPR (rw) register accessor: an alias for `Reg<WRPR_SPEC>`"]
pub type WRPR = crate::Reg<wrpr::WRPR_SPEC>;
#[doc = ""]
pub mod wrpr;
#[doc = "STCR (rw) register accessor: an alias for `Reg<STCR_SPEC>`"]
pub type STCR = crate::Reg<stcr::STCR_SPEC>;
#[doc = ""]
pub mod stcr;
#[doc = "PRETPE (rw) register accessor: an alias for `Reg<PRETPE_SPEC>`"]
pub type PRETPE = crate::Reg<pretpe::PRETPE_SPEC>;
#[doc = ""]
pub mod pretpe;
