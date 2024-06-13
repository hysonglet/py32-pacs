#[doc = "Register `REMD` reader"]
pub struct R(crate::R<REMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REMD` reader - Remainder"]
pub type REMD_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Remainder"]
    #[inline(always)]
    pub fn remd(&self) -> REMD_R {
        REMD_R::new(self.bits)
    }
}
#[doc = "Remainder\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [remd](index.html) module"]
pub struct REMD_SPEC;
impl crate::RegisterSpec for REMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [remd::R](R) reader structure"]
impl crate::Readable for REMD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets REMD to value 0"]
impl crate::Resettable for REMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
