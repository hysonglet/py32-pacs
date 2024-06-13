#[doc = "Register `SIN` reader"]
pub struct R(crate::R<SIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIN` reader - SIN result"]
pub type SIN_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - SIN result"]
    #[inline(always)]
    pub fn sin(&self) -> SIN_R {
        SIN_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
#[doc = "CORDIC SIN result register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sin](index.html) module"]
pub struct SIN_SPEC;
impl crate::RegisterSpec for SIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sin::R](R) reader structure"]
impl crate::Readable for SIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIN to value 0"]
impl crate::Resettable for SIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
