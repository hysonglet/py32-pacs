#[doc = "Register `PAENS` reader"]
pub struct R(crate::R<PAENS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAENS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAENS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAENS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAENS` writer"]
pub struct W(crate::W<PAENS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAENS_SPEC>;
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
impl From<crate::W<PAENS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAENS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA_ENS` reader - desc PA_ENS"]
pub type PA_ENS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PA_ENS` writer - desc PA_ENS"]
pub type PA_ENS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAENS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - desc PA_ENS"]
    #[inline(always)]
    pub fn pa_ens(&self) -> PA_ENS_R {
        PA_ENS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc PA_ENS"]
    #[inline(always)]
    pub fn pa_ens(&mut self) -> PA_ENS_W<0> {
        PA_ENS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc PAENS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paens](index.html) module"]
pub struct PAENS_SPEC;
impl crate::RegisterSpec for PAENS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [paens::R](R) reader structure"]
impl crate::Readable for PAENS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [paens::W](W) writer structure"]
impl crate::Writable for PAENS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAENS to value 0"]
impl crate::Resettable for PAENS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
