#[doc = "Register `SIGN` reader"]
pub struct R(crate::R<SIGN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGN` writer"]
pub struct W(crate::W<SIGN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGN_SPEC>;
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
impl From<crate::W<SIGN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIGN` reader - des SIGN"]
pub type SIGN_R = crate::BitReader<bool>;
#[doc = "Field `SIGN` writer - des SIGN"]
pub type SIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIGN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - des SIGN"]
    #[inline(always)]
    pub fn sign(&self) -> SIGN_R {
        SIGN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - des SIGN"]
    #[inline(always)]
    pub fn sign(&mut self) -> SIGN_W<0> {
        SIGN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "des SIGN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sign](index.html) module"]
pub struct SIGN_SPEC;
impl crate::RegisterSpec for SIGN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sign::R](R) reader structure"]
impl crate::Readable for SIGN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sign::W](W) writer structure"]
impl crate::Writable for SIGN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIGN to value 0"]
impl crate::Resettable for SIGN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
