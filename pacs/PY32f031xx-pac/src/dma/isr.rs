#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GIF1` reader - Channel 1 Global interrupt flag"]
pub type GIF1_R = crate::BitReader<bool>;
#[doc = "Field `TCIF1` reader - Channel 1 Transfer Complete flag"]
pub type TCIF1_R = crate::BitReader<bool>;
#[doc = "Field `HTIF1` reader - Channel 1 Half Transfer Complete flag"]
pub type HTIF1_R = crate::BitReader<bool>;
#[doc = "Field `TEIF1` reader - Channel 1 Transfer Error flag"]
pub type TEIF1_R = crate::BitReader<bool>;
#[doc = "Field `GIF2` reader - Channel 2 Global interrupt flag"]
pub type GIF2_R = crate::BitReader<bool>;
#[doc = "Field `TCIF2` reader - Channel 2 Transfer Complete flag"]
pub type TCIF2_R = crate::BitReader<bool>;
#[doc = "Field `HTIF2` reader - Channel 2 Half Transfer Complete flag"]
pub type HTIF2_R = crate::BitReader<bool>;
#[doc = "Field `TEIF2` reader - Channel 2 Transfer Error flag"]
pub type TEIF2_R = crate::BitReader<bool>;
#[doc = "Field `GIF3` reader - Channel 3 Global interrupt flag"]
pub type GIF3_R = crate::BitReader<bool>;
#[doc = "Field `TCIF3` reader - Channel 3 Transfer Complete flag"]
pub type TCIF3_R = crate::BitReader<bool>;
#[doc = "Field `HTIF3` reader - Channel 3 Half Transfer Complete flag"]
pub type HTIF3_R = crate::BitReader<bool>;
#[doc = "Field `TEIF3` reader - Channel 3 Transfer Error flag"]
pub type TEIF3_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Channel 1 Global interrupt flag"]
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 1 Transfer Error flag"]
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 2 Global interrupt flag"]
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 2 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 2 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 2 Transfer Error flag"]
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 3 Global interrupt flag"]
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 3 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 3 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 Transfer Error flag"]
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "DMA interrupt status register (DMA_ISR)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
