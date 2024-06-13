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
