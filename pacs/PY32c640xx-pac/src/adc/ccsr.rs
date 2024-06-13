#[doc = "Register `CCSR` reader"]
pub struct R(crate::R<CCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCSR` writer"]
pub struct W(crate::W<CCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALSEL` reader - Calibration contents selection"]
pub type CALSEL_R = crate::BitReader<bool>;
#[doc = "Field `CALSEL` writer - Calibration contents selection"]
pub type CALSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCSR_SPEC, bool, O>;
#[doc = "Field `CALSMP` reader - Calibration sample time selection"]
pub type CALSMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALSMP` writer - Calibration sample time selection"]
pub type CALSMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CALBYP` reader - desc CALBYP"]
pub type CALBYP_R = crate::BitReader<bool>;
#[doc = "Field `CALBYP` writer - desc CALBYP"]
pub type CALBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCSR_SPEC, bool, O>;
#[doc = "Field `CALSET` reader - Calibration factor selection"]
pub type CALSET_R = crate::BitReader<bool>;
#[doc = "Field `CALSET` writer - Calibration factor selection"]
pub type CALSET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCSR_SPEC, bool, O>;
#[doc = "Field `OFFSUC` reader - desc OFFSUC"]
pub type OFFSUC_R = crate::BitReader<bool>;
#[doc = "Field `OFFSUC` writer - desc OFFSUC"]
pub type OFFSUC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCSR_SPEC, bool, O>;
#[doc = "Field `CALFAIL` reader - Calibration fail flag"]
pub type CALFAIL_R = crate::BitReader<bool>;
#[doc = "Field `CALFAIL` writer - Calibration fail flag"]
pub type CALFAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCSR_SPEC, bool, O>;
#[doc = "Field `CALON` reader - Calibration flag"]
pub type CALON_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 11 - Calibration contents selection"]
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Calibration sample time selection"]
    #[inline(always)]
    pub fn calsmp(&self) -> CALSMP_R {
        CALSMP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - desc CALBYP"]
    #[inline(always)]
    pub fn calbyp(&self) -> CALBYP_R {
        CALBYP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Calibration factor selection"]
    #[inline(always)]
    pub fn calset(&self) -> CALSET_R {
        CALSET_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 29 - desc OFFSUC"]
    #[inline(always)]
    pub fn offsuc(&self) -> OFFSUC_R {
        OFFSUC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Calibration fail flag"]
    #[inline(always)]
    pub fn calfail(&self) -> CALFAIL_R {
        CALFAIL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Calibration flag"]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Calibration contents selection"]
    #[inline(always)]
    pub fn calsel(&mut self) -> CALSEL_W<11> {
        CALSEL_W::new(self)
    }
    #[doc = "Bits 12:13 - Calibration sample time selection"]
    #[inline(always)]
    pub fn calsmp(&mut self) -> CALSMP_W<12> {
        CALSMP_W::new(self)
    }
    #[doc = "Bit 14 - desc CALBYP"]
    #[inline(always)]
    pub fn calbyp(&mut self) -> CALBYP_W<14> {
        CALBYP_W::new(self)
    }
    #[doc = "Bit 15 - Calibration factor selection"]
    #[inline(always)]
    pub fn calset(&mut self) -> CALSET_W<15> {
        CALSET_W::new(self)
    }
    #[doc = "Bit 29 - desc OFFSUC"]
    #[inline(always)]
    pub fn offsuc(&mut self) -> OFFSUC_W<29> {
        OFFSUC_W::new(self)
    }
    #[doc = "Bit 30 - Calibration fail flag"]
    #[inline(always)]
    pub fn calfail(&mut self) -> CALFAIL_W<30> {
        CALFAIL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC calibration configuration and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccsr](index.html) module"]
pub struct CCSR_SPEC;
impl crate::RegisterSpec for CCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccsr::R](R) reader structure"]
impl crate::Readable for CCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccsr::W](W) writer structure"]
impl crate::Writable for CCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCSR to value 0"]
impl crate::Resettable for CCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
