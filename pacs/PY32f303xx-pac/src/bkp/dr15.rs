#[doc = "Register `DR15` reader"]
pub struct R(crate::R<DR15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DR15` writer"]
pub struct W(crate::W<DR15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR15_SPEC>;
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
impl From<crate::W<DR15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D` reader - desc D"]
pub type D_R = crate::FieldReader<u16, u16>;
#[doc = "Field `D` writer - desc D"]
pub type D_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR15_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - desc D"]
    #[inline(always)]
    pub fn d(&self) -> D_R {
        D_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc D"]
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
#[doc = "desc DR15\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr15](index.html) module"]
pub struct DR15_SPEC;
impl crate::RegisterSpec for DR15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr15::R](R) reader structure"]
impl crate::Readable for DR15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dr15::W](W) writer structure"]
impl crate::Writable for DR15_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DR15 to value 0"]
impl crate::Resettable for DR15_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
