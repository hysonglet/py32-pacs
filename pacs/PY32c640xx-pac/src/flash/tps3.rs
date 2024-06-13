#[doc = "Register `TPS3` reader"]
pub struct R(crate::R<TPS3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPS3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPS3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPS3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPS3` writer"]
pub struct W(crate::W<TPS3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPS3_SPEC>;
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
impl From<crate::W<TPS3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPS3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPS3` reader - FLash TPS3 register"]
pub type TPS3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TPS3` writer - FLash TPS3 register"]
pub type TPS3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPS3_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - FLash TPS3 register"]
    #[inline(always)]
    pub fn tps3(&self) -> TPS3_R {
        TPS3_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - FLash TPS3 register"]
    #[inline(always)]
    pub fn tps3(&mut self) -> TPS3_W<0> {
        TPS3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash TPS3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tps3](index.html) module"]
pub struct TPS3_SPEC;
impl crate::RegisterSpec for TPS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tps3::R](R) reader structure"]
impl crate::Readable for TPS3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tps3::W](W) writer structure"]
impl crate::Writable for TPS3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPS3 to value 0x06c0"]
impl crate::Resettable for TPS3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06c0
    }
}
