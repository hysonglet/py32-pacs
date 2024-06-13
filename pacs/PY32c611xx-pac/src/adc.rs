#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC interrupt and status register"]
    pub isr: ISR,
    #[doc = "0x04 - ADC interrupt enable register"]
    pub ier: IER,
    #[doc = "0x08 - ADC control register"]
    pub cr: CR,
    #[doc = "0x0c - ADC configuration register 1"]
    pub cfgr1: CFGR1,
    #[doc = "0x10 - ADC configuration register 2"]
    pub cfgr2: CFGR2,
    #[doc = "0x14 - ADC sampling time register"]
    pub smpr: SMPR,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - ADC analog watchdog 1 threshold register"]
    pub tr: TR,
    _reserved7: [u8; 0x04],
    #[doc = "0x28 - ADC group regular sequencer register"]
    pub chselr: CHSELR,
    _reserved8: [u8; 0x14],
    #[doc = "0x40 - ADC group regular data register"]
    pub dr: DR,
    #[doc = "0x44 - ADC calibration configuration and status register"]
    pub ccsr: CCSR,
    #[doc = "0x48 - ADC calibration result register 1"]
    pub calrr1: CALRR1,
    #[doc = "0x4c - ADC calibration result register 2"]
    pub calrr2: CALRR2,
    #[doc = "0x50 - ADC calibration factor input register 1"]
    pub calfir1: CALFIR1,
    #[doc = "0x54 - ADC calibration factor input register 2"]
    pub calfir2: CALFIR2,
    _reserved14: [u8; 0x02b0],
    #[doc = "0x308 - ADC common configuration register"]
    pub ccr: CCR,
}
#[doc = "ISR (rw) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "ADC interrupt and status register"]
pub mod isr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "ADC interrupt enable register"]
pub mod ier;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "ADC control register"]
pub mod cr;
#[doc = "CFGR1 (rw) register accessor: an alias for `Reg<CFGR1_SPEC>`"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
#[doc = "ADC configuration register 1"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "ADC configuration register 2"]
pub mod cfgr2;
#[doc = "SMPR (rw) register accessor: an alias for `Reg<SMPR_SPEC>`"]
pub type SMPR = crate::Reg<smpr::SMPR_SPEC>;
#[doc = "ADC sampling time register"]
pub mod smpr;
#[doc = "TR (rw) register accessor: an alias for `Reg<TR_SPEC>`"]
pub type TR = crate::Reg<tr::TR_SPEC>;
#[doc = "ADC analog watchdog 1 threshold register"]
pub mod tr;
#[doc = "CHSELR (rw) register accessor: an alias for `Reg<CHSELR_SPEC>`"]
pub type CHSELR = crate::Reg<chselr::CHSELR_SPEC>;
#[doc = "ADC group regular sequencer register"]
pub mod chselr;
#[doc = "DR (r) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "ADC group regular data register"]
pub mod dr;
#[doc = "CCSR (rw) register accessor: an alias for `Reg<CCSR_SPEC>`"]
pub type CCSR = crate::Reg<ccsr::CCSR_SPEC>;
#[doc = "ADC calibration configuration and status register"]
pub mod ccsr;
#[doc = "CALRR1 (r) register accessor: an alias for `Reg<CALRR1_SPEC>`"]
pub type CALRR1 = crate::Reg<calrr1::CALRR1_SPEC>;
#[doc = "ADC calibration result register 1"]
pub mod calrr1;
#[doc = "CALRR2 (r) register accessor: an alias for `Reg<CALRR2_SPEC>`"]
pub type CALRR2 = crate::Reg<calrr2::CALRR2_SPEC>;
#[doc = "ADC calibration result register 2"]
pub mod calrr2;
#[doc = "CALFIR1 (rw) register accessor: an alias for `Reg<CALFIR1_SPEC>`"]
pub type CALFIR1 = crate::Reg<calfir1::CALFIR1_SPEC>;
#[doc = "ADC calibration factor input register 1"]
pub mod calfir1;
#[doc = "CALFIR2 (rw) register accessor: an alias for `Reg<CALFIR2_SPEC>`"]
pub type CALFIR2 = crate::Reg<calfir2::CALFIR2_SPEC>;
#[doc = "ADC calibration factor input register 2"]
pub mod calfir2;
#[doc = "CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "ADC common configuration register"]
pub mod ccr;
