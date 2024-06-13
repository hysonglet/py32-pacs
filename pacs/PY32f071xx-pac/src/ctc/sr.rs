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
#[doc = "Field `CKOKIF` reader - desc CKOKIF"]
pub type CKOKIF_R = crate::BitReader<bool>;
#[doc = "Field `CKWARNIF` reader - desc CKWARNIF"]
pub type CKWARNIF_R = crate::BitReader<bool>;
#[doc = "Field `ERRIF` reader - desc ERRIF"]
pub type ERRIF_R = crate::BitReader<bool>;
#[doc = "Field `EREFIF` reader - desc EREFIF"]
pub type EREFIF_R = crate::BitReader<bool>;
#[doc = "Field `CKERR` reader - desc CKERR"]
pub type CKERR_R = crate::BitReader<bool>;
#[doc = "Field `REFMISS` reader - desc REFMISS"]
pub type REFMISS_R = crate::BitReader<bool>;
#[doc = "Field `TRIMERR` reader - desc TRIMERR"]
pub type TRIMERR_R = crate::BitReader<bool>;
#[doc = "Field `REFDIR` reader - desc REFDIR"]
pub type REFDIR_R = crate::BitReader<bool>;
#[doc = "Field `REFCAP` reader - desc REFCAP"]
pub type REFCAP_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - desc CKOKIF"]
    #[inline(always)]
    pub fn ckokif(&self) -> CKOKIF_R {
        CKOKIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CKWARNIF"]
    #[inline(always)]
    pub fn ckwarnif(&self) -> CKWARNIF_R {
        CKWARNIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc ERRIF"]
    #[inline(always)]
    pub fn errif(&self) -> ERRIF_R {
        ERRIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc EREFIF"]
    #[inline(always)]
    pub fn erefif(&self) -> EREFIF_R {
        EREFIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - desc CKERR"]
    #[inline(always)]
    pub fn ckerr(&self) -> CKERR_R {
        CKERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc REFMISS"]
    #[inline(always)]
    pub fn refmiss(&self) -> REFMISS_R {
        REFMISS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc TRIMERR"]
    #[inline(always)]
    pub fn trimerr(&self) -> TRIMERR_R {
        TRIMERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - desc REFDIR"]
    #[inline(always)]
    pub fn refdir(&self) -> REFDIR_R {
        REFDIR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - desc REFCAP"]
    #[inline(always)]
    pub fn refcap(&self) -> REFCAP_R {
        REFCAP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "desc SR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
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
