#[doc = "Register `PA_ANA2EN` reader"]
pub struct R(crate::R<PA_ANA2EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PA_ANA2EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PA_ANA2EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PA_ANA2EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PA_ANA2EN` writer"]
pub struct W(crate::W<PA_ANA2EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PA_ANA2EN_SPEC>;
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
impl From<crate::W<PA_ANA2EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PA_ANA2EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA_ANA2EN` reader - IO PORTA PAD_ANA2 enable"]
pub type PA_ANA2EN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PA_ANA2EN` writer - IO PORTA PAD_ANA2 enable"]
pub type PA_ANA2EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PA_ANA2EN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - IO PORTA PAD_ANA2 enable"]
    #[inline(always)]
    pub fn pa_ana2en(&self) -> PA_ANA2EN_R {
        PA_ANA2EN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IO PORTA PAD_ANA2 enable"]
    #[inline(always)]
    pub fn pa_ana2en(&mut self) -> PA_ANA2EN_W<0> {
        PA_ANA2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOA Analog2 enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_ana2en](index.html) module"]
pub struct PA_ANA2EN_SPEC;
impl crate::RegisterSpec for PA_ANA2EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pa_ana2en::R](R) reader structure"]
impl crate::Readable for PA_ANA2EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pa_ana2en::W](W) writer structure"]
impl crate::Writable for PA_ANA2EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PA_ANA2EN to value 0"]
impl crate::Resettable for PA_ANA2EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
