#[doc = "Register `TTSL` reader"]
pub struct R(crate::R<TTSL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTSL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTSL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTSL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TTS` reader - desc TTS"]
pub type TTS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - desc TTS"]
    #[inline(always)]
    pub fn tts(&self) -> TTS_R {
        TTS_R::new(self.bits)
    }
}
#[doc = "desc TTSL\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttsl](index.html) module"]
pub struct TTSL_SPEC;
impl crate::RegisterSpec for TTSL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttsl::R](R) reader structure"]
impl crate::Readable for TTSL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TTSL to value 0x0200_0000"]
impl crate::Resettable for TTSL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0000
    }
}
