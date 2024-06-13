#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc CTL0"]
    pub ctl0: CTL0,
    #[doc = "0x04 - desc CTL1"]
    pub ctl1: CTL1,
    #[doc = "0x08 - desc SR"]
    pub sr: SR,
    #[doc = "0x0c - desc INTC"]
    pub intc: INTC,
}
#[doc = "CTL0 (rw) register accessor: an alias for `Reg<CTL0_SPEC>`"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "desc CTL0"]
pub mod ctl0;
#[doc = "CTL1 (rw) register accessor: an alias for `Reg<CTL1_SPEC>`"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "desc CTL1"]
pub mod ctl1;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "desc SR"]
pub mod sr;
#[doc = "INTC (w) register accessor: an alias for `Reg<INTC_SPEC>`"]
pub type INTC = crate::Reg<intc::INTC_SPEC>;
#[doc = "desc INTC"]
pub mod intc;
