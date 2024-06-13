#[doc = "Register `APB_FZ2` reader"]
pub struct R(crate::R<APB_FZ2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_FZ2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_FZ2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_FZ2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_FZ2` writer"]
pub struct W(crate::W<APB_FZ2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_FZ2_SPEC>;
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
impl From<crate::W<APB_FZ2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_FZ2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_TIMER1_STOP` reader - Debug Timer 1 stopped when Core ishalted"]
pub type DBG_TIMER1_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIMER1_STOP` writer - Debug Timer 1 stopped when Core ishalted"]
pub type DBG_TIMER1_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ2_SPEC, bool, O>;
#[doc = "Field `DBG_TIM14_STOP` reader - Debug TIM 14 stopped whenCore is halted"]
pub type DBG_TIM14_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM14_STOP` writer - Debug TIM 14 stopped whenCore is halted"]
pub type DBG_TIM14_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 11 - Debug Timer 1 stopped when Core ishalted"]
    #[inline(always)]
    pub fn dbg_timer1_stop(&self) -> DBG_TIMER1_STOP_R {
        DBG_TIMER1_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Debug TIM 14 stopped whenCore is halted"]
    #[inline(always)]
    pub fn dbg_tim14_stop(&self) -> DBG_TIM14_STOP_R {
        DBG_TIM14_STOP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Debug Timer 1 stopped when Core ishalted"]
    #[inline(always)]
    pub fn dbg_timer1_stop(&mut self) -> DBG_TIMER1_STOP_W<11> {
        DBG_TIMER1_STOP_W::new(self)
    }
    #[doc = "Bit 15 - Debug TIM 14 stopped whenCore is halted"]
    #[inline(always)]
    pub fn dbg_tim14_stop(&mut self) -> DBG_TIM14_STOP_W<15> {
        DBG_TIM14_STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB Freeze Register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_fz2](index.html) module"]
pub struct APB_FZ2_SPEC;
impl crate::RegisterSpec for APB_FZ2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_fz2::R](R) reader structure"]
impl crate::Readable for APB_FZ2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_fz2::W](W) writer structure"]
impl crate::Writable for APB_FZ2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB_FZ2 to value 0"]
impl crate::Resettable for APB_FZ2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}