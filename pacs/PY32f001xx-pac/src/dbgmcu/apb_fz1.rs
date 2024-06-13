#[doc = "Register `APB_FZ1` reader"]
pub struct R(crate::R<APB_FZ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_FZ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_FZ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_FZ1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_FZ1` writer"]
pub struct W(crate::W<APB_FZ1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_FZ1_SPEC>;
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
impl From<crate::W<APB_FZ1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_FZ1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_IWDG_STOP` reader - Debug Independent Wachdog stopped whenCore is halted"]
pub type DBG_IWDG_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_IWDG_STOP` writer - Debug Independent Wachdog stopped whenCore is halted"]
pub type DBG_IWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
#[doc = "Field `DBG_LPTIM_STOP` reader - Debug LPTIM stopped when Core ishalted"]
pub type DBG_LPTIM_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_LPTIM_STOP` writer - Debug LPTIM stopped when Core ishalted"]
pub type DBG_LPTIM_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 12 - Debug Independent Wachdog stopped whenCore is halted"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 31 - Debug LPTIM stopped when Core ishalted"]
    #[inline(always)]
    pub fn dbg_lptim_stop(&self) -> DBG_LPTIM_STOP_R {
        DBG_LPTIM_STOP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Debug Independent Wachdog stopped whenCore is halted"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<12> {
        DBG_IWDG_STOP_W::new(self)
    }
    #[doc = "Bit 31 - Debug LPTIM stopped when Core ishalted"]
    #[inline(always)]
    pub fn dbg_lptim_stop(&mut self) -> DBG_LPTIM_STOP_W<31> {
        DBG_LPTIM_STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB Freeze Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_fz1](index.html) module"]
pub struct APB_FZ1_SPEC;
impl crate::RegisterSpec for APB_FZ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_fz1::R](R) reader structure"]
impl crate::Readable for APB_FZ1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_fz1::W](W) writer structure"]
impl crate::Writable for APB_FZ1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB_FZ1 to value 0"]
impl crate::Resettable for APB_FZ1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
