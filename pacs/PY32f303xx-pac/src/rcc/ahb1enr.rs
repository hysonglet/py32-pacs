#[doc = "Register `AHB1ENR` reader"]
pub struct R(crate::R<AHB1ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB1ENR` writer"]
pub struct W(crate::W<AHB1ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB1ENR_SPEC>;
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
impl From<crate::W<AHB1ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB1ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA1EN` reader - desc DMA1EN"]
pub type DMA1EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA1EN` writer - desc DMA1EN"]
pub type DMA1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, bool, O>;
#[doc = "Field `DMA2EN` reader - desc DMA2EN"]
pub type DMA2EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA2EN` writer - desc DMA2EN"]
pub type DMA2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, bool, O>;
#[doc = "Field `SRAMEN` reader - desc SRAMEN"]
pub type SRAMEN_R = crate::BitReader<bool>;
#[doc = "Field `SRAMEN` writer - desc SRAMEN"]
pub type SRAMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, bool, O>;
#[doc = "Field `FMCEN` reader - desc FMCEN"]
pub type FMCEN_R = crate::BitReader<bool>;
#[doc = "Field `FMCEN` writer - desc FMCEN"]
pub type FMCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, bool, O>;
#[doc = "Field `CRCEN` reader - desc CRCEN"]
pub type CRCEN_R = crate::BitReader<bool>;
#[doc = "Field `CRCEN` writer - desc CRCEN"]
pub type CRCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, bool, O>;
#[doc = "Field `SDIOEN` reader - desc SDIOEN"]
pub type SDIOEN_R = crate::BitReader<bool>;
#[doc = "Field `SDIOEN` writer - desc SDIOEN"]
pub type SDIOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, bool, O>;
#[doc = "Field `ESMCEN` reader - desc ESMCEN"]
pub type ESMCEN_R = crate::BitReader<bool>;
#[doc = "Field `ESMCEN` writer - desc ESMCEN"]
pub type ESMCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc DMA1EN"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc DMA2EN"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc SRAMEN"]
    #[inline(always)]
    pub fn sramen(&self) -> SRAMEN_R {
        SRAMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc FMCEN"]
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - desc CRCEN"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - desc SDIOEN"]
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - desc ESMCEN"]
    #[inline(always)]
    pub fn esmcen(&self) -> ESMCEN_R {
        ESMCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc DMA1EN"]
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W<0> {
        DMA1EN_W::new(self)
    }
    #[doc = "Bit 1 - desc DMA2EN"]
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W<1> {
        DMA2EN_W::new(self)
    }
    #[doc = "Bit 2 - desc SRAMEN"]
    #[inline(always)]
    pub fn sramen(&mut self) -> SRAMEN_W<2> {
        SRAMEN_W::new(self)
    }
    #[doc = "Bit 4 - desc FMCEN"]
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W<4> {
        FMCEN_W::new(self)
    }
    #[doc = "Bit 6 - desc CRCEN"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<6> {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 10 - desc SDIOEN"]
    #[inline(always)]
    pub fn sdioen(&mut self) -> SDIOEN_W<10> {
        SDIOEN_W::new(self)
    }
    #[doc = "Bit 12 - desc ESMCEN"]
    #[inline(always)]
    pub fn esmcen(&mut self) -> ESMCEN_W<12> {
        ESMCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB1 peripheral clock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1enr](index.html) module"]
pub struct AHB1ENR_SPEC;
impl crate::RegisterSpec for AHB1ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb1enr::R](R) reader structure"]
impl crate::Readable for AHB1ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb1enr::W](W) writer structure"]
impl crate::Writable for AHB1ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB1ENR to value 0x14"]
impl crate::Resettable for AHB1ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x14
    }
}
