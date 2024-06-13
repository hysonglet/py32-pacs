#[doc = "Register `CALRR1` reader"]
pub struct R(crate::R<CALRR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALRR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALRR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALRR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CALC10OUT` reader - desc CALC10OUT"]
pub type CALC10OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALC11OUT` reader - desc CALC11OUT"]
pub type CALC11OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALBOUT` reader - desc CALBOUT"]
pub type CALBOUT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - desc CALC10OUT"]
    #[inline(always)]
    pub fn calc10out(&self) -> CALC10OUT_R {
        CALC10OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - desc CALC11OUT"]
    #[inline(always)]
    pub fn calc11out(&self) -> CALC11OUT_R {
        CALC11OUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - desc CALBOUT"]
    #[inline(always)]
    pub fn calbout(&self) -> CALBOUT_R {
        CALBOUT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "desc CALRR1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calrr1](index.html) module"]
pub struct CALRR1_SPEC;
impl crate::RegisterSpec for CALRR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calrr1::R](R) reader structure"]
impl crate::Readable for CALRR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CALRR1 to value 0"]
impl crate::Resettable for CALRR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
