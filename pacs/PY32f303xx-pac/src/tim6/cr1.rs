#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEN` reader - desc CEN"]
pub type CEN_R = crate::BitReader<bool>;
#[doc = "Field `CEN` writer - desc CEN"]
pub type CEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `UDIS` reader - desc UDIS"]
pub type UDIS_R = crate::BitReader<bool>;
#[doc = "Field `UDIS` writer - desc UDIS"]
pub type UDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `URS` reader - desc URS"]
pub type URS_R = crate::BitReader<bool>;
#[doc = "Field `URS` writer - desc URS"]
pub type URS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `OPM` reader - desc OPM"]
pub type OPM_R = crate::BitReader<bool>;
#[doc = "Field `OPM` writer - desc OPM"]
pub type OPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ARPE` reader - desc ARPE"]
pub type ARPE_R = crate::BitReader<bool>;
#[doc = "Field `ARPE` writer - desc ARPE"]
pub type ARPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc CEN"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc UDIS"]
    #[inline(always)]
    pub fn udis(&self) -> UDIS_R {
        UDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc URS"]
    #[inline(always)]
    pub fn urs(&self) -> URS_R {
        URS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc OPM"]
    #[inline(always)]
    pub fn opm(&self) -> OPM_R {
        OPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - desc ARPE"]
    #[inline(always)]
    pub fn arpe(&self) -> ARPE_R {
        ARPE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CEN"]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W<0> {
        CEN_W::new(self)
    }
    #[doc = "Bit 1 - desc UDIS"]
    #[inline(always)]
    pub fn udis(&mut self) -> UDIS_W<1> {
        UDIS_W::new(self)
    }
    #[doc = "Bit 2 - desc URS"]
    #[inline(always)]
    pub fn urs(&mut self) -> URS_W<2> {
        URS_W::new(self)
    }
    #[doc = "Bit 3 - desc OPM"]
    #[inline(always)]
    pub fn opm(&mut self) -> OPM_W<3> {
        OPM_W::new(self)
    }
    #[doc = "Bit 7 - desc ARPE"]
    #[inline(always)]
    pub fn arpe(&mut self) -> ARPE_W<7> {
        ARPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
