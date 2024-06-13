#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CORDIC control register"]
    pub cr: CR,
    #[doc = "0x04 - SIN or COS input theta register"]
    pub theta: THETA,
    #[doc = "0x08 - CORDIC SIN result register"]
    pub sin: SIN,
    #[doc = "0x0c - CORDIC COS result register"]
    pub cos: COS,
    #[doc = "0x10 - Arctan input coordinate register"]
    pub x: X,
    #[doc = "0x14 - Arctan input coordinate register"]
    pub y: Y,
    #[doc = "0x18 - CORDIC Mod result register"]
    pub mod_: MOD,
    #[doc = "0x1c - CORDIC ARCTAN result register"]
    pub arctan: ARCTAN,
    #[doc = "0x20 - CORDIC Radicand register"]
    pub dsp_rad: DSP_RAD,
    #[doc = "0x24 - CORDIC SQRT result register"]
    pub dsp_sqrt: DSP_SQRT,
    #[doc = "0x28 - CORDIC Status register"]
    pub sr: SR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CORDIC control register"]
pub mod cr;
#[doc = "THETA (rw) register accessor: an alias for `Reg<THETA_SPEC>`"]
pub type THETA = crate::Reg<theta::THETA_SPEC>;
#[doc = "SIN or COS input theta register"]
pub mod theta;
#[doc = "SIN (r) register accessor: an alias for `Reg<SIN_SPEC>`"]
pub type SIN = crate::Reg<sin::SIN_SPEC>;
#[doc = "CORDIC SIN result register"]
pub mod sin;
#[doc = "COS (r) register accessor: an alias for `Reg<COS_SPEC>`"]
pub type COS = crate::Reg<cos::COS_SPEC>;
#[doc = "CORDIC COS result register"]
pub mod cos;
#[doc = "X (rw) register accessor: an alias for `Reg<X_SPEC>`"]
pub type X = crate::Reg<x::X_SPEC>;
#[doc = "Arctan input coordinate register"]
pub mod x;
#[doc = "Y (rw) register accessor: an alias for `Reg<Y_SPEC>`"]
pub type Y = crate::Reg<y::Y_SPEC>;
#[doc = "Arctan input coordinate register"]
pub mod y;
#[doc = "MOD (r) register accessor: an alias for `Reg<MOD_SPEC>`"]
pub type MOD = crate::Reg<mod_::MOD_SPEC>;
#[doc = "CORDIC Mod result register"]
pub mod mod_;
#[doc = "ARCTAN (r) register accessor: an alias for `Reg<ARCTAN_SPEC>`"]
pub type ARCTAN = crate::Reg<arctan::ARCTAN_SPEC>;
#[doc = "CORDIC ARCTAN result register"]
pub mod arctan;
#[doc = "DSP_RAD (rw) register accessor: an alias for `Reg<DSP_RAD_SPEC>`"]
pub type DSP_RAD = crate::Reg<dsp_rad::DSP_RAD_SPEC>;
#[doc = "CORDIC Radicand register"]
pub mod dsp_rad;
#[doc = "DSP_SQRT (r) register accessor: an alias for `Reg<DSP_SQRT_SPEC>`"]
pub type DSP_SQRT = crate::Reg<dsp_sqrt::DSP_SQRT_SPEC>;
#[doc = "CORDIC SQRT result register"]
pub mod dsp_sqrt;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "CORDIC Status register"]
pub mod sr;
