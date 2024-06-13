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
#[doc = "Field `CALC0IO` reader - Calibration C0 factor input"]
pub type CALC0IO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALC0IO` writer - Calibration C0 factor input"]
pub type CALC0IO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALFIR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `CALC1IO` reader - Calibration C1 factor input"]
pub type CALC1IO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALC1IO` writer - Calibration C1 factor input"]
pub type CALC1IO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALFIR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `CALC2IO` reader - Calibration C2 factor input"]
pub type CALC2IO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALC2IO` writer - Calibration C2 factor input"]
pub type CALC2IO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALFIR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `CALC3IO` reader - Calibration C3 factor input"]
pub type CALC3IO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALC3IO` writer - Calibration C3 factor input"]
pub type CALC3IO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALFIR2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Calibration C0 factor input"]
    #[inline(always)]
    pub fn calc0io(&self) -> CALC0IO_R {
        CALC0IO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Calibration C1 factor input"]
    #[inline(always)]
    pub fn calc1io(&self) -> CALC1IO_R {
        CALC1IO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Calibration C2 factor input"]
    #[inline(always)]
    pub fn calc2io(&self) -> CALC2IO_R {
        CALC2IO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Calibration C3 factor input"]
    #[inline(always)]
    pub fn calc3io(&self) -> CALC3IO_R {
        CALC3IO_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Calibration C0 factor input"]
    #[inline(always)]
    pub fn calc0io(&mut self) -> CALC0IO_W<0> {
        CALC0IO_W::new(self)
    }
    #[doc = "Bits 8:15 - Calibration C1 factor input"]
    #[inline(always)]
    pub fn calc1io(&mut self) -> CALC1IO_W<8> {
        CALC1IO_W::new(self)
    }
    #[doc = "Bits 16:23 - Calibration C2 factor input"]
    #[inline(always)]
    pub fn calc2io(&mut self) -> CALC2IO_W<16> {
        CALC2IO_W::new(self)
    }
    #[doc = "Bits 24:31 - Calibration C3 factor input"]
    #[inline(always)]
    pub fn calc3io(&mut self) -> CALC3IO_W<24> {
        CALC3IO_W::new(self)
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
