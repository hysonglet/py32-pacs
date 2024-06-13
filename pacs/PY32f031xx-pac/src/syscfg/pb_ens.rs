#[doc = "Register `PB_ENS` reader"]
pub struct R(crate::R<PB_ENS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PB_ENS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PB_ENS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PB_ENS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PB_ENS` writer"]
pub struct W(crate::W<PB_ENS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PB_ENS_SPEC>;
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
impl From<crate::W<PB_ENS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PB_ENS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PB_ENS` reader - Noise filter enable, active high"]
pub type PB_ENS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PB_ENS` writer - Noise filter enable, active high"]
pub type PB_ENS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PB_ENS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Noise filter enable, active high"]
    #[inline(always)]
    pub fn pb_ens(&self) -> PB_ENS_R {
        PB_ENS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Noise filter enable, active high"]
    #[inline(always)]
    pub fn pb_ens(&mut self) -> PB_ENS_W<0> {
        PB_ENS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOB noise filter enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb_ens](index.html) module"]
pub struct PB_ENS_SPEC;
impl crate::RegisterSpec for PB_ENS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pb_ens::R](R) reader structure"]
impl crate::Readable for PB_ENS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pb_ens::W](W) writer structure"]
impl crate::Writable for PB_ENS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PB_ENS to value 0"]
impl crate::Resettable for PB_ENS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
