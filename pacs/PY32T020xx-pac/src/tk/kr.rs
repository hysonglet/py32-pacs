#[doc = "Register `KR` reader"]
pub struct R(crate::R<KR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KR` writer"]
pub struct W(crate::W<KR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KR_SPEC>;
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
impl From<crate::W<KR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEN` reader - "]
pub type KEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEN` writer - "]
pub type KEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KR_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn ken(&self) -> KEN_R {
        KEN_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn ken(&mut self) -> KEN_W<0> {
        KEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kr](index.html) module"]
pub struct KR_SPEC;
impl crate::RegisterSpec for KR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [kr::R](R) reader structure"]
impl crate::Readable for KR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [kr::W](W) writer structure"]
impl crate::Writable for KR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KR to value 0"]
impl crate::Resettable for KR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
