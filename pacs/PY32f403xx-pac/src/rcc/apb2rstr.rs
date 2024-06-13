#[doc = "Register `APB2RSTR` reader"]
pub struct R(crate::R<APB2RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2RSTR` writer"]
pub struct W(crate::W<APB2RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2RSTR_SPEC>;
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
impl From<crate::W<APB2RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCFGRST` reader - desc SYSCFGRST"]
pub type SYSCFGRST_R = crate::BitReader<bool>;
#[doc = "Field `SYSCFGRST` writer - desc SYSCFGRST"]
pub type SYSCFGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `ADC1RST` reader - desc ADC1RST"]
pub type ADC1RST_R = crate::BitReader<bool>;
#[doc = "Field `ADC1RST` writer - desc ADC1RST"]
pub type ADC1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `ADC2RST` reader - desc ADC2RST"]
pub type ADC2RST_R = crate::BitReader<bool>;
#[doc = "Field `ADC2RST` writer - desc ADC2RST"]
pub type ADC2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `TIM1RST` reader - desc TIM1RST"]
pub type TIM1RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM1RST` writer - desc TIM1RST"]
pub type TIM1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `SPI1RST` reader - desc SPI1RST"]
pub type SPI1RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI1RST` writer - desc SPI1RST"]
pub type SPI1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `TIM8RST` reader - desc TIM8RST"]
pub type TIM8RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM8RST` writer - desc TIM8RST"]
pub type TIM8RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `USART1RST` reader - desc USART1RST"]
pub type USART1RST_R = crate::BitReader<bool>;
#[doc = "Field `USART1RST` writer - desc USART1RST"]
pub type USART1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `ADC3RST` reader - desc ADC3RST"]
pub type ADC3RST_R = crate::BitReader<bool>;
#[doc = "Field `ADC3RST` writer - desc ADC3RST"]
pub type ADC3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `TIM9RST` reader - desc TIM9RST"]
pub type TIM9RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM9RST` writer - desc TIM9RST"]
pub type TIM9RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `TIM10RST` reader - desc TIM10RST"]
pub type TIM10RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM10RST` writer - desc TIM10RST"]
pub type TIM10RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `TIM11RST` reader - desc TIM11RST"]
pub type TIM11RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM11RST` writer - desc TIM11RST"]
pub type TIM11RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc SYSCFGRST"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - desc ADC1RST"]
    #[inline(always)]
    pub fn adc1rst(&self) -> ADC1RST_R {
        ADC1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc ADC2RST"]
    #[inline(always)]
    pub fn adc2rst(&self) -> ADC2RST_R {
        ADC2RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc TIM1RST"]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc SPI1RST"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc TIM8RST"]
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc USART1RST"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc ADC3RST"]
    #[inline(always)]
    pub fn adc3rst(&self) -> ADC3RST_R {
        ADC3RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - desc TIM9RST"]
    #[inline(always)]
    pub fn tim9rst(&self) -> TIM9RST_R {
        TIM9RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc TIM10RST"]
    #[inline(always)]
    pub fn tim10rst(&self) -> TIM10RST_R {
        TIM10RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc TIM11RST"]
    #[inline(always)]
    pub fn tim11rst(&self) -> TIM11RST_R {
        TIM11RST_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SYSCFGRST"]
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<0> {
        SYSCFGRST_W::new(self)
    }
    #[doc = "Bit 9 - desc ADC1RST"]
    #[inline(always)]
    pub fn adc1rst(&mut self) -> ADC1RST_W<9> {
        ADC1RST_W::new(self)
    }
    #[doc = "Bit 10 - desc ADC2RST"]
    #[inline(always)]
    pub fn adc2rst(&mut self) -> ADC2RST_W<10> {
        ADC2RST_W::new(self)
    }
    #[doc = "Bit 11 - desc TIM1RST"]
    #[inline(always)]
    pub fn tim1rst(&mut self) -> TIM1RST_W<11> {
        TIM1RST_W::new(self)
    }
    #[doc = "Bit 12 - desc SPI1RST"]
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W<12> {
        SPI1RST_W::new(self)
    }
    #[doc = "Bit 13 - desc TIM8RST"]
    #[inline(always)]
    pub fn tim8rst(&mut self) -> TIM8RST_W<13> {
        TIM8RST_W::new(self)
    }
    #[doc = "Bit 14 - desc USART1RST"]
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W<14> {
        USART1RST_W::new(self)
    }
    #[doc = "Bit 15 - desc ADC3RST"]
    #[inline(always)]
    pub fn adc3rst(&mut self) -> ADC3RST_W<15> {
        ADC3RST_W::new(self)
    }
    #[doc = "Bit 19 - desc TIM9RST"]
    #[inline(always)]
    pub fn tim9rst(&mut self) -> TIM9RST_W<19> {
        TIM9RST_W::new(self)
    }
    #[doc = "Bit 20 - desc TIM10RST"]
    #[inline(always)]
    pub fn tim10rst(&mut self) -> TIM10RST_W<20> {
        TIM10RST_W::new(self)
    }
    #[doc = "Bit 21 - desc TIM11RST"]
    #[inline(always)]
    pub fn tim11rst(&mut self) -> TIM11RST_W<21> {
        TIM11RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2rstr](index.html) module"]
pub struct APB2RSTR_SPEC;
impl crate::RegisterSpec for APB2RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2rstr::R](R) reader structure"]
impl crate::Readable for APB2RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2rstr::W](W) writer structure"]
impl crate::Writable for APB2RSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB2RSTR to value 0"]
impl crate::Resettable for APB2RSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
