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
#[doc = "Field `CALC9OUT` reader - C9 result"]
pub type CALC9OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALC10OUT` reader - C10 result"]
pub type CALC10OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALC11OUT` reader - C11 result"]
pub type CALC11OUT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - C9 result"]
    #[inline(always)]
    pub fn calc9out(&self) -> CALC9OUT_R {
        CALC9OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 9:16 - C10 result"]
    #[inline(always)]
    pub fn calc10out(&self) -> CALC10OUT_R {
        CALC10OUT_R::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bits 18:24 - C11 result"]
    #[inline(always)]
    pub fn calc11out(&self) -> CALC11OUT_R {
        CALC11OUT_R::new(((self.bits >> 18) & 0x7f) as u8)
    }
}
#[doc = "ADC calibration result register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calrr1](index.html) module"]
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
