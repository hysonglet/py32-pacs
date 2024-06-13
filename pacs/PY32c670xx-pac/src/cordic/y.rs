#[doc = "Register `Y` reader"]
pub struct R(crate::R<Y_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<Y_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<Y_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<Y_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Y` writer"]
pub struct W(crate::W<Y_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<Y_SPEC>;
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
impl From<crate::W<Y_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<Y_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Y` reader - arctan or module Y register"]
pub type Y_R = crate::FieldReader<u32, u32>;
#[doc = "Field `Y` writer - arctan or module Y register"]
pub type Y_W<'a, const O: u8> = crate::FieldWriter<'a, u32, Y_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - arctan or module Y register"]
    #[inline(always)]
    pub fn y(&self) -> Y_R {
        Y_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - arctan or module Y register"]
    #[inline(always)]
    pub fn y(&mut self) -> Y_W<0> {
        Y_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Arctan input coordinate register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [y](index.html) module"]
pub struct Y_SPEC;
impl crate::RegisterSpec for Y_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [y::R](R) reader structure"]
impl crate::Readable for Y_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [y::W](W) writer structure"]
impl crate::Writable for Y_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Y to value 0"]
impl crate::Resettable for Y_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
