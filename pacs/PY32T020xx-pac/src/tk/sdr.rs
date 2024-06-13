#[doc = "Register `SDR` reader"]
pub struct R(crate::R<SDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDR` writer"]
pub struct W(crate::W<SDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDR_SPEC>;
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
impl From<crate::W<SDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWDIV` reader - "]
pub type SWDIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SWDIV` writer - "]
pub type SWDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDR_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn swdiv(&self) -> SWDIV_R {
        SWDIV_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn swdiv(&mut self) -> SWDIV_W<0> {
        SWDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Switch Divide register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdr](index.html) module"]
pub struct SDR_SPEC;
impl crate::RegisterSpec for SDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdr::R](R) reader structure"]
impl crate::Readable for SDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdr::W](W) writer structure"]
impl crate::Writable for SDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDR to value 0"]
impl crate::Resettable for SDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
