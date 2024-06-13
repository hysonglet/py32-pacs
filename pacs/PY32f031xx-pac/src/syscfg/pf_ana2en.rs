#[doc = "Register `PF_ANA2EN` reader"]
pub struct R(crate::R<PF_ANA2EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PF_ANA2EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PF_ANA2EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PF_ANA2EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PF_ANA2EN` writer"]
pub struct W(crate::W<PF_ANA2EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PF_ANA2EN_SPEC>;
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
impl From<crate::W<PF_ANA2EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PF_ANA2EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PF_ANA2EN` reader - IO PORTF PAD_ANA2 enable"]
pub type PF_ANA2EN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PF_ANA2EN` writer - IO PORTF PAD_ANA2 enable"]
pub type PF_ANA2EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PF_ANA2EN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - IO PORTF PAD_ANA2 enable"]
    #[inline(always)]
    pub fn pf_ana2en(&self) -> PF_ANA2EN_R {
        PF_ANA2EN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IO PORTF PAD_ANA2 enable"]
    #[inline(always)]
    pub fn pf_ana2en(&mut self) -> PF_ANA2EN_W<0> {
        PF_ANA2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOF Analog2 enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_ana2en](index.html) module"]
pub struct PF_ANA2EN_SPEC;
impl crate::RegisterSpec for PF_ANA2EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pf_ana2en::R](R) reader structure"]
impl crate::Readable for PF_ANA2EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pf_ana2en::W](W) writer structure"]
impl crate::Writable for PF_ANA2EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PF_ANA2EN to value 0"]
impl crate::Resettable for PF_ANA2EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
