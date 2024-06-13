#[doc = "Register `IDCODE` reader"]
pub struct R(crate::R<IDCODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDCODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDCODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDCODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REV_ID` reader - REV_ID"]
pub type REV_ID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:30 - REV_ID"]
    #[inline(always)]
    pub fn rev_id(&self) -> REV_ID_R {
        REV_ID_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
#[doc = "MCU Device ID Code Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idcode](index.html) module"]
pub struct IDCODE_SPEC;
impl crate::RegisterSpec for IDCODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idcode::R](R) reader structure"]
impl crate::Readable for IDCODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IDCODE to value 0"]
impl crate::Resettable for IDCODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
