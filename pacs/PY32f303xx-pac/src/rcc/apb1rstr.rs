#[doc = "Register `APB1RSTR` reader"]
pub struct R(crate::R<APB1RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1RSTR` writer"]
pub struct W(crate::W<APB1RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1RSTR_SPEC>;
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
impl From<crate::W<APB1RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2RST` reader - desc TIM2RST"]
pub type TIM2RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM2RST` writer - desc TIM2RST"]
pub type TIM2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `TIM3RST` reader - desc TIM3RST"]
pub type TIM3RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM3RST` writer - desc TIM3RST"]
pub type TIM3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `TIM4RST` reader - desc TIM4RST"]
pub type TIM4RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM4RST` writer - desc TIM4RST"]
pub type TIM4RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `TIM5RST` reader - desc TIM5RST"]
pub type TIM5RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM5RST` writer - desc TIM5RST"]
pub type TIM5RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `TIM6RST` reader - desc TIM6RST"]
pub type TIM6RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM6RST` writer - desc TIM6RST"]
pub type TIM6RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `TIM7RST` reader - desc TIM7RST"]
pub type TIM7RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM7RST` writer - desc TIM7RST"]
pub type TIM7RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `TIM12RST` reader - desc TIM12RST"]
pub type TIM12RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM12RST` writer - desc TIM12RST"]
pub type TIM12RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `TIM13RST` reader - desc TIM13RST"]
pub type TIM13RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM13RST` writer - desc TIM13RST"]
pub type TIM13RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `TIM14RST` reader - desc TIM14RST"]
pub type TIM14RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM14RST` writer - desc TIM14RST"]
pub type TIM14RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `WWDGRST` reader - desc WWDGRST"]
pub type WWDGRST_R = crate::BitReader<bool>;
#[doc = "Field `WWDGRST` writer - desc WWDGRST"]
pub type WWDGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `SPI2RST` reader - desc SPI2RST"]
pub type SPI2RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI2RST` writer - desc SPI2RST"]
pub type SPI2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `SPI3RST` reader - desc SPI3RST"]
pub type SPI3RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI3RST` writer - desc SPI3RST"]
pub type SPI3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `USART2RST` reader - desc USART2RST"]
pub type USART2RST_R = crate::BitReader<bool>;
#[doc = "Field `USART2RST` writer - desc USART2RST"]
pub type USART2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `USART3RST` reader - desc USART3RST"]
pub type USART3RST_R = crate::BitReader<bool>;
#[doc = "Field `USART3RST` writer - desc USART3RST"]
pub type USART3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `USART4RST` reader - desc USART4RST"]
pub type USART4RST_R = crate::BitReader<bool>;
#[doc = "Field `USART4RST` writer - desc USART4RST"]
pub type USART4RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `USART5RST` reader - desc USART5RST"]
pub type USART5RST_R = crate::BitReader<bool>;
#[doc = "Field `USART5RST` writer - desc USART5RST"]
pub type USART5RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `I2C1RST` reader - desc I2C1RST"]
pub type I2C1RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C1RST` writer - desc I2C1RST"]
pub type I2C1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `I2C2RST` reader - desc I2C2RST"]
pub type I2C2RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C2RST` writer - desc I2C2RST"]
pub type I2C2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `USBRST` reader - desc USBRST"]
pub type USBRST_R = crate::BitReader<bool>;
#[doc = "Field `USBRST` writer - desc USBRST"]
pub type USBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `CANRST` reader - desc CANRST"]
pub type CANRST_R = crate::BitReader<bool>;
#[doc = "Field `CANRST` writer - desc CANRST"]
pub type CANRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `BKPRST` reader - desc BKPRST"]
pub type BKPRST_R = crate::BitReader<bool>;
#[doc = "Field `BKPRST` writer - desc BKPRST"]
pub type BKPRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `PWRRST` reader - desc PWRRST"]
pub type PWRRST_R = crate::BitReader<bool>;
#[doc = "Field `PWRRST` writer - desc PWRRST"]
pub type PWRRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `DACRST` reader - desc DACRST"]
pub type DACRST_R = crate::BitReader<bool>;
#[doc = "Field `DACRST` writer - desc DACRST"]
pub type DACRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, bool, O>;
#[doc = "Field `CTCRST` reader - desc CTCRST"]
pub type CTCRST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTCRST` writer - desc CTCRST"]
pub type CTCRST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, APB1RSTR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - desc TIM2RST"]
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TIM3RST"]
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TIM4RST"]
    #[inline(always)]
    pub fn tim4rst(&self) -> TIM4RST_R {
        TIM4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TIM5RST"]
    #[inline(always)]
    pub fn tim5rst(&self) -> TIM5RST_R {
        TIM5RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc TIM6RST"]
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc TIM7RST"]
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc TIM12RST"]
    #[inline(always)]
    pub fn tim12rst(&self) -> TIM12RST_R {
        TIM12RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc TIM13RST"]
    #[inline(always)]
    pub fn tim13rst(&self) -> TIM13RST_R {
        TIM13RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc TIM14RST"]
    #[inline(always)]
    pub fn tim14rst(&self) -> TIM14RST_R {
        TIM14RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - desc WWDGRST"]
    #[inline(always)]
    pub fn wwdgrst(&self) -> WWDGRST_R {
        WWDGRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - desc SPI2RST"]
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc SPI3RST"]
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - desc USART2RST"]
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc USART3RST"]
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc USART4RST"]
    #[inline(always)]
    pub fn usart4rst(&self) -> USART4RST_R {
        USART4RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc USART5RST"]
    #[inline(always)]
    pub fn usart5rst(&self) -> USART5RST_R {
        USART5RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc I2C1RST"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc I2C2RST"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc USBRST"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - desc CANRST"]
    #[inline(always)]
    pub fn canrst(&self) -> CANRST_R {
        CANRST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - desc BKPRST"]
    #[inline(always)]
    pub fn bkprst(&self) -> BKPRST_R {
        BKPRST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - desc PWRRST"]
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc DACRST"]
    #[inline(always)]
    pub fn dacrst(&self) -> DACRST_R {
        DACRST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - desc CTCRST"]
    #[inline(always)]
    pub fn ctcrst(&self) -> CTCRST_R {
        CTCRST_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc TIM2RST"]
    #[inline(always)]
    pub fn tim2rst(&mut self) -> TIM2RST_W<0> {
        TIM2RST_W::new(self)
    }
    #[doc = "Bit 1 - desc TIM3RST"]
    #[inline(always)]
    pub fn tim3rst(&mut self) -> TIM3RST_W<1> {
        TIM3RST_W::new(self)
    }
    #[doc = "Bit 2 - desc TIM4RST"]
    #[inline(always)]
    pub fn tim4rst(&mut self) -> TIM4RST_W<2> {
        TIM4RST_W::new(self)
    }
    #[doc = "Bit 3 - desc TIM5RST"]
    #[inline(always)]
    pub fn tim5rst(&mut self) -> TIM5RST_W<3> {
        TIM5RST_W::new(self)
    }
    #[doc = "Bit 4 - desc TIM6RST"]
    #[inline(always)]
    pub fn tim6rst(&mut self) -> TIM6RST_W<4> {
        TIM6RST_W::new(self)
    }
    #[doc = "Bit 5 - desc TIM7RST"]
    #[inline(always)]
    pub fn tim7rst(&mut self) -> TIM7RST_W<5> {
        TIM7RST_W::new(self)
    }
    #[doc = "Bit 6 - desc TIM12RST"]
    #[inline(always)]
    pub fn tim12rst(&mut self) -> TIM12RST_W<6> {
        TIM12RST_W::new(self)
    }
    #[doc = "Bit 7 - desc TIM13RST"]
    #[inline(always)]
    pub fn tim13rst(&mut self) -> TIM13RST_W<7> {
        TIM13RST_W::new(self)
    }
    #[doc = "Bit 8 - desc TIM14RST"]
    #[inline(always)]
    pub fn tim14rst(&mut self) -> TIM14RST_W<8> {
        TIM14RST_W::new(self)
    }
    #[doc = "Bit 11 - desc WWDGRST"]
    #[inline(always)]
    pub fn wwdgrst(&mut self) -> WWDGRST_W<11> {
        WWDGRST_W::new(self)
    }
    #[doc = "Bit 14 - desc SPI2RST"]
    #[inline(always)]
    pub fn spi2rst(&mut self) -> SPI2RST_W<14> {
        SPI2RST_W::new(self)
    }
    #[doc = "Bit 15 - desc SPI3RST"]
    #[inline(always)]
    pub fn spi3rst(&mut self) -> SPI3RST_W<15> {
        SPI3RST_W::new(self)
    }
    #[doc = "Bit 17 - desc USART2RST"]
    #[inline(always)]
    pub fn usart2rst(&mut self) -> USART2RST_W<17> {
        USART2RST_W::new(self)
    }
    #[doc = "Bit 18 - desc USART3RST"]
    #[inline(always)]
    pub fn usart3rst(&mut self) -> USART3RST_W<18> {
        USART3RST_W::new(self)
    }
    #[doc = "Bit 19 - desc USART4RST"]
    #[inline(always)]
    pub fn usart4rst(&mut self) -> USART4RST_W<19> {
        USART4RST_W::new(self)
    }
    #[doc = "Bit 20 - desc USART5RST"]
    #[inline(always)]
    pub fn usart5rst(&mut self) -> USART5RST_W<20> {
        USART5RST_W::new(self)
    }
    #[doc = "Bit 21 - desc I2C1RST"]
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<21> {
        I2C1RST_W::new(self)
    }
    #[doc = "Bit 22 - desc I2C2RST"]
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<22> {
        I2C2RST_W::new(self)
    }
    #[doc = "Bit 23 - desc USBRST"]
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W<23> {
        USBRST_W::new(self)
    }
    #[doc = "Bit 25 - desc CANRST"]
    #[inline(always)]
    pub fn canrst(&mut self) -> CANRST_W<25> {
        CANRST_W::new(self)
    }
    #[doc = "Bit 27 - desc BKPRST"]
    #[inline(always)]
    pub fn bkprst(&mut self) -> BKPRST_W<27> {
        BKPRST_W::new(self)
    }
    #[doc = "Bit 28 - desc PWRRST"]
    #[inline(always)]
    pub fn pwrrst(&mut self) -> PWRRST_W<28> {
        PWRRST_W::new(self)
    }
    #[doc = "Bit 29 - desc DACRST"]
    #[inline(always)]
    pub fn dacrst(&mut self) -> DACRST_W<29> {
        DACRST_W::new(self)
    }
    #[doc = "Bits 30:31 - desc CTCRST"]
    #[inline(always)]
    pub fn ctcrst(&mut self) -> CTCRST_W<30> {
        CTCRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1rstr](index.html) module"]
pub struct APB1RSTR_SPEC;
impl crate::RegisterSpec for APB1RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1rstr::R](R) reader structure"]
impl crate::Readable for APB1RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1rstr::W](W) writer structure"]
impl crate::Writable for APB1RSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1RSTR to value 0"]
impl crate::Resettable for APB1RSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
