#[doc = "Register `PCENS` reader"]
pub struct R(crate::R<PCENS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCENS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCENS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCENS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCENS` writer"]
pub struct W(crate::W<PCENS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCENS_SPEC>;
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
impl From<crate::W<PCENS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCENS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC_ENS` reader - desc PC_ENS"]
pub type PC_ENS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PC_ENS` writer - desc PC_ENS"]
pub type PC_ENS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCENS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - desc PC_ENS"]
    #[inline(always)]
    pub fn pc_ens(&self) -> PC_ENS_R {
        PC_ENS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc PC_ENS"]
    #[inline(always)]
    pub fn pc_ens(&mut self) -> PC_ENS_W<0> {
        PC_ENS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc PCENS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcens](index.html) module"]
pub struct PCENS_SPEC;
impl crate::RegisterSpec for PCENS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcens::R](R) reader structure"]
impl crate::Readable for PCENS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcens::W](W) writer structure"]
impl crate::Writable for PCENS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCENS to value 0"]
impl crate::Resettable for PCENS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
