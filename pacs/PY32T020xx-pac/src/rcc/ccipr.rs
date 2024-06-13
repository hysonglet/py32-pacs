#[doc = "Register `CCIPR` reader"]
pub struct R(crate::R<CCIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCIPR` writer"]
pub struct W(crate::W<CCIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR_SPEC>;
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
impl From<crate::W<CCIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMCLK_CTRL` reader - "]
pub type TIMCLK_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `TIMCLK_CTRL` writer - "]
pub type TIMCLK_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCIPR_SPEC, bool, O>;
#[doc = "Field `COMP1SEL` reader - "]
pub type COMP1SEL_R = crate::BitReader<bool>;
#[doc = "Field `COMP1SEL` writer - "]
pub type COMP1SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCIPR_SPEC, bool, O>;
#[doc = "Field `COMP2SEL` reader - "]
pub type COMP2SEL_R = crate::BitReader<bool>;
#[doc = "Field `COMP2SEL` writer - "]
pub type COMP2SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCIPR_SPEC, bool, O>;
#[doc = "Field `ADC_CKMODE` reader - "]
pub type ADC_CKMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_CKMODE` writer - "]
pub type ADC_CKMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timclk_ctrl(&self) -> TIMCLK_CTRL_R {
        TIMCLK_CTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn comp1sel(&self) -> COMP1SEL_R {
        COMP1SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn comp2sel(&self) -> COMP2SEL_R {
        COMP2SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn adc_ckmode(&self) -> ADC_CKMODE_R {
        ADC_CKMODE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timclk_ctrl(&mut self) -> TIMCLK_CTRL_W<0> {
        TIMCLK_CTRL_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn comp1sel(&mut self) -> COMP1SEL_W<8> {
        COMP1SEL_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn comp2sel(&mut self) -> COMP2SEL_W<9> {
        COMP2SEL_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn adc_ckmode(&mut self) -> ADC_CKMODE_W<20> {
        ADC_CKMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripherals independent clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccipr](index.html) module"]
pub struct CCIPR_SPEC;
impl crate::RegisterSpec for CCIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccipr::R](R) reader structure"]
impl crate::Readable for CCIPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccipr::W](W) writer structure"]
impl crate::Writable for CCIPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCIPR to value 0"]
impl crate::Resettable for CCIPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
