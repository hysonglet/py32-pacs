#[doc = "Register `APBRSTR2` reader"]
pub struct R(crate::R<APBRSTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBRSTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBRSTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBRSTR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBRSTR2` writer"]
pub struct W(crate::W<APBRSTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBRSTR2_SPEC>;
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
impl From<crate::W<APBRSTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBRSTR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCFGRST` reader - SYSCFG and COMP reset"]
pub type SYSCFGRST_R = crate::BitReader<bool>;
#[doc = "Field `SYSCFGRST` writer - SYSCFG and COMP reset"]
pub type SYSCFGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR2_SPEC, bool, O>;
#[doc = "Field `TIM1RST` reader - TIM1 timer reset"]
pub type TIM1RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM1RST` writer - TIM1 timer reset"]
pub type TIM1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR2_SPEC, bool, O>;
#[doc = "Field `SPI1RST` reader - SPI1 reset"]
pub type SPI1RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI1RST` writer - SPI1 reset"]
pub type SPI1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR2_SPEC, bool, O>;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub type USART1RST_R = crate::BitReader<bool>;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub type USART1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR2_SPEC, bool, O>;
#[doc = "Field `TIM14RST` reader - TIM14 timer reset"]
pub type TIM14RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM14RST` writer - TIM14 timer reset"]
pub type TIM14RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR2_SPEC, bool, O>;
#[doc = "Field `TIM16RST` reader - TIM16 timer reset"]
pub type TIM16RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM16RST` writer - TIM16 timer reset"]
pub type TIM16RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR2_SPEC, bool, O>;
#[doc = "Field `TIM17RST` reader - TIM17 timer reset"]
pub type TIM17RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM17RST` writer - TIM17 timer reset"]
pub type TIM17RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR2_SPEC, bool, O>;
#[doc = "Field `ADCRST` reader - ADC reset"]
pub type ADCRST_R = crate::BitReader<bool>;
#[doc = "Field `ADCRST` writer - ADC reset"]
pub type ADCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR2_SPEC, bool, O>;
#[doc = "Field `COMP1RST` reader - COMP1 reset"]
pub type COMP1RST_R = crate::BitReader<bool>;
#[doc = "Field `COMP1RST` writer - COMP1 reset"]
pub type COMP1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR2_SPEC, bool, O>;
#[doc = "Field `COMP2RST` reader - COMP2 reset"]
pub type COMP2RST_R = crate::BitReader<bool>;
#[doc = "Field `COMP2RST` writer - COMP2 reset"]
pub type COMP2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR2_SPEC, bool, O>;
#[doc = "Field `LEDRST` reader - LED reset"]
pub type LEDRST_R = crate::BitReader<bool>;
#[doc = "Field `LEDRST` writer - LED reset"]
pub type LEDRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SYSCFG and COMP reset"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TIM14 timer reset"]
    #[inline(always)]
    pub fn tim14rst(&self) -> TIM14RST_R {
        TIM14RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer reset"]
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC reset"]
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - COMP1 reset"]
    #[inline(always)]
    pub fn comp1rst(&self) -> COMP1RST_R {
        COMP1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - COMP2 reset"]
    #[inline(always)]
    pub fn comp2rst(&self) -> COMP2RST_R {
        COMP2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LED reset"]
    #[inline(always)]
    pub fn ledrst(&self) -> LEDRST_R {
        LEDRST_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG and COMP reset"]
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<0> {
        SYSCFGRST_W::new(self)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    pub fn tim1rst(&mut self) -> TIM1RST_W<11> {
        TIM1RST_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W<12> {
        SPI1RST_W::new(self)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W<14> {
        USART1RST_W::new(self)
    }
    #[doc = "Bit 15 - TIM14 timer reset"]
    #[inline(always)]
    pub fn tim14rst(&mut self) -> TIM14RST_W<15> {
        TIM14RST_W::new(self)
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    pub fn tim16rst(&mut self) -> TIM16RST_W<17> {
        TIM16RST_W::new(self)
    }
    #[doc = "Bit 18 - TIM17 timer reset"]
    #[inline(always)]
    pub fn tim17rst(&mut self) -> TIM17RST_W<18> {
        TIM17RST_W::new(self)
    }
    #[doc = "Bit 20 - ADC reset"]
    #[inline(always)]
    pub fn adcrst(&mut self) -> ADCRST_W<20> {
        ADCRST_W::new(self)
    }
    #[doc = "Bit 21 - COMP1 reset"]
    #[inline(always)]
    pub fn comp1rst(&mut self) -> COMP1RST_W<21> {
        COMP1RST_W::new(self)
    }
    #[doc = "Bit 22 - COMP2 reset"]
    #[inline(always)]
    pub fn comp2rst(&mut self) -> COMP2RST_W<22> {
        COMP2RST_W::new(self)
    }
    #[doc = "Bit 23 - LED reset"]
    #[inline(always)]
    pub fn ledrst(&mut self) -> LEDRST_W<23> {
        LEDRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB peripheral reset register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbrstr2](index.html) module"]
pub struct APBRSTR2_SPEC;
impl crate::RegisterSpec for APBRSTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbrstr2::R](R) reader structure"]
impl crate::Readable for APBRSTR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbrstr2::W](W) writer structure"]
impl crate::Writable for APBRSTR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APBRSTR2 to value 0"]
impl crate::Resettable for APBRSTR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
