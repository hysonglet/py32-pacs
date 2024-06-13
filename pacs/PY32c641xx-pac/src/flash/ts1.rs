#[doc = "Register `TS1` reader"]
pub struct R(crate::R<TS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TS1` writer"]
pub struct W(crate::W<TS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TS1_SPEC>;
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
impl From<crate::W<TS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS1` reader - FLash TS1 register"]
pub type TS1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TS1` writer - FLash TS1 register"]
pub type TS1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TS1_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - FLash TS1 register"]
    #[inline(always)]
    pub fn ts1(&self) -> TS1_R {
        TS1_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - FLash TS1 register"]
    #[inline(always)]
    pub fn ts1(&mut self) -> TS1_W<0> {
        TS1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash TS1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ts1](index.html) module"]
pub struct TS1_SPEC;
impl crate::RegisterSpec for TS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ts1::R](R) reader structure"]
impl crate::Readable for TS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ts1::W](W) writer structure"]
impl crate::Writable for TS1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TS1 to value 0x01b0"]
impl crate::Resettable for TS1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01b0
    }
}
