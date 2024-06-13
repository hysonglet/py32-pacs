#[doc = "Register `RESP1` reader"]
pub struct R(crate::R<RESP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CARDSTATUS2` reader - desc CARDSTATUS2"]
pub type CARDSTATUS2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - desc CARDSTATUS2"]
    #[inline(always)]
    pub fn cardstatus2(&self) -> CARDSTATUS2_R {
        CARDSTATUS2_R::new(self.bits)
    }
}
#[doc = "desc RESP1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp1](index.html) module"]
pub struct RESP1_SPEC;
impl crate::RegisterSpec for RESP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resp1::R](R) reader structure"]
impl crate::Readable for RESP1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESP1 to value 0"]
impl crate::Resettable for RESP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
