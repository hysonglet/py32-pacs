#[doc = "Register `CFGR5` reader"]
pub struct R(crate::R<CFGR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR5` writer"]
pub struct W(crate::W<CFGR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR5_SPEC>;
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
impl From<crate::W<CFGR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA9_MAP` reader - desc DMA9_MAP"]
pub type DMA9_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA9_MAP` writer - desc DMA9_MAP"]
pub type DMA9_MAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR5_SPEC, u8, u8, 7, O>;
#[doc = "Field `DMA10_MAP` reader - desc DMA10_MAP"]
pub type DMA10_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA10_MAP` writer - desc DMA10_MAP"]
pub type DMA10_MAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR5_SPEC, u8, u8, 7, O>;
#[doc = "Field `DMA11_MAP` reader - desc DMA11_MAP"]
pub type DMA11_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA11_MAP` writer - desc DMA11_MAP"]
pub type DMA11_MAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR5_SPEC, u8, u8, 7, O>;
#[doc = "Field `DMA12_MAP` reader - desc DMA12_MAP"]
pub type DMA12_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA12_MAP` writer - desc DMA12_MAP"]
pub type DMA12_MAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR5_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - desc DMA9_MAP"]
    #[inline(always)]
    pub fn dma9_map(&self) -> DMA9_MAP_R {
        DMA9_MAP_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - desc DMA10_MAP"]
    #[inline(always)]
    pub fn dma10_map(&self) -> DMA10_MAP_R {
        DMA10_MAP_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - desc DMA11_MAP"]
    #[inline(always)]
    pub fn dma11_map(&self) -> DMA11_MAP_R {
        DMA11_MAP_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - desc DMA12_MAP"]
    #[inline(always)]
    pub fn dma12_map(&self) -> DMA12_MAP_R {
        DMA12_MAP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - desc DMA9_MAP"]
    #[inline(always)]
    pub fn dma9_map(&mut self) -> DMA9_MAP_W<0> {
        DMA9_MAP_W::new(self)
    }
    #[doc = "Bits 8:14 - desc DMA10_MAP"]
    #[inline(always)]
    pub fn dma10_map(&mut self) -> DMA10_MAP_W<8> {
        DMA10_MAP_W::new(self)
    }
    #[doc = "Bits 16:22 - desc DMA11_MAP"]
    #[inline(always)]
    pub fn dma11_map(&mut self) -> DMA11_MAP_W<16> {
        DMA11_MAP_W::new(self)
    }
    #[doc = "Bits 24:30 - desc DMA12_MAP"]
    #[inline(always)]
    pub fn dma12_map(&mut self) -> DMA12_MAP_W<24> {
        DMA12_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CFGR5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr5](index.html) module"]
pub struct CFGR5_SPEC;
impl crate::RegisterSpec for CFGR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr5::R](R) reader structure"]
impl crate::Readable for CFGR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr5::W](W) writer structure"]
impl crate::Writable for CFGR5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR5 to value 0"]
impl crate::Resettable for CFGR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
