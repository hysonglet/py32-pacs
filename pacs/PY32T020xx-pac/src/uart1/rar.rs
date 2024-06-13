#[doc = "Register `RAR` reader"]
pub struct R(crate::R<RAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAR` writer"]
pub struct W(crate::W<RAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAR_SPEC>;
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
impl From<crate::W<RAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAR` reader - "]
pub type RAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RAR` writer - "]
pub type RAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RAR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rar(&self) -> RAR_R {
        RAR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rar(&mut self) -> RAR_W<0> {
        RAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rar](index.html) module"]
pub struct RAR_SPEC;
impl crate::RegisterSpec for RAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rar::R](R) reader structure"]
impl crate::Readable for RAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rar::W](W) writer structure"]
impl crate::Writable for RAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAR to value 0"]
impl crate::Resettable for RAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
