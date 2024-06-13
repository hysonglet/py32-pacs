#[doc = "Register `IDACR` reader"]
pub struct R(crate::R<IDACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDACR` writer"]
pub struct W(crate::W<IDACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDACR_SPEC>;
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
impl From<crate::W<IDACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIDAC` reader - "]
pub type MIDAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIDAC` writer - "]
pub type MIDAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDACR_SPEC, u8, u8, 8, O>;
#[doc = "Field `SIDAC` reader - "]
pub type SIDAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIDAC` writer - "]
pub type SIDAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDACR_SPEC, u8, u8, 8, O>;
#[doc = "Field `SIDAC_CSA_TARGE` reader - "]
pub type SIDAC_CSA_TARGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIDAC_CSA_TARGE` writer - "]
pub type SIDAC_CSA_TARGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDACR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn midac(&self) -> MIDAC_R {
        MIDAC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn sidac(&self) -> SIDAC_R {
        SIDAC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sidac_csa_targe(&self) -> SIDAC_CSA_TARGE_R {
        SIDAC_CSA_TARGE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn midac(&mut self) -> MIDAC_W<0> {
        MIDAC_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn sidac(&mut self) -> SIDAC_W<16> {
        SIDAC_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sidac_csa_targe(&mut self) -> SIDAC_CSA_TARGE_W<24> {
        SIDAC_CSA_TARGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IDAC Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idacr](index.html) module"]
pub struct IDACR_SPEC;
impl crate::RegisterSpec for IDACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idacr::R](R) reader structure"]
impl crate::Readable for IDACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idacr::W](W) writer structure"]
impl crate::Writable for IDACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IDACR to value 0x8080_0080"]
impl crate::Resettable for IDACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8080_0080
    }
}
