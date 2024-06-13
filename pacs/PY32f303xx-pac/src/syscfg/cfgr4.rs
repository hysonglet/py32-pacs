#[doc = "Register `CFGR4` reader"]
pub struct R(crate::R<CFGR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR4` writer"]
pub struct W(crate::W<CFGR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR4_SPEC>;
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
impl From<crate::W<CFGR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA5_MAP` reader - desc DMA5_MAP"]
pub type DMA5_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA5_MAP` writer - desc DMA5_MAP"]
pub type DMA5_MAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR4_SPEC, u8, u8, 7, O>;
#[doc = "Field `DMA6_MAP` reader - desc DMA6_MAP"]
pub type DMA6_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA6_MAP` writer - desc DMA6_MAP"]
pub type DMA6_MAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR4_SPEC, u8, u8, 7, O>;
#[doc = "Field `DMA7_MAP` reader - desc DMA7_MAP"]
pub type DMA7_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA7_MAP` writer - desc DMA7_MAP"]
pub type DMA7_MAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR4_SPEC, u8, u8, 7, O>;
#[doc = "Field `DMA8_MAP` reader - desc DMA8_MAP"]
pub type DMA8_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA8_MAP` writer - desc DMA8_MAP"]
pub type DMA8_MAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR4_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - desc DMA5_MAP"]
    #[inline(always)]
    pub fn dma5_map(&self) -> DMA5_MAP_R {
        DMA5_MAP_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - desc DMA6_MAP"]
    #[inline(always)]
    pub fn dma6_map(&self) -> DMA6_MAP_R {
        DMA6_MAP_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - desc DMA7_MAP"]
    #[inline(always)]
    pub fn dma7_map(&self) -> DMA7_MAP_R {
        DMA7_MAP_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - desc DMA8_MAP"]
    #[inline(always)]
    pub fn dma8_map(&self) -> DMA8_MAP_R {
        DMA8_MAP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - desc DMA5_MAP"]
    #[inline(always)]
    pub fn dma5_map(&mut self) -> DMA5_MAP_W<0> {
        DMA5_MAP_W::new(self)
    }
    #[doc = "Bits 8:14 - desc DMA6_MAP"]
    #[inline(always)]
    pub fn dma6_map(&mut self) -> DMA6_MAP_W<8> {
        DMA6_MAP_W::new(self)
    }
    #[doc = "Bits 16:22 - desc DMA7_MAP"]
    #[inline(always)]
    pub fn dma7_map(&mut self) -> DMA7_MAP_W<16> {
        DMA7_MAP_W::new(self)
    }
    #[doc = "Bits 24:30 - desc DMA8_MAP"]
    #[inline(always)]
    pub fn dma8_map(&mut self) -> DMA8_MAP_W<24> {
        DMA8_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CFGR4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr4](index.html) module"]
pub struct CFGR4_SPEC;
impl crate::RegisterSpec for CFGR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr4::R](R) reader structure"]
impl crate::Readable for CFGR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr4::W](W) writer structure"]
impl crate::Writable for CFGR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR4 to value 0"]
impl crate::Resettable for CFGR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
