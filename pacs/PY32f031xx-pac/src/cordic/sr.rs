#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CCFF` reader - desc CCFF"]
pub type CCFF_R = crate::BitReader<bool>;
#[doc = "Field `SCFF` reader - desc SCFF"]
pub type SCFF_R = crate::BitReader<bool>;
#[doc = "Field `CCEF` reader - desc CCFF"]
pub type CCEF_R = crate::BitReader<bool>;
#[doc = "Field `ACEF` reader - desc ACFF"]
pub type ACEF_R = crate::BitReader<bool>;
#[doc = "Field `BSY` reader - desc BSY"]
pub type BSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - desc CCFF"]
    #[inline(always)]
    pub fn ccff(&self) -> CCFF_R {
        CCFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SCFF"]
    #[inline(always)]
    pub fn scff(&self) -> SCFF_R {
        SCFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CCFF"]
    #[inline(always)]
    pub fn ccef(&self) -> CCEF_R {
        CCEF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc ACFF"]
    #[inline(always)]
    pub fn acef(&self) -> ACEF_R {
        ACEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc BSY"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "CORDIC Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
