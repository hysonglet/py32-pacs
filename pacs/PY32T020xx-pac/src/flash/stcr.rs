#[doc = "Register `STCR` reader"]
pub struct R(crate::R<STCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STCR` writer"]
pub struct W(crate::W<STCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STCR_SPEC>;
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
impl From<crate::W<STCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEEP_EN` reader - FLASH Sleep Enable"]
pub type SLEEP_EN_R = crate::BitReader<bool>;
#[doc = "Field `SLEEP_EN` writer - FLASH Sleep Enable"]
pub type SLEEP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STCR_SPEC, bool, O>;
#[doc = "Field `SLEEP_TIME` reader - FLASH Sleep Time Configuration"]
pub type SLEEP_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLEEP_TIME` writer - FLASH Sleep Time Configuration"]
pub type SLEEP_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - FLASH Sleep Enable"]
    #[inline(always)]
    pub fn sleep_en(&self) -> SLEEP_EN_R {
        SLEEP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - FLASH Sleep Time Configuration"]
    #[inline(always)]
    pub fn sleep_time(&self) -> SLEEP_TIME_R {
        SLEEP_TIME_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FLASH Sleep Enable"]
    #[inline(always)]
    pub fn sleep_en(&mut self) -> SLEEP_EN_W<0> {
        SLEEP_EN_W::new(self)
    }
    #[doc = "Bits 8:15 - FLASH Sleep Time Configuration"]
    #[inline(always)]
    pub fn sleep_time(&mut self) -> SLEEP_TIME_W<8> {
        SLEEP_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcr](index.html) module"]
pub struct STCR_SPEC;
impl crate::RegisterSpec for STCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stcr::R](R) reader structure"]
impl crate::Readable for STCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stcr::W](W) writer structure"]
impl crate::Writable for STCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STCR to value 0"]
impl crate::Resettable for STCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
