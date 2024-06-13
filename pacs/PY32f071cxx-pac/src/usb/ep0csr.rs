#[doc = "Register `EP0CSR` writer"]
pub struct W(crate::W<EP0CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP0CSR_SPEC>;
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
impl From<crate::W<EP0CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP0CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OutPktRdy` writer - OutPktRdy"]
pub type OUT_PKT_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0CSR_SPEC, bool, O>;
#[doc = "Field `InPktRdy` writer - InPktRdy"]
pub type IN_PKT_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0CSR_SPEC, bool, O>;
#[doc = "Field `SentStall` writer - SentStall"]
pub type SENT_STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0CSR_SPEC, bool, O>;
#[doc = "Field `DataEnd` writer - DataEnd"]
pub type DATA_END_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0CSR_SPEC, bool, O>;
#[doc = "Field `SetupEnd` writer - SetupEnd"]
pub type SETUP_END_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0CSR_SPEC, bool, O>;
#[doc = "Field `SendStall` writer - OutPktRdy"]
pub type SEND_STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0CSR_SPEC, bool, O>;
#[doc = "Field `ServicedOutPktRdy` writer - ServicedOutPktRdy"]
pub type SERVICED_OUT_PKT_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0CSR_SPEC, bool, O>;
#[doc = "Field `ServicedSetupEnd` writer - ServicedSetupEnd"]
pub type SERVICED_SETUP_END_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0CSR_SPEC, bool, O>;
#[doc = "Field `COUNT0` writer - COUNT0"]
pub type COUNT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0CSR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - OutPktRdy"]
    #[inline(always)]
    pub fn out_pkt_rdy(&mut self) -> OUT_PKT_RDY_W<0> {
        OUT_PKT_RDY_W::new(self)
    }
    #[doc = "Bit 1 - InPktRdy"]
    #[inline(always)]
    pub fn in_pkt_rdy(&mut self) -> IN_PKT_RDY_W<1> {
        IN_PKT_RDY_W::new(self)
    }
    #[doc = "Bit 2 - SentStall"]
    #[inline(always)]
    pub fn sent_stall(&mut self) -> SENT_STALL_W<2> {
        SENT_STALL_W::new(self)
    }
    #[doc = "Bit 3 - DataEnd"]
    #[inline(always)]
    pub fn data_end(&mut self) -> DATA_END_W<3> {
        DATA_END_W::new(self)
    }
    #[doc = "Bit 4 - SetupEnd"]
    #[inline(always)]
    pub fn setup_end(&mut self) -> SETUP_END_W<4> {
        SETUP_END_W::new(self)
    }
    #[doc = "Bit 5 - OutPktRdy"]
    #[inline(always)]
    pub fn send_stall(&mut self) -> SEND_STALL_W<5> {
        SEND_STALL_W::new(self)
    }
    #[doc = "Bit 6 - ServicedOutPktRdy"]
    #[inline(always)]
    pub fn serviced_out_pkt_rdy(&mut self) -> SERVICED_OUT_PKT_RDY_W<6> {
        SERVICED_OUT_PKT_RDY_W::new(self)
    }
    #[doc = "Bit 7 - ServicedSetupEnd"]
    #[inline(always)]
    pub fn serviced_setup_end(&mut self) -> SERVICED_SETUP_END_W<7> {
        SERVICED_SETUP_END_W::new(self)
    }
    #[doc = "Bit 8 - COUNT0"]
    #[inline(always)]
    pub fn count0(&mut self) -> COUNT0_W<8> {
        COUNT0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EP0CSR\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep0csr](index.html) module"]
pub struct EP0CSR_SPEC;
impl crate::RegisterSpec for EP0CSR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ep0csr::W](W) writer structure"]
impl crate::Writable for EP0CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EP0CSR to value 0"]
impl crate::Resettable for EP0CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
