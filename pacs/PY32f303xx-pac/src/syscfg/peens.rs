#[doc = "Register `PEENS` reader"]
pub struct R(crate::R<PEENS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEENS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEENS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEENS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEENS` writer"]
pub struct W(crate::W<PEENS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEENS_SPEC>;
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
impl From<crate::W<PEENS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEENS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PE_ENS` reader - desc PE_ENS"]
pub type PE_ENS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PE_ENS` writer - desc PE_ENS"]
pub type PE_ENS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PEENS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - desc PE_ENS"]
    #[inline(always)]
    pub fn pe_ens(&self) -> PE_ENS_R {
        PE_ENS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc PE_ENS"]
    #[inline(always)]
    pub fn pe_ens(&mut self) -> PE_ENS_W<0> {
        PE_ENS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc PEENS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peens](index.html) module"]
pub struct PEENS_SPEC;
impl crate::RegisterSpec for PEENS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peens::R](R) reader structure"]
impl crate::Readable for PEENS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peens::W](W) writer structure"]
impl crate::Writable for PEENS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PEENS to value 0"]
impl crate::Resettable for PEENS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
