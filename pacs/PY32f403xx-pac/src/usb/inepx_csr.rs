#[doc = "Register `INEPxCSR` reader"]
pub struct R(crate::R<INEPX_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INEPX_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INEPX_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INEPX_CSR_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "Field `FRCDATATOG` reader - desc FRCDATATOG"]
pub type FRCDATATOG_R = crate::BitReader<bool>;
#[doc = "Field `FRCDATATOG` writer - desc FRCDATATOG"]
pub type FRCDATATOG_W<'a, const O: u8> = crate::BitWriter<'a, u32, INEPX_CSR_SPEC, bool, O>;
#[doc = "Field `DMAENAB` reader - desc DMAENAB"]
pub type DMAENAB_R = crate::BitReader<bool>;
#[doc = "Field `DMAENAB` writer - desc DMAENAB"]
pub type DMAENAB_W<'a, const O: u8> = crate::BitWriter<'a, u32, INEPX_CSR_SPEC, bool, O>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - desc MODE"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INEPX_CSR_SPEC, bool, O>;
#[doc = "Field `ISO` reader - desc ISO"]
pub type ISO_R = crate::BitReader<bool>;
#[doc = "Field `ISO` writer - desc ISO"]
pub type ISO_W<'a, const O: u8> = crate::BitWriter<'a, u32, INEPX_CSR_SPEC, bool, O>;
#[doc = "Field `AUTOSET` reader - des AUTOSET"]
pub type AUTOSET_R = crate::BitReader<bool>;
#[doc = "Field `AUTOSET` writer - des AUTOSET"]
pub type AUTOSET_W<'a, const O: u8> = crate::BitWriter<'a, u32, INEPX_CSR_SPEC, bool, O>;
#[doc = "Field `INPKTRDY` reader - des INPKTRDY"]
pub type INPKTRDY_R = crate::BitReader<bool>;
#[doc = "Field `INPKTRDY` writer - des INPKTRDY"]
pub type INPKTRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INEPX_CSR_SPEC, bool, O>;
#[doc = "Field `FIFONOTEMPTY` reader - desc FIFONOTEMPTY"]
pub type FIFONOTEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `FIFONOTEMPTY` writer - desc FIFONOTEMPTY"]
pub type FIFONOTEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INEPX_CSR_SPEC, bool, O>;
#[doc = "Field `UNDERRUN` reader - desc UNDERRUN"]
pub type UNDERRUN_R = crate::BitReader<bool>;
#[doc = "Field `FLUSHFIFO` writer - desc FLUSHFIFO"]
pub type FLUSHFIFO_W<'a, const O: u8> = crate::BitWriter<'a, u32, INEPX_CSR_SPEC, bool, O>;
#[doc = "Field `SENDSTALL` reader - desc SENDSTALL"]
pub type SENDSTALL_R = crate::BitReader<bool>;
#[doc = "Field `SENDSTALL` writer - desc SENDSTALL"]
pub type SENDSTALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INEPX_CSR_SPEC, bool, O>;
#[doc = "Field `SENTSTALL` reader - desc SENTSTALL"]
pub type SENTSTALL_R = crate::BitReader<bool>;
#[doc = "Field `SENTSTALL` writer - desc SENTSTALL"]
pub type SENTSTALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INEPX_CSR_SPEC, bool, O>;
#[doc = "Field `CLRDATATOG` writer - desc CLRDATATOG"]
pub type CLRDATATOG_W<'a, const O: u8> = crate::BitWriter<'a, u32, INEPX_CSR_SPEC, bool, O>;
#[doc = "Field `INMAXP` reader - INMAXP"]
pub type INMAXP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INMAXP` writer - INMAXP"]
pub type INMAXP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INEPX_CSR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 3 - desc FRCDATATOG"]
    #[inline(always)]
    pub fn frcdatatog(&self) -> FRCDATATOG_R {
        FRCDATATOG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc DMAENAB"]
    #[inline(always)]
    pub fn dmaenab(&self) -> DMAENAB_R {
        DMAENAB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc ISO"]
    #[inline(always)]
    pub fn iso(&self) -> ISO_R {
        ISO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - des AUTOSET"]
    #[inline(always)]
    pub fn autoset(&self) -> AUTOSET_R {
        AUTOSET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - des INPKTRDY"]
    #[inline(always)]
    pub fn inpktrdy(&self) -> INPKTRDY_R {
        INPKTRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc FIFONOTEMPTY"]
    #[inline(always)]
    pub fn fifonotempty(&self) -> FIFONOTEMPTY_R {
        FIFONOTEMPTY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc UNDERRUN"]
    #[inline(always)]
    pub fn underrun(&self) -> UNDERRUN_R {
        UNDERRUN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - desc SENDSTALL"]
    #[inline(always)]
    pub fn sendstall(&self) -> SENDSTALL_R {
        SENDSTALL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc SENTSTALL"]
    #[inline(always)]
    pub fn sentstall(&self) -> SENTSTALL_R {
        SENTSTALL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:23 - INMAXP"]
    #[inline(always)]
    pub fn inmaxp(&self) -> INMAXP_R {
        INMAXP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - desc FRCDATATOG"]
    #[inline(always)]
    pub fn frcdatatog(&mut self) -> FRCDATATOG_W<3> {
        FRCDATATOG_W::new(self)
    }
    #[doc = "Bit 4 - desc DMAENAB"]
    #[inline(always)]
    pub fn dmaenab(&mut self) -> DMAENAB_W<4> {
        DMAENAB_W::new(self)
    }
    #[doc = "Bit 5 - desc MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<5> {
        MODE_W::new(self)
    }
    #[doc = "Bit 6 - desc ISO"]
    #[inline(always)]
    pub fn iso(&mut self) -> ISO_W<6> {
        ISO_W::new(self)
    }
    #[doc = "Bit 7 - des AUTOSET"]
    #[inline(always)]
    pub fn autoset(&mut self) -> AUTOSET_W<7> {
        AUTOSET_W::new(self)
    }
    #[doc = "Bit 8 - des INPKTRDY"]
    #[inline(always)]
    pub fn inpktrdy(&mut self) -> INPKTRDY_W<8> {
        INPKTRDY_W::new(self)
    }
    #[doc = "Bit 9 - desc FIFONOTEMPTY"]
    #[inline(always)]
    pub fn fifonotempty(&mut self) -> FIFONOTEMPTY_W<9> {
        FIFONOTEMPTY_W::new(self)
    }
    #[doc = "Bit 11 - desc FLUSHFIFO"]
    #[inline(always)]
    pub fn flushfifo(&mut self) -> FLUSHFIFO_W<11> {
        FLUSHFIFO_W::new(self)
    }
    #[doc = "Bit 12 - desc SENDSTALL"]
    #[inline(always)]
    pub fn sendstall(&mut self) -> SENDSTALL_W<12> {
        SENDSTALL_W::new(self)
    }
    #[doc = "Bit 13 - desc SENTSTALL"]
    #[inline(always)]
    pub fn sentstall(&mut self) -> SENTSTALL_W<13> {
        SENTSTALL_W::new(self)
    }
    #[doc = "Bit 14 - desc CLRDATATOG"]
    #[inline(always)]
    pub fn clrdatatog(&mut self) -> CLRDATATOG_W<14> {
        CLRDATATOG_W::new(self)
    }
    #[doc = "Bits 16:23 - INMAXP"]
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
#[doc = "desc INEPxCSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inepx_csr](index.html) module"]
pub struct INEPX_CSR_SPEC;
impl crate::RegisterSpec for INEPX_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inepx_csr::R](R) reader structure"]
impl crate::Readable for INEPX_CSR_SPEC {
    type Reader = R;
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
