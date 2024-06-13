#[doc = "Register `INTC` writer"]
pub struct W(crate::W<INTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTC_SPEC>;
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
impl From<crate::W<INTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKOKIC` writer - desc CKOKIC"]
pub type CKOKIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTC_SPEC, bool, O>;
#[doc = "Field `CKWARNIC` writer - desc CKWARNIC"]
pub type CKWARNIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTC_SPEC, bool, O>;
#[doc = "Field `ERRIC` writer - desc ERRIC"]
pub type ERRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTC_SPEC, bool, O>;
#[doc = "Field `EREFIC` writer - desc EREFIC"]
pub type EREFIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - desc CKOKIC"]
    #[inline(always)]
    pub fn ckokic(&mut self) -> CKOKIC_W<0> {
        CKOKIC_W::new(self)
    }
    #[doc = "Bit 1 - desc CKWARNIC"]
    #[inline(always)]
    pub fn ckwarnic(&mut self) -> CKWARNIC_W<1> {
        CKWARNIC_W::new(self)
    }
    #[doc = "Bit 2 - desc ERRIC"]
    #[inline(always)]
    pub fn erric(&mut self) -> ERRIC_W<2> {
        ERRIC_W::new(self)
    }
    #[doc = "Bit 3 - desc EREFIC"]
    #[inline(always)]
    pub fn erefic(&mut self) -> EREFIC_W<3> {
        EREFIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc INTC\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intc](index.html) module"]
pub struct INTC_SPEC;
impl crate::RegisterSpec for INTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [intc::W](W) writer structure"]
impl crate::Writable for INTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTC to value 0"]
impl crate::Resettable for INTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
