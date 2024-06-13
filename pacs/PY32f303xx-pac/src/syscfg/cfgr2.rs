#[doc = "Register `CFGR2` reader"]
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR2` writer"]
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKUP_LOCK` reader - desc LOCKUP_LOCK"]
pub type LOCKUP_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCKUP_LOCK` writer - desc LOCKUP_LOCK"]
pub type LOCKUP_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `PVD_LOCK` reader - desc PVD_LOCK"]
pub type PVD_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `PVD_LOCK` writer - desc PVD_LOCK"]
pub type PVD_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `ADC1_ETRGINJ_REMAP` reader - desc ADC1_ETRGINJ_REMAP"]
pub type ADC1_ETRGINJ_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `ADC1_ETRGINJ_REMAP` writer - desc ADC1_ETRGINJ_REMAP"]
pub type ADC1_ETRGINJ_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `ADC1_ETRGREG_REMAP` reader - desc ADC1_ETRGREG_REMAP"]
pub type ADC1_ETRGREG_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `ADC1_ETRGREG_REMAP` writer - desc ADC1_ETRGREG_REMAP"]
pub type ADC1_ETRGREG_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `ADC2_ETRGINJ_REMAP` reader - desc ADC2_ETRGINJ_REMAP"]
pub type ADC2_ETRGINJ_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `ADC2_ETRGINJ_REMAP` writer - desc ADC2_ETRGINJ_REMAP"]
pub type ADC2_ETRGINJ_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `ADC2_ETRGREG_REMAP` reader - desc ADC2_ETRGREG_REMAP"]
pub type ADC2_ETRGREG_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `ADC2_ETRGREG_REMAP` writer - desc ADC2_ETRGREG_REMAP"]
pub type ADC2_ETRGREG_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc LOCKUP_LOCK"]
    #[inline(always)]
    pub fn lockup_lock(&self) -> LOCKUP_LOCK_R {
        LOCKUP_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - desc PVD_LOCK"]
    #[inline(always)]
    pub fn pvd_lock(&self) -> PVD_LOCK_R {
        PVD_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - desc ADC1_ETRGINJ_REMAP"]
    #[inline(always)]
    pub fn adc1_etrginj_remap(&self) -> ADC1_ETRGINJ_REMAP_R {
        ADC1_ETRGINJ_REMAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc ADC1_ETRGREG_REMAP"]
    #[inline(always)]
    pub fn adc1_etrgreg_remap(&self) -> ADC1_ETRGREG_REMAP_R {
        ADC1_ETRGREG_REMAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc ADC2_ETRGINJ_REMAP"]
    #[inline(always)]
    pub fn adc2_etrginj_remap(&self) -> ADC2_ETRGINJ_REMAP_R {
        ADC2_ETRGINJ_REMAP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc ADC2_ETRGREG_REMAP"]
    #[inline(always)]
    pub fn adc2_etrgreg_remap(&self) -> ADC2_ETRGREG_REMAP_R {
        ADC2_ETRGREG_REMAP_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc LOCKUP_LOCK"]
    #[inline(always)]
    pub fn lockup_lock(&mut self) -> LOCKUP_LOCK_W<0> {
        LOCKUP_LOCK_W::new(self)
    }
    #[doc = "Bit 2 - desc PVD_LOCK"]
    #[inline(always)]
    pub fn pvd_lock(&mut self) -> PVD_LOCK_W<2> {
        PVD_LOCK_W::new(self)
    }
    #[doc = "Bit 8 - desc ADC1_ETRGINJ_REMAP"]
    #[inline(always)]
    pub fn adc1_etrginj_remap(&mut self) -> ADC1_ETRGINJ_REMAP_W<8> {
        ADC1_ETRGINJ_REMAP_W::new(self)
    }
    #[doc = "Bit 9 - desc ADC1_ETRGREG_REMAP"]
    #[inline(always)]
    pub fn adc1_etrgreg_remap(&mut self) -> ADC1_ETRGREG_REMAP_W<9> {
        ADC1_ETRGREG_REMAP_W::new(self)
    }
    #[doc = "Bit 10 - desc ADC2_ETRGINJ_REMAP"]
    #[inline(always)]
    pub fn adc2_etrginj_remap(&mut self) -> ADC2_ETRGINJ_REMAP_W<10> {
        ADC2_ETRGINJ_REMAP_W::new(self)
    }
    #[doc = "Bit 11 - desc ADC2_ETRGREG_REMAP"]
    #[inline(always)]
    pub fn adc2_etrgreg_remap(&mut self) -> ADC2_ETRGREG_REMAP_W<11> {
        ADC2_ETRGREG_REMAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CFGR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr2::R](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR2 to value 0x04"]
impl crate::Resettable for CFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
