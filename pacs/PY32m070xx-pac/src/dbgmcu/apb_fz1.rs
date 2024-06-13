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
#[doc = "Field `DBG_TIMER2_STOP` reader - Debug Timer 2 stopped when Core is halted"]
pub type DBG_TIMER2_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIMER2_STOP` writer - Debug Timer 2 stopped when Core is halted"]
pub type DBG_TIMER2_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
#[doc = "Field `DBG_TIMER3_STOP` reader - Debug Timer 3 stopped when Core is halted"]
pub type DBG_TIMER3_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIMER3_STOP` writer - Debug Timer 3 stopped when Core is halted"]
pub type DBG_TIMER3_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
#[doc = "Field `DBG_TIMER6_STOP` reader - Debug Timer 6 stopped when Core is halted"]
pub type DBG_TIMER6_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIMER6_STOP` writer - Debug Timer 6 stopped when Core is halted"]
pub type DBG_TIMER6_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
#[doc = "Field `DBG_TIMER7_STOP` reader - Debug Timer 7 stopped when Core is halted"]
pub type DBG_TIMER7_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIMER7_STOP` writer - Debug Timer 7 stopped when Core is halted"]
pub type DBG_TIMER7_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
#[doc = "Field `DBG_RTC_STOP` reader - Debug RTC stopped when Core is halted"]
pub type DBG_RTC_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_RTC_STOP` writer - Debug RTC stopped when Core is halted"]
pub type DBG_RTC_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
#[doc = "Field `DBG_WWDG_STOP` reader - Debug Window Wachdog stopped when Core is halted"]
pub type DBG_WWDG_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_WWDG_STOP` writer - Debug Window Wachdog stopped when Core is halted"]
pub type DBG_WWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
#[doc = "Field `DBG_IWDG_STOP` reader - Debug Independent Wachdog stopped when Core is halted"]
pub type DBG_IWDG_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_IWDG_STOP` writer - Debug Independent Wachdog stopped when Core is halted"]
pub type DBG_IWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
#[doc = "Field `DBG_CAN_STOP` reader - DBG_CAN_STOP"]
pub type DBG_CAN_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_CAN_STOP` writer - DBG_CAN_STOP"]
pub type DBG_CAN_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
#[doc = "Field `DBG_I2C1_SMBUS_TIMEOUT` reader - DBG_I2C1_SMBUS_TIMEOUT"]
pub type DBG_I2C1_SMBUS_TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `DBG_I2C1_SMBUS_TIMEOUT` writer - DBG_I2C1_SMBUS_TIMEOUT"]
pub type DBG_I2C1_SMBUS_TIMEOUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
#[doc = "Field `DBG_I2C2_SMBUS_TIMEOUT` reader - DBG_I2C2_SMBUS_TIMEOUT"]
pub type DBG_I2C2_SMBUS_TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `DBG_LPTIM_STOP` reader - Debug LPTIM stopped when Core is halted"]
pub type DBG_LPTIM_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_LPTIM_STOP` writer - Debug LPTIM stopped when Core is halted"]
pub type DBG_LPTIM_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Debug Timer 2 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer2_stop(&self) -> DBG_TIMER2_STOP_R {
        DBG_TIMER2_STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug Timer 3 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer3_stop(&self) -> DBG_TIMER3_STOP_R {
        DBG_TIMER3_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Debug Timer 6 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer6_stop(&self) -> DBG_TIMER6_STOP_R {
        DBG_TIMER6_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Debug Timer 7 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer7_stop(&self) -> DBG_TIMER7_STOP_R {
        DBG_TIMER7_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - Debug RTC stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 19 - DBG_CAN_STOP"]
    #[inline(always)]
    pub fn dbg_can_stop(&self) -> DBG_CAN_STOP_R {
        DBG_CAN_STOP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - DBG_I2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&self) -> DBG_I2C1_SMBUS_TIMEOUT_R {
        DBG_I2C1_SMBUS_TIMEOUT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DBG_I2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_i2c2_smbus_timeout(&self) -> DBG_I2C2_SMBUS_TIMEOUT_R {
        DBG_I2C2_SMBUS_TIMEOUT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 31 - Debug LPTIM stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_lptim_stop(&self) -> DBG_LPTIM_STOP_R {
        DBG_LPTIM_STOP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Timer 2 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer2_stop(&mut self) -> DBG_TIMER2_STOP_W<0> {
        DBG_TIMER2_STOP_W::new(self)
    }
    #[doc = "Bit 1 - Debug Timer 3 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer3_stop(&mut self) -> DBG_TIMER3_STOP_W<1> {
        DBG_TIMER3_STOP_W::new(self)
    }
    #[doc = "Bit 4 - Debug Timer 6 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer6_stop(&mut self) -> DBG_TIMER6_STOP_W<4> {
        DBG_TIMER6_STOP_W::new(self)
    }
    #[doc = "Bit 5 - Debug Timer 7 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer7_stop(&mut self) -> DBG_TIMER7_STOP_W<5> {
        DBG_TIMER7_STOP_W::new(self)
    }
    #[doc = "Bit 10 - Debug RTC stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<10> {
        DBG_RTC_STOP_W::new(self)
    }
    #[doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<11> {
        DBG_WWDG_STOP_W::new(self)
    }
    #[doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<12> {
        DBG_IWDG_STOP_W::new(self)
    }
    #[doc = "Bit 19 - DBG_CAN_STOP"]
    #[inline(always)]
    pub fn dbg_can_stop(&mut self) -> DBG_CAN_STOP_W<19> {
        DBG_CAN_STOP_W::new(self)
    }
    #[doc = "Bit 21 - DBG_I2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&mut self) -> DBG_I2C1_SMBUS_TIMEOUT_W<21> {
        DBG_I2C1_SMBUS_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 31 - Debug LPTIM stopped when Core is halted"]
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
