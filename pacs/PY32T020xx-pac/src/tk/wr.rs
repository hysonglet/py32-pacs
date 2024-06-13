#[doc = "Register `WR` reader"]
pub struct R(crate::R<WR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WR` writer"]
pub struct W(crate::W<WR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WR_SPEC>;
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
impl From<crate::W<WR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIN` reader - "]
pub type WIN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WIN` writer - "]
pub type WIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn win(&mut self) -> WIN_W<0> {
        WIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Window register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr](index.html) module"]
pub struct WR_SPEC;
impl crate::RegisterSpec for WR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wr::R](R) reader structure"]
impl crate::Readable for WR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wr::W](W) writer structure"]
impl crate::Writable for WR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WR to value 0xffff"]
impl crate::Resettable for WR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
