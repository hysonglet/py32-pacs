#[doc = "Register `SMERTPE` reader"]
pub struct R(crate::R<SMERTPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMERTPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMERTPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMERTPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMERTPE` writer"]
pub struct W(crate::W<SMERTPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMERTPE_SPEC>;
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
impl From<crate::W<SMERTPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMERTPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMERTPE` reader - desc SMERTPE"]
pub type SMERTPE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SMERTPE` writer - desc SMERTPE"]
pub type SMERTPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMERTPE_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bits 0:16 - desc SMERTPE"]
    #[inline(always)]
    pub fn smertpe(&self) -> SMERTPE_R {
        SMERTPE_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - desc SMERTPE"]
    #[inline(always)]
    pub fn smertpe(&mut self) -> SMERTPE_W<0> {
        SMERTPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc SMERTPE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smertpe](index.html) module"]
pub struct SMERTPE_SPEC;
impl crate::RegisterSpec for SMERTPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smertpe::R](R) reader structure"]
impl crate::Readable for SMERTPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smertpe::W](W) writer structure"]
impl crate::Writable for SMERTPE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMERTPE to value 0xfd20"]
impl crate::Resettable for SMERTPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfd20
    }
}
