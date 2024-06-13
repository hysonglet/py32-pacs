#[doc = "Register `PRGTPE` reader"]
pub struct R(crate::R<PRGTPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRGTPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRGTPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRGTPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRGTPE` writer"]
pub struct W(crate::W<PRGTPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRGTPE_SPEC>;
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
impl From<crate::W<PRGTPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRGTPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRGTPE` reader - FLash PRGTPE register"]
pub type PRGTPE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRGTPE` writer - FLash PRGTPE register"]
pub type PRGTPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRGTPE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - FLash PRGTPE register"]
    #[inline(always)]
    pub fn prgtpe(&self) -> PRGTPE_R {
        PRGTPE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - FLash PRGTPE register"]
    #[inline(always)]
    pub fn prgtpe(&mut self) -> PRGTPE_W<0> {
        PRGTPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash PRGTPE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prgtpe](index.html) module"]
pub struct PRGTPE_SPEC;
impl crate::RegisterSpec for PRGTPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prgtpe::R](R) reader structure"]
impl crate::Readable for PRGTPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prgtpe::W](W) writer structure"]
impl crate::Writable for PRGTPE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRGTPE to value 0x5dc0"]
impl crate::Resettable for PRGTPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x5dc0
    }
}
