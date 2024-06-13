#[doc = "Register `DSP_RAD` reader"]
pub struct R(crate::R<DSP_RAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSP_RAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSP_RAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSP_RAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSP_RAD` writer"]
pub struct W(crate::W<DSP_RAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSP_RAD_SPEC>;
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
impl From<crate::W<DSP_RAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSP_RAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAD` reader - Number of open parties"]
pub type RAD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RAD` writer - Number of open parties"]
pub type RAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DSP_RAD_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Number of open parties"]
    #[inline(always)]
    pub fn rad(&self) -> RAD_R {
        RAD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of open parties"]
    #[inline(always)]
    pub fn rad(&mut self) -> RAD_W<0> {
        RAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CORDIC Radicand register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsp_rad](index.html) module"]
pub struct DSP_RAD_SPEC;
impl crate::RegisterSpec for DSP_RAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsp_rad::R](R) reader structure"]
impl crate::Readable for DSP_RAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsp_rad::W](W) writer structure"]
impl crate::Writable for DSP_RAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSP_RAD to value 0"]
impl crate::Resettable for DSP_RAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
