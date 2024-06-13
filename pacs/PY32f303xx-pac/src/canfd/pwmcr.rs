#[doc = "Register `PWMCR` reader"]
pub struct R(crate::R<PWMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWMCR` writer"]
pub struct W(crate::W<PWMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWMCR_SPEC>;
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
impl From<crate::W<PWMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWMO` reader - desc PWMO"]
pub type PWMO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWMO` writer - desc PWMO"]
pub type PWMO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWMCR_SPEC, u8, u8, 6, O>;
#[doc = "Field `PWMS` reader - desc PWMS"]
pub type PWMS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWMS` writer - desc PWMS"]
pub type PWMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWMCR_SPEC, u8, u8, 6, O>;
#[doc = "Field `PWML` reader - desc PWML"]
pub type PWML_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWML` writer - desc PWML"]
pub type PWML_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWMCR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - desc PWMO"]
    #[inline(always)]
    pub fn pwmo(&self) -> PWMO_R {
        PWMO_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - desc PWMS"]
    #[inline(always)]
    pub fn pwms(&self) -> PWMS_R {
        PWMS_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - desc PWML"]
    #[inline(always)]
    pub fn pwml(&self) -> PWML_R {
        PWML_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - desc PWMO"]
    #[inline(always)]
    pub fn pwmo(&mut self) -> PWMO_W<0> {
        PWMO_W::new(self)
    }
    #[doc = "Bits 8:13 - desc PWMS"]
    #[inline(always)]
    pub fn pwms(&mut self) -> PWMS_W<8> {
        PWMS_W::new(self)
    }
    #[doc = "Bits 16:21 - desc PWML"]
    #[inline(always)]
    pub fn pwml(&mut self) -> PWML_W<16> {
        PWML_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc PWMCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwmcr](index.html) module"]
pub struct PWMCR_SPEC;
impl crate::RegisterSpec for PWMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwmcr::R](R) reader structure"]
impl crate::Readable for PWMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwmcr::W](W) writer structure"]
impl crate::Writable for PWMCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWMCR to value 0x0208_0400"]
impl crate::Resettable for PWMCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0208_0400
    }
}
