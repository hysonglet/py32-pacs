#[doc = "Register `WINR` reader"]
pub struct R(crate::R<WINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WINR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WIN` reader - window counter"]
pub type WIN_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - window counter"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Window register (IWDG_SR)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winr](index.html) module"]
pub struct WINR_SPEC;
impl crate::RegisterSpec for WINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [winr::R](R) reader structure"]
impl crate::Readable for WINR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WINR to value 0"]
impl crate::Resettable for WINR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
