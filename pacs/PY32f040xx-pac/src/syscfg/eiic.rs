#[doc = "Register `EIIC` reader"]
pub struct R(crate::R<EIIC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EIIC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EIIC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EIIC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EIIC` writer"]
pub struct W(crate::W<EIIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EIIC_SPEC>;
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
impl From<crate::W<EIIC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EIIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA_EIIC` reader - desc PA_EIIC"]
pub type PA_EIIC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PA_EIIC` writer - desc PA_EIIC"]
pub type PA_EIIC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EIIC_SPEC, u8, u8, 2, O>;
#[doc = "Field `PB_EIIC` reader - desc PB_EIIC"]
pub type PB_EIIC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PB_EIIC` writer - desc PB_EIIC"]
pub type PB_EIIC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EIIC_SPEC, u16, u16, 9, O>;
#[doc = "Field `PF_EIIC` reader - desc PF_EIIC"]
pub type PF_EIIC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PF_EIIC` writer - desc PF_EIIC"]
pub type PF_EIIC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EIIC_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - desc PA_EIIC"]
    #[inline(always)]
    pub fn pa_eiic(&self) -> PA_EIIC_R {
        PA_EIIC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:16 - desc PB_EIIC"]
    #[inline(always)]
    pub fn pb_eiic(&self) -> PB_EIIC_R {
        PB_EIIC_R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bits 24:25 - desc PF_EIIC"]
    #[inline(always)]
    pub fn pf_eiic(&self) -> PF_EIIC_R {
        PF_EIIC_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc PA_EIIC"]
    #[inline(always)]
    pub fn pa_eiic(&mut self) -> PA_EIIC_W<0> {
        PA_EIIC_W::new(self)
    }
    #[doc = "Bits 8:16 - desc PB_EIIC"]
    #[inline(always)]
    pub fn pb_eiic(&mut self) -> PB_EIIC_W<8> {
        PB_EIIC_W::new(self)
    }
    #[doc = "Bits 24:25 - desc PF_EIIC"]
    #[inline(always)]
    pub fn pf_eiic(&mut self) -> PF_EIIC_W<24> {
        PF_EIIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc EIIC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eiic](index.html) module"]
pub struct EIIC_SPEC;
impl crate::RegisterSpec for EIIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eiic::R](R) reader structure"]
impl crate::Readable for EIIC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eiic::W](W) writer structure"]
impl crate::Writable for EIIC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EIIC to value 0"]
impl crate::Resettable for EIIC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
