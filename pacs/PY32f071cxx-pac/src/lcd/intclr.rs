#[doc = "Register `INTCLR` reader"]
pub struct R(crate::R<INTCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTCLR` writer"]
pub struct W(crate::W<INTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTCLR_SPEC>;
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
impl From<crate::W<INTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTF_CLR` reader - INTF_CLR"]
pub type INTF_CLR_R = crate::BitReader<bool>;
#[doc = "Field `INTF_CLR` writer - INTF_CLR"]
pub type INTF_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTCLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 10 - INTF_CLR"]
    #[inline(always)]
    pub fn intf_clr(&self) -> INTF_CLR_R {
        INTF_CLR_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - INTF_CLR"]
    #[inline(always)]
    pub fn intf_clr(&mut self) -> INTF_CLR_W<10> {
        INTF_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INTCLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intclr](index.html) module"]
pub struct INTCLR_SPEC;
impl crate::RegisterSpec for INTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intclr::R](R) reader structure"]
impl crate::Readable for INTCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intclr::W](W) writer structure"]
impl crate::Writable for INTCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTCLR to value 0"]
impl crate::Resettable for INTCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
