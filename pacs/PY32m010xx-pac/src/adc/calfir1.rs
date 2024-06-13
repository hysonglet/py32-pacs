#[doc = "Register `CALFIR1` reader"]
pub struct R(crate::R<CALFIR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALFIR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALFIR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALFIR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALFIR1` writer"]
pub struct W(crate::W<CALFIR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALFIR1_SPEC>;
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
impl From<crate::W<CALFIR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALFIR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALC9IO` reader - Calibration C4 factor input"]
pub type CALC9IO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALC9IO` writer - Calibration C4 factor input"]
pub type CALC9IO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALFIR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `CALC10IO` reader - Calibration C5 factor input"]
pub type CALC10IO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALC10IO` writer - Calibration C5 factor input"]
pub type CALC10IO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALFIR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `CALC11IO` reader - Calibration offset factor input"]
pub type CALC11IO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALC11IO` writer - Calibration offset factor input"]
pub type CALC11IO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALFIR1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Calibration C4 factor input"]
    #[inline(always)]
    pub fn calc9io(&self) -> CALC9IO_R {
        CALC9IO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 9:16 - Calibration C5 factor input"]
    #[inline(always)]
    pub fn calc10io(&self) -> CALC10IO_R {
        CALC10IO_R::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bits 18:25 - Calibration offset factor input"]
    #[inline(always)]
    pub fn calc11io(&self) -> CALC11IO_R {
        CALC11IO_R::new(((self.bits >> 18) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Calibration C4 factor input"]
    #[inline(always)]
    pub fn calc9io(&mut self) -> CALC9IO_W<0> {
        CALC9IO_W::new(self)
    }
    #[doc = "Bits 9:16 - Calibration C5 factor input"]
    #[inline(always)]
    pub fn calc10io(&mut self) -> CALC10IO_W<9> {
        CALC10IO_W::new(self)
    }
    #[doc = "Bits 18:25 - Calibration offset factor input"]
    #[inline(always)]
    pub fn calc11io(&mut self) -> CALC11IO_W<18> {
        CALC11IO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC calibration factor input register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calfir1](index.html) module"]
pub struct CALFIR1_SPEC;
impl crate::RegisterSpec for CALFIR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calfir1::R](R) reader structure"]
impl crate::Readable for CALFIR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calfir1::W](W) writer structure"]
impl crate::Writable for CALFIR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALFIR1 to value 0"]
impl crate::Resettable for CALFIR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
