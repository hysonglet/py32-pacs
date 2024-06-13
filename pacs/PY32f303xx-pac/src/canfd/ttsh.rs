#[doc = "Register `TTSH` reader"]
pub struct R(crate::R<TTSH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTSH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTSH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTSH_SPEC>) -> Self {
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
#[doc = "desc TTSH\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttsh](index.html) module"]
pub struct TTSH_SPEC;
impl crate::RegisterSpec for TTSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttsh::R](R) reader structure"]
impl crate::Readable for TTSH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TTSH to value 0x0200_0000"]
impl crate::Resettable for TTSH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0000
    }
}
