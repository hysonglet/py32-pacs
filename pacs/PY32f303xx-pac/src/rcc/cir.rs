#[doc = "Register `CIR` reader"]
pub struct R(crate::R<CIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIR` writer"]
pub struct W(crate::W<CIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIR_SPEC>;
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
impl From<crate::W<CIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSIRDYF` reader - desc LSIRDYF"]
pub type LSIRDYF_R = crate::BitReader<bool>;
#[doc = "Field `LSERDYF` reader - desc LSERDYF"]
pub type LSERDYF_R = crate::BitReader<bool>;
#[doc = "Field `HSIRDYF` reader - desc HSIRDYF"]
pub type HSIRDYF_R = crate::BitReader<bool>;
#[doc = "Field `HSERDYF` reader - desc HSERDYF"]
pub type HSERDYF_R = crate::BitReader<bool>;
#[doc = "Field `PLLRDYF` reader - desc PLLRDYF"]
pub type PLLRDYF_R = crate::BitReader<bool>;
#[doc = "Field `HSI48RDYF` reader - desc HSI48RDYF"]
pub type HSI48RDYF_R = crate::BitReader<bool>;
#[doc = "Field `CSSF` reader - desc CSSF"]
pub type CSSF_R = crate::BitReader<bool>;
#[doc = "Field `CSSF` writer - desc CSSF"]
pub type CSSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `LSIRDYIE` reader - desc LSIRDYIE"]
pub type LSIRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `LSIRDYIE` writer - desc LSIRDYIE"]
pub type LSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `LSERDYIE` reader - desc LSERDYIE"]
pub type LSERDYIE_R = crate::BitReader<bool>;
#[doc = "Field `LSERDYIE` writer - desc LSERDYIE"]
pub type LSERDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `HSIRDYIE` reader - desc HSIRDYIE"]
pub type HSIRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `HSIRDYIE` writer - desc HSIRDYIE"]
pub type HSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `HSERDYIE` reader - desc HSERDYIE"]
pub type HSERDYIE_R = crate::BitReader<bool>;
#[doc = "Field `HSERDYIE` writer - desc HSERDYIE"]
pub type HSERDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `PLLRDYIE` reader - desc PLLRDYIE"]
pub type PLLRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `PLLRDYIE` writer - desc PLLRDYIE"]
pub type PLLRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `HSI48RDYIE` reader - desc HSI48RDYIE"]
pub type HSI48RDYIE_R = crate::BitReader<bool>;
#[doc = "Field `HSI48RDYIE` writer - desc HSI48RDYIE"]
pub type HSI48RDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `LSIRDYC` writer - desc LSIRDYC"]
pub type LSIRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `LSERDYC` writer - desc LSERDYC"]
pub type LSERDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `HSIRDYC` writer - desc HSIRDYC"]
pub type HSIRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `HSERDYC` writer - desc HSERDYC"]
pub type HSERDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `PLLRDYC` writer - desc PLLRDYC"]
pub type PLLRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `HSI48RDYC` writer - desc HSI48RDYC"]
pub type HSI48RDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `CSSC` writer - desc CSSC"]
pub type CSSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc LSIRDYF"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc LSERDYF"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc HSIRDYF"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc HSERDYF"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PLLRDYF"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc HSI48RDYF"]
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - desc CSSF"]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc LSIRDYIE"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc LSERDYIE"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc HSIRDYIE"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc HSERDYIE"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc PLLRDYIE"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc HSI48RDYIE"]
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - desc CSSF"]
    #[inline(always)]
    pub fn cssf(&mut self) -> CSSF_W<7> {
        CSSF_W::new(self)
    }
    #[doc = "Bit 8 - desc LSIRDYIE"]
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<8> {
        LSIRDYIE_W::new(self)
    }
    #[doc = "Bit 9 - desc LSERDYIE"]
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<9> {
        LSERDYIE_W::new(self)
    }
    #[doc = "Bit 10 - desc HSIRDYIE"]
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<10> {
        HSIRDYIE_W::new(self)
    }
    #[doc = "Bit 11 - desc HSERDYIE"]
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<11> {
        HSERDYIE_W::new(self)
    }
    #[doc = "Bit 12 - desc PLLRDYIE"]
    #[inline(always)]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<12> {
        PLLRDYIE_W::new(self)
    }
    #[doc = "Bit 13 - desc HSI48RDYIE"]
    #[inline(always)]
    pub fn hsi48rdyie(&mut self) -> HSI48RDYIE_W<13> {
        HSI48RDYIE_W::new(self)
    }
    #[doc = "Bit 16 - desc LSIRDYC"]
    #[inline(always)]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<16> {
        LSIRDYC_W::new(self)
    }
    #[doc = "Bit 17 - desc LSERDYC"]
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LSERDYC_W<17> {
        LSERDYC_W::new(self)
    }
    #[doc = "Bit 18 - desc HSIRDYC"]
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<18> {
        HSIRDYC_W::new(self)
    }
    #[doc = "Bit 19 - desc HSERDYC"]
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HSERDYC_W<19> {
        HSERDYC_W::new(self)
    }
    #[doc = "Bit 20 - desc PLLRDYC"]
    #[inline(always)]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<20> {
        PLLRDYC_W::new(self)
    }
    #[doc = "Bit 21 - desc HSI48RDYC"]
    #[inline(always)]
    pub fn hsi48rdyc(&mut self) -> HSI48RDYC_W<21> {
        HSI48RDYC_W::new(self)
    }
    #[doc = "Bit 23 - desc CSSC"]
    #[inline(always)]
    pub fn cssc(&mut self) -> CSSC_W<23> {
        CSSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clock interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir](index.html) module"]
pub struct CIR_SPEC;
impl crate::RegisterSpec for CIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cir::R](R) reader structure"]
impl crate::Readable for CIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cir::W](W) writer structure"]
impl crate::Writable for CIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CIR to value 0"]
impl crate::Resettable for CIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
