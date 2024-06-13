#[doc = "Register `CALFIR2` reader"]
pub struct R(crate::R<CALFIR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALFIR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALFIR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALFIR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALFIR2` writer"]
pub struct W(crate::W<CALFIR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALFIR2_SPEC>;
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
impl From<crate::W<CALFIR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALFIR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALC6IO` reader - Calibration C1 factor input"]
pub type CALC6IO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALC6IO` writer - Calibration C1 factor input"]
pub type CALC6IO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALFIR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `CALC7IO` reader - Calibration C2 factor input"]
pub type CALC7IO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALC7IO` writer - Calibration C2 factor input"]
pub type CALC7IO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALFIR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `CALC8IO` reader - Calibration C3 factor input"]
pub type CALC8IO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALC8IO` writer - Calibration C3 factor input"]
pub type CALC8IO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALFIR2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Calibration C1 factor input"]
    #[inline(always)]
    pub fn calc6io(&self) -> CALC6IO_R {
        CALC6IO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 9:16 - Calibration C2 factor input"]
    #[inline(always)]
    pub fn calc7io(&self) -> CALC7IO_R {
        CALC7IO_R::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bits 18:25 - Calibration C3 factor input"]
    #[inline(always)]
    pub fn calc8io(&self) -> CALC8IO_R {
        CALC8IO_R::new(((self.bits >> 18) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Calibration C1 factor input"]
    #[inline(always)]
    pub fn calc6io(&mut self) -> CALC6IO_W<0> {
        CALC6IO_W::new(self)
    }
    #[doc = "Bits 9:16 - Calibration C2 factor input"]
    #[inline(always)]
    pub fn calc7io(&mut self) -> CALC7IO_W<9> {
        CALC7IO_W::new(self)
    }
    #[doc = "Bits 18:25 - Calibration C3 factor input"]
    #[inline(always)]
    pub fn calc8io(&mut self) -> CALC8IO_W<18> {
        CALC8IO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC calibration factor input register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calfir2](index.html) module"]
pub struct CALFIR2_SPEC;
impl crate::RegisterSpec for CALFIR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calfir2::R](R) reader structure"]
impl crate::Readable for CALFIR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calfir2::W](W) writer structure"]
impl crate::Writable for CALFIR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALFIR2 to value 0"]
impl crate::Resettable for CALFIR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
