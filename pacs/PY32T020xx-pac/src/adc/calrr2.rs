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
#[doc = "Field `CALC6UT` reader - C6 result"]
pub type CALC6UT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALC7OUT` reader - C7 result"]
pub type CALC7OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALC8OUT` reader - C8 result"]
pub type CALC8OUT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - C6 result"]
    #[inline(always)]
    pub fn calc6ut(&self) -> CALC6UT_R {
        CALC6UT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 9:16 - C7 result"]
    #[inline(always)]
    pub fn calc7out(&self) -> CALC7OUT_R {
        CALC7OUT_R::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bits 18:25 - C8 result"]
    #[inline(always)]
    pub fn calc8out(&self) -> CALC8OUT_R {
        CALC8OUT_R::new(((self.bits >> 18) & 0xff) as u8)
    }
}
#[doc = "ADC calibration result register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calrr2](index.html) module"]
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
