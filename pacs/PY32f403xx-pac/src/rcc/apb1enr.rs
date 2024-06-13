#[doc = "Register `APB1ENR` reader"]
pub struct R(crate::R<APB1ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1ENR` writer"]
pub struct W(crate::W<APB1ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1ENR_SPEC>;
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
impl From<crate::W<APB1ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2EN` reader - desc TIM2EN"]
pub type TIM2EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM2EN` writer - desc TIM2EN"]
pub type TIM2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `TIM3EN` reader - desc TIM3EN"]
pub type TIM3EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM3EN` writer - desc TIM3EN"]
pub type TIM3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `TIM4EN` reader - desc TIM4EN"]
pub type TIM4EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM4EN` writer - desc TIM4EN"]
pub type TIM4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `TIM5EN` reader - desc TIM5EN"]
pub type TIM5EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM5EN` writer - desc TIM5EN"]
pub type TIM5EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `TIM6EN` reader - desc TIM6EN"]
pub type TIM6EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM6EN` writer - desc TIM6EN"]
pub type TIM6EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `TIM7EN` reader - desc TIM7EN"]
pub type TIM7EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM7EN` writer - desc TIM7EN"]
pub type TIM7EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `TIM12EN` reader - desc TIM12EN"]
pub type TIM12EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM12EN` writer - desc TIM12EN"]
pub type TIM12EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `TIM13EN` reader - desc TIM13EN"]
pub type TIM13EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM13EN` writer - desc TIM13EN"]
pub type TIM13EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `TIM14EN` reader - desc TIM14EN"]
pub type TIM14EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM14EN` writer - desc TIM14EN"]
pub type TIM14EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `WWDGEN` reader - desc WWDGEN"]
pub type WWDGEN_R = crate::BitReader<bool>;
#[doc = "Field `WWDGEN` writer - desc WWDGEN"]
pub type WWDGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `SPI2EN` reader - desc SPI2EN"]
pub type SPI2EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI2EN` writer - desc SPI2EN"]
pub type SPI2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `SPI3EN` reader - desc SPI3EN"]
pub type SPI3EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI3EN` writer - desc SPI3EN"]
pub type SPI3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `USART2EN` reader - desc USART2EN"]
pub type USART2EN_R = crate::BitReader<bool>;
#[doc = "Field `USART2EN` writer - desc USART2EN"]
pub type USART2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `USART3EN` reader - desc USART3EN"]
pub type USART3EN_R = crate::BitReader<bool>;
#[doc = "Field `USART3EN` writer - desc USART3EN"]
pub type USART3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `USART4EN` reader - desc USART4EN"]
pub type USART4EN_R = crate::BitReader<bool>;
#[doc = "Field `USART4EN` writer - desc USART4EN"]
pub type USART4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `USART5EN` reader - desc USART5EN"]
pub type USART5EN_R = crate::BitReader<bool>;
#[doc = "Field `USART5EN` writer - desc USART5EN"]
pub type USART5EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `I2C1EN` reader - desc I2C1EN"]
pub type I2C1EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C1EN` writer - desc I2C1EN"]
pub type I2C1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `I2C2EN` reader - desc I2C2EN"]
pub type I2C2EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C2EN` writer - desc I2C2EN"]
pub type I2C2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `USBEN` reader - desc USBEN"]
pub type USBEN_R = crate::BitReader<bool>;
#[doc = "Field `USBEN` writer - desc USBEN"]
pub type USBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `CANEN` reader - desc CANEN"]
pub type CANEN_R = crate::BitReader<bool>;
#[doc = "Field `CANEN` writer - desc CANEN"]
pub type CANEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `BKPEN` reader - desc BKPEN"]
pub type BKPEN_R = crate::BitReader<bool>;
#[doc = "Field `BKPEN` writer - desc BKPEN"]
pub type BKPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `PWREN` reader - desc PWREN"]
pub type PWREN_R = crate::BitReader<bool>;
#[doc = "Field `PWREN` writer - desc PWREN"]
pub type PWREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
#[doc = "Field `CTCEN` reader - desc CTCEN"]
pub type CTCEN_R = crate::BitReader<bool>;
#[doc = "Field `CTCEN` writer - desc CTCEN"]
pub type CTCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc TIM2EN"]
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TIM3EN"]
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TIM4EN"]
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TIM5EN"]
    #[inline(always)]
    pub fn tim5en(&self) -> TIM5EN_R {
        TIM5EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc TIM6EN"]
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc TIM7EN"]
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc TIM12EN"]
    #[inline(always)]
    pub fn tim12en(&self) -> TIM12EN_R {
        TIM12EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc TIM13EN"]
    #[inline(always)]
    pub fn tim13en(&self) -> TIM13EN_R {
        TIM13EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc TIM14EN"]
    #[inline(always)]
    pub fn tim14en(&self) -> TIM14EN_R {
        TIM14EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - desc WWDGEN"]
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - desc SPI2EN"]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc SPI3EN"]
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - desc USART2EN"]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc USART3EN"]
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc USART4EN"]
    #[inline(always)]
    pub fn usart4en(&self) -> USART4EN_R {
        USART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc USART5EN"]
    #[inline(always)]
    pub fn usart5en(&self) -> USART5EN_R {
        USART5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc I2C1EN"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc I2C2EN"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc USBEN"]
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - desc CANEN"]
    #[inline(always)]
    pub fn canen(&self) -> CANEN_R {
        CANEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - desc BKPEN"]
    #[inline(always)]
    pub fn bkpen(&self) -> BKPEN_R {
        BKPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - desc PWREN"]
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - desc CTCEN"]
    #[inline(always)]
    pub fn ctcen(&self) -> CTCEN_R {
        CTCEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc TIM2EN"]
    #[inline(always)]
    pub fn tim2en(&mut self) -> TIM2EN_W<0> {
        TIM2EN_W::new(self)
    }
    #[doc = "Bit 1 - desc TIM3EN"]
    #[inline(always)]
    pub fn tim3en(&mut self) -> TIM3EN_W<1> {
        TIM3EN_W::new(self)
    }
    #[doc = "Bit 2 - desc TIM4EN"]
    #[inline(always)]
    pub fn tim4en(&mut self) -> TIM4EN_W<2> {
        TIM4EN_W::new(self)
    }
    #[doc = "Bit 3 - desc TIM5EN"]
    #[inline(always)]
    pub fn tim5en(&mut self) -> TIM5EN_W<3> {
        TIM5EN_W::new(self)
    }
    #[doc = "Bit 4 - desc TIM6EN"]
    #[inline(always)]
    pub fn tim6en(&mut self) -> TIM6EN_W<4> {
        TIM6EN_W::new(self)
    }
    #[doc = "Bit 5 - desc TIM7EN"]
    #[inline(always)]
    pub fn tim7en(&mut self) -> TIM7EN_W<5> {
        TIM7EN_W::new(self)
    }
    #[doc = "Bit 6 - desc TIM12EN"]
    #[inline(always)]
    pub fn tim12en(&mut self) -> TIM12EN_W<6> {
        TIM12EN_W::new(self)
    }
    #[doc = "Bit 7 - desc TIM13EN"]
    #[inline(always)]
    pub fn tim13en(&mut self) -> TIM13EN_W<7> {
        TIM13EN_W::new(self)
    }
    #[doc = "Bit 8 - desc TIM14EN"]
    #[inline(always)]
    pub fn tim14en(&mut self) -> TIM14EN_W<8> {
        TIM14EN_W::new(self)
    }
    #[doc = "Bit 11 - desc WWDGEN"]
    #[inline(always)]
    pub fn wwdgen(&mut self) -> WWDGEN_W<11> {
        WWDGEN_W::new(self)
    }
    #[doc = "Bit 14 - desc SPI2EN"]
    #[inline(always)]
    pub fn spi2en(&mut self) -> SPI2EN_W<14> {
        SPI2EN_W::new(self)
    }
    #[doc = "Bit 15 - desc SPI3EN"]
    #[inline(always)]
    pub fn spi3en(&mut self) -> SPI3EN_W<15> {
        SPI3EN_W::new(self)
    }
    #[doc = "Bit 17 - desc USART2EN"]
    #[inline(always)]
    pub fn usart2en(&mut self) -> USART2EN_W<17> {
        USART2EN_W::new(self)
    }
    #[doc = "Bit 18 - desc USART3EN"]
    #[inline(always)]
    pub fn usart3en(&mut self) -> USART3EN_W<18> {
        USART3EN_W::new(self)
    }
    #[doc = "Bit 19 - desc USART4EN"]
    #[inline(always)]
    pub fn usart4en(&mut self) -> USART4EN_W<19> {
        USART4EN_W::new(self)
    }
    #[doc = "Bit 20 - desc USART5EN"]
    #[inline(always)]
    pub fn usart5en(&mut self) -> USART5EN_W<20> {
        USART5EN_W::new(self)
    }
    #[doc = "Bit 21 - desc I2C1EN"]
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W<21> {
        I2C1EN_W::new(self)
    }
    #[doc = "Bit 22 - desc I2C2EN"]
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2C2EN_W<22> {
        I2C2EN_W::new(self)
    }
    #[doc = "Bit 23 - desc USBEN"]
    #[inline(always)]
    pub fn usben(&mut self) -> USBEN_W<23> {
        USBEN_W::new(self)
    }
    #[doc = "Bit 25 - desc CANEN"]
    #[inline(always)]
    pub fn canen(&mut self) -> CANEN_W<25> {
        CANEN_W::new(self)
    }
    #[doc = "Bit 27 - desc BKPEN"]
    #[inline(always)]
    pub fn bkpen(&mut self) -> BKPEN_W<27> {
        BKPEN_W::new(self)
    }
    #[doc = "Bit 28 - desc PWREN"]
    #[inline(always)]
    pub fn pwren(&mut self) -> PWREN_W<28> {
        PWREN_W::new(self)
    }
    #[doc = "Bit 31 - desc CTCEN"]
    #[inline(always)]
    pub fn ctcen(&mut self) -> CTCEN_W<31> {
        CTCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1enr](index.html) module"]
pub struct APB1ENR_SPEC;
impl crate::RegisterSpec for APB1ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1enr::R](R) reader structure"]
impl crate::Readable for APB1ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1enr::W](W) writer structure"]
impl crate::Writable for APB1ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1ENR to value 0x14"]
impl crate::Resettable for APB1ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x14
    }
}
