#[doc = "Register `LPDR` reader"]
pub struct R(crate::R<LPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPDR` writer"]
pub struct W(crate::W<LPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPDR_SPEC>;
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
impl From<crate::W<LPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NORMALDELTA` reader - "]
pub type NORMALDELTA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NORMALDELTA` writer - "]
pub type NORMALDELTA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPDR_SPEC, u16, u16, 10, O>;
#[doc = "Field `ABNORMALDELTA` reader - "]
pub type ABNORMALDELTA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ABNORMALDELTA` writer - "]
pub type ABNORMALDELTA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPDR_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn normaldelta(&self) -> NORMALDELTA_R {
        NORMALDELTA_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn abnormaldelta(&self) -> ABNORMALDELTA_R {
        ABNORMALDELTA_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn normaldelta(&mut self) -> NORMALDELTA_W<0> {
        NORMALDELTA_W::new(self)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn abnormaldelta(&mut self) -> ABNORMALDELTA_W<16> {
        ABNORMALDELTA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lowpower Delta register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpdr](index.html) module"]
pub struct LPDR_SPEC;
impl crate::RegisterSpec for LPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpdr::R](R) reader structure"]
impl crate::Readable for LPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpdr::W](W) writer structure"]
impl crate::Writable for LPDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPDR to value 0"]
impl crate::Resettable for LPDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
