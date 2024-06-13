#[doc = "Register `QUOT` reader"]
pub struct R(crate::R<QUOT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUOT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUOT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUOT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `QUOT` reader - Quotient"]
pub type QUOT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Quotient"]
    #[inline(always)]
    pub fn quot(&self) -> QUOT_R {
        QUOT_R::new(self.bits)
    }
}
#[doc = "Quotient\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quot](index.html) module"]
pub struct QUOT_SPEC;
impl crate::RegisterSpec for QUOT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [quot::R](R) reader structure"]
impl crate::Readable for QUOT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QUOT to value 0"]
impl crate::Resettable for QUOT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
