#[doc = "Register `STR2` reader"]
pub struct R(crate::R<STR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STR2` writer"]
pub struct W(crate::W<STR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STR2_SPEC>;
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
impl From<crate::W<STR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VREFTIME` reader - "]
pub type VREFTIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREFTIME` writer - "]
pub type VREFTIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `SETVTTIME` reader - "]
pub type SETVTTIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SETVTTIME` writer - "]
pub type SETVTTIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STR2_SPEC, u16, u16, 10, O>;
#[doc = "Field `SIDACTIME` reader - "]
pub type SIDACTIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIDACTIME` writer - "]
pub type SIDACTIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `DISCHSWTIME` reader - "]
pub type DISCHSWTIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DISCHSWTIME` writer - "]
pub type DISCHSWTIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STR2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn vreftime(&self) -> VREFTIME_R {
        VREFTIME_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:14"]
    #[inline(always)]
    pub fn setvttime(&self) -> SETVTTIME_R {
        SETVTTIME_R::new(((self.bits >> 5) & 0x03ff) as u16)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn sidactime(&self) -> SIDACTIME_R {
        SIDACTIME_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn dischswtime(&self) -> DISCHSWTIME_R {
        DISCHSWTIME_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn vreftime(&mut self) -> VREFTIME_W<0> {
        VREFTIME_W::new(self)
    }
    #[doc = "Bits 5:14"]
    #[inline(always)]
    pub fn setvttime(&mut self) -> SETVTTIME_W<5> {
        SETVTTIME_W::new(self)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn sidactime(&mut self) -> SIDACTIME_W<16> {
        SIDACTIME_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn dischswtime(&mut self) -> DISCHSWTIME_W<24> {
        DISCHSWTIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stable Time register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [str2](index.html) module"]
pub struct STR2_SPEC;
impl crate::RegisterSpec for STR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [str2::R](R) reader structure"]
impl crate::Readable for STR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [str2::W](W) writer structure"]
impl crate::Writable for STR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STR2 to value 0"]
impl crate::Resettable for STR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
