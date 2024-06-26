#[doc = "Register `TXCRCR` reader"]
pub struct R(crate::R<TXCRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXCRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXCRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXCRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXCRC` reader - desc TXCRC"]
pub type TXCRC_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc TXCRC"]
    #[inline(always)]
    pub fn txcrc(&self) -> TXCRC_R {
        TXCRC_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "desc TXCRCR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcrcr](index.html) module"]
pub struct TXCRCR_SPEC;
impl crate::RegisterSpec for TXCRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txcrcr::R](R) reader structure"]
impl crate::Readable for TXCRCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXCRCR to value 0"]
impl crate::Resettable for TXCRCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
