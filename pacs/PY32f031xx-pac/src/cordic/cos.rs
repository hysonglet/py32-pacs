#[doc = "Register `COS` reader"]
pub struct R(crate::R<COS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COS` reader - COS result"]
pub type COS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - COS result"]
    #[inline(always)]
    pub fn cos(&self) -> COS_R {
        COS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
#[doc = "CORDIC COS result register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cos](index.html) module"]
pub struct COS_SPEC;
impl crate::RegisterSpec for COS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cos::R](R) reader structure"]
impl crate::Readable for COS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets COS to value 0"]
impl crate::Resettable for COS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
