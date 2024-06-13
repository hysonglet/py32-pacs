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
#[doc = "Field `CALC4OUT` reader - C4 result"]
pub type CALC4OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALC5OUT` reader - C5 result"]
pub type CALC5OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALBOUT` reader - offset result"]
pub type CALBOUT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - C4 result"]
    #[inline(always)]
    pub fn calc4out(&self) -> CALC4OUT_R {
        CALC4OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - C5 result"]
    #[inline(always)]
    pub fn calc5out(&self) -> CALC5OUT_R {
        CALC5OUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - offset result"]
    #[inline(always)]
    pub fn calbout(&self) -> CALBOUT_R {
        CALBOUT_R::new(((self.bits >> 16) & 0x7f) as u8)
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
