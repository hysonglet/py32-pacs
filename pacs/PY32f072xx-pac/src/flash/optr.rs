#[doc = "Register `OPTR` reader"]
pub struct R(crate::R<OPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDP` reader - desc RDP"]
pub type RDP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IWDG_SW` reader - desc IWDG_SW"]
pub type IWDG_SW_R = crate::BitReader<bool>;
#[doc = "Field `WWDG_SW` reader - desc WWDG_SW"]
pub type WWDG_SW_R = crate::BitReader<bool>;
#[doc = "Field `NRST_MODE` reader - desc NRST_MODE"]
pub type NRST_MODE_R = crate::BitReader<bool>;
#[doc = "Field `NBOOT1` reader - desc nBOOT1"]
pub type NBOOT1_R = crate::BitReader<bool>;
#[doc = "Field `IWDG_STOP` reader - desc IWDG_STOP"]
pub type IWDG_STOP_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:7 - desc RDP"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 11 - desc IWDG_SW"]
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc WWDG_SW"]
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc NRST_MODE"]
    #[inline(always)]
    pub fn nrst_mode(&self) -> NRST_MODE_R {
        NRST_MODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc nBOOT1"]
    #[inline(always)]
    pub fn nboot1(&self) -> NBOOT1_R {
        NBOOT1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc IWDG_STOP"]
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "desc OPTR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optr](index.html) module"]
pub struct OPTR_SPEC;
impl crate::RegisterSpec for OPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [optr::R](R) reader structure"]
impl crate::Readable for OPTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OPTR to value 0"]
impl crate::Resettable for OPTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
