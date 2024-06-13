#[doc = "Register `TS3` reader"]
pub struct R(crate::R<TS3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TS3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TS3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TS3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TS3` writer"]
pub struct W(crate::W<TS3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TS3_SPEC>;
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
impl From<crate::W<TS3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TS3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS3` reader - FLash TS3 register"]
pub type TS3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TS3` writer - FLash TS3 register"]
pub type TS3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TS3_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - FLash TS3 register"]
    #[inline(always)]
    pub fn ts3(&self) -> TS3_R {
        TS3_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - FLash TS3 register"]
    #[inline(always)]
    pub fn ts3(&mut self) -> TS3_W<0> {
        TS3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash TS3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ts3](index.html) module"]
pub struct TS3_SPEC;
impl crate::RegisterSpec for TS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ts3::R](R) reader structure"]
impl crate::Readable for TS3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ts3::W](W) writer structure"]
impl crate::Writable for TS3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TS3 to value 0xb4"]
impl crate::Resettable for TS3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb4
    }
}
