#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc CR1"]
    pub cr1: CR1,
    _reserved1: [u8; 0x08],
    #[doc = "0x0c - desc DIER"]
    pub dier: DIER,
    #[doc = "0x10 - desc SR"]
    pub sr: SR,
    #[doc = "0x14 - desc EGR"]
    pub egr: EGR,
    _reserved_4_ccmr1: [u8; 0x04],
    _reserved5: [u8; 0x04],
    #[doc = "0x20 - desc CCER"]
    pub ccer: CCER,
    #[doc = "0x24 - desc CNT"]
    pub cnt: CNT,
    #[doc = "0x28 - desc PSC"]
    pub psc: PSC,
    #[doc = "0x2c - desc ARR"]
    pub arr: ARR,
    _reserved9: [u8; 0x04],
    #[doc = "0x34 - desc CCR1"]
    pub ccr1: CCR1,
}
impl RegisterBlock {
    #[doc = "0x18 - desc CCMR1:INPUT"]
    #[inline(always)]
    pub fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const CCMR1_INPUT) }
    }
    #[doc = "0x18 - desc CCMR1:OUTPUT"]
    #[inline(always)]
    pub fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const CCMR1_OUTPUT) }
    }
}
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "desc CR1"]
pub mod cr1;
#[doc = "DIER (rw) register accessor: an alias for `Reg<DIER_SPEC>`"]
pub type DIER = crate::Reg<dier::DIER_SPEC>;
#[doc = "desc DIER"]
pub mod dier;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "desc SR"]
pub mod sr;
#[doc = "EGR (w) register accessor: an alias for `Reg<EGR_SPEC>`"]
pub type EGR = crate::Reg<egr::EGR_SPEC>;
#[doc = "desc EGR"]
pub mod egr;
#[doc = "CCMR1_OUTPUT (rw) register accessor: an alias for `Reg<CCMR1_OUTPUT_SPEC>`"]
pub type CCMR1_OUTPUT = crate::Reg<ccmr1_output::CCMR1_OUTPUT_SPEC>;
#[doc = "desc CCMR1:OUTPUT"]
pub mod ccmr1_output;
#[doc = "CCMR1_INPUT (rw) register accessor: an alias for `Reg<CCMR1_INPUT_SPEC>`"]
pub type CCMR1_INPUT = crate::Reg<ccmr1_input::CCMR1_INPUT_SPEC>;
#[doc = "desc CCMR1:INPUT"]
pub mod ccmr1_input;
#[doc = "CCER (rw) register accessor: an alias for `Reg<CCER_SPEC>`"]
pub type CCER = crate::Reg<ccer::CCER_SPEC>;
#[doc = "desc CCER"]
pub mod ccer;
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "desc CNT"]
pub mod cnt;
#[doc = "PSC (rw) register accessor: an alias for `Reg<PSC_SPEC>`"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "desc PSC"]
pub mod psc;
#[doc = "ARR (rw) register accessor: an alias for `Reg<ARR_SPEC>`"]
pub type ARR = crate::Reg<arr::ARR_SPEC>;
#[doc = "desc ARR"]
pub mod arr;
#[doc = "CCR1 (rw) register accessor: an alias for `Reg<CCR1_SPEC>`"]
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
#[doc = "desc CCR1"]
pub mod ccr1;