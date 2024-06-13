#[doc = "Register `APBRSTR1` reader"]
pub struct R(crate::R<APBRSTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBRSTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBRSTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBRSTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBRSTR1` writer"]
pub struct W(crate::W<APBRSTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBRSTR1_SPEC>;
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
impl From<crate::W<APBRSTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBRSTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2RST` reader - TIM2 timer reset"]
pub type TIM2RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM2RST` writer - TIM2 timer reset"]
pub type TIM2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
#[doc = "Field `RTCAPBRST` reader - RTC APB reset"]
pub type RTCAPBRST_R = crate::BitReader<bool>;
#[doc = "Field `RTCAPBRST` writer - RTC APB reset"]
pub type RTCAPBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
#[doc = "Field `WWDGRST` reader - WWDG reset"]
pub type WWDGRST_R = crate::BitReader<bool>;
#[doc = "Field `WWDGRST` writer - WWDG reset"]
pub type WWDGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
#[doc = "Field `SPI2RST` reader - SPI2 reset"]
pub type SPI2RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI2RST` writer - SPI2 reset"]
pub type SPI2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
#[doc = "Field `USART2RST` reader - USART2 reset"]
pub type USART2RST_R = crate::BitReader<bool>;
#[doc = "Field `USART2RST` writer - USART2 reset"]
pub type USART2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
#[doc = "Field `USART3RST` reader - USART3 reset"]
pub type USART3RST_R = crate::BitReader<bool>;
#[doc = "Field `USART3RST` writer - USART3 reset"]
pub type USART3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
#[doc = "Field `I2C1RST` reader - I2C1 reset"]
pub type I2C1RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C1RST` writer - I2C1 reset"]
pub type I2C1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
#[doc = "Field `I2C2RST` reader - I2C2 reset"]
pub type I2C2RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C2RST` writer - I2C2 reset"]
pub type I2C2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
#[doc = "Field `DBGRST` reader - Debug support reset"]
pub type DBGRST_R = crate::BitReader<bool>;
#[doc = "Field `DBGRST` writer - Debug support reset"]
pub type DBGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
#[doc = "Field `PWRRST` reader - Power interface reset"]
pub type PWRRST_R = crate::BitReader<bool>;
#[doc = "Field `PWRRST` writer - Power interface reset"]
pub type PWRRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
#[doc = "Field `OPARST` reader - OPA reset"]
pub type OPARST_R = crate::BitReader<bool>;
#[doc = "Field `OPARST` writer - OPA reset"]
pub type OPARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
#[doc = "Field `LPTIMRST` reader - Low Power Timer reset"]
pub type LPTIMRST_R = crate::BitReader<bool>;
#[doc = "Field `LPTIMRST` writer - Low Power Timer reset"]
pub type LPTIMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIM2 timer reset"]
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB reset"]
    #[inline(always)]
    pub fn rtcapbrst(&self) -> RTCAPBRST_R {
        RTCAPBRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG reset"]
    #[inline(always)]
    pub fn wwdgrst(&self) -> WWDGRST_R {
        WWDGRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 reset"]
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 27 - Debug support reset"]
    #[inline(always)]
    pub fn dbgrst(&self) -> DBGRST_R {
        DBGRST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - OPA reset"]
    #[inline(always)]
    pub fn oparst(&self) -> OPARST_R {
        OPARST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low Power Timer reset"]
    #[inline(always)]
    pub fn lptimrst(&self) -> LPTIMRST_R {
        LPTIMRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer reset"]
    #[inline(always)]
    pub fn tim2rst(&mut self) -> TIM2RST_W<0> {
        TIM2RST_W::new(self)
    }
    #[doc = "Bit 10 - RTC APB reset"]
    #[inline(always)]
    pub fn rtcapbrst(&mut self) -> RTCAPBRST_W<10> {
        RTCAPBRST_W::new(self)
    }
    #[doc = "Bit 11 - WWDG reset"]
    #[inline(always)]
    pub fn wwdgrst(&mut self) -> WWDGRST_W<11> {
        WWDGRST_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2rst(&mut self) -> SPI2RST_W<14> {
        SPI2RST_W::new(self)
    }
    #[doc = "Bit 17 - USART2 reset"]
    #[inline(always)]
    pub fn usart2rst(&mut self) -> USART2RST_W<17> {
        USART2RST_W::new(self)
    }
    #[doc = "Bit 18 - USART3 reset"]
    #[inline(always)]
    pub fn usart3rst(&mut self) -> USART3RST_W<18> {
        USART3RST_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<21> {
        I2C1RST_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<22> {
        I2C2RST_W::new(self)
    }
    #[doc = "Bit 27 - Debug support reset"]
    #[inline(always)]
    pub fn dbgrst(&mut self) -> DBGRST_W<27> {
        DBGRST_W::new(self)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwrrst(&mut self) -> PWRRST_W<28> {
        PWRRST_W::new(self)
    }
    #[doc = "Bit 30 - OPA reset"]
    #[inline(always)]
    pub fn oparst(&mut self) -> OPARST_W<30> {
        OPARST_W::new(self)
    }
    #[doc = "Bit 31 - Low Power Timer reset"]
    #[inline(always)]
    pub fn lptimrst(&mut self) -> LPTIMRST_W<31> {
        LPTIMRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB peripheral reset register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbrstr1](index.html) module"]
pub struct APBRSTR1_SPEC;
impl crate::RegisterSpec for APBRSTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbrstr1::R](R) reader structure"]
impl crate::Readable for APBRSTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbrstr1::W](W) writer structure"]
impl crate::Writable for APBRSTR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APBRSTR1 to value 0"]
impl crate::Resettable for APBRSTR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
