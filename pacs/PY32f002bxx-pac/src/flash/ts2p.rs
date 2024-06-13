#[doc = "Register `TS2P` reader"]
pub struct R(crate::R<TS2P_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TS2P_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TS2P_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TS2P_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TS2P` writer"]
pub struct W(crate::W<TS2P_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TS2P_SPEC>;
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
impl From<crate::W<TS2P_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TS2P_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS2P` reader - FLash TS2P register"]
pub type TS2P_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TS2P` writer - FLash TS2P register"]
pub type TS2P_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TS2P_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - FLash TS2P register"]
    #[inline(always)]
    pub fn ts2p(&self) -> TS2P_R {
        TS2P_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - FLash TS2P register"]
    #[inline(always)]
    pub fn ts2p(&mut self) -> TS2P_W<0> {
        TS2P_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash TS2P register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ts2p](index.html) module"]
pub struct TS2P_SPEC;
impl crate::RegisterSpec for TS2P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ts2p::R](R) reader structure"]
impl crate::Readable for TS2P_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ts2p::W](W) writer structure"]
impl crate::Writable for TS2P_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TS2P to value 0xb4"]
impl crate::Resettable for TS2P_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb4
    }
}