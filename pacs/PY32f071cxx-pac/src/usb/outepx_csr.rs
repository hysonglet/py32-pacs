#[doc = "Register `OUTEPxCSR` writer"]
pub struct W(crate::W<OUTEPX_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTEPX_CSR_SPEC>;
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
impl From<crate::W<OUTEPX_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTEPX_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAMode` writer - DMAMode"]
pub type DMAMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEPX_CSR_SPEC, bool, O>;
#[doc = "Field `DMAEnab` writer - DMAEnab"]
pub type DMAENAB_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEPX_CSR_SPEC, bool, O>;
#[doc = "Field `ISO` writer - ISO"]
pub type ISO_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEPX_CSR_SPEC, bool, O>;
#[doc = "Field `AutoClear` writer - AutoClear"]
pub type AUTO_CLEAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEPX_CSR_SPEC, bool, O>;
#[doc = "Field `OutPktRdy` writer - OutPktRdy"]
pub type OUT_PKT_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEPX_CSR_SPEC, bool, O>;
#[doc = "Field `FIFOFull` writer - FIFOFull"]
pub type FIFOFULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEPX_CSR_SPEC, bool, O>;
#[doc = "Field `OverRun` writer - OverRun"]
pub type OVER_RUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEPX_CSR_SPEC, bool, O>;
#[doc = "Field `DataError` writer - DataError"]
pub type DATA_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEPX_CSR_SPEC, bool, O>;
#[doc = "Field `FlushFIFO` writer - FlushFIFO"]
pub type FLUSH_FIFO_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEPX_CSR_SPEC, bool, O>;
#[doc = "Field `SendStall` writer - SendStall"]
pub type SEND_STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEPX_CSR_SPEC, bool, O>;
#[doc = "Field `SentStall` writer - SentStall"]
pub type SENT_STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEPX_CSR_SPEC, bool, O>;
#[doc = "Field `ClrDataTog` writer - ClrDataTog"]
pub type CLR_DATA_TOG_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEPX_CSR_SPEC, bool, O>;
#[doc = "Field `INMAXP` writer - INMAXP"]
pub type INMAXP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEPX_CSR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 4 - DMAMode"]
    #[inline(always)]
    pub fn dmamode(&mut self) -> DMAMODE_W<4> {
        DMAMODE_W::new(self)
    }
    #[doc = "Bit 5 - DMAEnab"]
    #[inline(always)]
    pub fn dmaenab(&mut self) -> DMAENAB_W<5> {
        DMAENAB_W::new(self)
    }
    #[doc = "Bit 6 - ISO"]
    #[inline(always)]
    pub fn iso(&mut self) -> ISO_W<6> {
        ISO_W::new(self)
    }
    #[doc = "Bit 7 - AutoClear"]
    #[inline(always)]
    pub fn auto_clear(&mut self) -> AUTO_CLEAR_W<7> {
        AUTO_CLEAR_W::new(self)
    }
    #[doc = "Bit 8 - OutPktRdy"]
    #[inline(always)]
    pub fn out_pkt_rdy(&mut self) -> OUT_PKT_RDY_W<8> {
        OUT_PKT_RDY_W::new(self)
    }
    #[doc = "Bit 9 - FIFOFull"]
    #[inline(always)]
    pub fn fifofull(&mut self) -> FIFOFULL_W<9> {
        FIFOFULL_W::new(self)
    }
    #[doc = "Bit 10 - OverRun"]
    #[inline(always)]
    pub fn over_run(&mut self) -> OVER_RUN_W<10> {
        OVER_RUN_W::new(self)
    }
    #[doc = "Bit 11 - DataError"]
    #[inline(always)]
    pub fn data_error(&mut self) -> DATA_ERROR_W<11> {
        DATA_ERROR_W::new(self)
    }
    #[doc = "Bit 12 - FlushFIFO"]
    #[inline(always)]
    pub fn flush_fifo(&mut self) -> FLUSH_FIFO_W<12> {
        FLUSH_FIFO_W::new(self)
    }
    #[doc = "Bit 13 - SendStall"]
    #[inline(always)]
    pub fn send_stall(&mut self) -> SEND_STALL_W<13> {
        SEND_STALL_W::new(self)
    }
    #[doc = "Bit 14 - SentStall"]
    #[inline(always)]
    pub fn sent_stall(&mut self) -> SENT_STALL_W<14> {
        SENT_STALL_W::new(self)
    }
    #[doc = "Bit 15 - ClrDataTog"]
    #[inline(always)]
    pub fn clr_data_tog(&mut self) -> CLR_DATA_TOG_W<15> {
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
#[doc = "OUTEPxCSR\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outepx_csr](index.html) module"]
pub struct OUTEPX_CSR_SPEC;
impl crate::RegisterSpec for OUTEPX_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [outepx_csr::W](W) writer structure"]
impl crate::Writable for OUTEPX_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTEPxCSR to value 0"]
impl crate::Resettable for OUTEPX_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
