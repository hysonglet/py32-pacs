#[doc = "Register `SR2` reader"]
pub struct R(crate::R<SR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MSL` reader - desc MSL"]
pub type MSL_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` reader - desc BUSY"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `TRA` reader - desc TRA"]
pub type TRA_R = crate::BitReader<bool>;
#[doc = "Field `GENCALL` reader - desc GENCALL"]
pub type GENCALL_R = crate::BitReader<bool>;
#[doc = "Field `SMBDEFAULT` reader - desc SMBDEFAULT"]
pub type SMBDEFAULT_R = crate::BitReader<bool>;
#[doc = "Field `SMBHOST` reader - desc SMBHOST"]
pub type SMBHOST_R = crate::BitReader<bool>;
#[doc = "Field `DUALF` reader - desc DUALF"]
pub type DUALF_R = crate::BitReader<bool>;
#[doc = "Field `PEC` reader - desc PEC"]
pub type PEC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - desc MSL"]
    #[inline(always)]
    pub fn msl(&self) -> MSL_R {
        MSL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TRA"]
    #[inline(always)]
    pub fn tra(&self) -> TRA_R {
        TRA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc GENCALL"]
    #[inline(always)]
    pub fn gencall(&self) -> GENCALL_R {
        GENCALL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc SMBDEFAULT"]
    #[inline(always)]
    pub fn smbdefault(&self) -> SMBDEFAULT_R {
        SMBDEFAULT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc SMBHOST"]
    #[inline(always)]
    pub fn smbhost(&self) -> SMBHOST_R {
        SMBHOST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc DUALF"]
    #[inline(always)]
    pub fn dualf(&self) -> DUALF_R {
        DUALF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - desc PEC"]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "desc SR2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr2](index.html) module"]
pub struct SR2_SPEC;
impl crate::RegisterSpec for SR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr2::R](R) reader structure"]
impl crate::Readable for SR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR2 to value 0"]
impl crate::Resettable for SR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
