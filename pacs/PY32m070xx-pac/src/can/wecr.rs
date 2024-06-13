#[doc = "Register `WECR` reader"]
pub struct R(crate::R<WECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WECR` writer"]
pub struct W(crate::W<WECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WECR_SPEC>;
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
impl From<crate::W<WECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EWL` reader - desc EWL"]
pub type EWL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EWL` writer - desc EWL"]
pub type EWL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WECR_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFWL` reader - desc AFWL"]
pub type AFWL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFWL` writer - desc AFWL"]
pub type AFWL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WECR_SPEC, u8, u8, 4, O>;
#[doc = "Field `ALC` reader - desc ALC"]
pub type ALC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KOER` reader - desc KOER"]
pub type KOER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RECNT` reader - desc RECNT"]
pub type RECNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TECNT` reader - desc TECNT"]
pub type TECNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - desc EWL"]
    #[inline(always)]
    pub fn ewl(&self) -> EWL_R {
        EWL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc AFWL"]
    #[inline(always)]
    pub fn afwl(&self) -> AFWL_R {
        AFWL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - desc ALC"]
    #[inline(always)]
    pub fn alc(&self) -> ALC_R {
        ALC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - desc KOER"]
    #[inline(always)]
    pub fn koer(&self) -> KOER_R {
        KOER_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:23 - desc RECNT"]
    #[inline(always)]
    pub fn recnt(&self) -> RECNT_R {
        RECNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - desc TECNT"]
    #[inline(always)]
    pub fn tecnt(&self) -> TECNT_R {
        TECNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc EWL"]
    #[inline(always)]
    pub fn ewl(&mut self) -> EWL_W<0> {
        EWL_W::new(self)
    }
    #[doc = "Bits 4:7 - desc AFWL"]
    #[inline(always)]
    pub fn afwl(&mut self) -> AFWL_W<4> {
        AFWL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc WECR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wecr](index.html) module"]
pub struct WECR_SPEC;
impl crate::RegisterSpec for WECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wecr::R](R) reader structure"]
impl crate::Readable for WECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wecr::W](W) writer structure"]
impl crate::Writable for WECR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WECR to value 0x1b"]
impl crate::Resettable for WECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1b
    }
}
