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
#[doc = "Field `LOCKUP_LOCK` reader - Cortex-M0+ LOCKUP bit enable bit"]
pub type LOCKUP_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCKUP_LOCK` writer - Cortex-M0+ LOCKUP bit enable bit"]
pub type LOCKUP_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `PVD_LOCK` reader - PVD lock enable bit"]
pub type PVD_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `PVD_LOCK` writer - PVD lock enable bit"]
pub type PVD_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `COMP1_BRK_TIM1` reader - COMP1 is enable to input of TIM1 break"]
pub type COMP1_BRK_TIM1_R = crate::BitReader<bool>;
#[doc = "Field `COMP1_BRK_TIM1` writer - COMP1 is enable to input of TIM1 break"]
pub type COMP1_BRK_TIM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `COMP2_BRK_TIM1` reader - COMP2 is enable to input of TIM1 break"]
pub type COMP2_BRK_TIM1_R = crate::BitReader<bool>;
#[doc = "Field `COMP2_BRK_TIM1` writer - COMP2 is enable to input of TIM1 break"]
pub type COMP2_BRK_TIM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `COMP1_BRK_TIM16` reader - COMP1 is enable to input of TIM16 break"]
pub type COMP1_BRK_TIM16_R = crate::BitReader<bool>;
#[doc = "Field `COMP1_BRK_TIM16` writer - COMP1 is enable to input of TIM16 break"]
pub type COMP1_BRK_TIM16_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `COMP2_BRK_TIM16` reader - COMP2 is enable to input of TIM16 break"]
pub type COMP2_BRK_TIM16_R = crate::BitReader<bool>;
#[doc = "Field `COMP2_BRK_TIM16` writer - COMP2 is enable to input of TIM16 break"]
pub type COMP2_BRK_TIM16_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `COMP1_BRK_TIM17` reader - COMP1 is enable to input of TIM17 break"]
pub type COMP1_BRK_TIM17_R = crate::BitReader<bool>;
#[doc = "Field `COMP1_BRK_TIM17` writer - COMP1 is enable to input of TIM17 break"]
pub type COMP1_BRK_TIM17_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `COMP2_BRK_TIM17` reader - COMP2 is enable to input of TIM17 break"]
pub type COMP2_BRK_TIM17_R = crate::BitReader<bool>;
#[doc = "Field `COMP2_BRK_TIM17` writer - COMP2 is enable to input of TIM17 break"]
pub type COMP2_BRK_TIM17_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `ETR_SRC_TIM1` reader - TIM1 ETR source selection"]
pub type ETR_SRC_TIM1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETR_SRC_TIM1` writer - TIM1 ETR source selection"]
pub type ETR_SRC_TIM1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Cortex-M0+ LOCKUP bit enable bit"]
    #[inline(always)]
    pub fn lockup_lock(&self) -> LOCKUP_LOCK_R {
        LOCKUP_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvd_lock(&self) -> PVD_LOCK_R {
        PVD_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - COMP1 is enable to input of TIM1 break"]
    #[inline(always)]
    pub fn comp1_brk_tim1(&self) -> COMP1_BRK_TIM1_R {
        COMP1_BRK_TIM1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - COMP2 is enable to input of TIM1 break"]
    #[inline(always)]
    pub fn comp2_brk_tim1(&self) -> COMP2_BRK_TIM1_R {
        COMP2_BRK_TIM1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COMP1 is enable to input of TIM16 break"]
    #[inline(always)]
    pub fn comp1_brk_tim16(&self) -> COMP1_BRK_TIM16_R {
        COMP1_BRK_TIM16_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - COMP2 is enable to input of TIM16 break"]
    #[inline(always)]
    pub fn comp2_brk_tim16(&self) -> COMP2_BRK_TIM16_R {
        COMP2_BRK_TIM16_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - COMP1 is enable to input of TIM17 break"]
    #[inline(always)]
    pub fn comp1_brk_tim17(&self) -> COMP1_BRK_TIM17_R {
        COMP1_BRK_TIM17_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - COMP2 is enable to input of TIM17 break"]
    #[inline(always)]
    pub fn comp2_brk_tim17(&self) -> COMP2_BRK_TIM17_R {
        COMP2_BRK_TIM17_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - TIM1 ETR source selection"]
    #[inline(always)]
    pub fn etr_src_tim1(&self) -> ETR_SRC_TIM1_R {
        ETR_SRC_TIM1_R::new(((self.bits >> 9) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Cortex-M0+ LOCKUP bit enable bit"]
    #[inline(always)]
    pub fn lockup_lock(&mut self) -> LOCKUP_LOCK_W<0> {
        LOCKUP_LOCK_W::new(self)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvd_lock(&mut self) -> PVD_LOCK_W<2> {
        PVD_LOCK_W::new(self)
    }
    #[doc = "Bit 3 - COMP1 is enable to input of TIM1 break"]
    #[inline(always)]
    pub fn comp1_brk_tim1(&mut self) -> COMP1_BRK_TIM1_W<3> {
        COMP1_BRK_TIM1_W::new(self)
    }
    #[doc = "Bit 4 - COMP2 is enable to input of TIM1 break"]
    #[inline(always)]
    pub fn comp2_brk_tim1(&mut self) -> COMP2_BRK_TIM1_W<4> {
        COMP2_BRK_TIM1_W::new(self)
    }
    #[doc = "Bit 5 - COMP1 is enable to input of TIM16 break"]
    #[inline(always)]
    pub fn comp1_brk_tim16(&mut self) -> COMP1_BRK_TIM16_W<5> {
        COMP1_BRK_TIM16_W::new(self)
    }
    #[doc = "Bit 6 - COMP2 is enable to input of TIM16 break"]
    #[inline(always)]
    pub fn comp2_brk_tim16(&mut self) -> COMP2_BRK_TIM16_W<6> {
        COMP2_BRK_TIM16_W::new(self)
    }
    #[doc = "Bit 7 - COMP1 is enable to input of TIM17 break"]
    #[inline(always)]
    pub fn comp1_brk_tim17(&mut self) -> COMP1_BRK_TIM17_W<7> {
        COMP1_BRK_TIM17_W::new(self)
    }
    #[doc = "Bit 8 - COMP2 is enable to input of TIM17 break"]
    #[inline(always)]
    pub fn comp2_brk_tim17(&mut self) -> COMP2_BRK_TIM17_W<8> {
        COMP2_BRK_TIM17_W::new(self)
    }
    #[doc = "Bits 9:10 - TIM1 ETR source selection"]
    #[inline(always)]
    pub fn etr_src_tim1(&mut self) -> ETR_SRC_TIM1_W<9> {
        ETR_SRC_TIM1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
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
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
