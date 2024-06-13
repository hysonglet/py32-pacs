#[doc = "Register `SQR1` reader"]
pub struct R(crate::R<SQR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SQR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SQR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SQR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SQR1` writer"]
pub struct W(crate::W<SQR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SQR1_SPEC>;
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
impl From<crate::W<SQR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SQR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SQ13` reader - desc SQ13"]
pub type SQ13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ13` writer - desc SQ13"]
pub type SQ13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ14` reader - desc SQ14"]
pub type SQ14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ14` writer - desc SQ14"]
pub type SQ14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ15` reader - desc SQ15"]
pub type SQ15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ15` writer - desc SQ15"]
pub type SQ15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ16` reader - desc SQ16"]
pub type SQ16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ16` writer - desc SQ16"]
pub type SQ16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `L` reader - desc L"]
pub type L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `L` writer - desc L"]
pub type L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:4 - desc SQ13"]
    #[inline(always)]
    pub fn sq13(&self) -> SQ13_R {
        SQ13_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - desc SQ14"]
    #[inline(always)]
    pub fn sq14(&self) -> SQ14_R {
        SQ14_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - desc SQ15"]
    #[inline(always)]
    pub fn sq15(&self) -> SQ15_R {
        SQ15_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - desc SQ16"]
    #[inline(always)]
    pub fn sq16(&self) -> SQ16_R {
        SQ16_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:23 - desc L"]
    #[inline(always)]
    pub fn l(&self) -> L_R {
        L_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - desc SQ13"]
    #[inline(always)]
    pub fn sq13(&mut self) -> SQ13_W<0> {
        SQ13_W::new(self)
    }
    #[doc = "Bits 5:9 - desc SQ14"]
    #[inline(always)]
    pub fn sq14(&mut self) -> SQ14_W<5> {
        SQ14_W::new(self)
    }
    #[doc = "Bits 10:14 - desc SQ15"]
    #[inline(always)]
    pub fn sq15(&mut self) -> SQ15_W<10> {
        SQ15_W::new(self)
    }
    #[doc = "Bits 15:19 - desc SQ16"]
    #[inline(always)]
    pub fn sq16(&mut self) -> SQ16_W<15> {
        SQ16_W::new(self)
    }
    #[doc = "Bits 20:23 - desc L"]
    #[inline(always)]
    pub fn l(&mut self) -> L_W<20> {
        L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc SQR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqr1](index.html) module"]
pub struct SQR1_SPEC;
impl crate::RegisterSpec for SQR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sqr1::R](R) reader structure"]
impl crate::Readable for SQR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sqr1::W](W) writer structure"]
impl crate::Writable for SQR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SQR1 to value 0"]
impl crate::Resettable for SQR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
