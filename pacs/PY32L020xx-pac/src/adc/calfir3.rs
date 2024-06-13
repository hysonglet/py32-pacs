#[doc = "Register `CALFIR3` reader"]
pub struct R(crate::R<CALFIR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALFIR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALFIR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALFIR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALFIR3` writer"]
pub struct W(crate::W<CALFIR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALFIR3_SPEC>;
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
impl From<crate::W<CALFIR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALFIR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALCBIO` reader - Calibration C0 factor input"]
pub type CALCBIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALCBIO` writer - Calibration C0 factor input"]
pub type CALCBIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALFIR3_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Calibration C0 factor input"]
    #[inline(always)]
    pub fn calcbio(&self) -> CALCBIO_R {
        CALCBIO_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Calibration C0 factor input"]
    #[inline(always)]
    pub fn calcbio(&mut self) -> CALCBIO_W<0> {
        CALCBIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC calibration factor input register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calfir3](index.html) module"]
pub struct CALFIR3_SPEC;
impl crate::RegisterSpec for CALFIR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calfir3::R](R) reader structure"]
impl crate::Readable for CALFIR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calfir3::W](W) writer structure"]
impl crate::Writable for CALFIR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALFIR3 to value 0"]
impl crate::Resettable for CALFIR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
