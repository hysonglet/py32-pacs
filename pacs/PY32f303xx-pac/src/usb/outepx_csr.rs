#[doc = "Register `OUTEPxCSR` reader"]
pub struct R(crate::R<OUTEPX_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTEPX_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTEPX_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTEPX_CSR_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "Field `DMAMODE` reader - desc DMAMODE"]
pub type DMAMODE_R = crate::BitReader<bool>;
#[doc = "Field `DMAMODE` writer - desc DMAMODE"]
pub type DMAMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEPX_CSR_SPEC, bool, O>;
#[doc = "Field `DMAENABLE` reader - desc DMAENABLE"]
pub type DMAENABLE_R = crate::BitReader<bool>;
#[doc = "Field `DMAENABLE` writer - desc DMAENABLE"]
pub type DMAENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEPX_CSR_SPEC, bool, O>;
#[doc = "Field `ISO` reader - ISO"]
pub type ISO_R = crate::BitReader<bool>;
#[doc = "Field `ISO` writer - ISO"]
pub type ISO_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEPX_CSR_SPEC, bool, O>;
#[doc = "Field `AUTOCLEAR` reader - desc AUTOCLEAR"]
pub type AUTOCLEAR_R = crate::BitReader<bool>;
#[doc = "Field `AUTOCLEAR` writer - desc AUTOCLEAR"]
pub type AUTOCLEAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEPX_CSR_SPEC, bool, O>;
#[doc = "Field `OUTPKTRDY` reader - desc OUTPKTRDY"]
pub type OUTPKTRDY_R = crate::BitReader<bool>;
#[doc = "Field `OUTPKTRDY` writer - desc OUTPKTRDY"]
pub type OUTPKTRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEPX_CSR_SPEC, bool, O>;
#[doc = "Field `FIFOFULL` reader - desc FIFOFULL"]
pub type FIFOFULL_R = crate::BitReader<bool>;
#[doc = "Field `OVERRUN` reader - desc OVERRUN"]
pub type OVERRUN_R = crate::BitReader<bool>;
#[doc = "Field `OVERRUN` writer - desc OVERRUN"]
pub type OVERRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEPX_CSR_SPEC, bool, O>;
#[doc = "Field `DATAERROR` reader - desc DATAERROR"]
pub type DATAERROR_R = crate::BitReader<bool>;
#[doc = "Field `FLUSHFIFO` writer - desc FLUSHFIFO"]
pub type FLUSHFIFO_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEPX_CSR_SPEC, bool, O>;
#[doc = "Field `SENDSTALL` reader - desc SENDSTALL"]
pub type SENDSTALL_R = crate::BitReader<bool>;
#[doc = "Field `SENDSTALL` writer - desc SENDSTALL"]
pub type SENDSTALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEPX_CSR_SPEC, bool, O>;
#[doc = "Field `SENTSTALL` reader - desc SENTSTALL"]
pub type SENTSTALL_R = crate::BitReader<bool>;
#[doc = "Field `SENTSTALL` writer - desc SENTSTALL"]
pub type SENTSTALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEPX_CSR_SPEC, bool, O>;
#[doc = "Field `CLRDATATOG` writer - desc CLRDATATOG"]
pub type CLRDATATOG_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTEPX_CSR_SPEC, bool, O>;
#[doc = "Field `OUTMAXP` reader - desc OUTMAXP"]
pub type OUTMAXP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUTMAXP` writer - desc OUTMAXP"]
pub type OUTMAXP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTEPX_CSR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 4 - desc DMAMODE"]
    #[inline(always)]
    pub fn dmamode(&self) -> DMAMODE_R {
        DMAMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc DMAENABLE"]
    #[inline(always)]
    pub fn dmaenable(&self) -> DMAENABLE_R {
        DMAENABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ISO"]
    #[inline(always)]
    pub fn iso(&self) -> ISO_R {
        ISO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc AUTOCLEAR"]
    #[inline(always)]
    pub fn autoclear(&self) -> AUTOCLEAR_R {
        AUTOCLEAR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc OUTPKTRDY"]
    #[inline(always)]
    pub fn outpktrdy(&self) -> OUTPKTRDY_R {
        OUTPKTRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc FIFOFULL"]
    #[inline(always)]
    pub fn fifofull(&self) -> FIFOFULL_R {
        FIFOFULL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc OVERRUN"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc DATAERROR"]
    #[inline(always)]
    pub fn dataerror(&self) -> DATAERROR_R {
        DATAERROR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - desc SENDSTALL"]
    #[inline(always)]
    pub fn sendstall(&self) -> SENDSTALL_R {
        SENDSTALL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc SENTSTALL"]
    #[inline(always)]
    pub fn sentstall(&self) -> SENTSTALL_R {
        SENTSTALL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - desc OUTMAXP"]
    #[inline(always)]
    pub fn outmaxp(&self) -> OUTMAXP_R {
        OUTMAXP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - desc DMAMODE"]
    #[inline(always)]
    pub fn dmamode(&mut self) -> DMAMODE_W<4> {
        DMAMODE_W::new(self)
    }
    #[doc = "Bit 5 - desc DMAENABLE"]
    #[inline(always)]
    pub fn dmaenable(&mut self) -> DMAENABLE_W<5> {
        DMAENABLE_W::new(self)
    }
    #[doc = "Bit 6 - ISO"]
    #[inline(always)]
    pub fn iso(&mut self) -> ISO_W<6> {
        ISO_W::new(self)
    }
    #[doc = "Bit 7 - desc AUTOCLEAR"]
    #[inline(always)]
    pub fn autoclear(&mut self) -> AUTOCLEAR_W<7> {
        AUTOCLEAR_W::new(self)
    }
    #[doc = "Bit 8 - desc OUTPKTRDY"]
    #[inline(always)]
    pub fn outpktrdy(&mut self) -> OUTPKTRDY_W<8> {
        OUTPKTRDY_W::new(self)
    }
    #[doc = "Bit 10 - desc OVERRUN"]
    #[inline(always)]
    pub fn overrun(&mut self) -> OVERRUN_W<10> {
        OVERRUN_W::new(self)
    }
    #[doc = "Bit 12 - desc FLUSHFIFO"]
    #[inline(always)]
    pub fn flushfifo(&mut self) -> FLUSHFIFO_W<12> {
        FLUSHFIFO_W::new(self)
    }
    #[doc = "Bit 13 - desc SENDSTALL"]
    #[inline(always)]
    pub fn sendstall(&mut self) -> SENDSTALL_W<13> {
        SENDSTALL_W::new(self)
    }
    #[doc = "Bit 14 - desc SENTSTALL"]
    #[inline(always)]
    pub fn sentstall(&mut self) -> SENTSTALL_W<14> {
        SENTSTALL_W::new(self)
    }
    #[doc = "Bit 15 - desc CLRDATATOG"]
    #[inline(always)]
    pub fn clrdatatog(&mut self) -> CLRDATATOG_W<15> {
        CLRDATATOG_W::new(self)
    }
    #[doc = "Bits 16:23 - desc OUTMAXP"]
    #[inline(always)]
    pub fn outmaxp(&mut self) -> OUTMAXP_W<16> {
        OUTMAXP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc OUTEPxCSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outepx_csr](index.html) module"]
pub struct OUTEPX_CSR_SPEC;
impl crate::RegisterSpec for OUTEPX_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outepx_csr::R](R) reader structure"]
impl crate::Readable for OUTEPX_CSR_SPEC {
    type Reader = R;
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
