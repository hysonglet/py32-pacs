#[doc = "Register `XSFCR` reader"]
pub struct R(crate::R<XSFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XSFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XSFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XSFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XSFCR` writer"]
pub struct W(crate::W<XSFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XSFCR_SPEC>;
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
impl From<crate::W<XSFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XSFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XINS` reader - desc XINS"]
pub type XINS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XINS` writer - desc XINS"]
pub type XINS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, XSFCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - desc XINS"]
    #[inline(always)]
    pub fn xins(&self) -> XINS_R {
        XINS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc XINS"]
    #[inline(always)]
    pub fn xins(&mut self) -> XINS_W<0> {
        XINS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc XSFCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xsfcr](index.html) module"]
pub struct XSFCR_SPEC;
impl crate::RegisterSpec for XSFCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [xsfcr::R](R) reader structure"]
impl crate::Readable for XSFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xsfcr::W](W) writer structure"]
impl crate::Writable for XSFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XSFCR to value 0x0b"]
impl crate::Resettable for XSFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0b
    }
}
