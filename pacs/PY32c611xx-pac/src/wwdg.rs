#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register (WWDG_CR)"]
    pub cr: CR,
    #[doc = "0x04 - Configuration register (WWDG_CFR)"]
    pub cfr: CFR,
    #[doc = "0x08 - Status register (WWDG_SR)"]
    pub sr: SR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control register (WWDG_CR)"]
pub mod cr;
#[doc = "CFR (rw) register accessor: an alias for `Reg<CFR_SPEC>`"]
pub type CFR = crate::Reg<cfr::CFR_SPEC>;
#[doc = "Configuration register (WWDG_CFR)"]
pub mod cfr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register (WWDG_SR)"]
pub mod sr;
