#[doc = "Register `PERTPE` reader"]
pub struct R(crate::R<PERTPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERTPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERTPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERTPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERTPE` writer"]
pub struct W(crate::W<PERTPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERTPE_SPEC>;
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
impl From<crate::W<PERTPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERTPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERTPE` reader - FLash PERTPE register"]
pub type PERTPE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PERTPE` writer - FLash PERTPE register"]
pub type PERTPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PERTPE_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bits 0:16 - FLash PERTPE register"]
    #[inline(always)]
    pub fn pertpe(&self) -> PERTPE_R {
        PERTPE_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - FLash PERTPE register"]
    #[inline(always)]
    pub fn pertpe(&mut self) -> PERTPE_W<0> {
        PERTPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash PERTPE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pertpe](index.html) module"]
pub struct PERTPE_SPEC;
impl crate::RegisterSpec for PERTPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pertpe::R](R) reader structure"]
impl crate::Readable for PERTPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pertpe::W](W) writer structure"]
impl crate::Writable for PERTPE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERTPE to value 0xea60"]
impl crate::Resettable for PERTPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xea60
    }
}
