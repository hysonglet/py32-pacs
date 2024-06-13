#[doc = "Register `OUTCOUNT` reader"]
pub struct R(crate::R<OUTCOUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTCOUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTCOUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTCOUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUTCOUNT` reader - desc OUTCOUNT"]
pub type OUTCOUNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - desc OUTCOUNT"]
    #[inline(always)]
    pub fn outcount(&self) -> OUTCOUNT_R {
        OUTCOUNT_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "desc OUTCOUNT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outcount](index.html) module"]
pub struct OUTCOUNT_SPEC;
impl crate::RegisterSpec for OUTCOUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outcount::R](R) reader structure"]
impl crate::Readable for OUTCOUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OUTCOUNT to value 0"]
impl crate::Resettable for OUTCOUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
