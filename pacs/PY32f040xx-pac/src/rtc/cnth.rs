#[doc = "Register `CNTH` reader"]
pub struct R(crate::R<CNTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTH` writer"]
pub struct W(crate::W<CNTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTH_SPEC>;
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
impl From<crate::W<CNTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_CNT` reader - desc RTC_CNT"]
pub type RTC_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RTC_CNT` writer - desc RTC_CNT"]
pub type RTC_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNTH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - desc RTC_CNT"]
    #[inline(always)]
    pub fn rtc_cnt(&self) -> RTC_CNT_R {
        RTC_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc RTC_CNT"]
    #[inline(always)]
    pub fn rtc_cnt(&mut self) -> RTC_CNT_W<0> {
        RTC_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CNTH\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnth](index.html) module"]
pub struct CNTH_SPEC;
impl crate::RegisterSpec for CNTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnth::R](R) reader structure"]
impl crate::Readable for CNTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnth::W](W) writer structure"]
impl crate::Writable for CNTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNTH to value 0"]
impl crate::Resettable for CNTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
