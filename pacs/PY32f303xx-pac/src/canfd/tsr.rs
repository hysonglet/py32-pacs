#[doc = "Register `TSR` reader"]
pub struct R(crate::R<TSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HANDLE_L` reader - desc HANDLE_L"]
pub type HANDLE_L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSTAT_L` reader - desc TSTAT_L"]
pub type TSTAT_L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HANDLE_H` reader - desc HANDLE_H"]
pub type HANDLE_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSTAT_H` reader - desc TSTAT_H"]
pub type TSTAT_H_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - desc HANDLE_L"]
    #[inline(always)]
    pub fn handle_l(&self) -> HANDLE_L_R {
        HANDLE_L_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - desc TSTAT_L"]
    #[inline(always)]
    pub fn tstat_l(&self) -> TSTAT_L_R {
        TSTAT_L_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:23 - desc HANDLE_H"]
    #[inline(always)]
    pub fn handle_h(&self) -> HANDLE_H_R {
        HANDLE_H_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - desc TSTAT_H"]
    #[inline(always)]
    pub fn tstat_h(&self) -> TSTAT_H_R {
        TSTAT_H_R::new(((self.bits >> 24) & 7) as u8)
    }
}
#[doc = "desc TSR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsr](index.html) module"]
pub struct TSR_SPEC;
impl crate::RegisterSpec for TSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsr::R](R) reader structure"]
impl crate::Readable for TSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSR to value 0"]
impl crate::Resettable for TSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
