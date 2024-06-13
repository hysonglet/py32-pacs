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
#[doc = "Field `HSE_FREQ` reader - HSE clock freqency selection"]
pub type HSE_FREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSE_FREQ` writer - HSE clock freqency selection"]
pub type HSE_FREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECSCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 2:3 - HSE clock freqency selection"]
    #[inline(always)]
    pub fn hse_freq(&self) -> HSE_FREQ_R {
        HSE_FREQ_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3 - HSE clock freqency selection"]
    #[inline(always)]
    pub fn hse_freq(&mut self) -> HSE_FREQ_W<2> {
        HSE_FREQ_W::new(self)
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
