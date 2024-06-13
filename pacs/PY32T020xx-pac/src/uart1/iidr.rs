#[doc = "Register `IIDR` reader"]
pub struct R(crate::R<IIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IIDR` writer"]
pub struct W(crate::W<IIDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IIDR_SPEC>;
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
impl From<crate::W<IIDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IIDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IID` reader - "]
pub type IID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IID` writer - "]
pub type IID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IIDR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn iid(&self) -> IID_R {
        IID_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn iid(&mut self) -> IID_W<0> {
        IID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iidr](index.html) module"]
pub struct IIDR_SPEC;
impl crate::RegisterSpec for IIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iidr::R](R) reader structure"]
impl crate::Readable for IIDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iidr::W](W) writer structure"]
impl crate::Writable for IIDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IIDR to value 0"]
impl crate::Resettable for IIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
