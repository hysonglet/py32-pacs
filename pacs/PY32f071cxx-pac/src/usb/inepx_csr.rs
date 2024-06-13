#[doc = "Register `INEPxCSR` writer"]
pub struct W(crate::W<INEPX_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INEPX_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INEPX_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INEPX_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FrcDataTog` writer - FrcDataTog"]
pub type FRC_DATA_TOG_W<'a, const O: u8> = crate::BitWriter<'a, u32, INEPX_CSR_SPEC, bool, O>;
#[doc = "Field `DMAEnab` writer - DMAEnab"]
pub type DMAENAB_W<'a, const O: u8> = crate::BitWriter<'a, u32, INEPX_CSR_SPEC, bool, O>;
#[doc = "Field `Mode` writer - Mode"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INEPX_CSR_SPEC, bool, O>;
#[doc = "Field `ISO` writer - ISO"]
pub type ISO_W<'a, const O: u8> = crate::BitWriter<'a, u32, INEPX_CSR_SPEC, bool, O>;
#[doc = "Field `AutoSet` writer - AutoSet"]
pub type AUTO_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, INEPX_CSR_SPEC, bool, O>;
#[doc = "Field `InPktRdy` writer - InPktRdy"]
pub type IN_PKT_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INEPX_CSR_SPEC, bool, O>;
#[doc = "Field `FIFONotEmpty` writer - FIFONotEmpty"]
pub type FIFONOT_EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INEPX_CSR_SPEC, bool, O>;
#[doc = "Field `UnderRun` writer - UnderRun"]
pub type UNDER_RUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INEPX_CSR_SPEC, bool, O>;
#[doc = "Field `FlushFIFO` writer - FlushFIFO"]
pub type FLUSH_FIFO_W<'a, const O: u8> = crate::BitWriter<'a, u32, INEPX_CSR_SPEC, bool, O>;
#[doc = "Field `SendStall` writer - SendStall"]
pub type SEND_STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INEPX_CSR_SPEC, bool, O>;
#[doc = "Field `SentStall` writer - SentStall"]
pub type SENT_STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INEPX_CSR_SPEC, bool, O>;
#[doc = "Field `ClrDataTog` writer - ClrDataTog"]
pub type CLR_DATA_TOG_W<'a, const O: u8> = crate::BitWriter<'a, u32, INEPX_CSR_SPEC, bool, O>;
#[doc = "Field `INMAXP` writer - INMAXP"]
pub type INMAXP_W<'a, const O: u8> = crate::BitWriter<'a, u32, INEPX_CSR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 3 - FrcDataTog"]
    #[inline(always)]
    pub fn frc_data_tog(&mut self) -> FRC_DATA_TOG_W<3> {
        FRC_DATA_TOG_W::new(self)
    }
    #[doc = "Bit 4 - DMAEnab"]
    #[inline(always)]
    pub fn dmaenab(&mut self) -> DMAENAB_W<4> {
        DMAENAB_W::new(self)
    }
    #[doc = "Bit 5 - Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<5> {
        MODE_W::new(self)
    }
    #[doc = "Bit 6 - ISO"]
    #[inline(always)]
    pub fn iso(&mut self) -> ISO_W<6> {
        ISO_W::new(self)
    }
    #[doc = "Bit 7 - AutoSet"]
    #[inline(always)]
    pub fn auto_set(&mut self) -> AUTO_SET_W<7> {
        AUTO_SET_W::new(self)
    }
    #[doc = "Bit 8 - InPktRdy"]
    #[inline(always)]
    pub fn in_pkt_rdy(&mut self) -> IN_PKT_RDY_W<8> {
        IN_PKT_RDY_W::new(self)
    }
    #[doc = "Bit 9 - FIFONotEmpty"]
    #[inline(always)]
    pub fn fifonot_empty(&mut self) -> FIFONOT_EMPTY_W<9> {
        FIFONOT_EMPTY_W::new(self)
    }
    #[doc = "Bit 10 - UnderRun"]
    #[inline(always)]
    pub fn under_run(&mut self) -> UNDER_RUN_W<10> {
        UNDER_RUN_W::new(self)
    }
    #[doc = "Bit 11 - FlushFIFO"]
    #[inline(always)]
    pub fn flush_fifo(&mut self) -> FLUSH_FIFO_W<11> {
        FLUSH_FIFO_W::new(self)
    }
    #[doc = "Bit 12 - SendStall"]
    #[inline(always)]
    pub fn send_stall(&mut self) -> SEND_STALL_W<12> {
        SEND_STALL_W::new(self)
    }
    #[doc = "Bit 13 - SentStall"]
    #[inline(always)]
    pub fn sent_stall(&mut self) -> SENT_STALL_W<13> {
        SENT_STALL_W::new(self)
    }
    #[doc = "Bit 14 - ClrDataTog"]
    #[inline(always)]
    pub fn clr_data_tog(&mut self) -> CLR_DATA_TOG_W<14> {
        CLR_DATA_TOG_W::new(self)
    }
    #[doc = "Bit 16 - INMAXP"]
    #[inline(always)]
    pub fn inmaxp(&mut self) -> INMAXP_W<16> {
        INMAXP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INEPxCSR\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inepx_csr](index.html) module"]
pub struct INEPX_CSR_SPEC;
impl crate::RegisterSpec for INEPX_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [inepx_csr::W](W) writer structure"]
impl crate::Writable for INEPX_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INEPxCSR to value 0"]
impl crate::Resettable for INEPX_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
