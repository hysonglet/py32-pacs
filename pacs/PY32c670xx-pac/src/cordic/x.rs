#[doc = "Register `X` reader"]
pub struct R(crate::R<X_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<X_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<X_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<X_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `X` writer"]
pub struct W(crate::W<X_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<X_SPEC>;
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
impl From<crate::W<X_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<X_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `X` reader - arctan or module X register"]
pub type X_R = crate::FieldReader<u32, u32>;
#[doc = "Field `X` writer - arctan or module X register"]
pub type X_W<'a, const O: u8> = crate::FieldWriter<'a, u32, X_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - arctan or module X register"]
    #[inline(always)]
    pub fn x(&self) -> X_R {
        X_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - arctan or module X register"]
    #[inline(always)]
    pub fn x(&mut self) -> X_W<0> {
        X_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Arctan input coordinate register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [x](index.html) module"]
pub struct X_SPEC;
impl crate::RegisterSpec for X_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [x::R](R) reader structure"]
impl crate::Readable for X_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [x::W](W) writer structure"]
impl crate::Writable for X_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets X to value 0"]
impl crate::Resettable for X_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
