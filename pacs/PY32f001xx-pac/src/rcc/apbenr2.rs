#[doc = "Register `APBENR2` reader"]
pub struct R(crate::R<APBENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBENR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBENR2` writer"]
pub struct W(crate::W<APBENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBENR2_SPEC>;
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
impl From<crate::W<APBENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBENR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCFGEN` reader - SYSCFG, COMP and VREFBUF clock enable"]
pub type SYSCFGEN_R = crate::BitReader<bool>;
#[doc = "Field `SYSCFGEN` writer - SYSCFG, COMP and VREFBUF clock enable"]
pub type SYSCFGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR2_SPEC, bool, O>;
#[doc = "Field `TIM1EN` reader - TIM1 timer clock enable"]
pub type TIM1EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM1EN` writer - TIM1 timer clock enable"]
pub type TIM1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR2_SPEC, bool, O>;
#[doc = "Field `TIM14EN` reader - TIM14 clock enable"]
pub type TIM14EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM14EN` writer - TIM14 clock enable"]
pub type TIM14EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR2_SPEC, bool, O>;
#[doc = "Field `COMP1EN` reader - COMP1 clock enable"]
pub type COMP1EN_R = crate::BitReader<bool>;
#[doc = "Field `COMP1EN` writer - COMP1 clock enable"]
pub type COMP1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR2_SPEC, bool, O>;
#[doc = "Field `COMP2EN` reader - COMP2 clock enable"]
pub type COMP2EN_R = crate::BitReader<bool>;
#[doc = "Field `COMP2EN` writer - COMP2 clock enable"]
pub type COMP2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF clock enable"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - TIM14 clock enable"]
    #[inline(always)]
    pub fn tim14en(&self) -> TIM14EN_R {
        TIM14EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - COMP1 clock enable"]
    #[inline(always)]
    pub fn comp1en(&self) -> COMP1EN_R {
        COMP1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - COMP2 clock enable"]
    #[inline(always)]
    pub fn comp2en(&self) -> COMP2EN_R {
        COMP2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF clock enable"]
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<0> {
        SYSCFGEN_W::new(self)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W<11> {
        TIM1EN_W::new(self)
    }
    #[doc = "Bit 15 - TIM14 clock enable"]
    #[inline(always)]
    pub fn tim14en(&mut self) -> TIM14EN_W<15> {
        TIM14EN_W::new(self)
    }
    #[doc = "Bit 21 - COMP1 clock enable"]
    #[inline(always)]
    pub fn comp1en(&mut self) -> COMP1EN_W<21> {
        COMP1EN_W::new(self)
    }
    #[doc = "Bit 22 - COMP2 clock enable"]
    #[inline(always)]
    pub fn comp2en(&mut self) -> COMP2EN_W<22> {
        COMP2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB peripheral clock enable register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbenr2](index.html) module"]
pub struct APBENR2_SPEC;
impl crate::RegisterSpec for APBENR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbenr2::R](R) reader structure"]
impl crate::Readable for APBENR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbenr2::W](W) writer structure"]
impl crate::Writable for APBENR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APBENR2 to value 0"]
impl crate::Resettable for APBENR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
