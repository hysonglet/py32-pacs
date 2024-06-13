#[doc = "Register `DHR12R2` reader"]
pub struct R(crate::R<DHR12R2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DHR12R2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DHR12R2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DHR12R2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DHR12R2` writer"]
pub struct W(crate::W<DHR12R2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DHR12R2_SPEC>;
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
impl From<crate::W<DHR12R2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DHR12R2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACC2DHR` reader - desc DACC2DHR"]
pub type DACC2DHR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DACC2DHR` writer - desc DACC2DHR"]
pub type DACC2DHR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DHR12R2_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - desc DACC2DHR"]
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc DACC2DHR"]
    #[inline(always)]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W<0> {
        DACC2DHR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc DHR12R2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhr12r2](index.html) module"]
pub struct DHR12R2_SPEC;
impl crate::RegisterSpec for DHR12R2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dhr12r2::R](R) reader structure"]
impl crate::Readable for DHR12R2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dhr12r2::W](W) writer structure"]
impl crate::Writable for DHR12R2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DHR12R2 to value 0"]
impl crate::Resettable for DHR12R2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
