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
#[doc = "Field `INMSEL` reader - Comparator signal selector for inverting input INM"]
pub type INMSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INMSEL` writer - Comparator signal selector for inverting input INM"]
pub type INMSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 4, O>;
#[doc = "Field `INPSEL` reader - Comparator signal selector for non-inverting input"]
pub type INPSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INPSEL` writer - Comparator signal selector for non-inverting input"]
pub type INPSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 4, O>;
#[doc = "Field `WINMODE` reader - Comparator non-inverting input selector for window mode"]
pub type WINMODE_R = crate::BitReader<bool>;
#[doc = "Field `WINMODE` writer - Comparator non-inverting input selector for window mode"]
pub type WINMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `POLARITY` reader - Comparator polarity selector"]
pub type POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `POLARITY` writer - Comparator polarity selector"]
pub type POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `HYST` reader - Comparator hysteresis enable selector"]
pub type HYST_R = crate::BitReader<bool>;
#[doc = "Field `HYST` writer - Comparator hysteresis enable selector"]
pub type HYST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `PWRMODE` reader - Comparator power mode selector"]
pub type PWRMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWRMODE` writer - Comparator power mode selector"]
pub type PWRMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `VCDIV` reader - VCDIV"]
pub type VCDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VCDIV` writer - VCDIV"]
pub type VCDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 6, O>;
#[doc = "Field `COMP_VCSEL` reader - COMP_VCSEL"]
pub type COMP_VCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP_VCSEL` writer - COMP_VCSEL"]
pub type COMP_VCSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 2, O>;
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
    #[doc = "Bits 2:5 - Comparator signal selector for inverting input INM"]
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - Comparator signal selector for non-inverting input"]
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 6) & 0x0f) as u8)
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
    #[doc = "Bit 16 - Comparator hysteresis enable selector"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Comparator power mode selector"]
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:25 - VCDIV"]
    #[inline(always)]
    pub fn vcdiv(&self) -> VCDIV_R {
        VCDIV_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 26:27 - COMP_VCSEL"]
    #[inline(always)]
    pub fn comp_vcsel(&self) -> COMP_VCSEL_R {
        COMP_VCSEL_R::new(((self.bits >> 26) & 3) as u8)
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
    #[doc = "Bits 2:5 - Comparator signal selector for inverting input INM"]
    #[inline(always)]
    pub fn inmsel(&mut self) -> INMSEL_W<2> {
        INMSEL_W::new(self)
    }
    #[doc = "Bits 6:9 - Comparator signal selector for non-inverting input"]
    #[inline(always)]
    pub fn inpsel(&mut self) -> INPSEL_W<6> {
        INPSEL_W::new(self)
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
    #[doc = "Bit 16 - Comparator hysteresis enable selector"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W<16> {
        HYST_W::new(self)
    }
    #[doc = "Bits 18:19 - Comparator power mode selector"]
    #[inline(always)]
    pub fn pwrmode(&mut self) -> PWRMODE_W<18> {
        PWRMODE_W::new(self)
    }
    #[doc = "Bits 20:25 - VCDIV"]
    #[inline(always)]
    pub fn vcdiv(&mut self) -> VCDIV_W<20> {
        VCDIV_W::new(self)
    }
    #[doc = "Bits 26:27 - COMP_VCSEL"]
    #[inline(always)]
    pub fn comp_vcsel(&mut self) -> COMP_VCSEL_W<26> {
        COMP_VCSEL_W::new(self)
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
