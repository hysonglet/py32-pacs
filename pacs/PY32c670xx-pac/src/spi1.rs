#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - control register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - status register"]
    pub sr: SR,
    #[doc = "0x0c - data register"]
    pub dr: DR,
    #[doc = "0x10 - CRC polynomial register"]
    pub crcpr: CRCPR,
    #[doc = "0x14 - RX CRC register"]
    pub rxcrc: RXCRC,
    #[doc = "0x18 - TX CRC register"]
    pub txcrc: TXCRC,
    #[doc = "0x1c - I2S Configuration register"]
    pub i2scfgr: I2SCFGR,
    #[doc = "0x20 - I2S prescaler register"]
    pub i2spr: I2SPR,
}
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "data register"]
pub mod dr;
#[doc = "CRCPR (rw) register accessor: an alias for `Reg<CRCPR_SPEC>`"]
pub type CRCPR = crate::Reg<crcpr::CRCPR_SPEC>;
#[doc = "CRC polynomial register"]
pub mod crcpr;
#[doc = "RXCRC (rw) register accessor: an alias for `Reg<RXCRC_SPEC>`"]
pub type RXCRC = crate::Reg<rxcrc::RXCRC_SPEC>;
#[doc = "RX CRC register"]
pub mod rxcrc;
#[doc = "TXCRC (rw) register accessor: an alias for `Reg<TXCRC_SPEC>`"]
pub type TXCRC = crate::Reg<txcrc::TXCRC_SPEC>;
#[doc = "TX CRC register"]
pub mod txcrc;
#[doc = "I2SCFGR (rw) register accessor: an alias for `Reg<I2SCFGR_SPEC>`"]
pub type I2SCFGR = crate::Reg<i2scfgr::I2SCFGR_SPEC>;
#[doc = "I2S Configuration register"]
pub mod i2scfgr;
#[doc = "I2SPR (rw) register accessor: an alias for `Reg<I2SPR_SPEC>`"]
pub type I2SPR = crate::Reg<i2spr::I2SPR_SPEC>;
#[doc = "I2S prescaler register"]
pub mod i2spr;
