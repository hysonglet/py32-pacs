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
#[doc = "Field `COMP1_BRK_TIM1` reader - desc COMP1_BRK_TIM1"]
pub type COMP1_BRK_TIM1_R = crate::BitReader<bool>;
#[doc = "Field `COMP1_BRK_TIM1` writer - desc COMP1_BRK_TIM1"]
pub type COMP1_BRK_TIM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `COMP2_BRK_TIM1` reader - desc COMP2_BRK_TIM1"]
pub type COMP2_BRK_TIM1_R = crate::BitReader<bool>;
#[doc = "Field `COMP2_BRK_TIM1` writer - desc COMP2_BRK_TIM1"]
pub type COMP2_BRK_TIM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `COMP1_BRK_TIM15` reader - desc COMP1_BRK_TIM15"]
pub type COMP1_BRK_TIM15_R = crate::BitReader<bool>;
#[doc = "Field `COMP1_BRK_TIM15` writer - desc COMP1_BRK_TIM15"]
pub type COMP1_BRK_TIM15_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `COMP2_BRK_TIM15` reader - desc COMP2_BRK_TIM15"]
pub type COMP2_BRK_TIM15_R = crate::BitReader<bool>;
#[doc = "Field `COMP2_BRK_TIM15` writer - desc COMP2_BRK_TIM15"]
pub type COMP2_BRK_TIM15_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `COMP1_BRK_TIM16` reader - desc COMP1_BRK_TIM16"]
pub type COMP1_BRK_TIM16_R = crate::BitReader<bool>;
#[doc = "Field `COMP1_BRK_TIM16` writer - desc COMP1_BRK_TIM16"]
pub type COMP1_BRK_TIM16_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `COMP2_BRK_TIM16` reader - desc COMP2_BRK_TIM16"]
pub type COMP2_BRK_TIM16_R = crate::BitReader<bool>;
#[doc = "Field `COMP2_BRK_TIM16` writer - desc COMP2_BRK_TIM16"]
pub type COMP2_BRK_TIM16_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `COMP1_BRK_TIM17` reader - desc COMP1_BRK_TIM17"]
pub type COMP1_BRK_TIM17_R = crate::BitReader<bool>;
#[doc = "Field `COMP1_BRK_TIM17` writer - desc COMP1_BRK_TIM17"]
pub type COMP1_BRK_TIM17_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `COMP2_BRK_TIM17` reader - desc COMP2_BRK_TIM17"]
pub type COMP2_BRK_TIM17_R = crate::BitReader<bool>;
#[doc = "Field `COMP2_BRK_TIM17` writer - desc COMP2_BRK_TIM17"]
pub type COMP2_BRK_TIM17_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `COMP1_OCREF_CLR_TIM1` reader - desc COMP1_OCREF_CLR_TIM1"]
pub type COMP1_OCREF_CLR_TIM1_R = crate::BitReader<bool>;
#[doc = "Field `COMP1_OCREF_CLR_TIM1` writer - desc COMP1_OCREF_CLR_TIM1"]
pub type COMP1_OCREF_CLR_TIM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `COMP1_OCREF_CLR_TIM2` reader - desc COMP1_OCREF_CLR_TIM2"]
pub type COMP1_OCREF_CLR_TIM2_R = crate::BitReader<bool>;
#[doc = "Field `COMP1_OCREF_CLR_TIM2` writer - desc COMP1_OCREF_CLR_TIM2"]
pub type COMP1_OCREF_CLR_TIM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `COMP1_OCREF_CLR_TIM3` reader - desc COMP1_OCREF_CLR_TIM3"]
pub type COMP1_OCREF_CLR_TIM3_R = crate::BitReader<bool>;
#[doc = "Field `COMP1_OCREF_CLR_TIM3` writer - desc COMP1_OCREF_CLR_TIM3"]
pub type COMP1_OCREF_CLR_TIM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `COMP2_OCREF_CLR_TIM1` reader - desc COMP2_OCREF_CLR_TIM1"]
pub type COMP2_OCREF_CLR_TIM1_R = crate::BitReader<bool>;
#[doc = "Field `COMP2_OCREF_CLR_TIM1` writer - desc COMP2_OCREF_CLR_TIM1"]
pub type COMP2_OCREF_CLR_TIM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `COMP2_OCREF_CLR_TIM2` reader - desc COMP2_OCREF_CLR_TIM2"]
pub type COMP2_OCREF_CLR_TIM2_R = crate::BitReader<bool>;
#[doc = "Field `COMP2_OCREF_CLR_TIM2` writer - desc COMP2_OCREF_CLR_TIM2"]
pub type COMP2_OCREF_CLR_TIM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `COMP2_OCREF_CLR_TIM3` reader - desc COMP2_OCREF_CLR_TIM3"]
pub type COMP2_OCREF_CLR_TIM3_R = crate::BitReader<bool>;
#[doc = "Field `COMP2_OCREF_CLR_TIM3` writer - desc COMP2_OCREF_CLR_TIM3"]
pub type COMP2_OCREF_CLR_TIM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
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
    #[doc = "Bit 3 - desc COMP1_BRK_TIM1"]
    #[inline(always)]
    pub fn comp1_brk_tim1(&self) -> COMP1_BRK_TIM1_R {
        COMP1_BRK_TIM1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc COMP2_BRK_TIM1"]
    #[inline(always)]
    pub fn comp2_brk_tim1(&self) -> COMP2_BRK_TIM1_R {
        COMP2_BRK_TIM1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - desc COMP1_BRK_TIM15"]
    #[inline(always)]
    pub fn comp1_brk_tim15(&self) -> COMP1_BRK_TIM15_R {
        COMP1_BRK_TIM15_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc COMP2_BRK_TIM15"]
    #[inline(always)]
    pub fn comp2_brk_tim15(&self) -> COMP2_BRK_TIM15_R {
        COMP2_BRK_TIM15_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - desc COMP1_BRK_TIM16"]
    #[inline(always)]
    pub fn comp1_brk_tim16(&self) -> COMP1_BRK_TIM16_R {
        COMP1_BRK_TIM16_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc COMP2_BRK_TIM16"]
    #[inline(always)]
    pub fn comp2_brk_tim16(&self) -> COMP2_BRK_TIM16_R {
        COMP2_BRK_TIM16_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - desc COMP1_BRK_TIM17"]
    #[inline(always)]
    pub fn comp1_brk_tim17(&self) -> COMP1_BRK_TIM17_R {
        COMP1_BRK_TIM17_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc COMP2_BRK_TIM17"]
    #[inline(always)]
    pub fn comp2_brk_tim17(&self) -> COMP2_BRK_TIM17_R {
        COMP2_BRK_TIM17_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - desc COMP1_OCREF_CLR_TIM1"]
    #[inline(always)]
    pub fn comp1_ocref_clr_tim1(&self) -> COMP1_OCREF_CLR_TIM1_R {
        COMP1_OCREF_CLR_TIM1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc COMP1_OCREF_CLR_TIM2"]
    #[inline(always)]
    pub fn comp1_ocref_clr_tim2(&self) -> COMP1_OCREF_CLR_TIM2_R {
        COMP1_OCREF_CLR_TIM2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc COMP1_OCREF_CLR_TIM3"]
    #[inline(always)]
    pub fn comp1_ocref_clr_tim3(&self) -> COMP1_OCREF_CLR_TIM3_R {
        COMP1_OCREF_CLR_TIM3_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc COMP2_OCREF_CLR_TIM1"]
    #[inline(always)]
    pub fn comp2_ocref_clr_tim1(&self) -> COMP2_OCREF_CLR_TIM1_R {
        COMP2_OCREF_CLR_TIM1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc COMP2_OCREF_CLR_TIM2"]
    #[inline(always)]
    pub fn comp2_ocref_clr_tim2(&self) -> COMP2_OCREF_CLR_TIM2_R {
        COMP2_OCREF_CLR_TIM2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc COMP2_OCREF_CLR_TIM3"]
    #[inline(always)]
    pub fn comp2_ocref_clr_tim3(&self) -> COMP2_OCREF_CLR_TIM3_R {
        COMP2_OCREF_CLR_TIM3_R::new(((self.bits >> 20) & 1) != 0)
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
    #[doc = "Bit 3 - desc COMP1_BRK_TIM1"]
    #[inline(always)]
    pub fn comp1_brk_tim1(&mut self) -> COMP1_BRK_TIM1_W<3> {
        COMP1_BRK_TIM1_W::new(self)
    }
    #[doc = "Bit 4 - desc COMP2_BRK_TIM1"]
    #[inline(always)]
    pub fn comp2_brk_tim1(&mut self) -> COMP2_BRK_TIM1_W<4> {
        COMP2_BRK_TIM1_W::new(self)
    }
    #[doc = "Bit 6 - desc COMP1_BRK_TIM15"]
    #[inline(always)]
    pub fn comp1_brk_tim15(&mut self) -> COMP1_BRK_TIM15_W<6> {
        COMP1_BRK_TIM15_W::new(self)
    }
    #[doc = "Bit 7 - desc COMP2_BRK_TIM15"]
    #[inline(always)]
    pub fn comp2_brk_tim15(&mut self) -> COMP2_BRK_TIM15_W<7> {
        COMP2_BRK_TIM15_W::new(self)
    }
    #[doc = "Bit 9 - desc COMP1_BRK_TIM16"]
    #[inline(always)]
    pub fn comp1_brk_tim16(&mut self) -> COMP1_BRK_TIM16_W<9> {
        COMP1_BRK_TIM16_W::new(self)
    }
    #[doc = "Bit 10 - desc COMP2_BRK_TIM16"]
    #[inline(always)]
    pub fn comp2_brk_tim16(&mut self) -> COMP2_BRK_TIM16_W<10> {
        COMP2_BRK_TIM16_W::new(self)
    }
    #[doc = "Bit 12 - desc COMP1_BRK_TIM17"]
    #[inline(always)]
    pub fn comp1_brk_tim17(&mut self) -> COMP1_BRK_TIM17_W<12> {
        COMP1_BRK_TIM17_W::new(self)
    }
    #[doc = "Bit 13 - desc COMP2_BRK_TIM17"]
    #[inline(always)]
    pub fn comp2_brk_tim17(&mut self) -> COMP2_BRK_TIM17_W<13> {
        COMP2_BRK_TIM17_W::new(self)
    }
    #[doc = "Bit 15 - desc COMP1_OCREF_CLR_TIM1"]
    #[inline(always)]
    pub fn comp1_ocref_clr_tim1(&mut self) -> COMP1_OCREF_CLR_TIM1_W<15> {
        COMP1_OCREF_CLR_TIM1_W::new(self)
    }
    #[doc = "Bit 16 - desc COMP1_OCREF_CLR_TIM2"]
    #[inline(always)]
    pub fn comp1_ocref_clr_tim2(&mut self) -> COMP1_OCREF_CLR_TIM2_W<16> {
        COMP1_OCREF_CLR_TIM2_W::new(self)
    }
    #[doc = "Bit 17 - desc COMP1_OCREF_CLR_TIM3"]
    #[inline(always)]
    pub fn comp1_ocref_clr_tim3(&mut self) -> COMP1_OCREF_CLR_TIM3_W<17> {
        COMP1_OCREF_CLR_TIM3_W::new(self)
    }
    #[doc = "Bit 18 - desc COMP2_OCREF_CLR_TIM1"]
    #[inline(always)]
    pub fn comp2_ocref_clr_tim1(&mut self) -> COMP2_OCREF_CLR_TIM1_W<18> {
        COMP2_OCREF_CLR_TIM1_W::new(self)
    }
    #[doc = "Bit 19 - desc COMP2_OCREF_CLR_TIM2"]
    #[inline(always)]
    pub fn comp2_ocref_clr_tim2(&mut self) -> COMP2_OCREF_CLR_TIM2_W<19> {
        COMP2_OCREF_CLR_TIM2_W::new(self)
    }
    #[doc = "Bit 20 - desc COMP2_OCREF_CLR_TIM3"]
    #[inline(always)]
    pub fn comp2_ocref_clr_tim3(&mut self) -> COMP2_OCREF_CLR_TIM3_W<20> {
        COMP2_OCREF_CLR_TIM3_W::new(self)
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
