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
#[doc = "Field `CALC0OUT` reader - C0 result"]
pub type CALC0OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALC1OUT` reader - C1 result"]
pub type CALC1OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALC2OUT` reader - C2 result"]
pub type CALC2OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALC3OUT` reader - C3 result"]
pub type CALC3OUT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - C0 result"]
    #[inline(always)]
    pub fn calc0out(&self) -> CALC0OUT_R {
        CALC0OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - C1 result"]
    #[inline(always)]
    pub fn calc1out(&self) -> CALC1OUT_R {
        CALC1OUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - C2 result"]
    #[inline(always)]
    pub fn calc2out(&self) -> CALC2OUT_R {
        CALC2OUT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - C3 result"]
    #[inline(always)]
    pub fn calc3out(&self) -> CALC3OUT_R {
        CALC3OUT_R::new(((self.bits >> 24) & 0xff) as u8)
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
