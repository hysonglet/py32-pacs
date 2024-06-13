#[doc = "Register `PFENS` reader"]
pub struct R(crate::R<PFENS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFENS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFENS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFENS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFENS` writer"]
pub struct W(crate::W<PFENS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFENS_SPEC>;
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
impl From<crate::W<PFENS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFENS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PF_ENS` reader - desc PF_ENS"]
pub type PF_ENS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PF_ENS` writer - desc PF_ENS"]
pub type PF_ENS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PFENS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - desc PF_ENS"]
    #[inline(always)]
    pub fn pf_ens(&self) -> PF_ENS_R {
        PF_ENS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc PF_ENS"]
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
#[doc = "desc PFENS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfens](index.html) module"]
pub struct PFENS_SPEC;
impl crate::RegisterSpec for PFENS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfens::R](R) reader structure"]
impl crate::Readable for PFENS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfens::W](W) writer structure"]
impl crate::Writable for PFENS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PFENS to value 0"]
impl crate::Resettable for PFENS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
