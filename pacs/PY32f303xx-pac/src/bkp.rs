#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - desc DR1"]
    pub dr1: DR1,
    #[doc = "0x08 - desc DR2"]
    pub dr2: DR2,
    #[doc = "0x0c - desc DR3"]
    pub dr3: DR3,
    #[doc = "0x10 - desc DR4"]
    pub dr4: DR4,
    #[doc = "0x14 - desc DR5"]
    pub dr5: DR5,
    #[doc = "0x18 - desc DR6"]
    pub dr6: DR6,
    #[doc = "0x1c - desc DR7"]
    pub dr7: DR7,
    #[doc = "0x20 - desc DR8"]
    pub dr8: DR8,
    #[doc = "0x24 - desc DR9"]
    pub dr9: DR9,
    #[doc = "0x28 - desc DR10"]
    pub dr10: DR10,
    #[doc = "0x2c - desc RTCCR"]
    pub rtccr: RTCCR,
    #[doc = "0x30 - desc CR"]
    pub cr: CR,
    #[doc = "0x34 - desc CSR"]
    pub csr: CSR,
    _reserved13: [u8; 0x08],
    #[doc = "0x40 - desc DR11"]
    pub dr11: DR11,
    #[doc = "0x44 - desc DR12"]
    pub dr12: DR12,
    #[doc = "0x48 - desc DR13"]
    pub dr13: DR13,
    #[doc = "0x4c - desc DR14"]
    pub dr14: DR14,
    #[doc = "0x50 - desc DR15"]
    pub dr15: DR15,
    #[doc = "0x54 - desc DR16"]
    pub dr16: DR16,
    #[doc = "0x58 - desc DR17"]
    pub dr17: DR17,
    #[doc = "0x5c - desc DR18"]
    pub dr18: DR18,
    #[doc = "0x60 - desc DR19"]
    pub dr19: DR19,
    #[doc = "0x64 - desc DR20"]
    pub dr20: DR20,
    #[doc = "0x68 - desc DR21"]
    pub dr21: DR21,
    #[doc = "0x6c - desc DR22"]
    pub dr22: DR22,
    #[doc = "0x70 - desc DR23"]
    pub dr23: DR23,
    #[doc = "0x74 - desc DR24"]
    pub dr24: DR24,
    #[doc = "0x78 - desc DR25"]
    pub dr25: DR25,
    #[doc = "0x7c - desc DR26"]
    pub dr26: DR26,
    #[doc = "0x80 - desc DR27"]
    pub dr27: DR27,
    #[doc = "0x84 - desc DR28"]
    pub dr28: DR28,
    #[doc = "0x88 - desc DR29"]
    pub dr29: DR29,
    #[doc = "0x8c - desc DR30"]
    pub dr30: DR30,
    #[doc = "0x90 - desc DR31"]
    pub dr31: DR31,
    #[doc = "0x94 - desc DR32"]
    pub dr32: DR32,
    #[doc = "0x98 - desc DR33"]
    pub dr33: DR33,
    #[doc = "0x9c - desc DR34"]
    pub dr34: DR34,
    #[doc = "0xa0 - desc DR35"]
    pub dr35: DR35,
    #[doc = "0xa4 - desc DR36"]
    pub dr36: DR36,
    #[doc = "0xa8 - desc DR37"]
    pub dr37: DR37,
    #[doc = "0xac - desc DR38"]
    pub dr38: DR38,
    #[doc = "0xb0 - desc DR39"]
    pub dr39: DR39,
    #[doc = "0xb4 - desc DR40"]
    pub dr40: DR40,
    #[doc = "0xb8 - desc DR41"]
    pub dr41: DR41,
    #[doc = "0xbc - desc DR42"]
    pub dr42: DR42,
}
#[doc = "DR1 (rw) register accessor: an alias for `Reg<DR1_SPEC>`"]
pub type DR1 = crate::Reg<dr1::DR1_SPEC>;
#[doc = "desc DR1"]
pub mod dr1;
#[doc = "DR2 (rw) register accessor: an alias for `Reg<DR2_SPEC>`"]
pub type DR2 = crate::Reg<dr2::DR2_SPEC>;
#[doc = "desc DR2"]
pub mod dr2;
#[doc = "DR3 (rw) register accessor: an alias for `Reg<DR3_SPEC>`"]
pub type DR3 = crate::Reg<dr3::DR3_SPEC>;
#[doc = "desc DR3"]
pub mod dr3;
#[doc = "DR4 (rw) register accessor: an alias for `Reg<DR4_SPEC>`"]
pub type DR4 = crate::Reg<dr4::DR4_SPEC>;
#[doc = "desc DR4"]
pub mod dr4;
#[doc = "DR5 (rw) register accessor: an alias for `Reg<DR5_SPEC>`"]
pub type DR5 = crate::Reg<dr5::DR5_SPEC>;
#[doc = "desc DR5"]
pub mod dr5;
#[doc = "DR6 (rw) register accessor: an alias for `Reg<DR6_SPEC>`"]
pub type DR6 = crate::Reg<dr6::DR6_SPEC>;
#[doc = "desc DR6"]
pub mod dr6;
#[doc = "DR7 (rw) register accessor: an alias for `Reg<DR7_SPEC>`"]
pub type DR7 = crate::Reg<dr7::DR7_SPEC>;
#[doc = "desc DR7"]
pub mod dr7;
#[doc = "DR8 (rw) register accessor: an alias for `Reg<DR8_SPEC>`"]
pub type DR8 = crate::Reg<dr8::DR8_SPEC>;
#[doc = "desc DR8"]
pub mod dr8;
#[doc = "DR9 (rw) register accessor: an alias for `Reg<DR9_SPEC>`"]
pub type DR9 = crate::Reg<dr9::DR9_SPEC>;
#[doc = "desc DR9"]
pub mod dr9;
#[doc = "DR10 (rw) register accessor: an alias for `Reg<DR10_SPEC>`"]
pub type DR10 = crate::Reg<dr10::DR10_SPEC>;
#[doc = "desc DR10"]
pub mod dr10;
#[doc = "RTCCR (rw) register accessor: an alias for `Reg<RTCCR_SPEC>`"]
pub type RTCCR = crate::Reg<rtccr::RTCCR_SPEC>;
#[doc = "desc RTCCR"]
pub mod rtccr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "desc CSR"]
pub mod csr;
#[doc = "DR11 (rw) register accessor: an alias for `Reg<DR11_SPEC>`"]
pub type DR11 = crate::Reg<dr11::DR11_SPEC>;
#[doc = "desc DR11"]
pub mod dr11;
#[doc = "DR12 (rw) register accessor: an alias for `Reg<DR12_SPEC>`"]
pub type DR12 = crate::Reg<dr12::DR12_SPEC>;
#[doc = "desc DR12"]
pub mod dr12;
#[doc = "DR13 (rw) register accessor: an alias for `Reg<DR13_SPEC>`"]
pub type DR13 = crate::Reg<dr13::DR13_SPEC>;
#[doc = "desc DR13"]
pub mod dr13;
#[doc = "DR14 (rw) register accessor: an alias for `Reg<DR14_SPEC>`"]
pub type DR14 = crate::Reg<dr14::DR14_SPEC>;
#[doc = "desc DR14"]
pub mod dr14;
#[doc = "DR15 (rw) register accessor: an alias for `Reg<DR15_SPEC>`"]
pub type DR15 = crate::Reg<dr15::DR15_SPEC>;
#[doc = "desc DR15"]
pub mod dr15;
#[doc = "DR16 (rw) register accessor: an alias for `Reg<DR16_SPEC>`"]
pub type DR16 = crate::Reg<dr16::DR16_SPEC>;
#[doc = "desc DR16"]
pub mod dr16;
#[doc = "DR17 (rw) register accessor: an alias for `Reg<DR17_SPEC>`"]
pub type DR17 = crate::Reg<dr17::DR17_SPEC>;
#[doc = "desc DR17"]
pub mod dr17;
#[doc = "DR18 (rw) register accessor: an alias for `Reg<DR18_SPEC>`"]
pub type DR18 = crate::Reg<dr18::DR18_SPEC>;
#[doc = "desc DR18"]
pub mod dr18;
#[doc = "DR19 (rw) register accessor: an alias for `Reg<DR19_SPEC>`"]
pub type DR19 = crate::Reg<dr19::DR19_SPEC>;
#[doc = "desc DR19"]
pub mod dr19;
#[doc = "DR20 (rw) register accessor: an alias for `Reg<DR20_SPEC>`"]
pub type DR20 = crate::Reg<dr20::DR20_SPEC>;
#[doc = "desc DR20"]
pub mod dr20;
#[doc = "DR21 (rw) register accessor: an alias for `Reg<DR21_SPEC>`"]
pub type DR21 = crate::Reg<dr21::DR21_SPEC>;
#[doc = "desc DR21"]
pub mod dr21;
#[doc = "DR22 (rw) register accessor: an alias for `Reg<DR22_SPEC>`"]
pub type DR22 = crate::Reg<dr22::DR22_SPEC>;
#[doc = "desc DR22"]
pub mod dr22;
#[doc = "DR23 (rw) register accessor: an alias for `Reg<DR23_SPEC>`"]
pub type DR23 = crate::Reg<dr23::DR23_SPEC>;
#[doc = "desc DR23"]
pub mod dr23;
#[doc = "DR24 (rw) register accessor: an alias for `Reg<DR24_SPEC>`"]
pub type DR24 = crate::Reg<dr24::DR24_SPEC>;
#[doc = "desc DR24"]
pub mod dr24;
#[doc = "DR25 (rw) register accessor: an alias for `Reg<DR25_SPEC>`"]
pub type DR25 = crate::Reg<dr25::DR25_SPEC>;
#[doc = "desc DR25"]
pub mod dr25;
#[doc = "DR26 (rw) register accessor: an alias for `Reg<DR26_SPEC>`"]
pub type DR26 = crate::Reg<dr26::DR26_SPEC>;
#[doc = "desc DR26"]
pub mod dr26;
#[doc = "DR27 (rw) register accessor: an alias for `Reg<DR27_SPEC>`"]
pub type DR27 = crate::Reg<dr27::DR27_SPEC>;
#[doc = "desc DR27"]
pub mod dr27;
#[doc = "DR28 (rw) register accessor: an alias for `Reg<DR28_SPEC>`"]
pub type DR28 = crate::Reg<dr28::DR28_SPEC>;
#[doc = "desc DR28"]
pub mod dr28;
#[doc = "DR29 (rw) register accessor: an alias for `Reg<DR29_SPEC>`"]
pub type DR29 = crate::Reg<dr29::DR29_SPEC>;
#[doc = "desc DR29"]
pub mod dr29;
#[doc = "DR30 (rw) register accessor: an alias for `Reg<DR30_SPEC>`"]
pub type DR30 = crate::Reg<dr30::DR30_SPEC>;
#[doc = "desc DR30"]
pub mod dr30;
#[doc = "DR31 (rw) register accessor: an alias for `Reg<DR31_SPEC>`"]
pub type DR31 = crate::Reg<dr31::DR31_SPEC>;
#[doc = "desc DR31"]
pub mod dr31;
#[doc = "DR32 (rw) register accessor: an alias for `Reg<DR32_SPEC>`"]
pub type DR32 = crate::Reg<dr32::DR32_SPEC>;
#[doc = "desc DR32"]
pub mod dr32;
#[doc = "DR33 (rw) register accessor: an alias for `Reg<DR33_SPEC>`"]
pub type DR33 = crate::Reg<dr33::DR33_SPEC>;
#[doc = "desc DR33"]
pub mod dr33;
#[doc = "DR34 (rw) register accessor: an alias for `Reg<DR34_SPEC>`"]
pub type DR34 = crate::Reg<dr34::DR34_SPEC>;
#[doc = "desc DR34"]
pub mod dr34;
#[doc = "DR35 (rw) register accessor: an alias for `Reg<DR35_SPEC>`"]
pub type DR35 = crate::Reg<dr35::DR35_SPEC>;
#[doc = "desc DR35"]
pub mod dr35;
#[doc = "DR36 (rw) register accessor: an alias for `Reg<DR36_SPEC>`"]
pub type DR36 = crate::Reg<dr36::DR36_SPEC>;
#[doc = "desc DR36"]
pub mod dr36;
#[doc = "DR37 (rw) register accessor: an alias for `Reg<DR37_SPEC>`"]
pub type DR37 = crate::Reg<dr37::DR37_SPEC>;
#[doc = "desc DR37"]
pub mod dr37;
#[doc = "DR38 (rw) register accessor: an alias for `Reg<DR38_SPEC>`"]
pub type DR38 = crate::Reg<dr38::DR38_SPEC>;
#[doc = "desc DR38"]
pub mod dr38;
#[doc = "DR39 (rw) register accessor: an alias for `Reg<DR39_SPEC>`"]
pub type DR39 = crate::Reg<dr39::DR39_SPEC>;
#[doc = "desc DR39"]
pub mod dr39;
#[doc = "DR40 (rw) register accessor: an alias for `Reg<DR40_SPEC>`"]
pub type DR40 = crate::Reg<dr40::DR40_SPEC>;
#[doc = "desc DR40"]
pub mod dr40;
#[doc = "DR41 (rw) register accessor: an alias for `Reg<DR41_SPEC>`"]
pub type DR41 = crate::Reg<dr41::DR41_SPEC>;
#[doc = "desc DR41"]
pub mod dr41;
#[doc = "DR42 (rw) register accessor: an alias for `Reg<DR42_SPEC>`"]
pub type DR42 = crate::Reg<dr42::DR42_SPEC>;
#[doc = "desc DR42"]
pub mod dr42;
