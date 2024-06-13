#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SYSCFG configuration register 1"]
    pub cfgr1: CFGR1,
    #[doc = "0x04 - SYSCFG configuration register 2"]
    pub cfgr2: CFGR2,
    #[doc = "0x08 - SYSCFG configuration register 3"]
    pub cfgr3: CFGR3,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - GPIOA noise filter enable register"]
    pub pa_ens: PA_ENS,
    #[doc = "0x14 - GPIOB noise filter enable register"]
    pub pb_ens: PB_ENS,
    #[doc = "0x18 - GPIOF noise filter enable register"]
    pub pf_ens: PF_ENS,
    #[doc = "0x1c - I2C type IO configure register"]
    pub io_cfg: IO_CFG,
    #[doc = "0x20 - GPIOA Analog2 enable register"]
    pub pa_ana2en: PA_ANA2EN,
    #[doc = "0x24 - GPIOB Analog2 enable register"]
    pub pb_ana2en: PB_ANA2EN,
    #[doc = "0x28 - GPIOF Analog2 enable register"]
    pub pf_ana2en: PF_ANA2EN,
}
#[doc = "CFGR1 (rw) register accessor: an alias for `Reg<CFGR1_SPEC>`"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
#[doc = "SYSCFG configuration register 1"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "SYSCFG configuration register 2"]
pub mod cfgr2;
#[doc = "CFGR3 (rw) register accessor: an alias for `Reg<CFGR3_SPEC>`"]
pub type CFGR3 = crate::Reg<cfgr3::CFGR3_SPEC>;
#[doc = "SYSCFG configuration register 3"]
pub mod cfgr3;
#[doc = "PA_ENS (rw) register accessor: an alias for `Reg<PA_ENS_SPEC>`"]
pub type PA_ENS = crate::Reg<pa_ens::PA_ENS_SPEC>;
#[doc = "GPIOA noise filter enable register"]
pub mod pa_ens;
#[doc = "PB_ENS (rw) register accessor: an alias for `Reg<PB_ENS_SPEC>`"]
pub type PB_ENS = crate::Reg<pb_ens::PB_ENS_SPEC>;
#[doc = "GPIOB noise filter enable register"]
pub mod pb_ens;
#[doc = "PF_ENS (rw) register accessor: an alias for `Reg<PF_ENS_SPEC>`"]
pub type PF_ENS = crate::Reg<pf_ens::PF_ENS_SPEC>;
#[doc = "GPIOF noise filter enable register"]
pub mod pf_ens;
#[doc = "IO_CFG (rw) register accessor: an alias for `Reg<IO_CFG_SPEC>`"]
pub type IO_CFG = crate::Reg<io_cfg::IO_CFG_SPEC>;
#[doc = "I2C type IO configure register"]
pub mod io_cfg;
#[doc = "PA_ANA2EN (rw) register accessor: an alias for `Reg<PA_ANA2EN_SPEC>`"]
pub type PA_ANA2EN = crate::Reg<pa_ana2en::PA_ANA2EN_SPEC>;
#[doc = "GPIOA Analog2 enable register"]
pub mod pa_ana2en;
#[doc = "PB_ANA2EN (rw) register accessor: an alias for `Reg<PB_ANA2EN_SPEC>`"]
pub type PB_ANA2EN = crate::Reg<pb_ana2en::PB_ANA2EN_SPEC>;
#[doc = "GPIOB Analog2 enable register"]
pub mod pb_ana2en;
#[doc = "PF_ANA2EN (rw) register accessor: an alias for `Reg<PF_ANA2EN_SPEC>`"]
pub type PF_ANA2EN = crate::Reg<pf_ana2en::PF_ANA2EN_SPEC>;
#[doc = "GPIOF Analog2 enable register"]
pub mod pf_ana2en;
