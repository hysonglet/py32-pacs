#[doc = "Register `ECSCR` reader"]
pub struct R(crate::R<ECSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECSCR` writer"]
pub struct W(crate::W<ECSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECSCR_SPEC>;
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
impl From<crate::W<ECSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSE_DRV` reader - HSE_DRV"]
pub type HSE_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSE_DRV` writer - HSE_DRV"]
pub type HSE_DRV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECSCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `HSE_STARTUP` reader - HSE_STARTUP"]
pub type HSE_STARTUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSE_STARTUP` writer - HSE_STARTUP"]
pub type HSE_STARTUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECSCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `LSE_DRIVER` reader - LSE clock driver selection"]
pub type LSE_DRIVER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LSE_DRIVER` writer - LSE clock driver selection"]
pub type LSE_DRIVER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECSCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `LSE_STARTUP` reader - LSE_STARTUP"]
pub type LSE_STARTUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LSE_STARTUP` writer - LSE_STARTUP"]
pub type LSE_STARTUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECSCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - HSE_DRV"]
    #[inline(always)]
    pub fn hse_drv(&self) -> HSE_DRV_R {
        HSE_DRV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 3:4 - HSE_STARTUP"]
    #[inline(always)]
    pub fn hse_startup(&self) -> HSE_STARTUP_R {
        HSE_STARTUP_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 16:17 - LSE clock driver selection"]
    #[inline(always)]
    pub fn lse_driver(&self) -> LSE_DRIVER_R {
        LSE_DRIVER_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - LSE_STARTUP"]
    #[inline(always)]
    pub fn lse_startup(&self) -> LSE_STARTUP_R {
        LSE_STARTUP_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - HSE_DRV"]
    #[inline(always)]
    pub fn hse_drv(&mut self) -> HSE_DRV_W<0> {
        HSE_DRV_W::new(self)
    }
    #[doc = "Bits 3:4 - HSE_STARTUP"]
    #[inline(always)]
    pub fn hse_startup(&mut self) -> HSE_STARTUP_W<3> {
        HSE_STARTUP_W::new(self)
    }
    #[doc = "Bits 16:17 - LSE clock driver selection"]
    #[inline(always)]
    pub fn lse_driver(&mut self) -> LSE_DRIVER_W<16> {
        LSE_DRIVER_W::new(self)
    }
    #[doc = "Bits 20:21 - LSE_STARTUP"]
    #[inline(always)]
    pub fn lse_startup(&mut self) -> LSE_STARTUP_W<20> {
        LSE_STARTUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External clock source control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecscr](index.html) module"]
pub struct ECSCR_SPEC;
impl crate::RegisterSpec for ECSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecscr::R](R) reader structure"]
impl crate::Readable for ECSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecscr::W](W) writer structure"]
impl crate::Writable for ECSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ECSCR to value 0"]
impl crate::Resettable for ECSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
