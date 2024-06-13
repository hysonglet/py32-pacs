#[doc = "Register `PBENS` reader"]
pub struct R(crate::R<PBENS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBENS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBENS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBENS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBENS` writer"]
pub struct W(crate::W<PBENS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBENS_SPEC>;
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
impl From<crate::W<PBENS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBENS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PB_ENS` reader - desc PB_ENS"]
pub type PB_ENS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PB_ENS` writer - desc PB_ENS"]
pub type PB_ENS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PBENS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - desc PB_ENS"]
    #[inline(always)]
    pub fn pb_ens(&self) -> PB_ENS_R {
        PB_ENS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc PB_ENS"]
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
#[doc = "desc PBENS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbens](index.html) module"]
pub struct PBENS_SPEC;
impl crate::RegisterSpec for PBENS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbens::R](R) reader structure"]
impl crate::Readable for PBENS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbens::W](W) writer structure"]
impl crate::Writable for PBENS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PBENS to value 0"]
impl crate::Resettable for PBENS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
