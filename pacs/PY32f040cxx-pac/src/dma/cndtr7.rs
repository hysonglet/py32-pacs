#[doc = "Register `CNDTR7` reader"]
pub struct R(crate::R<CNDTR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNDTR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNDTR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNDTR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNDTR7` writer"]
pub struct W(crate::W<CNDTR7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNDTR7_SPEC>;
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
impl From<crate::W<CNDTR7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNDTR7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDT` reader - desc NDT"]
pub type NDT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NDT` writer - desc NDT"]
pub type NDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNDTR7_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - desc NDT"]
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc NDT"]
    #[inline(always)]
    pub fn ndt(&mut self) -> NDT_W<0> {
        NDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CNDTR7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndtr7](index.html) module"]
pub struct CNDTR7_SPEC;
impl crate::RegisterSpec for CNDTR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cndtr7::R](R) reader structure"]
impl crate::Readable for CNDTR7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cndtr7::W](W) writer structure"]
impl crate::Writable for CNDTR7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNDTR7 to value 0"]
impl crate::Resettable for CNDTR7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
