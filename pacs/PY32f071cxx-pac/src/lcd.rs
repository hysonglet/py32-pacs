#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub cr0: CR0,
    #[doc = "0x04 - CR1"]
    pub cr1: CR1,
    #[doc = "0x08 - INTCLR"]
    pub intclr: INTCLR,
    #[doc = "0x0c - POEN0"]
    pub poen0: POEN0,
    #[doc = "0x10 - POEN1"]
    pub poen1: POEN1,
    #[doc = "0x14 - des RAM0"]
    pub ram0: RAM0,
    #[doc = "0x18 - des RAM1"]
    pub ram1: RAM1,
    #[doc = "0x1c - des RAM2"]
    pub ram2: RAM2,
    #[doc = "0x20 - des RAM3"]
    pub ram3: RAM3,
    #[doc = "0x24 - des RAM4"]
    pub ram4: RAM4,
    #[doc = "0x28 - des RAM5"]
    pub ram5: RAM5,
    #[doc = "0x2c - des RAM6"]
    pub ram6: RAM6,
    #[doc = "0x30 - des RAM7"]
    pub ram7: RAM7,
    #[doc = "0x34 - des RAM8"]
    pub ram8: RAM8,
    #[doc = "0x38 - des RAM9"]
    pub ram9: RAM9,
    #[doc = "0x3c - des RAM10"]
    pub ram10: RAM10,
    #[doc = "0x40 - des RAM11"]
    pub ram11: RAM11,
    #[doc = "0x44 - des RAM12"]
    pub ram12: RAM12,
    #[doc = "0x48 - des RAM13"]
    pub ram13: RAM13,
    #[doc = "0x4c - des RAM14"]
    pub ram14: RAM14,
    #[doc = "0x50 - des RAM15"]
    pub ram15: RAM15,
}
#[doc = "CR0 (rw) register accessor: an alias for `Reg<CR0_SPEC>`"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "Control register"]
pub mod cr0;
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "CR1"]
pub mod cr1;
#[doc = "INTCLR (rw) register accessor: an alias for `Reg<INTCLR_SPEC>`"]
pub type INTCLR = crate::Reg<intclr::INTCLR_SPEC>;
#[doc = "INTCLR"]
pub mod intclr;
#[doc = "POEN0 (rw) register accessor: an alias for `Reg<POEN0_SPEC>`"]
pub type POEN0 = crate::Reg<poen0::POEN0_SPEC>;
#[doc = "POEN0"]
pub mod poen0;
#[doc = "POEN1 (rw) register accessor: an alias for `Reg<POEN1_SPEC>`"]
pub type POEN1 = crate::Reg<poen1::POEN1_SPEC>;
#[doc = "POEN1"]
pub mod poen1;
#[doc = "RAM0 (rw) register accessor: an alias for `Reg<RAM0_SPEC>`"]
pub type RAM0 = crate::Reg<ram0::RAM0_SPEC>;
#[doc = "des RAM0"]
pub mod ram0;
#[doc = "RAM1 (rw) register accessor: an alias for `Reg<RAM1_SPEC>`"]
pub type RAM1 = crate::Reg<ram1::RAM1_SPEC>;
#[doc = "des RAM1"]
pub mod ram1;
#[doc = "RAM2 (rw) register accessor: an alias for `Reg<RAM2_SPEC>`"]
pub type RAM2 = crate::Reg<ram2::RAM2_SPEC>;
#[doc = "des RAM2"]
pub mod ram2;
#[doc = "RAM3 (rw) register accessor: an alias for `Reg<RAM3_SPEC>`"]
pub type RAM3 = crate::Reg<ram3::RAM3_SPEC>;
#[doc = "des RAM3"]
pub mod ram3;
#[doc = "RAM4 (rw) register accessor: an alias for `Reg<RAM4_SPEC>`"]
pub type RAM4 = crate::Reg<ram4::RAM4_SPEC>;
#[doc = "des RAM4"]
pub mod ram4;
#[doc = "RAM5 (rw) register accessor: an alias for `Reg<RAM5_SPEC>`"]
pub type RAM5 = crate::Reg<ram5::RAM5_SPEC>;
#[doc = "des RAM5"]
pub mod ram5;
#[doc = "RAM6 (rw) register accessor: an alias for `Reg<RAM6_SPEC>`"]
pub type RAM6 = crate::Reg<ram6::RAM6_SPEC>;
#[doc = "des RAM6"]
pub mod ram6;
#[doc = "RAM7 (rw) register accessor: an alias for `Reg<RAM7_SPEC>`"]
pub type RAM7 = crate::Reg<ram7::RAM7_SPEC>;
#[doc = "des RAM7"]
pub mod ram7;
#[doc = "RAM8 (rw) register accessor: an alias for `Reg<RAM8_SPEC>`"]
pub type RAM8 = crate::Reg<ram8::RAM8_SPEC>;
#[doc = "des RAM8"]
pub mod ram8;
#[doc = "RAM9 (rw) register accessor: an alias for `Reg<RAM9_SPEC>`"]
pub type RAM9 = crate::Reg<ram9::RAM9_SPEC>;
#[doc = "des RAM9"]
pub mod ram9;
#[doc = "RAM10 (rw) register accessor: an alias for `Reg<RAM10_SPEC>`"]
pub type RAM10 = crate::Reg<ram10::RAM10_SPEC>;
#[doc = "des RAM10"]
pub mod ram10;
#[doc = "RAM11 (rw) register accessor: an alias for `Reg<RAM11_SPEC>`"]
pub type RAM11 = crate::Reg<ram11::RAM11_SPEC>;
#[doc = "des RAM11"]
pub mod ram11;
#[doc = "RAM12 (rw) register accessor: an alias for `Reg<RAM12_SPEC>`"]
pub type RAM12 = crate::Reg<ram12::RAM12_SPEC>;
#[doc = "des RAM12"]
pub mod ram12;
#[doc = "RAM13 (rw) register accessor: an alias for `Reg<RAM13_SPEC>`"]
pub type RAM13 = crate::Reg<ram13::RAM13_SPEC>;
#[doc = "des RAM13"]
pub mod ram13;
#[doc = "RAM14 (rw) register accessor: an alias for `Reg<RAM14_SPEC>`"]
pub type RAM14 = crate::Reg<ram14::RAM14_SPEC>;
#[doc = "des RAM14"]
pub mod ram14;
#[doc = "RAM15 (rw) register accessor: an alias for `Reg<RAM15_SPEC>`"]
pub type RAM15 = crate::Reg<ram15::RAM15_SPEC>;
#[doc = "des RAM15"]
pub mod ram15;
