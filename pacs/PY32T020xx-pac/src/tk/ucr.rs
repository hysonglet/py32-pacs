#[doc = "Register `UCR` reader"]
pub struct R(crate::R<UCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCR` writer"]
pub struct W(crate::W<UCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCR_SPEC>;
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
impl From<crate::W<UCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COPAMODEIO` reader - "]
pub type COPAMODEIO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COPAMODEIO` writer - "]
pub type COPAMODEIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UCR_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn copamodeio(&self) -> COPAMODEIO_R {
        COPAMODEIO_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn copamodeio(&mut self) -> COPAMODEIO_W<0> {
        COPAMODEIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Unsel Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucr](index.html) module"]
pub struct UCR_SPEC;
impl crate::RegisterSpec for UCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ucr::R](R) reader structure"]
impl crate::Readable for UCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucr::W](W) writer structure"]
impl crate::Writable for UCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCR to value 0"]
impl crate::Resettable for UCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
