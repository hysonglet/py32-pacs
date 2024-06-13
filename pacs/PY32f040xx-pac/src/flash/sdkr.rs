#[doc = "Register `SDKR` reader"]
pub struct R(crate::R<SDKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SDK_STRT` reader - desc SDK_STRT"]
pub type SDK_STRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOR_EN` reader - desc BOR_EN"]
pub type BOR_EN_R = crate::BitReader<bool>;
#[doc = "Field `SDK_END` reader - desc SDK_END"]
pub type SDK_END_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOR_LEV` reader - desc BOR_LEV"]
pub type BOR_LEV_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - desc SDK_STRT"]
    #[inline(always)]
    pub fn sdk_strt(&self) -> SDK_STRT_R {
        SDK_STRT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - desc BOR_EN"]
    #[inline(always)]
    pub fn bor_en(&self) -> BOR_EN_R {
        BOR_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:12 - desc SDK_END"]
    #[inline(always)]
    pub fn sdk_end(&self) -> SDK_END_R {
        SDK_END_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - desc BOR_LEV"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 13) & 7) as u8)
    }
}
#[doc = "desc SDKR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdkr](index.html) module"]
pub struct SDKR_SPEC;
impl crate::RegisterSpec for SDKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdkr::R](R) reader structure"]
impl crate::Readable for SDKR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SDKR to value 0"]
impl crate::Resettable for SDKR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
