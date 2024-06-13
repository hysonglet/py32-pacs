#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc SR"]
    pub sr: SR,
    #[doc = "0x04 - desc CR1"]
    pub cr1: CR1,
    #[doc = "0x08 - desc CR2"]
    pub cr2: CR2,
    #[doc = "0x0c - desc SMPR1"]
    pub smpr1: SMPR1,
    #[doc = "0x10 - desc SMPR2"]
    pub smpr2: SMPR2,
    #[doc = "0x14 - desc SMPR2"]
    pub smpr3: SMPR3,
    #[doc = "0x18 - desc JOFR1"]
    pub jofr1: JOFR1,
    #[doc = "0x1c - desc JOFR2"]
    pub jofr2: JOFR2,
    #[doc = "0x20 - desc JOFR3"]
    pub jofr3: JOFR3,
    #[doc = "0x24 - desc JOFR4"]
    pub jofr4: JOFR4,
    #[doc = "0x28 - desc HTR"]
    pub htr: HTR,
    #[doc = "0x2c - desc LTR"]
    pub ltr: LTR,
    #[doc = "0x30 - desc SQR1"]
    pub sqr1: SQR1,
    #[doc = "0x34 - desc SQR2"]
    pub sqr2: SQR2,
    #[doc = "0x38 - desc SQR3"]
    pub sqr3: SQR3,
    #[doc = "0x3c - desc JSQR"]
    pub jsqr: JSQR,
    #[doc = "0x40 - desc JDR1"]
    pub jdr1: JDR1,
    #[doc = "0x44 - desc JDR2"]
    pub jdr2: JDR2,
    #[doc = "0x48 - desc JDR3"]
    pub jdr3: JDR3,
    #[doc = "0x4c - desc JDR4"]
    pub jdr4: JDR4,
    #[doc = "0x50 - desc DR"]
    pub dr: DR,
    #[doc = "0x54 - desc CCSR"]
    pub ccsr: CCSR,
}
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "desc SR"]
pub mod sr;
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "desc CR1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "desc CR2"]
pub mod cr2;
#[doc = "SMPR1 (rw) register accessor: an alias for `Reg<SMPR1_SPEC>`"]
pub type SMPR1 = crate::Reg<smpr1::SMPR1_SPEC>;
#[doc = "desc SMPR1"]
pub mod smpr1;
#[doc = "SMPR2 (rw) register accessor: an alias for `Reg<SMPR2_SPEC>`"]
pub type SMPR2 = crate::Reg<smpr2::SMPR2_SPEC>;
#[doc = "desc SMPR2"]
pub mod smpr2;
#[doc = "SMPR3 (rw) register accessor: an alias for `Reg<SMPR3_SPEC>`"]
pub type SMPR3 = crate::Reg<smpr3::SMPR3_SPEC>;
#[doc = "desc SMPR2"]
pub mod smpr3;
#[doc = "JOFR1 (rw) register accessor: an alias for `Reg<JOFR1_SPEC>`"]
pub type JOFR1 = crate::Reg<jofr1::JOFR1_SPEC>;
#[doc = "desc JOFR1"]
pub mod jofr1;
#[doc = "JOFR2 (rw) register accessor: an alias for `Reg<JOFR2_SPEC>`"]
pub type JOFR2 = crate::Reg<jofr2::JOFR2_SPEC>;
#[doc = "desc JOFR2"]
pub mod jofr2;
#[doc = "JOFR3 (rw) register accessor: an alias for `Reg<JOFR3_SPEC>`"]
pub type JOFR3 = crate::Reg<jofr3::JOFR3_SPEC>;
#[doc = "desc JOFR3"]
pub mod jofr3;
#[doc = "JOFR4 (rw) register accessor: an alias for `Reg<JOFR4_SPEC>`"]
pub type JOFR4 = crate::Reg<jofr4::JOFR4_SPEC>;
#[doc = "desc JOFR4"]
pub mod jofr4;
#[doc = "HTR (rw) register accessor: an alias for `Reg<HTR_SPEC>`"]
pub type HTR = crate::Reg<htr::HTR_SPEC>;
#[doc = "desc HTR"]
pub mod htr;
#[doc = "LTR (rw) register accessor: an alias for `Reg<LTR_SPEC>`"]
pub type LTR = crate::Reg<ltr::LTR_SPEC>;
#[doc = "desc LTR"]
pub mod ltr;
#[doc = "SQR1 (rw) register accessor: an alias for `Reg<SQR1_SPEC>`"]
pub type SQR1 = crate::Reg<sqr1::SQR1_SPEC>;
#[doc = "desc SQR1"]
pub mod sqr1;
#[doc = "SQR2 (rw) register accessor: an alias for `Reg<SQR2_SPEC>`"]
pub type SQR2 = crate::Reg<sqr2::SQR2_SPEC>;
#[doc = "desc SQR2"]
pub mod sqr2;
#[doc = "SQR3 (rw) register accessor: an alias for `Reg<SQR3_SPEC>`"]
pub type SQR3 = crate::Reg<sqr3::SQR3_SPEC>;
#[doc = "desc SQR3"]
pub mod sqr3;
#[doc = "JSQR (rw) register accessor: an alias for `Reg<JSQR_SPEC>`"]
pub type JSQR = crate::Reg<jsqr::JSQR_SPEC>;
#[doc = "desc JSQR"]
pub mod jsqr;
#[doc = "JDR1 (r) register accessor: an alias for `Reg<JDR1_SPEC>`"]
pub type JDR1 = crate::Reg<jdr1::JDR1_SPEC>;
#[doc = "desc JDR1"]
pub mod jdr1;
#[doc = "JDR2 (r) register accessor: an alias for `Reg<JDR2_SPEC>`"]
pub type JDR2 = crate::Reg<jdr2::JDR2_SPEC>;
#[doc = "desc JDR2"]
pub mod jdr2;
#[doc = "JDR3 (r) register accessor: an alias for `Reg<JDR3_SPEC>`"]
pub type JDR3 = crate::Reg<jdr3::JDR3_SPEC>;
#[doc = "desc JDR3"]
pub mod jdr3;
#[doc = "JDR4 (r) register accessor: an alias for `Reg<JDR4_SPEC>`"]
pub type JDR4 = crate::Reg<jdr4::JDR4_SPEC>;
#[doc = "desc JDR4"]
pub mod jdr4;
#[doc = "DR (r) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "desc DR"]
pub mod dr;
#[doc = "CCSR (rw) register accessor: an alias for `Reg<CCSR_SPEC>`"]
pub type CCSR = crate::Reg<ccsr::CCSR_SPEC>;
#[doc = "desc CCSR"]
pub mod ccsr;
