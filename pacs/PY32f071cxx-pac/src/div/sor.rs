#[doc = "Register `SOR` reader"]
pub struct R(crate::R<SOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOR` writer"]
pub struct W(crate::W<SOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOR_SPEC>;
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
impl From<crate::W<SOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOR` reader - Divisor"]
pub type SOR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SOR` writer - Divisor"]
pub type SOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SOR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Divisor"]
    #[inline(always)]
    pub fn sor(&self) -> SOR_R {
        SOR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Divisor"]
    #[inline(always)]
    pub fn sor(&mut self) -> SOR_W<0> {
        SOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Divisor\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sor](index.html) module"]
pub struct SOR_SPEC;
impl crate::RegisterSpec for SOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sor::R](R) reader structure"]
impl crate::Readable for SOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sor::W](W) writer structure"]
impl crate::Writable for SOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOR to value 0"]
impl crate::Resettable for SOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
