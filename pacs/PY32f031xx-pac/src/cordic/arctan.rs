#[doc = "Register `ARCTAN` reader"]
pub struct R(crate::R<ARCTAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARCTAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARCTAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARCTAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ARCTAN` reader - arctan result"]
pub type ARCTAN_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - arctan result"]
    #[inline(always)]
    pub fn arctan(&self) -> ARCTAN_R {
        ARCTAN_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "CORDIC ARCTAN result register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arctan](index.html) module"]
pub struct ARCTAN_SPEC;
impl crate::RegisterSpec for ARCTAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arctan::R](R) reader structure"]
impl crate::Readable for ARCTAN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ARCTAN to value 0"]
impl crate::Resettable for ARCTAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
