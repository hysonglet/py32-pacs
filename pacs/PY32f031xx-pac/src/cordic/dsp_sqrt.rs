#[doc = "Register `DSP_SQRT` reader"]
pub struct R(crate::R<DSP_SQRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSP_SQRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSP_SQRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSP_SQRT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SQRT` reader - sqrt result"]
pub type SQRT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - sqrt result"]
    #[inline(always)]
    pub fn sqrt(&self) -> SQRT_R {
        SQRT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "CORDIC SQRT result register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsp_sqrt](index.html) module"]
pub struct DSP_SQRT_SPEC;
impl crate::RegisterSpec for DSP_SQRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsp_sqrt::R](R) reader structure"]
impl crate::Readable for DSP_SQRT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DSP_SQRT to value 0"]
impl crate::Resettable for DSP_SQRT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
