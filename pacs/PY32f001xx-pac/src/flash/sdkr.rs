#[doc = "Register `SDKR` reader"]
pub struct R(crate::R<SDKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDKR` writer"]
pub struct W(crate::W<SDKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDKR_SPEC>;
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
impl From<crate::W<SDKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDK_STRT` reader - SDK area start address"]
pub type SDK_STRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDK_STRT` writer - SDK area start address"]
pub type SDK_STRT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDKR_SPEC, u8, u8, 4, O>;
#[doc = "Field `SDK_END` reader - SDK area end address"]
pub type SDK_END_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDK_END` writer - SDK area end address"]
pub type SDK_END_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDKR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - SDK area start address"]
    #[inline(always)]
    pub fn sdk_strt(&self) -> SDK_STRT_R {
        SDK_STRT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SDK area end address"]
    #[inline(always)]
    pub fn sdk_end(&self) -> SDK_END_R {
        SDK_END_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SDK area start address"]
    #[inline(always)]
    pub fn sdk_strt(&mut self) -> SDK_STRT_W<0> {
        SDK_STRT_W::new(self)
    }
    #[doc = "Bits 8:11 - SDK area end address"]
    #[inline(always)]
    pub fn sdk_end(&mut self) -> SDK_END_W<8> {
        SDK_END_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash SDK address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdkr](index.html) module"]
pub struct SDKR_SPEC;
impl crate::RegisterSpec for SDKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdkr::R](R) reader structure"]
impl crate::Readable for SDKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdkr::W](W) writer structure"]
impl crate::Writable for SDKR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDKR to value 0xffe0_001f"]
impl crate::Resettable for SDKR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffe0_001f
    }
}
