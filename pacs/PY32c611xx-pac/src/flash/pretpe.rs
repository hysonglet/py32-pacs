#[doc = "Register `PRETPE` reader"]
pub struct R(crate::R<PRETPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRETPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRETPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRETPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRETPE` writer"]
pub struct W(crate::W<PRETPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRETPE_SPEC>;
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
impl From<crate::W<PRETPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRETPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRETPE` reader - FLash PRETPE register"]
pub type PRETPE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRETPE` writer - FLash PRETPE register"]
pub type PRETPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRETPE_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:12 - FLash PRETPE register"]
    #[inline(always)]
    pub fn pretpe(&self) -> PRETPE_R {
        PRETPE_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - FLash PRETPE register"]
    #[inline(always)]
    pub fn pretpe(&mut self) -> PRETPE_W<0> {
        PRETPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash PRETPE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pretpe](index.html) module"]
pub struct PRETPE_SPEC;
impl crate::RegisterSpec for PRETPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pretpe::R](R) reader structure"]
impl crate::Readable for PRETPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pretpe::W](W) writer structure"]
impl crate::Writable for PRETPE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRETPE to value 0x12c0"]
impl crate::Resettable for PRETPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x12c0
    }
}
