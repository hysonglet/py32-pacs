#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Dividend"]
    pub dend: DEND,
    #[doc = "0x04 - Divisor"]
    pub sor: SOR,
    #[doc = "0x08 - Quotient"]
    pub quot: QUOT,
    #[doc = "0x0c - Remainder"]
    pub rema: REMA,
    #[doc = "0x10 - des SIGN"]
    pub sign: SIGN,
    #[doc = "0x14 - des SIGN"]
    pub stat: STAT,
}
#[doc = "DEND (rw) register accessor: an alias for `Reg<DEND_SPEC>`"]
pub type DEND = crate::Reg<dend::DEND_SPEC>;
#[doc = "Dividend"]
pub mod dend;
#[doc = "SOR (rw) register accessor: an alias for `Reg<SOR_SPEC>`"]
pub type SOR = crate::Reg<sor::SOR_SPEC>;
#[doc = "Divisor"]
pub mod sor;
#[doc = "QUOT (r) register accessor: an alias for `Reg<QUOT_SPEC>`"]
pub type QUOT = crate::Reg<quot::QUOT_SPEC>;
#[doc = "Quotient"]
pub mod quot;
#[doc = "REMA (r) register accessor: an alias for `Reg<REMA_SPEC>`"]
pub type REMA = crate::Reg<rema::REMA_SPEC>;
#[doc = "Remainder"]
pub mod rema;
#[doc = "SIGN (rw) register accessor: an alias for `Reg<SIGN_SPEC>`"]
pub type SIGN = crate::Reg<sign::SIGN_SPEC>;
#[doc = "des SIGN"]
pub mod sign;
#[doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "des SIGN"]
pub mod stat;
