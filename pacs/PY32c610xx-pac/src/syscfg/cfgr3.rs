#[doc = "Register `CFGR3` reader"]
pub struct R(crate::R<CFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR3` writer"]
pub struct W(crate::W<CFGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR3_SPEC>;
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
impl From<crate::W<CFGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA1_MAP` reader - DMA channel1 requeset selection"]
pub type DMA1_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA1_MAP` writer - DMA channel1 requeset selection"]
pub type DMA1_MAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR3_SPEC, u8, u8, 5, O>;
#[doc = "Field `DMA2_MAP` reader - DMA channel2 requeset selection"]
pub type DMA2_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA2_MAP` writer - DMA channel2 requeset selection"]
pub type DMA2_MAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR3_SPEC, u8, u8, 5, O>;
#[doc = "Field `DMA3_MAP` reader - DMA channel3 requeset selection"]
pub type DMA3_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA3_MAP` writer - DMA channel3 requeset selection"]
pub type DMA3_MAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR3_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - DMA channel1 requeset selection"]
    #[inline(always)]
    pub fn dma1_map(&self) -> DMA1_MAP_R {
        DMA1_MAP_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DMA channel2 requeset selection"]
    #[inline(always)]
    pub fn dma2_map(&self) -> DMA2_MAP_R {
        DMA2_MAP_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - DMA channel3 requeset selection"]
    #[inline(always)]
    pub fn dma3_map(&self) -> DMA3_MAP_R {
        DMA3_MAP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA channel1 requeset selection"]
    #[inline(always)]
    pub fn dma1_map(&mut self) -> DMA1_MAP_W<0> {
        DMA1_MAP_W::new(self)
    }
    #[doc = "Bits 8:12 - DMA channel2 requeset selection"]
    #[inline(always)]
    pub fn dma2_map(&mut self) -> DMA2_MAP_W<8> {
        DMA2_MAP_W::new(self)
    }
    #[doc = "Bits 16:20 - DMA channel3 requeset selection"]
    #[inline(always)]
    pub fn dma3_map(&mut self) -> DMA3_MAP_W<16> {
        DMA3_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr3](index.html) module"]
pub struct CFGR3_SPEC;
impl crate::RegisterSpec for CFGR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr3::R](R) reader structure"]
impl crate::Readable for CFGR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr3::W](W) writer structure"]
impl crate::Writable for CFGR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR3 to value 0"]
impl crate::Resettable for CFGR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
