#[doc = "Register `EGR` writer"]
pub struct W(crate::W<EGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EGR_SPEC>;
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
impl From<crate::W<EGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UG` writer - "]
pub type UG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, bool, O>;
#[doc = "Field `CC1G` writer - Capture/Compare 1 Generation"]
pub type CC1G_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, bool, O>;
#[doc = "Field `CC2G` writer - "]
pub type CC2G_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, bool, O>;
#[doc = "Field `CC3G` writer - "]
pub type CC3G_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, bool, O>;
#[doc = "Field `CC4G` writer - "]
pub type CC4G_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, bool, O>;
#[doc = "Field `COMG` writer - "]
pub type COMG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, bool, O>;
#[doc = "Field `TG` writer - "]
pub type TG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, bool, O>;
#[doc = "Field `BG` writer - "]
pub type BG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W<0> {
        UG_W::new(self)
    }
    #[doc = "Bit 1 - Capture/Compare 1 Generation"]
    #[inline(always)]
    pub fn cc1g(&mut self) -> CC1G_W<1> {
        CC1G_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cc2g(&mut self) -> CC2G_W<2> {
        CC2G_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cc3g(&mut self) -> CC3G_W<3> {
        CC3G_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cc4g(&mut self) -> CC4G_W<4> {
        CC4G_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn comg(&mut self) -> COMG_W<5> {
        COMG_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tg(&mut self) -> TG_W<6> {
        TG_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bg(&mut self) -> BG_W<7> {
        BG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [egr](index.html) module"]
pub struct EGR_SPEC;
impl crate::RegisterSpec for EGR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [egr::W](W) writer structure"]
impl crate::Writable for EGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EGR to value 0"]
impl crate::Resettable for EGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
