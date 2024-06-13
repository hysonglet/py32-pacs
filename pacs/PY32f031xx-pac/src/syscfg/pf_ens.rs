#[doc = "Register `PF_ENS` reader"]
pub struct R(crate::R<PF_ENS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PF_ENS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PF_ENS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PF_ENS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PF_ENS` writer"]
pub struct W(crate::W<PF_ENS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PF_ENS_SPEC>;
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
impl From<crate::W<PF_ENS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PF_ENS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PF_ENS` reader - Noise filter enable, active high"]
pub type PF_ENS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PF_ENS` writer - Noise filter enable, active high"]
pub type PF_ENS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PF_ENS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Noise filter enable, active high"]
    #[inline(always)]
    pub fn pf_ens(&self) -> PF_ENS_R {
        PF_ENS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Noise filter enable, active high"]
    #[inline(always)]
    pub fn pf_ens(&mut self) -> PF_ENS_W<0> {
        PF_ENS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOF noise filter enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_ens](index.html) module"]
pub struct PF_ENS_SPEC;
impl crate::RegisterSpec for PF_ENS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pf_ens::R](R) reader structure"]
impl crate::Readable for PF_ENS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pf_ens::W](W) writer structure"]
impl crate::Writable for PF_ENS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PF_ENS to value 0"]
impl crate::Resettable for PF_ENS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
