#[doc = "Register `ALRL` reader"]
pub struct R(crate::R<ALRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALRL` writer"]
pub struct W(crate::W<ALRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALRL_SPEC>;
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
impl From<crate::W<ALRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALRL` reader - RTC alarm register low"]
pub type ALRL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ALRL` writer - RTC alarm register low"]
pub type ALRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALRL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - RTC alarm register low"]
    #[inline(always)]
    pub fn alrl(&self) -> ALRL_R {
        ALRL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC alarm register low"]
    #[inline(always)]
    pub fn alrl(&mut self) -> ALRL_W<0> {
        ALRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Alarm Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrl](index.html) module"]
pub struct ALRL_SPEC;
impl crate::RegisterSpec for ALRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alrl::R](R) reader structure"]
impl crate::Readable for ALRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alrl::W](W) writer structure"]
impl crate::Writable for ALRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALRL to value 0xffff"]
impl crate::Resettable for ALRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
