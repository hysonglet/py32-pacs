#[doc = "Register `TS0` reader"]
pub struct R(crate::R<TS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TS0` writer"]
pub struct W(crate::W<TS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TS0_SPEC>;
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
impl From<crate::W<TS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS0` reader - FLash TS0 register"]
pub type TS0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TS0` writer - FLash TS0 register"]
pub type TS0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TS0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - FLash TS0 register"]
    #[inline(always)]
    pub fn ts0(&self) -> TS0_R {
        TS0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FLash TS0 register"]
    #[inline(always)]
    pub fn ts0(&mut self) -> TS0_W<0> {
        TS0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash TS0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ts0](index.html) module"]
pub struct TS0_SPEC;
impl crate::RegisterSpec for TS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ts0::R](R) reader structure"]
impl crate::Readable for TS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ts0::W](W) writer structure"]
impl crate::Writable for TS0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TS0 to value 0xb4"]
impl crate::Resettable for TS0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb4
    }
}
