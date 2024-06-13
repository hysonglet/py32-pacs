#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc MODER"]
    pub moder: MODER,
    #[doc = "0x04 - desc OTYPER"]
    pub otyper: OTYPER,
    #[doc = "0x08 - desc OSPEEDR"]
    pub ospeedr: OSPEEDR,
    #[doc = "0x0c - desc PUPDR"]
    pub pupdr: PUPDR,
    #[doc = "0x10 - desc IDR"]
    pub idr: IDR,
    #[doc = "0x14 - desc ODR"]
    pub odr: ODR,
    #[doc = "0x18 - desc BSRR"]
    pub bsrr: BSRR,
    #[doc = "0x1c - desc LCKR"]
    pub lckr: LCKR,
    #[doc = "0x20 - desc AFRL"]
    pub afrl: AFRL,
    #[doc = "0x24 - desc AFRH"]
    pub afrh: AFRH,
    #[doc = "0x28 - desc BRR"]
    pub brr: BRR,
}
#[doc = "MODER (rw) register accessor: an alias for `Reg<MODER_SPEC>`"]
pub type MODER = crate::Reg<moder::MODER_SPEC>;
#[doc = "desc MODER"]
pub mod moder;
#[doc = "OTYPER (rw) register accessor: an alias for `Reg<OTYPER_SPEC>`"]
pub type OTYPER = crate::Reg<otyper::OTYPER_SPEC>;
#[doc = "desc OTYPER"]
pub mod otyper;
#[doc = "OSPEEDR (rw) register accessor: an alias for `Reg<OSPEEDR_SPEC>`"]
pub type OSPEEDR = crate::Reg<ospeedr::OSPEEDR_SPEC>;
#[doc = "desc OSPEEDR"]
pub mod ospeedr;
#[doc = "PUPDR (rw) register accessor: an alias for `Reg<PUPDR_SPEC>`"]
pub type PUPDR = crate::Reg<pupdr::PUPDR_SPEC>;
#[doc = "desc PUPDR"]
pub mod pupdr;
#[doc = "IDR (r) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "desc IDR"]
pub mod idr;
#[doc = "ODR (rw) register accessor: an alias for `Reg<ODR_SPEC>`"]
pub type ODR = crate::Reg<odr::ODR_SPEC>;
#[doc = "desc ODR"]
pub mod odr;
#[doc = "BSRR (w) register accessor: an alias for `Reg<BSRR_SPEC>`"]
pub type BSRR = crate::Reg<bsrr::BSRR_SPEC>;
#[doc = "desc BSRR"]
pub mod bsrr;
#[doc = "LCKR (rw) register accessor: an alias for `Reg<LCKR_SPEC>`"]
pub type LCKR = crate::Reg<lckr::LCKR_SPEC>;
#[doc = "desc LCKR"]
pub mod lckr;
#[doc = "AFRL (rw) register accessor: an alias for `Reg<AFRL_SPEC>`"]
pub type AFRL = crate::Reg<afrl::AFRL_SPEC>;
#[doc = "desc AFRL"]
pub mod afrl;
#[doc = "AFRH (rw) register accessor: an alias for `Reg<AFRH_SPEC>`"]
pub type AFRH = crate::Reg<afrh::AFRH_SPEC>;
#[doc = "desc AFRH"]
pub mod afrh;
#[doc = "BRR (w) register accessor: an alias for `Reg<BRR_SPEC>`"]
pub type BRR = crate::Reg<brr::BRR_SPEC>;
#[doc = "desc BRR"]
pub mod brr;
