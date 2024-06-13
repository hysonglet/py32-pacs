#[doc = "Register `PRSR` reader"]
pub struct R(crate::R<PRSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRSR` writer"]
pub struct W(crate::W<PRSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRSR_SPEC>;
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
impl From<crate::W<PRSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LFSRP` reader - "]
pub type LFSRP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LFSRP` writer - "]
pub type LFSRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRSR_SPEC, u16, u16, 16, O>;
#[doc = "Field `SEED` reader - "]
pub type SEED_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SEED` writer - "]
pub type SEED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRSR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn lfsrp(&self) -> LFSRP_R {
        LFSRP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn seed(&self) -> SEED_R {
        SEED_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn lfsrp(&mut self) -> LFSRP_W<0> {
        LFSRP_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn seed(&mut self) -> SEED_W<16> {
        SEED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pseudo Random Sequence register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prsr](index.html) module"]
pub struct PRSR_SPEC;
impl crate::RegisterSpec for PRSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prsr::R](R) reader structure"]
impl crate::Readable for PRSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prsr::W](W) writer structure"]
impl crate::Writable for PRSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRSR to value 0xffff_ffff"]
impl crate::Resettable for PRSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
