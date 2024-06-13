#[doc = "Register `SMPR1` reader"]
pub struct R(crate::R<SMPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPR1` writer"]
pub struct W(crate::W<SMPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR1_SPEC>;
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
impl From<crate::W<SMPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMP20` reader - desc SMP20"]
pub type SMP20_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP20` writer - desc SMP20"]
pub type SMP20_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP21` reader - desc SMP21"]
pub type SMP21_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP21` writer - desc SMP21"]
pub type SMP21_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP22` reader - desc SMP22"]
pub type SMP22_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP22` writer - desc SMP22"]
pub type SMP22_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP23` reader - desc SMP23"]
pub type SMP23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP23` writer - desc SMP23"]
pub type SMP23_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR1_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - desc SMP20"]
    #[inline(always)]
    pub fn smp20(&self) -> SMP20_R {
        SMP20_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - desc SMP21"]
    #[inline(always)]
    pub fn smp21(&self) -> SMP21_R {
        SMP21_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - desc SMP22"]
    #[inline(always)]
    pub fn smp22(&self) -> SMP22_R {
        SMP22_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - desc SMP23"]
    #[inline(always)]
    pub fn smp23(&self) -> SMP23_R {
        SMP23_R::new(((self.bits >> 9) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc SMP20"]
    #[inline(always)]
    pub fn smp20(&mut self) -> SMP20_W<0> {
        SMP20_W::new(self)
    }
    #[doc = "Bits 3:5 - desc SMP21"]
    #[inline(always)]
    pub fn smp21(&mut self) -> SMP21_W<3> {
        SMP21_W::new(self)
    }
    #[doc = "Bits 6:8 - desc SMP22"]
    #[inline(always)]
    pub fn smp22(&mut self) -> SMP22_W<6> {
        SMP22_W::new(self)
    }
    #[doc = "Bits 9:11 - desc SMP23"]
    #[inline(always)]
    pub fn smp23(&mut self) -> SMP23_W<9> {
        SMP23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc SMPR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr1](index.html) module"]
pub struct SMPR1_SPEC;
impl crate::RegisterSpec for SMPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpr1::R](R) reader structure"]
impl crate::Readable for SMPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpr1::W](W) writer structure"]
impl crate::Writable for SMPR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMPR1 to value 0"]
impl crate::Resettable for SMPR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
