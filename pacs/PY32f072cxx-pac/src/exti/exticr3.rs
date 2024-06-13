#[doc = "Register `EXTICR3` reader"]
pub struct R(crate::R<EXTICR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTICR3` writer"]
pub struct W(crate::W<EXTICR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR3_SPEC>;
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
impl From<crate::W<EXTICR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI8` reader - GPIO port selection"]
pub type EXTI8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI8` writer - GPIO port selection"]
pub type EXTI8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `EXTI9` reader - GPIO port selection"]
pub type EXTI9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI9` writer - GPIO port selection"]
pub type EXTI9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `EXTI10` reader - GPIO port selection"]
pub type EXTI10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI10` writer - GPIO port selection"]
pub type EXTI10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `EXTI11` reader - GPIO port selection"]
pub type EXTI11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI11` writer - GPIO port selection"]
pub type EXTI11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR3_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - GPIO port selection"]
    #[inline(always)]
    pub fn exti8(&self) -> EXTI8_R {
        EXTI8_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO port selection"]
    #[inline(always)]
    pub fn exti9(&self) -> EXTI9_R {
        EXTI9_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - GPIO port selection"]
    #[inline(always)]
    pub fn exti10(&self) -> EXTI10_R {
        EXTI10_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - GPIO port selection"]
    #[inline(always)]
    pub fn exti11(&self) -> EXTI11_R {
        EXTI11_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO port selection"]
    #[inline(always)]
    pub fn exti8(&mut self) -> EXTI8_W<0> {
        EXTI8_W::new(self)
    }
    #[doc = "Bits 8:9 - GPIO port selection"]
    #[inline(always)]
    pub fn exti9(&mut self) -> EXTI9_W<8> {
        EXTI9_W::new(self)
    }
    #[doc = "Bits 16:17 - GPIO port selection"]
    #[inline(always)]
    pub fn exti10(&mut self) -> EXTI10_W<16> {
        EXTI10_W::new(self)
    }
    #[doc = "Bits 24:25 - GPIO port selection"]
    #[inline(always)]
    pub fn exti11(&mut self) -> EXTI11_W<24> {
        EXTI11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI external interrupt selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr3](index.html) module"]
pub struct EXTICR3_SPEC;
impl crate::RegisterSpec for EXTICR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exticr3::R](R) reader structure"]
impl crate::Readable for EXTICR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exticr3::W](W) writer structure"]
impl crate::Writable for EXTICR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTICR3 to value 0"]
impl crate::Resettable for EXTICR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
