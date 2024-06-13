#[doc = "Register `STR1` reader"]
pub struct R(crate::R<STR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STR1` writer"]
pub struct W(crate::W<STR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STR1_SPEC>;
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
impl From<crate::W<STR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETTIME` reader - "]
pub type SETTIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SETTIME` writer - "]
pub type SETTIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STR1_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn settime(&self) -> SETTIME_R {
        SETTIME_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn settime(&mut self) -> SETTIME_W<0> {
        SETTIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stable Time register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [str1](index.html) module"]
pub struct STR1_SPEC;
impl crate::RegisterSpec for STR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [str1::R](R) reader structure"]
impl crate::Readable for STR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [str1::W](W) writer structure"]
impl crate::Writable for STR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STR1 to value 0"]
impl crate::Resettable for STR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
