#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCU Device ID Code Register"]
    pub idcode: IDCODE,
    #[doc = "0x04 - Debug MCU Configuration Register"]
    pub cr: CR,
    #[doc = "0x08 - APB Freeze Register1"]
    pub apb_fz1: APB_FZ1,
    #[doc = "0x0c - APB Freeze Register2"]
    pub apb_fz2: APB_FZ2,
}
#[doc = "IDCODE (r) register accessor: an alias for `Reg<IDCODE_SPEC>`"]
pub type IDCODE = crate::Reg<idcode::IDCODE_SPEC>;
#[doc = "MCU Device ID Code Register"]
pub mod idcode;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Debug MCU Configuration Register"]
pub mod cr;
#[doc = "APB_FZ1 (rw) register accessor: an alias for `Reg<APB_FZ1_SPEC>`"]
pub type APB_FZ1 = crate::Reg<apb_fz1::APB_FZ1_SPEC>;
#[doc = "APB Freeze Register1"]
pub mod apb_fz1;
#[doc = "APB_FZ2 (rw) register accessor: an alias for `Reg<APB_FZ2_SPEC>`"]
pub type APB_FZ2 = crate::Reg<apb_fz2::APB_FZ2_SPEC>;
#[doc = "APB Freeze Register2"]
pub mod apb_fz2;
