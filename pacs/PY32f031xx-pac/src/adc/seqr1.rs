#[doc = "Register `SEQR1` reader"]
pub struct R(crate::R<SEQR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQR1` writer"]
pub struct W(crate::W<SEQR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQR1_SPEC>;
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
impl From<crate::W<SEQR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SQ8` reader - ADC sequence select"]
pub type SQ8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ8` writer - ADC sequence select"]
pub type SQ8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `SQ9` reader - ADC sequence select"]
pub type SQ9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ9` writer - ADC sequence select"]
pub type SQ9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `SQ10` reader - ADC sequence select"]
pub type SQ10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ10` writer - ADC sequence select"]
pub type SQ10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `SQ11` reader - ADC sequence select"]
pub type SQ11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ11` writer - ADC sequence select"]
pub type SQ11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `SQ12` reader - ADC sequence select"]
pub type SQ12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ12` writer - ADC sequence select"]
pub type SQ12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `SQ13` reader - ADC sequence select"]
pub type SQ13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ13` writer - ADC sequence select"]
pub type SQ13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `SQ14` reader - ADC sequence select"]
pub type SQ14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ14` writer - ADC sequence select"]
pub type SQ14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQR1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - ADC sequence select"]
    #[inline(always)]
    pub fn sq8(&self) -> SQ8_R {
        SQ8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ADC sequence select"]
    #[inline(always)]
    pub fn sq9(&self) -> SQ9_R {
        SQ9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ADC sequence select"]
    #[inline(always)]
    pub fn sq10(&self) -> SQ10_R {
        SQ10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - ADC sequence select"]
    #[inline(always)]
    pub fn sq11(&self) -> SQ11_R {
        SQ11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - ADC sequence select"]
    #[inline(always)]
    pub fn sq12(&self) -> SQ12_R {
        SQ12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - ADC sequence select"]
    #[inline(always)]
    pub fn sq13(&self) -> SQ13_R {
        SQ13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ADC sequence select"]
    #[inline(always)]
    pub fn sq14(&self) -> SQ14_R {
        SQ14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC sequence select"]
    #[inline(always)]
    pub fn sq8(&mut self) -> SQ8_W<0> {
        SQ8_W::new(self)
    }
    #[doc = "Bits 4:7 - ADC sequence select"]
    #[inline(always)]
    pub fn sq9(&mut self) -> SQ9_W<4> {
        SQ9_W::new(self)
    }
    #[doc = "Bits 8:11 - ADC sequence select"]
    #[inline(always)]
    pub fn sq10(&mut self) -> SQ10_W<8> {
        SQ10_W::new(self)
    }
    #[doc = "Bits 12:15 - ADC sequence select"]
    #[inline(always)]
    pub fn sq11(&mut self) -> SQ11_W<12> {
        SQ11_W::new(self)
    }
    #[doc = "Bits 16:19 - ADC sequence select"]
    #[inline(always)]
    pub fn sq12(&mut self) -> SQ12_W<16> {
        SQ12_W::new(self)
    }
    #[doc = "Bits 20:23 - ADC sequence select"]
    #[inline(always)]
    pub fn sq13(&mut self) -> SQ13_W<20> {
        SQ13_W::new(self)
    }
    #[doc = "Bits 24:27 - ADC sequence select"]
    #[inline(always)]
    pub fn sq14(&mut self) -> SQ14_W<24> {
        SQ14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC sequence select register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seqr1](index.html) module"]
pub struct SEQR1_SPEC;
impl crate::RegisterSpec for SEQR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seqr1::R](R) reader structure"]
impl crate::Readable for SEQR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seqr1::W](W) writer structure"]
impl crate::Writable for SEQR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEQR1 to value 0"]
impl crate::Resettable for SEQR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
