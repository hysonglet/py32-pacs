#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADON` reader - desc ADON"]
pub type ADON_R = crate::BitReader<bool>;
#[doc = "Field `ADON` writer - desc ADON"]
pub type ADON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `CONT` reader - desc CONT"]
pub type CONT_R = crate::BitReader<bool>;
#[doc = "Field `CONT` writer - desc CONT"]
pub type CONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `CAL` reader - desc CAL"]
pub type CAL_R = crate::BitReader<bool>;
#[doc = "Field `CAL` writer - desc CAL"]
pub type CAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `RSTCAL` reader - desc RSTCAL"]
pub type RSTCAL_R = crate::BitReader<bool>;
#[doc = "Field `RSTCAL` writer - desc RSTCAL"]
pub type RSTCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `DMA` reader - desc DMA"]
pub type DMA_R = crate::BitReader<bool>;
#[doc = "Field `DMA` writer - desc DMA"]
pub type DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `ALIGN` reader - desc ALIGN"]
pub type ALIGN_R = crate::BitReader<bool>;
#[doc = "Field `ALIGN` writer - desc ALIGN"]
pub type ALIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `JEXTSEL` reader - desc JEXTSEL"]
pub type JEXTSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JEXTSEL` writer - desc JEXTSEL"]
pub type JEXTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `JEXTTRIG` reader - desc JEXTTRIG"]
pub type JEXTTRIG_R = crate::BitReader<bool>;
#[doc = "Field `JEXTTRIG` writer - desc JEXTTRIG"]
pub type JEXTTRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `EXTSEL` reader - desc EXTSEL"]
pub type EXTSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTSEL` writer - desc EXTSEL"]
pub type EXTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `EXTTRIG` reader - desc EXTTRIG"]
pub type EXTTRIG_R = crate::BitReader<bool>;
#[doc = "Field `EXTTRIG` writer - desc EXTTRIG"]
pub type EXTTRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `JSWSTART` reader - desc JSWSTART"]
pub type JSWSTART_R = crate::BitReader<bool>;
#[doc = "Field `JSWSTART` writer - desc JSWSTART"]
pub type JSWSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `SWSTART` reader - desc SWSTART"]
pub type SWSTART_R = crate::BitReader<bool>;
#[doc = "Field `SWSTART` writer - desc SWSTART"]
pub type SWSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TSVREFE` reader - desc TSVREFE"]
pub type TSVREFE_R = crate::BitReader<bool>;
#[doc = "Field `TSVREFE` writer - desc TSVREFE"]
pub type TSVREFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `VBAT` reader - desc VBAT"]
pub type VBAT_R = crate::BitReader<bool>;
#[doc = "Field `VBAT` writer - desc VBAT"]
pub type VBAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc ADON"]
    #[inline(always)]
    pub fn adon(&self) -> ADON_R {
        ADON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CONT"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CAL"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc RSTCAL"]
    #[inline(always)]
    pub fn rstcal(&self) -> RSTCAL_R {
        RSTCAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - desc DMA"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - desc ALIGN"]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - desc JEXTSEL"]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - desc JEXTTRIG"]
    #[inline(always)]
    pub fn jexttrig(&self) -> JEXTTRIG_R {
        JEXTTRIG_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:19 - desc EXTSEL"]
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - desc EXTTRIG"]
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc JSWSTART"]
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc SWSTART"]
    #[inline(always)]
    pub fn swstart(&self) -> SWSTART_R {
        SWSTART_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc TSVREFE"]
    #[inline(always)]
    pub fn tsvrefe(&self) -> TSVREFE_R {
        TSVREFE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - desc VBAT"]
    #[inline(always)]
    pub fn vbat(&self) -> VBAT_R {
        VBAT_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc ADON"]
    #[inline(always)]
    pub fn adon(&mut self) -> ADON_W<0> {
        ADON_W::new(self)
    }
    #[doc = "Bit 1 - desc CONT"]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W<1> {
        CONT_W::new(self)
    }
    #[doc = "Bit 2 - desc CAL"]
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W<2> {
        CAL_W::new(self)
    }
    #[doc = "Bit 3 - desc RSTCAL"]
    #[inline(always)]
    pub fn rstcal(&mut self) -> RSTCAL_W<3> {
        RSTCAL_W::new(self)
    }
    #[doc = "Bit 8 - desc DMA"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W<8> {
        DMA_W::new(self)
    }
    #[doc = "Bit 11 - desc ALIGN"]
    #[inline(always)]
    pub fn align(&mut self) -> ALIGN_W<11> {
        ALIGN_W::new(self)
    }
    #[doc = "Bits 12:14 - desc JEXTSEL"]
    #[inline(always)]
    pub fn jextsel(&mut self) -> JEXTSEL_W<12> {
        JEXTSEL_W::new(self)
    }
    #[doc = "Bit 15 - desc JEXTTRIG"]
    #[inline(always)]
    pub fn jexttrig(&mut self) -> JEXTTRIG_W<15> {
        JEXTTRIG_W::new(self)
    }
    #[doc = "Bits 17:19 - desc EXTSEL"]
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W<17> {
        EXTSEL_W::new(self)
    }
    #[doc = "Bit 20 - desc EXTTRIG"]
    #[inline(always)]
    pub fn exttrig(&mut self) -> EXTTRIG_W<20> {
        EXTTRIG_W::new(self)
    }
    #[doc = "Bit 21 - desc JSWSTART"]
    #[inline(always)]
    pub fn jswstart(&mut self) -> JSWSTART_W<21> {
        JSWSTART_W::new(self)
    }
    #[doc = "Bit 22 - desc SWSTART"]
    #[inline(always)]
    pub fn swstart(&mut self) -> SWSTART_W<22> {
        SWSTART_W::new(self)
    }
    #[doc = "Bit 23 - desc TSVREFE"]
    #[inline(always)]
    pub fn tsvrefe(&mut self) -> TSVREFE_W<23> {
        TSVREFE_W::new(self)
    }
    #[doc = "Bit 24 - desc VBAT"]
    #[inline(always)]
    pub fn vbat(&mut self) -> VBAT_W<24> {
        VBAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
