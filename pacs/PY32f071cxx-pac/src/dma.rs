#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc ISR"]
    pub isr: ISR,
    #[doc = "0x04 - desc IFCR"]
    pub ifcr: IFCR,
    #[doc = "0x08 - desc CCR1"]
    pub ccr1: CCR1,
    #[doc = "0x0c - desc CNDTR1"]
    pub cndtr1: CNDTR1,
    #[doc = "0x10 - desc CPAR1"]
    pub cpar1: CPAR1,
    #[doc = "0x14 - desc CMAR1"]
    pub cmar1: CMAR1,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - desc CCR2"]
    pub ccr2: CCR2,
    #[doc = "0x20 - desc CNDTR2"]
    pub cndtr2: CNDTR2,
    #[doc = "0x24 - desc CPAR2"]
    pub cpar2: CPAR2,
    #[doc = "0x28 - desc CMAR2"]
    pub cmar2: CMAR2,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - desc CCR3"]
    pub ccr3: CCR3,
    #[doc = "0x34 - desc CNDTR3"]
    pub cndtr3: CNDTR3,
    #[doc = "0x38 - desc CPAR3"]
    pub cpar3: CPAR3,
    #[doc = "0x3c - desc CMAR3"]
    pub cmar3: CMAR3,
    _reserved14: [u8; 0x04],
    #[doc = "0x44 - desc CCR4"]
    pub ccr4: CCR4,
    #[doc = "0x48 - desc CNDTR4"]
    pub cndtr4: CNDTR4,
    #[doc = "0x4c - desc CPAR4"]
    pub cpar4: CPAR4,
    #[doc = "0x50 - desc CMAR4"]
    pub cmar4: CMAR4,
    _reserved18: [u8; 0x04],
    #[doc = "0x58 - desc CCR5"]
    pub ccr5: CCR5,
    #[doc = "0x5c - desc CNDTR5"]
    pub cndtr5: CNDTR5,
    #[doc = "0x60 - desc CPAR5"]
    pub cpar5: CPAR5,
    #[doc = "0x64 - desc CMAR5"]
    pub cmar5: CMAR5,
    _reserved22: [u8; 0x04],
    #[doc = "0x6c - desc CCR6"]
    pub ccr6: CCR6,
    #[doc = "0x70 - desc CNDTR6"]
    pub cndtr6: CNDTR6,
    #[doc = "0x74 - desc CPAR6"]
    pub cpar6: CPAR6,
    #[doc = "0x78 - desc CMAR6"]
    pub cmar6: CMAR6,
    _reserved26: [u8; 0x04],
    #[doc = "0x80 - desc CCR7"]
    pub ccr7: CCR7,
    #[doc = "0x84 - desc CNDTR7"]
    pub cndtr7: CNDTR7,
    #[doc = "0x88 - desc CPAR7"]
    pub cpar7: CPAR7,
    #[doc = "0x8c - desc CMAR7"]
    pub cmar7: CMAR7,
}
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "desc ISR"]
pub mod isr;
#[doc = "IFCR (w) register accessor: an alias for `Reg<IFCR_SPEC>`"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "desc IFCR"]
pub mod ifcr;
#[doc = "CCR1 (rw) register accessor: an alias for `Reg<CCR1_SPEC>`"]
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
#[doc = "desc CCR1"]
pub mod ccr1;
#[doc = "CNDTR1 (rw) register accessor: an alias for `Reg<CNDTR1_SPEC>`"]
pub type CNDTR1 = crate::Reg<cndtr1::CNDTR1_SPEC>;
#[doc = "desc CNDTR1"]
pub mod cndtr1;
#[doc = "CPAR1 (rw) register accessor: an alias for `Reg<CPAR1_SPEC>`"]
pub type CPAR1 = crate::Reg<cpar1::CPAR1_SPEC>;
#[doc = "desc CPAR1"]
pub mod cpar1;
#[doc = "CMAR1 (rw) register accessor: an alias for `Reg<CMAR1_SPEC>`"]
pub type CMAR1 = crate::Reg<cmar1::CMAR1_SPEC>;
#[doc = "desc CMAR1"]
pub mod cmar1;
#[doc = "CCR2 (rw) register accessor: an alias for `Reg<CCR2_SPEC>`"]
pub type CCR2 = crate::Reg<ccr2::CCR2_SPEC>;
#[doc = "desc CCR2"]
pub mod ccr2;
#[doc = "CNDTR2 (rw) register accessor: an alias for `Reg<CNDTR2_SPEC>`"]
pub type CNDTR2 = crate::Reg<cndtr2::CNDTR2_SPEC>;
#[doc = "desc CNDTR2"]
pub mod cndtr2;
#[doc = "CPAR2 (rw) register accessor: an alias for `Reg<CPAR2_SPEC>`"]
pub type CPAR2 = crate::Reg<cpar2::CPAR2_SPEC>;
#[doc = "desc CPAR2"]
pub mod cpar2;
#[doc = "CMAR2 (rw) register accessor: an alias for `Reg<CMAR2_SPEC>`"]
pub type CMAR2 = crate::Reg<cmar2::CMAR2_SPEC>;
#[doc = "desc CMAR2"]
pub mod cmar2;
#[doc = "CCR3 (rw) register accessor: an alias for `Reg<CCR3_SPEC>`"]
pub type CCR3 = crate::Reg<ccr3::CCR3_SPEC>;
#[doc = "desc CCR3"]
pub mod ccr3;
#[doc = "CNDTR3 (rw) register accessor: an alias for `Reg<CNDTR3_SPEC>`"]
pub type CNDTR3 = crate::Reg<cndtr3::CNDTR3_SPEC>;
#[doc = "desc CNDTR3"]
pub mod cndtr3;
#[doc = "CPAR3 (rw) register accessor: an alias for `Reg<CPAR3_SPEC>`"]
pub type CPAR3 = crate::Reg<cpar3::CPAR3_SPEC>;
#[doc = "desc CPAR3"]
pub mod cpar3;
#[doc = "CMAR3 (rw) register accessor: an alias for `Reg<CMAR3_SPEC>`"]
pub type CMAR3 = crate::Reg<cmar3::CMAR3_SPEC>;
#[doc = "desc CMAR3"]
pub mod cmar3;
#[doc = "CCR4 (rw) register accessor: an alias for `Reg<CCR4_SPEC>`"]
pub type CCR4 = crate::Reg<ccr4::CCR4_SPEC>;
#[doc = "desc CCR4"]
pub mod ccr4;
#[doc = "CNDTR4 (rw) register accessor: an alias for `Reg<CNDTR4_SPEC>`"]
pub type CNDTR4 = crate::Reg<cndtr4::CNDTR4_SPEC>;
#[doc = "desc CNDTR4"]
pub mod cndtr4;
#[doc = "CPAR4 (rw) register accessor: an alias for `Reg<CPAR4_SPEC>`"]
pub type CPAR4 = crate::Reg<cpar4::CPAR4_SPEC>;
#[doc = "desc CPAR4"]
pub mod cpar4;
#[doc = "CMAR4 (rw) register accessor: an alias for `Reg<CMAR4_SPEC>`"]
pub type CMAR4 = crate::Reg<cmar4::CMAR4_SPEC>;
#[doc = "desc CMAR4"]
pub mod cmar4;
#[doc = "CCR5 (rw) register accessor: an alias for `Reg<CCR5_SPEC>`"]
pub type CCR5 = crate::Reg<ccr5::CCR5_SPEC>;
#[doc = "desc CCR5"]
pub mod ccr5;
#[doc = "CNDTR5 (rw) register accessor: an alias for `Reg<CNDTR5_SPEC>`"]
pub type CNDTR5 = crate::Reg<cndtr5::CNDTR5_SPEC>;
#[doc = "desc CNDTR5"]
pub mod cndtr5;
#[doc = "CPAR5 (rw) register accessor: an alias for `Reg<CPAR5_SPEC>`"]
pub type CPAR5 = crate::Reg<cpar5::CPAR5_SPEC>;
#[doc = "desc CPAR5"]
pub mod cpar5;
#[doc = "CMAR5 (rw) register accessor: an alias for `Reg<CMAR5_SPEC>`"]
pub type CMAR5 = crate::Reg<cmar5::CMAR5_SPEC>;
#[doc = "desc CMAR5"]
pub mod cmar5;
#[doc = "CCR6 (rw) register accessor: an alias for `Reg<CCR6_SPEC>`"]
pub type CCR6 = crate::Reg<ccr6::CCR6_SPEC>;
#[doc = "desc CCR6"]
pub mod ccr6;
#[doc = "CNDTR6 (rw) register accessor: an alias for `Reg<CNDTR6_SPEC>`"]
pub type CNDTR6 = crate::Reg<cndtr6::CNDTR6_SPEC>;
#[doc = "desc CNDTR6"]
pub mod cndtr6;
#[doc = "CPAR6 (rw) register accessor: an alias for `Reg<CPAR6_SPEC>`"]
pub type CPAR6 = crate::Reg<cpar6::CPAR6_SPEC>;
#[doc = "desc CPAR6"]
pub mod cpar6;
#[doc = "CMAR6 (rw) register accessor: an alias for `Reg<CMAR6_SPEC>`"]
pub type CMAR6 = crate::Reg<cmar6::CMAR6_SPEC>;
#[doc = "desc CMAR6"]
pub mod cmar6;
#[doc = "CCR7 (rw) register accessor: an alias for `Reg<CCR7_SPEC>`"]
pub type CCR7 = crate::Reg<ccr7::CCR7_SPEC>;
#[doc = "desc CCR7"]
pub mod ccr7;
#[doc = "CNDTR7 (rw) register accessor: an alias for `Reg<CNDTR7_SPEC>`"]
pub type CNDTR7 = crate::Reg<cndtr7::CNDTR7_SPEC>;
#[doc = "desc CNDTR7"]
pub mod cndtr7;
#[doc = "CPAR7 (rw) register accessor: an alias for `Reg<CPAR7_SPEC>`"]
pub type CPAR7 = crate::Reg<cpar7::CPAR7_SPEC>;
#[doc = "desc CPAR7"]
pub mod cpar7;
#[doc = "CMAR7 (rw) register accessor: an alias for `Reg<CMAR7_SPEC>`"]
pub type CMAR7 = crate::Reg<cmar7::CMAR7_SPEC>;
#[doc = "desc CMAR7"]
pub mod cmar7;
