#[doc = "Register `JDR4` reader"]
pub struct R(crate::R<JDR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JDR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JDR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JDR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JDR4` reader - desc JDR4"]
pub type JDR4_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc JDR4"]
    #[inline(always)]
    pub fn jdr4(&self) -> JDR4_R {
        JDR4_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "desc JDR4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdr4](index.html) module"]
pub struct JDR4_SPEC;
impl crate::RegisterSpec for JDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jdr4::R](R) reader structure"]
impl crate::Readable for JDR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets JDR4 to value 0"]
impl crate::Resettable for JDR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
