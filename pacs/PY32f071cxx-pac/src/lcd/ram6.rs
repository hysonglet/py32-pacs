#[doc = "Register `RAM6` reader"]
pub struct R(crate::R<RAM6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAM6` writer"]
pub struct W(crate::W<RAM6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM6_SPEC>;
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
impl From<crate::W<RAM6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D` reader - des D"]
pub type D_R = crate::FieldReader<u32, u32>;
#[doc = "Field `D` writer - des D"]
pub type D_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RAM6_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - des D"]
    #[inline(always)]
    pub fn d(&self) -> D_R {
        D_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - des D"]
    #[inline(always)]
    pub fn d(&mut self) -> D_W<0> {
        D_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "des RAM6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram6](index.html) module"]
pub struct RAM6_SPEC;
impl crate::RegisterSpec for RAM6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram6::R](R) reader structure"]
impl crate::Readable for RAM6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram6::W](W) writer structure"]
impl crate::Writable for RAM6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAM6 to value 0"]
impl crate::Resettable for RAM6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
