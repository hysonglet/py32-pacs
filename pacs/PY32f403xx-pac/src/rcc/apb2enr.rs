#[doc = "Register `APB2ENR` reader"]
pub struct R(crate::R<APB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2ENR` writer"]
pub struct W(crate::W<APB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2ENR_SPEC>;
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
impl From<crate::W<APB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCFGEN` reader - desc SYSCFGEN"]
pub type SYSCFGEN_R = crate::BitReader<bool>;
#[doc = "Field `SYSCFGEN` writer - desc SYSCFGEN"]
pub type SYSCFGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `ADC1EN` reader - desc ADC1EN"]
pub type ADC1EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC1EN` writer - desc ADC1EN"]
pub type ADC1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `ADC2EN` reader - desc ADC2EN"]
pub type ADC2EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC2EN` writer - desc ADC2EN"]
pub type ADC2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `TIM1EN` reader - desc TIM1EN"]
pub type TIM1EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM1EN` writer - desc TIM1EN"]
pub type TIM1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `SPI1EN` reader - desc SPI1EN"]
pub type SPI1EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI1EN` writer - desc SPI1EN"]
pub type SPI1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `TIM8EN` reader - desc TIM8EN"]
pub type TIM8EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM8EN` writer - desc TIM8EN"]
pub type TIM8EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `USART1EN` reader - desc USART1EN"]
pub type USART1EN_R = crate::BitReader<bool>;
#[doc = "Field `USART1EN` writer - desc USART1EN"]
pub type USART1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `ADC3EN` reader - desc ADC3EN"]
pub type ADC3EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC3EN` writer - desc ADC3EN"]
pub type ADC3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `TIM9EN` reader - desc TIM9EN"]
pub type TIM9EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM9EN` writer - desc TIM9EN"]
pub type TIM9EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `TIM10EN` reader - desc TIM10EN"]
pub type TIM10EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM10EN` writer - desc TIM10EN"]
pub type TIM10EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `TIM11EN` reader - desc TIM11EN"]
pub type TIM11EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM11EN` writer - desc TIM11EN"]
pub type TIM11EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc SYSCFGEN"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - desc ADC1EN"]
    #[inline(always)]
    pub fn adc1en(&self) -> ADC1EN_R {
        ADC1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc ADC2EN"]
    #[inline(always)]
    pub fn adc2en(&self) -> ADC2EN_R {
        ADC2EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc TIM1EN"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc SPI1EN"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc TIM8EN"]
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc USART1EN"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc ADC3EN"]
    #[inline(always)]
    pub fn adc3en(&self) -> ADC3EN_R {
        ADC3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - desc TIM9EN"]
    #[inline(always)]
    pub fn tim9en(&self) -> TIM9EN_R {
        TIM9EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc TIM10EN"]
    #[inline(always)]
    pub fn tim10en(&self) -> TIM10EN_R {
        TIM10EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc TIM11EN"]
    #[inline(always)]
    pub fn tim11en(&self) -> TIM11EN_R {
        TIM11EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SYSCFGEN"]
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<0> {
        SYSCFGEN_W::new(self)
    }
    #[doc = "Bit 9 - desc ADC1EN"]
    #[inline(always)]
    pub fn adc1en(&mut self) -> ADC1EN_W<9> {
        ADC1EN_W::new(self)
    }
    #[doc = "Bit 10 - desc ADC2EN"]
    #[inline(always)]
    pub fn adc2en(&mut self) -> ADC2EN_W<10> {
        ADC2EN_W::new(self)
    }
    #[doc = "Bit 11 - desc TIM1EN"]
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W<11> {
        TIM1EN_W::new(self)
    }
    #[doc = "Bit 12 - desc SPI1EN"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W<12> {
        SPI1EN_W::new(self)
    }
    #[doc = "Bit 13 - desc TIM8EN"]
    #[inline(always)]
    pub fn tim8en(&mut self) -> TIM8EN_W<13> {
        TIM8EN_W::new(self)
    }
    #[doc = "Bit 14 - desc USART1EN"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W<14> {
        USART1EN_W::new(self)
    }
    #[doc = "Bit 15 - desc ADC3EN"]
    #[inline(always)]
    pub fn adc3en(&mut self) -> ADC3EN_W<15> {
        ADC3EN_W::new(self)
    }
    #[doc = "Bit 19 - desc TIM9EN"]
    #[inline(always)]
    pub fn tim9en(&mut self) -> TIM9EN_W<19> {
        TIM9EN_W::new(self)
    }
    #[doc = "Bit 20 - desc TIM10EN"]
    #[inline(always)]
    pub fn tim10en(&mut self) -> TIM10EN_W<20> {
        TIM10EN_W::new(self)
    }
    #[doc = "Bit 21 - desc TIM11EN"]
    #[inline(always)]
    pub fn tim11en(&mut self) -> TIM11EN_W<21> {
        TIM11EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2enr](index.html) module"]
pub struct APB2ENR_SPEC;
impl crate::RegisterSpec for APB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2enr::R](R) reader structure"]
impl crate::Readable for APB2ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2enr::W](W) writer structure"]
impl crate::Writable for APB2ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB2ENR to value 0"]
impl crate::Resettable for APB2ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
