#[doc = "Register `MOD` reader"]
pub struct R(crate::R<MOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MOD` reader - mod result"]
pub type MOD_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:24 - mod result"]
    #[inline(always)]
    pub fn mod_(&self) -> MOD_R {
        MOD_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
#[doc = "CORDIC Mod result register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mod_](index.html) module"]
pub struct MOD_SPEC;
impl crate::RegisterSpec for MOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mod_::R](R) reader structure"]
impl crate::Readable for MOD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MOD to value 0"]
impl crate::Resettable for MOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
