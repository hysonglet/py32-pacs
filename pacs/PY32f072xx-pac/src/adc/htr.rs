#[doc = "Register `HTR` reader"]
pub struct R(crate::R<HTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HTR` writer"]
pub struct W(crate::W<HTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HTR_SPEC>;
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
impl From<crate::W<HTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HT` reader - desc HT"]
pub type HT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HT` writer - desc HT"]
pub type HT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HTR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - desc HT"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc HT"]
    #[inline(always)]
    pub fn ht(&mut self) -> HT_W<0> {
        HT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc HTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htr](index.html) module"]
pub struct HTR_SPEC;
impl crate::RegisterSpec for HTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [htr::R](R) reader structure"]
impl crate::Readable for HTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [htr::W](W) writer structure"]
impl crate::Writable for HTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HTR to value 0"]
impl crate::Resettable for HTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
