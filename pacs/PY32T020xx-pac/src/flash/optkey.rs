#[doc = "Register `OPTKEY` reader"]
pub struct R(crate::R<OPTKEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTKEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTKEY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTKEY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTKEY` writer"]
pub struct W(crate::W<OPTKEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTKEY_SPEC>;
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
impl From<crate::W<OPTKEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTKEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPTKEY` writer - Flash Option key"]
pub type OPTKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTKEY_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Flash Option key"]
    #[inline(always)]
    pub fn optkey(&mut self) -> OPTKEY_W<0> {
        OPTKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optkey](index.html) module"]
pub struct OPTKEY_SPEC;
impl crate::RegisterSpec for OPTKEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [optkey::R](R) reader structure"]
impl crate::Readable for OPTKEY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [optkey::W](W) writer structure"]
impl crate::Writable for OPTKEY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPTKEY to value 0"]
impl crate::Resettable for OPTKEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
