#[doc = "Register `TTTR` reader"]
pub struct R(crate::R<TTTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTTR` writer"]
pub struct W(crate::W<TTTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTTR_SPEC>;
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
impl From<crate::W<TTTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TT_TRIG` reader - desc TT_TRIG"]
pub type TT_TRIG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TT_TRIG` writer - desc TT_TRIG"]
pub type TT_TRIG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTTR_SPEC, u16, u16, 16, O>;
#[doc = "Field `TT_WTRIG` reader - desc TT_WTRIG"]
pub type TT_WTRIG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TT_WTRIG` writer - desc TT_WTRIG"]
pub type TT_WTRIG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTTR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - desc TT_TRIG"]
    #[inline(always)]
    pub fn tt_trig(&self) -> TT_TRIG_R {
        TT_TRIG_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - desc TT_WTRIG"]
    #[inline(always)]
    pub fn tt_wtrig(&self) -> TT_WTRIG_R {
        TT_WTRIG_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc TT_TRIG"]
    #[inline(always)]
    pub fn tt_trig(&mut self) -> TT_TRIG_W<0> {
        TT_TRIG_W::new(self)
    }
    #[doc = "Bits 16:31 - desc TT_WTRIG"]
    #[inline(always)]
    pub fn tt_wtrig(&mut self) -> TT_WTRIG_W<16> {
        TT_WTRIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc TTTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tttr](index.html) module"]
pub struct TTTR_SPEC;
impl crate::RegisterSpec for TTTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tttr::R](R) reader structure"]
impl crate::Readable for TTTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tttr::W](W) writer structure"]
impl crate::Writable for TTTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TTTR to value 0xffff_0000"]
impl crate::Resettable for TTTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_0000
    }
}
