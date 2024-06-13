#[doc = "Register `PDENS` reader"]
pub struct R(crate::R<PDENS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDENS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDENS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDENS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDENS` writer"]
pub struct W(crate::W<PDENS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDENS_SPEC>;
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
impl From<crate::W<PDENS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDENS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD_ENS` reader - desc PD_ENS"]
pub type PD_ENS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PD_ENS` writer - desc PD_ENS"]
pub type PD_ENS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDENS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - desc PD_ENS"]
    #[inline(always)]
    pub fn pd_ens(&self) -> PD_ENS_R {
        PD_ENS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc PD_ENS"]
    #[inline(always)]
    pub fn pd_ens(&mut self) -> PD_ENS_W<0> {
        PD_ENS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc PDENS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdens](index.html) module"]
pub struct PDENS_SPEC;
impl crate::RegisterSpec for PDENS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdens::R](R) reader structure"]
impl crate::Readable for PDENS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdens::W](W) writer structure"]
impl crate::Writable for PDENS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDENS to value 0"]
impl crate::Resettable for PDENS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
