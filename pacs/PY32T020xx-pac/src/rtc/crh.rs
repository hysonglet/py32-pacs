#[doc = "Register `CRH` reader"]
pub struct R(crate::R<CRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRH` writer"]
pub struct W(crate::W<CRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRH_SPEC>;
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
impl From<crate::W<CRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECIE` reader - Second interrupt Enable"]
pub type SECIE_R = crate::BitReader<bool>;
#[doc = "Field `SECIE` writer - Second interrupt Enable"]
pub type SECIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRH_SPEC, bool, O>;
#[doc = "Field `ALRIE` reader - Alarm interrupt Enable"]
pub type ALRIE_R = crate::BitReader<bool>;
#[doc = "Field `ALRIE` writer - Alarm interrupt Enable"]
pub type ALRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRH_SPEC, bool, O>;
#[doc = "Field `OWIE` reader - Overflow interrupt Enable"]
pub type OWIE_R = crate::BitReader<bool>;
#[doc = "Field `OWIE` writer - Overflow interrupt Enable"]
pub type OWIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRH_SPEC, bool, O>;
#[doc = "Field `TK_SECIE` reader - "]
pub type TK_SECIE_R = crate::BitReader<bool>;
#[doc = "Field `TK_SECIE` writer - "]
pub type TK_SECIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRH_SPEC, bool, O>;
#[doc = "Field `TK_ALRIE` reader - "]
pub type TK_ALRIE_R = crate::BitReader<bool>;
#[doc = "Field `TK_ALRIE` writer - "]
pub type TK_ALRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRH_SPEC, bool, O>;
#[doc = "Field `TK_OWIE` reader - "]
pub type TK_OWIE_R = crate::BitReader<bool>;
#[doc = "Field `TK_OWIE` writer - "]
pub type TK_OWIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRH_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Second interrupt Enable"]
    #[inline(always)]
    pub fn secie(&self) -> SECIE_R {
        SECIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm interrupt Enable"]
    #[inline(always)]
    pub fn alrie(&self) -> ALRIE_R {
        ALRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow interrupt Enable"]
    #[inline(always)]
    pub fn owie(&self) -> OWIE_R {
        OWIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tk_secie(&self) -> TK_SECIE_R {
        TK_SECIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tk_alrie(&self) -> TK_ALRIE_R {
        TK_ALRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tk_owie(&self) -> TK_OWIE_R {
        TK_OWIE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Second interrupt Enable"]
    #[inline(always)]
    pub fn secie(&mut self) -> SECIE_W<0> {
        SECIE_W::new(self)
    }
    #[doc = "Bit 1 - Alarm interrupt Enable"]
    #[inline(always)]
    pub fn alrie(&mut self) -> ALRIE_W<1> {
        ALRIE_W::new(self)
    }
    #[doc = "Bit 2 - Overflow interrupt Enable"]
    #[inline(always)]
    pub fn owie(&mut self) -> OWIE_W<2> {
        OWIE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tk_secie(&mut self) -> TK_SECIE_W<3> {
        TK_SECIE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tk_alrie(&mut self) -> TK_ALRIE_W<4> {
        TK_ALRIE_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tk_owie(&mut self) -> TK_OWIE_W<5> {
        TK_OWIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Control Register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crh](index.html) module"]
pub struct CRH_SPEC;
impl crate::RegisterSpec for CRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crh::R](R) reader structure"]
impl crate::Readable for CRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crh::W](W) writer structure"]
impl crate::Writable for CRH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRH to value 0"]
impl crate::Resettable for CRH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
