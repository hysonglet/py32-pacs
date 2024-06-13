#[doc = "Register `EXTICR4` reader"]
pub struct R(crate::R<EXTICR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTICR4` writer"]
pub struct W(crate::W<EXTICR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR4_SPEC>;
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
impl From<crate::W<EXTICR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI12` reader - GPIO port selection"]
pub type EXTI12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI12` writer - GPIO port selection"]
pub type EXTI12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, u8, 2, O>;
#[doc = "Field `EXTI13` reader - GPIO port selection"]
pub type EXTI13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI13` writer - GPIO port selection"]
pub type EXTI13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, u8, 2, O>;
#[doc = "Field `EXTI14` reader - GPIO port selection"]
pub type EXTI14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI14` writer - GPIO port selection"]
pub type EXTI14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, u8, 2, O>;
#[doc = "Field `EXTI15` reader - GPIO port selection"]
pub type EXTI15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI15` writer - GPIO port selection"]
pub type EXTI15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - GPIO port selection"]
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO port selection"]
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - GPIO port selection"]
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - GPIO port selection"]
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO port selection"]
    #[inline(always)]
    pub fn exti12(&mut self) -> EXTI12_W<0> {
        EXTI12_W::new(self)
    }
    #[doc = "Bits 8:9 - GPIO port selection"]
    #[inline(always)]
    pub fn exti13(&mut self) -> EXTI13_W<8> {
        EXTI13_W::new(self)
    }
    #[doc = "Bits 16:17 - GPIO port selection"]
    #[inline(always)]
    pub fn exti14(&mut self) -> EXTI14_W<16> {
        EXTI14_W::new(self)
    }
    #[doc = "Bits 24:25 - GPIO port selection"]
    #[inline(always)]
    pub fn exti15(&mut self) -> EXTI15_W<24> {
        EXTI15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI external interrupt selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr4](index.html) module"]
pub struct EXTICR4_SPEC;
impl crate::RegisterSpec for EXTICR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exticr4::R](R) reader structure"]
impl crate::Readable for EXTICR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exticr4::W](W) writer structure"]
impl crate::Writable for EXTICR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTICR4 to value 0"]
impl crate::Resettable for EXTICR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
