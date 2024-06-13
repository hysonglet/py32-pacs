#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc CFGR1"]
    pub cfgr1: CFGR1,
    #[doc = "0x04 - desc CFGR2"]
    pub cfgr2: CFGR2,
    #[doc = "0x08 - desc CFGR3"]
    pub cfgr3: CFGR3,
    #[doc = "0x0c - desc CFGR4"]
    pub cfgr4: CFGR4,
    #[doc = "0x10 - desc CFGR5"]
    pub cfgr5: CFGR5,
    #[doc = "0x14 - desc EXTICR1"]
    pub exticr1: EXTICR1,
    #[doc = "0x18 - desc EXTICR2"]
    pub exticr2: EXTICR2,
    #[doc = "0x1c - desc EXTICR3"]
    pub exticr3: EXTICR3,
    #[doc = "0x20 - desc EXTICR4"]
    pub exticr4: EXTICR4,
    #[doc = "0x24 - desc PAENS"]
    pub paens: PAENS,
    #[doc = "0x28 - desc PBENS"]
    pub pbens: PBENS,
    #[doc = "0x2c - desc PCENS"]
    pub pcens: PCENS,
    #[doc = "0x30 - desc PDENS"]
    pub pdens: PDENS,
    #[doc = "0x34 - desc PEENS"]
    pub peens: PEENS,
    #[doc = "0x38 - desc GPIOENA"]
    pub gpioena: GPIOENA,
    _reserved15: [u8; 0x0140],
    #[doc = "0x17c - desc GPIOENA"]
    pub tim_clk_ext: TIM_CLK_EXT,
}
#[doc = "CFGR1 (rw) register accessor: an alias for `Reg<CFGR1_SPEC>`"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
#[doc = "desc CFGR1"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "desc CFGR2"]
pub mod cfgr2;
#[doc = "CFGR3 (rw) register accessor: an alias for `Reg<CFGR3_SPEC>`"]
pub type CFGR3 = crate::Reg<cfgr3::CFGR3_SPEC>;
#[doc = "desc CFGR3"]
pub mod cfgr3;
#[doc = "CFGR4 (rw) register accessor: an alias for `Reg<CFGR4_SPEC>`"]
pub type CFGR4 = crate::Reg<cfgr4::CFGR4_SPEC>;
#[doc = "desc CFGR4"]
pub mod cfgr4;
#[doc = "CFGR5 (rw) register accessor: an alias for `Reg<CFGR5_SPEC>`"]
pub type CFGR5 = crate::Reg<cfgr5::CFGR5_SPEC>;
#[doc = "desc CFGR5"]
pub mod cfgr5;
#[doc = "EXTICR1 (rw) register accessor: an alias for `Reg<EXTICR1_SPEC>`"]
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1_SPEC>;
#[doc = "desc EXTICR1"]
pub mod exticr1;
#[doc = "EXTICR2 (rw) register accessor: an alias for `Reg<EXTICR2_SPEC>`"]
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2_SPEC>;
#[doc = "desc EXTICR2"]
pub mod exticr2;
#[doc = "EXTICR3 (rw) register accessor: an alias for `Reg<EXTICR3_SPEC>`"]
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3_SPEC>;
#[doc = "desc EXTICR3"]
pub mod exticr3;
#[doc = "EXTICR4 (rw) register accessor: an alias for `Reg<EXTICR4_SPEC>`"]
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4_SPEC>;
#[doc = "desc EXTICR4"]
pub mod exticr4;
#[doc = "PAENS (rw) register accessor: an alias for `Reg<PAENS_SPEC>`"]
pub type PAENS = crate::Reg<paens::PAENS_SPEC>;
#[doc = "desc PAENS"]
pub mod paens;
#[doc = "PBENS (rw) register accessor: an alias for `Reg<PBENS_SPEC>`"]
pub type PBENS = crate::Reg<pbens::PBENS_SPEC>;
#[doc = "desc PBENS"]
pub mod pbens;
#[doc = "PCENS (rw) register accessor: an alias for `Reg<PCENS_SPEC>`"]
pub type PCENS = crate::Reg<pcens::PCENS_SPEC>;
#[doc = "desc PCENS"]
pub mod pcens;
#[doc = "PDENS (rw) register accessor: an alias for `Reg<PDENS_SPEC>`"]
pub type PDENS = crate::Reg<pdens::PDENS_SPEC>;
#[doc = "desc PDENS"]
pub mod pdens;
#[doc = "PEENS (rw) register accessor: an alias for `Reg<PEENS_SPEC>`"]
pub type PEENS = crate::Reg<peens::PEENS_SPEC>;
#[doc = "desc PEENS"]
pub mod peens;
#[doc = "GPIOENA (rw) register accessor: an alias for `Reg<GPIOENA_SPEC>`"]
pub type GPIOENA = crate::Reg<gpioena::GPIOENA_SPEC>;
#[doc = "desc GPIOENA"]
pub mod gpioena;
#[doc = "TIM_CLK_EXT (rw) register accessor: an alias for `Reg<TIM_CLK_EXT_SPEC>`"]
pub type TIM_CLK_EXT = crate::Reg<tim_clk_ext::TIM_CLK_EXT_SPEC>;
#[doc = "desc GPIOENA"]
pub mod tim_clk_ext;
