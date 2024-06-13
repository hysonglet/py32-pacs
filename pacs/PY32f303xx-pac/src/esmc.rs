#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc CR"]
    pub cr: CR,
    #[doc = "0x01 - desc CR2"]
    pub cr2: CR2,
    #[doc = "0x02 - desc TCR"]
    pub tcr: TCR,
    #[doc = "0x03 - desc BAUD"]
    pub baud: BAUD,
    #[doc = "0x04 - desc SFCR"]
    pub sfcr: SFCR,
    #[doc = "0x05 - desc SOCR"]
    pub socr: SOCR,
    #[doc = "0x06 - desc DCR"]
    pub dcr: DCR,
    #[doc = "0x07 - desc CR3"]
    pub cr3: CR3,
    #[doc = "0x08 - desc ADDR24"]
    pub addr24: ADDR24,
    #[doc = "0x0c - desc ADDR32"]
    pub addr32: ADDR32,
    _reserved10: [u8; 0x04],
    #[doc = "0x14 - desc SR"]
    pub sr: SR,
    #[doc = "0x15 - desc IFR"]
    pub ifr: IFR,
    #[doc = "0x16 - desc IER"]
    pub ier: IER,
    _reserved13: [u8; 0x05],
    #[doc = "0x1c - desc XSFCR"]
    pub xsfcr: XSFCR,
    #[doc = "0x1d - desc XSOCR"]
    pub xsocr: XSOCR,
    #[doc = "0x1e - desc DCR"]
    pub xdcr: XDCR,
    #[doc = "0x1f - desc XCR3"]
    pub xcr3: XCR3,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "desc CR2"]
pub mod cr2;
#[doc = "TCR (rw) register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "desc TCR"]
pub mod tcr;
#[doc = "BAUD (rw) register accessor: an alias for `Reg<BAUD_SPEC>`"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "desc BAUD"]
pub mod baud;
#[doc = "SFCR (rw) register accessor: an alias for `Reg<SFCR_SPEC>`"]
pub type SFCR = crate::Reg<sfcr::SFCR_SPEC>;
#[doc = "desc SFCR"]
pub mod sfcr;
#[doc = "SOCR (rw) register accessor: an alias for `Reg<SOCR_SPEC>`"]
pub type SOCR = crate::Reg<socr::SOCR_SPEC>;
#[doc = "desc SOCR"]
pub mod socr;
#[doc = "DCR (rw) register accessor: an alias for `Reg<DCR_SPEC>`"]
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
#[doc = "desc DCR"]
pub mod dcr;
#[doc = "CR3 (rw) register accessor: an alias for `Reg<CR3_SPEC>`"]
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
#[doc = "desc CR3"]
pub mod cr3;
#[doc = "ADDR24 (rw) register accessor: an alias for `Reg<ADDR24_SPEC>`"]
pub type ADDR24 = crate::Reg<addr24::ADDR24_SPEC>;
#[doc = "desc ADDR24"]
pub mod addr24;
#[doc = "ADDR32 (rw) register accessor: an alias for `Reg<ADDR32_SPEC>`"]
pub type ADDR32 = crate::Reg<addr32::ADDR32_SPEC>;
#[doc = "desc ADDR32"]
pub mod addr32;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "desc SR"]
pub mod sr;
#[doc = "IFR (rw) register accessor: an alias for `Reg<IFR_SPEC>`"]
pub type IFR = crate::Reg<ifr::IFR_SPEC>;
#[doc = "desc IFR"]
pub mod ifr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "desc IER"]
pub mod ier;
#[doc = "XSFCR (rw) register accessor: an alias for `Reg<XSFCR_SPEC>`"]
pub type XSFCR = crate::Reg<xsfcr::XSFCR_SPEC>;
#[doc = "desc XSFCR"]
pub mod xsfcr;
#[doc = "XSOCR (rw) register accessor: an alias for `Reg<XSOCR_SPEC>`"]
pub type XSOCR = crate::Reg<xsocr::XSOCR_SPEC>;
#[doc = "desc XSOCR"]
pub mod xsocr;
#[doc = "XDCR (rw) register accessor: an alias for `Reg<XDCR_SPEC>`"]
pub type XDCR = crate::Reg<xdcr::XDCR_SPEC>;
#[doc = "desc DCR"]
pub mod xdcr;
#[doc = "XCR3 (rw) register accessor: an alias for `Reg<XCR3_SPEC>`"]
pub type XCR3 = crate::Reg<xcr3::XCR3_SPEC>;
#[doc = "desc XCR3"]
pub mod xcr3;
