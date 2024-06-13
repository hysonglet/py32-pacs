#[doc = "Register `CALRR2` reader"]
pub struct R(crate::R<CALRR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALRR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALRR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALRR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CALC6OUT` reader - desc CALC6OUT"]
pub type CALC6OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALC7OUT` reader - desc CALC7OUT"]
pub type CALC7OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALC8OUT` reader - desc CALC8OUT"]
pub type CALC8OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALC9OUT` reader - desc CALC9OUT"]
pub type CALC9OUT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - desc CALC6OUT"]
    #[inline(always)]
    pub fn calc6out(&self) -> CALC6OUT_R {
        CALC6OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - desc CALC7OUT"]
    #[inline(always)]
    pub fn calc7out(&self) -> CALC7OUT_R {
        CALC7OUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - desc CALC8OUT"]
    #[inline(always)]
    pub fn calc8out(&self) -> CALC8OUT_R {
        CALC8OUT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - desc CALC9OUT"]
    #[inline(always)]
    pub fn calc9out(&self) -> CALC9OUT_R {
        CALC9OUT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "desc CALRR2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calrr2](index.html) module"]
pub struct CALRR2_SPEC;
impl crate::RegisterSpec for CALRR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calrr2::R](R) reader structure"]
impl crate::Readable for CALRR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CALRR2 to value 0"]
impl crate::Resettable for CALRR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
