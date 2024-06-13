#[doc = "Register `OENR` reader"]
pub struct R(crate::R<OENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OENR` writer"]
pub struct W(crate::W<OENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OENR_SPEC>;
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
impl From<crate::W<OENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPA1OEN` reader - OPA1 output enable"]
pub type OPA1OEN_R = crate::BitReader<bool>;
#[doc = "Field `OPA1OEN` writer - OPA1 output enable"]
pub type OPA1OEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OENR_SPEC, bool, O>;
#[doc = "Field `OPA2OEN` reader - OPA2 output enable"]
pub type OPA2OEN_R = crate::BitReader<bool>;
#[doc = "Field `OPA2OEN` writer - OPA2 output enable"]
pub type OPA2OEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - OPA1 output enable"]
    #[inline(always)]
    pub fn opa1oen(&self) -> OPA1OEN_R {
        OPA1OEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - OPA2 output enable"]
    #[inline(always)]
    pub fn opa2oen(&self) -> OPA2OEN_R {
        OPA2OEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - OPA1 output enable"]
    #[inline(always)]
    pub fn opa1oen(&mut self) -> OPA1OEN_W<1> {
        OPA1OEN_W::new(self)
    }
    #[doc = "Bit 6 - OPA2 output enable"]
    #[inline(always)]
    pub fn opa2oen(&mut self) -> OPA2OEN_W<6> {
        OPA2OEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPA output enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oenr](index.html) module"]
pub struct OENR_SPEC;
impl crate::RegisterSpec for OENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oenr::R](R) reader structure"]
impl crate::Readable for OENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oenr::W](W) writer structure"]
impl crate::Writable for OENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OENR to value 0"]
impl crate::Resettable for OENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
