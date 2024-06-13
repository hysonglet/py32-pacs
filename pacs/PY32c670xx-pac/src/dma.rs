#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register (DMA_ISR)"]
    pub isr: ISR,
    #[doc = "0x04 - DMA interrupt flag clear register (DMA_IFCR)"]
    pub ifcr: IFCR,
    #[doc = "0x08 - DMA channel configuration register (DMA_CCR)"]
    pub ccr1: CCR1,
    #[doc = "0x0c - DMA channel 1 number of data register"]
    pub cndtr1: CNDTR1,
    #[doc = "0x10 - DMA channel 1 peripheral address register"]
    pub cpar1: CPAR1,
    #[doc = "0x14 - DMA channel 1 memory address register"]
    pub cmar1: CMAR1,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - DMA channel configuration register (DMA_CCR)"]
    pub ccr2: CCR2,
    #[doc = "0x20 - DMA channel 2 number of data register"]
    pub cndtr2: CNDTR2,
    #[doc = "0x24 - DMA channel 2 peripheral address register"]
    pub cpar2: CPAR2,
    #[doc = "0x28 - DMA channel 2 memory address register"]
    pub cmar2: CMAR2,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - DMA channel configuration register (DMA_CCR)"]
    pub ccr3: CCR3,
    #[doc = "0x34 - DMA channel 3 number of data register"]
    pub cndtr3: CNDTR3,
    #[doc = "0x38 - DMA channel 3 peripheral address register"]
    pub cpar3: CPAR3,
    #[doc = "0x3c - DMA channel 3 memory address register"]
    pub cmar3: CMAR3,
}
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "DMA interrupt status register (DMA_ISR)"]
pub mod isr;
#[doc = "IFCR (w) register accessor: an alias for `Reg<IFCR_SPEC>`"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "DMA interrupt flag clear register (DMA_IFCR)"]
pub mod ifcr;
#[doc = "CCR1 (rw) register accessor: an alias for `Reg<CCR1_SPEC>`"]
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
#[doc = "DMA channel configuration register (DMA_CCR)"]
pub mod ccr1;
#[doc = "CNDTR1 (rw) register accessor: an alias for `Reg<CNDTR1_SPEC>`"]
pub type CNDTR1 = crate::Reg<cndtr1::CNDTR1_SPEC>;
#[doc = "DMA channel 1 number of data register"]
pub mod cndtr1;
#[doc = "CPAR1 (rw) register accessor: an alias for `Reg<CPAR1_SPEC>`"]
pub type CPAR1 = crate::Reg<cpar1::CPAR1_SPEC>;
#[doc = "DMA channel 1 peripheral address register"]
pub mod cpar1;
#[doc = "CMAR1 (rw) register accessor: an alias for `Reg<CMAR1_SPEC>`"]
pub type CMAR1 = crate::Reg<cmar1::CMAR1_SPEC>;
#[doc = "DMA channel 1 memory address register"]
pub mod cmar1;
#[doc = "CCR2 (rw) register accessor: an alias for `Reg<CCR2_SPEC>`"]
pub type CCR2 = crate::Reg<ccr2::CCR2_SPEC>;
#[doc = "DMA channel configuration register (DMA_CCR)"]
pub mod ccr2;
#[doc = "CNDTR2 (rw) register accessor: an alias for `Reg<CNDTR2_SPEC>`"]
pub type CNDTR2 = crate::Reg<cndtr2::CNDTR2_SPEC>;
#[doc = "DMA channel 2 number of data register"]
pub mod cndtr2;
#[doc = "CPAR2 (rw) register accessor: an alias for `Reg<CPAR2_SPEC>`"]
pub type CPAR2 = crate::Reg<cpar2::CPAR2_SPEC>;
#[doc = "DMA channel 2 peripheral address register"]
pub mod cpar2;
#[doc = "CMAR2 (rw) register accessor: an alias for `Reg<CMAR2_SPEC>`"]
pub type CMAR2 = crate::Reg<cmar2::CMAR2_SPEC>;
#[doc = "DMA channel 2 memory address register"]
pub mod cmar2;
#[doc = "CCR3 (rw) register accessor: an alias for `Reg<CCR3_SPEC>`"]
pub type CCR3 = crate::Reg<ccr3::CCR3_SPEC>;
#[doc = "DMA channel configuration register (DMA_CCR)"]
pub mod ccr3;
#[doc = "CNDTR3 (rw) register accessor: an alias for `Reg<CNDTR3_SPEC>`"]
pub type CNDTR3 = crate::Reg<cndtr3::CNDTR3_SPEC>;
#[doc = "DMA channel 3 number of data register"]
pub mod cndtr3;
#[doc = "CPAR3 (rw) register accessor: an alias for `Reg<CPAR3_SPEC>`"]
pub type CPAR3 = crate::Reg<cpar3::CPAR3_SPEC>;
#[doc = "DMA channel 3 peripheral address register"]
pub mod cpar3;
#[doc = "CMAR3 (rw) register accessor: an alias for `Reg<CMAR3_SPEC>`"]
pub type CMAR3 = crate::Reg<cmar3::CMAR3_SPEC>;
#[doc = "DMA channel 3 memory address register"]
pub mod cmar3;
