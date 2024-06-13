#[doc = "Register `DEND` reader"]
pub struct R(crate::R<DEND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEND` writer"]
pub struct W(crate::W<DEND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEND_SPEC>;
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
impl From<crate::W<DEND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEND` reader - Dividend"]
pub type DEND_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DEND` writer - Dividend"]
pub type DEND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEND_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Dividend"]
    #[inline(always)]
    pub fn dend(&self) -> DEND_R {
        DEND_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Dividend"]
    #[inline(always)]
    pub fn dend(&mut self) -> DEND_W<0> {
        DEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dividend\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dend](index.html) module"]
pub struct DEND_SPEC;
impl crate::RegisterSpec for DEND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dend::R](R) reader structure"]
impl crate::Readable for DEND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dend::W](W) writer structure"]
impl crate::Writable for DEND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEND to value 0"]
impl crate::Resettable for DEND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
