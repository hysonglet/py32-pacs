#[doc = "Register `BUSY` reader"]
pub struct R(crate::R<BUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUSY` writer"]
pub struct W(crate::W<BUSY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUSY_SPEC>;
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
impl From<crate::W<BUSY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUSY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSY` reader - "]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - "]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUSY_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W<0> {
        BUSY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busy](index.html) module"]
pub struct BUSY_SPEC;
impl crate::RegisterSpec for BUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [busy::R](R) reader structure"]
impl crate::Readable for BUSY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [busy::W](W) writer structure"]
impl crate::Writable for BUSY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUSY to value 0"]
impl crate::Resettable for BUSY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
