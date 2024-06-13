#[doc = "Register `RAM9` reader"]
pub struct R(crate::R<RAM9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAM9` writer"]
pub struct W(crate::W<RAM9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM9_SPEC>;
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
impl From<crate::W<RAM9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D` reader - des D"]
pub type D_R = crate::FieldReader<u8, u8>;
#[doc = "Field `D` writer - des D"]
pub type D_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RAM9_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - des D"]
    #[inline(always)]
    pub fn d(&self) -> D_R {
        D_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - des D"]
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
#[doc = "des RAM9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram9](index.html) module"]
pub struct RAM9_SPEC;
impl crate::RegisterSpec for RAM9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram9::R](R) reader structure"]
impl crate::Readable for RAM9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram9::W](W) writer structure"]
impl crate::Writable for RAM9_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAM9 to value 0"]
impl crate::Resettable for RAM9_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
