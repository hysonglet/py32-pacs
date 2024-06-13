#[doc = "Register `SEQR0` reader"]
pub struct R(crate::R<SEQR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQR0` writer"]
pub struct W(crate::W<SEQR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQR0_SPEC>;
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
impl From<crate::W<SEQR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SQ0` reader - ADC sequence select"]
pub type SQ0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ0` writer - ADC sequence select"]
pub type SQ0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `SQ1` reader - ADC sequence select"]
pub type SQ1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ1` writer - ADC sequence select"]
pub type SQ1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `SQ2` reader - ADC sequence select"]
pub type SQ2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ2` writer - ADC sequence select"]
pub type SQ2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `SQ3` reader - ADC sequence select"]
pub type SQ3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ3` writer - ADC sequence select"]
pub type SQ3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `SQ4` reader - ADC sequence select"]
pub type SQ4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ4` writer - ADC sequence select"]
pub type SQ4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `SQ5` reader - ADC sequence select"]
pub type SQ5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ5` writer - ADC sequence select"]
pub type SQ5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `SQ6` reader - ADC sequence select"]
pub type SQ6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ6` writer - ADC sequence select"]
pub type SQ6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `SQ7` reader - ADC sequence select"]
pub type SQ7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ7` writer - ADC sequence select"]
pub type SQ7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQR0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - ADC sequence select"]
    #[inline(always)]
    pub fn sq0(&self) -> SQ0_R {
        SQ0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ADC sequence select"]
    #[inline(always)]
    pub fn sq1(&self) -> SQ1_R {
        SQ1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ADC sequence select"]
    #[inline(always)]
    pub fn sq2(&self) -> SQ2_R {
        SQ2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - ADC sequence select"]
    #[inline(always)]
    pub fn sq3(&self) -> SQ3_R {
        SQ3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - ADC sequence select"]
    #[inline(always)]
    pub fn sq4(&self) -> SQ4_R {
        SQ4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - ADC sequence select"]
    #[inline(always)]
    pub fn sq5(&self) -> SQ5_R {
        SQ5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ADC sequence select"]
    #[inline(always)]
    pub fn sq6(&self) -> SQ6_R {
        SQ6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - ADC sequence select"]
    #[inline(always)]
    pub fn sq7(&self) -> SQ7_R {
        SQ7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC sequence select"]
    #[inline(always)]
    pub fn sq0(&mut self) -> SQ0_W<0> {
        SQ0_W::new(self)
    }
    #[doc = "Bits 4:7 - ADC sequence select"]
    #[inline(always)]
    pub fn sq1(&mut self) -> SQ1_W<4> {
        SQ1_W::new(self)
    }
    #[doc = "Bits 8:11 - ADC sequence select"]
    #[inline(always)]
    pub fn sq2(&mut self) -> SQ2_W<8> {
        SQ2_W::new(self)
    }
    #[doc = "Bits 12:15 - ADC sequence select"]
    #[inline(always)]
    pub fn sq3(&mut self) -> SQ3_W<12> {
        SQ3_W::new(self)
    }
    #[doc = "Bits 16:19 - ADC sequence select"]
    #[inline(always)]
    pub fn sq4(&mut self) -> SQ4_W<16> {
        SQ4_W::new(self)
    }
    #[doc = "Bits 20:23 - ADC sequence select"]
    #[inline(always)]
    pub fn sq5(&mut self) -> SQ5_W<20> {
        SQ5_W::new(self)
    }
    #[doc = "Bits 24:27 - ADC sequence select"]
    #[inline(always)]
    pub fn sq6(&mut self) -> SQ6_W<24> {
        SQ6_W::new(self)
    }
    #[doc = "Bits 28:31 - ADC sequence select"]
    #[inline(always)]
    pub fn sq7(&mut self) -> SQ7_W<28> {
        SQ7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC sequence select register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seqr0](index.html) module"]
pub struct SEQR0_SPEC;
impl crate::RegisterSpec for SEQR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seqr0::R](R) reader structure"]
impl crate::Readable for SEQR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seqr0::W](W) writer structure"]
impl crate::Writable for SEQR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEQR0 to value 0"]
impl crate::Resettable for SEQR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
