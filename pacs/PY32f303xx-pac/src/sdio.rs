#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc POWER"]
    pub power: POWER,
    #[doc = "0x04 - desc CLKCR"]
    pub clkcr: CLKCR,
    #[doc = "0x08 - desc ARG"]
    pub arg: ARG,
    #[doc = "0x0c - desc CMD"]
    pub cmd: CMD,
    #[doc = "0x10 - desc RESPCMD"]
    pub respcmd: RESPCMD,
    #[doc = "0x14 - desc RESP0"]
    pub resp0: RESP0,
    #[doc = "0x18 - desc RESP1"]
    pub resp1: RESP1,
    #[doc = "0x1c - desc RESP2"]
    pub resp2: RESP2,
    #[doc = "0x20 - desc RESP3"]
    pub resp3: RESP3,
    #[doc = "0x24 - desc TMOUT"]
    pub tmout: TMOUT,
    #[doc = "0x28 - desc BLKSIZ"]
    pub blksiz: BLKSIZ,
    #[doc = "0x2c - desc DLEN"]
    pub dlen: DLEN,
    #[doc = "0x30 - desc CTRL"]
    pub ctrl: CTRL,
    #[doc = "0x34 - desc STATUS"]
    pub status: STATUS,
    #[doc = "0x38 - desc INTSTS"]
    pub intsts: INTSTS,
    #[doc = "0x3c - desc INTMASK"]
    pub intmask: INTMASK,
    #[doc = "0x40 - desc FIFOTH"]
    pub fifoth: FIFOTH,
    #[doc = "0x44 - desc TCBCNT"]
    pub tcbcnt: TCBCNT,
    #[doc = "0x48 - desc TBBCNT"]
    pub tbbcnt: TBBCNT,
}
#[doc = "POWER (rw) register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "desc POWER"]
pub mod power;
#[doc = "CLKCR (rw) register accessor: an alias for `Reg<CLKCR_SPEC>`"]
pub type CLKCR = crate::Reg<clkcr::CLKCR_SPEC>;
#[doc = "desc CLKCR"]
pub mod clkcr;
#[doc = "ARG (rw) register accessor: an alias for `Reg<ARG_SPEC>`"]
pub type ARG = crate::Reg<arg::ARG_SPEC>;
#[doc = "desc ARG"]
pub mod arg;
#[doc = "CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "desc CMD"]
pub mod cmd;
#[doc = "RESPCMD (r) register accessor: an alias for `Reg<RESPCMD_SPEC>`"]
pub type RESPCMD = crate::Reg<respcmd::RESPCMD_SPEC>;
#[doc = "desc RESPCMD"]
pub mod respcmd;
#[doc = "RESP0 (r) register accessor: an alias for `Reg<RESP0_SPEC>`"]
pub type RESP0 = crate::Reg<resp0::RESP0_SPEC>;
#[doc = "desc RESP0"]
pub mod resp0;
#[doc = "RESP1 (r) register accessor: an alias for `Reg<RESP1_SPEC>`"]
pub type RESP1 = crate::Reg<resp1::RESP1_SPEC>;
#[doc = "desc RESP1"]
pub mod resp1;
#[doc = "RESP2 (r) register accessor: an alias for `Reg<RESP2_SPEC>`"]
pub type RESP2 = crate::Reg<resp2::RESP2_SPEC>;
#[doc = "desc RESP2"]
pub mod resp2;
#[doc = "RESP3 (r) register accessor: an alias for `Reg<RESP3_SPEC>`"]
pub type RESP3 = crate::Reg<resp3::RESP3_SPEC>;
#[doc = "desc RESP3"]
pub mod resp3;
#[doc = "TMOUT (rw) register accessor: an alias for `Reg<TMOUT_SPEC>`"]
pub type TMOUT = crate::Reg<tmout::TMOUT_SPEC>;
#[doc = "desc TMOUT"]
pub mod tmout;
#[doc = "BLKSIZ (rw) register accessor: an alias for `Reg<BLKSIZ_SPEC>`"]
pub type BLKSIZ = crate::Reg<blksiz::BLKSIZ_SPEC>;
#[doc = "desc BLKSIZ"]
pub mod blksiz;
#[doc = "DLEN (rw) register accessor: an alias for `Reg<DLEN_SPEC>`"]
pub type DLEN = crate::Reg<dlen::DLEN_SPEC>;
#[doc = "desc DLEN"]
pub mod dlen;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "desc CTRL"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "desc STATUS"]
pub mod status;
#[doc = "INTSTS (rw) register accessor: an alias for `Reg<INTSTS_SPEC>`"]
pub type INTSTS = crate::Reg<intsts::INTSTS_SPEC>;
#[doc = "desc INTSTS"]
pub mod intsts;
#[doc = "INTMASK (rw) register accessor: an alias for `Reg<INTMASK_SPEC>`"]
pub type INTMASK = crate::Reg<intmask::INTMASK_SPEC>;
#[doc = "desc INTMASK"]
pub mod intmask;
#[doc = "FIFOTH (rw) register accessor: an alias for `Reg<FIFOTH_SPEC>`"]
pub type FIFOTH = crate::Reg<fifoth::FIFOTH_SPEC>;
#[doc = "desc FIFOTH"]
pub mod fifoth;
#[doc = "TCBCNT (r) register accessor: an alias for `Reg<TCBCNT_SPEC>`"]
pub type TCBCNT = crate::Reg<tcbcnt::TCBCNT_SPEC>;
#[doc = "desc TCBCNT"]
pub mod tcbcnt;
#[doc = "TBBCNT (r) register accessor: an alias for `Reg<TBBCNT_SPEC>`"]
pub type TBBCNT = crate::Reg<tbbcnt::TBBCNT_SPEC>;
#[doc = "desc TBBCNT"]
pub mod tbbcnt;
