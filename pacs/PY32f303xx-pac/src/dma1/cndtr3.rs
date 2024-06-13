#[doc = "Register `CNDTR3` reader"]
pub struct R(crate::R<CNDTR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNDTR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNDTR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNDTR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNDTR3` writer"]
pub struct W(crate::W<CNDTR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNDTR3_SPEC>;
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
impl From<crate::W<CNDTR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNDTR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDT` reader - desc NDT"]
pub type NDT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NDT` writer - desc NDT"]
pub type NDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNDTR3_SPEC, u16, u16, 16, O>;
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
#[doc = "desc CNDTR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndtr3](index.html) module"]
pub struct CNDTR3_SPEC;
impl crate::RegisterSpec for CNDTR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cndtr3::R](R) reader structure"]
impl crate::Readable for CNDTR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cndtr3::W](W) writer structure"]
impl crate::Writable for CNDTR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNDTR3 to value 0"]
impl crate::Resettable for CNDTR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
