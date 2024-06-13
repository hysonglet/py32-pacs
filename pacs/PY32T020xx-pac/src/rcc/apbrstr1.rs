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
#[doc = "Field `RTCAPBRST` reader - "]
pub type RTCAPBRST_R = crate::BitReader<bool>;
#[doc = "Field `RTCAPBRST` writer - "]
pub type RTCAPBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
#[doc = "Field `UART1RST` reader - UART1 reset"]
pub type UART1RST_R = crate::BitReader<bool>;
#[doc = "Field `UART1RST` writer - UART1 reset"]
pub type UART1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
#[doc = "Field `UART2RST` reader - UART2 reset"]
pub type UART2RST_R = crate::BitReader<bool>;
#[doc = "Field `UART2RST` writer - UART2 reset"]
pub type UART2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
#[doc = "Field `I2C1RST` reader - I2C1 reset"]
pub type I2C1RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C1RST` writer - I2C1 reset"]
pub type I2C1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
#[doc = "Field `DBGRST` reader - Debug support reset"]
pub type DBGRST_R = crate::BitReader<bool>;
#[doc = "Field `DBGRST` writer - Debug support reset"]
pub type DBGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
#[doc = "Field `PWRRST` reader - Power interface reset"]
pub type PWRRST_R = crate::BitReader<bool>;
#[doc = "Field `PWRRST` writer - Power interface reset"]
pub type PWRRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBRSTR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rtcapbrst(&self) -> RTCAPBRST_R {
        RTCAPBRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 17 - UART1 reset"]
    #[inline(always)]
    pub fn uart1rst(&self) -> UART1RST_R {
        UART1RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - UART2 reset"]
    #[inline(always)]
    pub fn uart2rst(&self) -> UART2RST_R {
        UART2RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
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
}
impl W {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rtcapbrst(&mut self) -> RTCAPBRST_W<10> {
        RTCAPBRST_W::new(self)
    }
    #[doc = "Bit 17 - UART1 reset"]
    #[inline(always)]
    pub fn uart1rst(&mut self) -> UART1RST_W<17> {
        UART1RST_W::new(self)
    }
    #[doc = "Bit 18 - UART2 reset"]
    #[inline(always)]
    pub fn uart2rst(&mut self) -> UART2RST_W<18> {
        UART2RST_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<21> {
        I2C1RST_W::new(self)
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
