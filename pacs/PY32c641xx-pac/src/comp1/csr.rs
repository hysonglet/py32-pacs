#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - COMP enable bit"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - COMP enable bit"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `INNSEL` reader - desc INNSEL"]
pub type INNSEL_R = crate::BitReader<bool>;
#[doc = "Field `INNSEL` writer - desc INNSEL"]
pub type INNSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `WINMODE` reader - Comparator non-inverting input selector for window mode"]
pub type WINMODE_R = crate::BitReader<bool>;
#[doc = "Field `WINMODE` writer - Comparator non-inverting input selector for window mode"]
pub type WINMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `POLARITY` reader - Comparator polarity selector"]
pub type POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `POLARITY` writer - Comparator polarity selector"]
pub type POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `VCDIV` reader - VCDIV"]
pub type VCDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VCDIV` writer - VCDIV"]
pub type VCDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 4, O>;
#[doc = "Field `VCSEL` reader - VCSEL"]
pub type VCSEL_R = crate::BitReader<bool>;
#[doc = "Field `VCSEL` writer - VCSEL"]
pub type VCSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `COMP_OUT` reader - Comparator output status"]
pub type COMP_OUT_R = crate::BitReader<bool>;
#[doc = "Field `COMP_OUT` writer - Comparator output status"]
pub type COMP_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - COMP enable bit"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - desc INNSEL"]
    #[inline(always)]
    pub fn innsel(&self) -> INNSEL_R {
        INNSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - Comparator non-inverting input selector for window mode"]
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator polarity selector"]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 22:25 - VCDIV"]
    #[inline(always)]
    pub fn vcdiv(&self) -> VCDIV_R {
        VCDIV_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - VCSEL"]
    #[inline(always)]
    pub fn vcsel(&self) -> VCSEL_R {
        VCSEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - Comparator output status"]
    #[inline(always)]
    pub fn comp_out(&self) -> COMP_OUT_R {
        COMP_OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - COMP enable bit"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 5 - desc INNSEL"]
    #[inline(always)]
    pub fn innsel(&mut self) -> INNSEL_W<5> {
        INNSEL_W::new(self)
    }
    #[doc = "Bit 11 - Comparator non-inverting input selector for window mode"]
    #[inline(always)]
    pub fn winmode(&mut self) -> WINMODE_W<11> {
        WINMODE_W::new(self)
    }
    #[doc = "Bit 15 - Comparator polarity selector"]
    #[inline(always)]
    pub fn polarity(&mut self) -> POLARITY_W<15> {
        POLARITY_W::new(self)
    }
    #[doc = "Bits 22:25 - VCDIV"]
    #[inline(always)]
    pub fn vcdiv(&mut self) -> VCDIV_W<22> {
        VCDIV_W::new(self)
    }
    #[doc = "Bit 27 - VCSEL"]
    #[inline(always)]
    pub fn vcsel(&mut self) -> VCSEL_W<27> {
        VCSEL_W::new(self)
    }
    #[doc = "Bit 30 - Comparator output status"]
    #[inline(always)]
    pub fn comp_out(&mut self) -> COMP_OUT_W<30> {
        COMP_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "COMP control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
