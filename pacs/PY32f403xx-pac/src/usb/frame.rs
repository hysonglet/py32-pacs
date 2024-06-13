#[doc = "Register `FRAME` reader"]
pub struct R(crate::R<FRAME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRAMNUM` reader - desc FRAMNUM"]
pub type FRAMNUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INDEX` reader - INDEX"]
pub type INDEX_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:10 - desc FRAMNUM"]
    #[inline(always)]
    pub fn framnum(&self) -> FRAMNUM_R {
        FRAMNUM_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:19 - INDEX"]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "desc FRAME\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frame](index.html) module"]
pub struct FRAME_SPEC;
impl crate::RegisterSpec for FRAME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frame::R](R) reader structure"]
impl crate::Readable for FRAME_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FRAME to value 0"]
impl crate::Resettable for FRAME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
