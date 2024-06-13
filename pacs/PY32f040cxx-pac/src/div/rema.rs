#[doc = "Register `REMA` reader"]
pub struct R(crate::R<REMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REMA` reader - Remainder"]
pub type REMA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Remainder"]
    #[inline(always)]
    pub fn rema(&self) -> REMA_R {
        REMA_R::new(self.bits)
    }
}
#[doc = "Remainder\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rema](index.html) module"]
pub struct REMA_SPEC;
impl crate::RegisterSpec for REMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rema::R](R) reader structure"]
impl crate::Readable for REMA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets REMA to value 0"]
impl crate::Resettable for REMA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
