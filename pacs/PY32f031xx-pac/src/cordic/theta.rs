#[doc = "Register `THETA` reader"]
pub struct R(crate::R<THETA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THETA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THETA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THETA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `THETA` writer"]
pub struct W(crate::W<THETA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THETA_SPEC>;
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
impl From<crate::W<THETA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THETA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THETA` reader - THETA"]
pub type THETA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `THETA` writer - THETA"]
pub type THETA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, THETA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - THETA"]
    #[inline(always)]
    pub fn theta(&self) -> THETA_R {
        THETA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - THETA"]
    #[inline(always)]
    pub fn theta(&mut self) -> THETA_W<0> {
        THETA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SIN or COS input theta register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [theta](index.html) module"]
pub struct THETA_SPEC;
impl crate::RegisterSpec for THETA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [theta::R](R) reader structure"]
impl crate::Readable for THETA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [theta::W](W) writer structure"]
impl crate::Writable for THETA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets THETA to value 0"]
impl crate::Resettable for THETA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
