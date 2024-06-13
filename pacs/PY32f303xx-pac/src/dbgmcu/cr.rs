#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_SLEEP` reader - desc DBG_SLEEP"]
pub type DBG_SLEEP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_SLEEP` writer - desc DBG_SLEEP"]
pub type DBG_SLEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBG_STOP` reader - desc DBG_STOP"]
pub type DBG_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_STOP` writer - desc DBG_STOP"]
pub type DBG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBG_STDBY` reader - desc DBG_STDBY"]
pub type DBG_STDBY_R = crate::BitReader<bool>;
#[doc = "Field `DBG_STDBY` writer - desc DBG_STDBY"]
pub type DBG_STDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TRACE_IOEN` reader - desc TRACE_IOEN"]
pub type TRACE_IOEN_R = crate::BitReader<bool>;
#[doc = "Field `TRACE_IOEN` writer - desc TRACE_IOEN"]
pub type TRACE_IOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TRACE_MODE` reader - desc TRACE_MODE"]
pub type TRACE_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRACE_MODE` writer - desc TRACE_MODE"]
pub type TRACE_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DBG_IWDG_STOP` reader - desc DBG_IWDG_STOP"]
pub type DBG_IWDG_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_IWDG_STOP` writer - desc DBG_IWDG_STOP"]
pub type DBG_IWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBG_WWDG_STOP` reader - desc DBG_WWDG_STOP"]
pub type DBG_WWDG_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_WWDG_STOP` writer - desc DBG_WWDG_STOP"]
pub type DBG_WWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBG_TIM1_STOP` reader - desc DBG_TIM1_STOP"]
pub type DBG_TIM1_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM1_STOP` writer - desc DBG_TIM1_STOP"]
pub type DBG_TIM1_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBG_TIM2_STOP` reader - desc DBG_TIM2_STOP"]
pub type DBG_TIM2_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM2_STOP` writer - desc DBG_TIM2_STOP"]
pub type DBG_TIM2_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBG_TIM3_STOP` reader - desc DBG_TIM3_STOP"]
pub type DBG_TIM3_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM3_STOP` writer - desc DBG_TIM3_STOP"]
pub type DBG_TIM3_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBG_TIM4_STOP` reader - desc DBG_TIM4_STOP"]
pub type DBG_TIM4_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM4_STOP` writer - desc DBG_TIM4_STOP"]
pub type DBG_TIM4_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBG_CAN_STOP` reader - desc DBG_CAN_STOP"]
pub type DBG_CAN_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_CAN_STOP` writer - desc DBG_CAN_STOP"]
pub type DBG_CAN_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBG_I2C1_SMBUS_TIMEOUT` reader - desc DBG_I2C1_SMBUS_TIMEOUT"]
pub type DBG_I2C1_SMBUS_TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `DBG_I2C1_SMBUS_TIMEOUT` writer - desc DBG_I2C1_SMBUS_TIMEOUT"]
pub type DBG_I2C1_SMBUS_TIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBG_I2C2_SMBUS_TIMEOUT` reader - desc DBG_I2C2_SMBUS_TIMEOUT"]
pub type DBG_I2C2_SMBUS_TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `DBG_I2C2_SMBUS_TIMEOUT` writer - desc DBG_I2C2_SMBUS_TIMEOUT"]
pub type DBG_I2C2_SMBUS_TIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBG_TIM8_STOP` reader - desc DBG_TIM8_STOP"]
pub type DBG_TIM8_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM8_STOP` writer - desc DBG_TIM8_STOP"]
pub type DBG_TIM8_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBG_TIM5_STOP` reader - desc DBG_TIM5_STOP"]
pub type DBG_TIM5_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM5_STOP` writer - desc DBG_TIM5_STOP"]
pub type DBG_TIM5_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBG_TIM6_STOP` reader - desc DBG_TIM6_STOP"]
pub type DBG_TIM6_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM6_STOP` writer - desc DBG_TIM6_STOP"]
pub type DBG_TIM6_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBG_TIM7_STOP` reader - desc DBG_TIM7_STOP"]
pub type DBG_TIM7_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM7_STOP` writer - desc DBG_TIM7_STOP"]
pub type DBG_TIM7_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBG_TIM12_STOP` reader - desc DBG_TIM12_STOP"]
pub type DBG_TIM12_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM12_STOP` writer - desc DBG_TIM12_STOP"]
pub type DBG_TIM12_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBG_TIM13_STOP` reader - desc DBG_TIM13_STOP"]
pub type DBG_TIM13_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM13_STOP` writer - desc DBG_TIM13_STOP"]
pub type DBG_TIM13_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBG_TIM14_STOP` reader - desc DBG_TIM14_STOP"]
pub type DBG_TIM14_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM14_STOP` writer - desc DBG_TIM14_STOP"]
pub type DBG_TIM14_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBG_TIM9_STOP` reader - desc DBG_TIM9_STOP"]
pub type DBG_TIM9_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM9_STOP` writer - desc DBG_TIM9_STOP"]
pub type DBG_TIM9_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBG_TIM10_STOP` reader - desc DBG_TIM10_STOP"]
pub type DBG_TIM10_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM10_STOP` writer - desc DBG_TIM10_STOP"]
pub type DBG_TIM10_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBG_TIM11_STOP` reader - desc DBG_TIM11_STOP"]
pub type DBG_TIM11_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM11_STOP` writer - desc DBG_TIM11_STOP"]
pub type DBG_TIM11_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc DBG_SLEEP"]
    #[inline(always)]
    pub fn dbg_sleep(&self) -> DBG_SLEEP_R {
        DBG_SLEEP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc DBG_STOP"]
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc DBG_STDBY"]
    #[inline(always)]
    pub fn dbg_stdby(&self) -> DBG_STDBY_R {
        DBG_STDBY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - desc TRACE_IOEN"]
    #[inline(always)]
    pub fn trace_ioen(&self) -> TRACE_IOEN_R {
        TRACE_IOEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - desc TRACE_MODE"]
    #[inline(always)]
    pub fn trace_mode(&self) -> TRACE_MODE_R {
        TRACE_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - desc DBG_IWDG_STOP"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc DBG_WWDG_STOP"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc DBG_TIM1_STOP"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc DBG_TIM2_STOP"]
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DBG_TIM2_STOP_R {
        DBG_TIM2_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc DBG_TIM3_STOP"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc DBG_TIM4_STOP"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&self) -> DBG_TIM4_STOP_R {
        DBG_TIM4_STOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc DBG_CAN_STOP"]
    #[inline(always)]
    pub fn dbg_can_stop(&self) -> DBG_CAN_STOP_R {
        DBG_CAN_STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc DBG_I2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&self) -> DBG_I2C1_SMBUS_TIMEOUT_R {
        DBG_I2C1_SMBUS_TIMEOUT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc DBG_I2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_i2c2_smbus_timeout(&self) -> DBG_I2C2_SMBUS_TIMEOUT_R {
        DBG_I2C2_SMBUS_TIMEOUT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc DBG_TIM8_STOP"]
    #[inline(always)]
    pub fn dbg_tim8_stop(&self) -> DBG_TIM8_STOP_R {
        DBG_TIM8_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc DBG_TIM5_STOP"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&self) -> DBG_TIM5_STOP_R {
        DBG_TIM5_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc DBG_TIM6_STOP"]
    #[inline(always)]
    pub fn dbg_tim6_stop(&self) -> DBG_TIM6_STOP_R {
        DBG_TIM6_STOP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc DBG_TIM7_STOP"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DBG_TIM7_STOP_R {
        DBG_TIM7_STOP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 25 - desc DBG_TIM12_STOP"]
    #[inline(always)]
    pub fn dbg_tim12_stop(&self) -> DBG_TIM12_STOP_R {
        DBG_TIM12_STOP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - desc DBG_TIM13_STOP"]
    #[inline(always)]
    pub fn dbg_tim13_stop(&self) -> DBG_TIM13_STOP_R {
        DBG_TIM13_STOP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - desc DBG_TIM14_STOP"]
    #[inline(always)]
    pub fn dbg_tim14_stop(&self) -> DBG_TIM14_STOP_R {
        DBG_TIM14_STOP_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - desc DBG_TIM9_STOP"]
    #[inline(always)]
    pub fn dbg_tim9_stop(&self) -> DBG_TIM9_STOP_R {
        DBG_TIM9_STOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc DBG_TIM10_STOP"]
    #[inline(always)]
    pub fn dbg_tim10_stop(&self) -> DBG_TIM10_STOP_R {
        DBG_TIM10_STOP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc DBG_TIM11_STOP"]
    #[inline(always)]
    pub fn dbg_tim11_stop(&self) -> DBG_TIM11_STOP_R {
        DBG_TIM11_STOP_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc DBG_SLEEP"]
    #[inline(always)]
    pub fn dbg_sleep(&mut self) -> DBG_SLEEP_W<0> {
        DBG_SLEEP_W::new(self)
    }
    #[doc = "Bit 1 - desc DBG_STOP"]
    #[inline(always)]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W<1> {
        DBG_STOP_W::new(self)
    }
    #[doc = "Bit 2 - desc DBG_STDBY"]
    #[inline(always)]
    pub fn dbg_stdby(&mut self) -> DBG_STDBY_W<2> {
        DBG_STDBY_W::new(self)
    }
    #[doc = "Bit 5 - desc TRACE_IOEN"]
    #[inline(always)]
    pub fn trace_ioen(&mut self) -> TRACE_IOEN_W<5> {
        TRACE_IOEN_W::new(self)
    }
    #[doc = "Bits 6:7 - desc TRACE_MODE"]
    #[inline(always)]
    pub fn trace_mode(&mut self) -> TRACE_MODE_W<6> {
        TRACE_MODE_W::new(self)
    }
    #[doc = "Bit 8 - desc DBG_IWDG_STOP"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<8> {
        DBG_IWDG_STOP_W::new(self)
    }
    #[doc = "Bit 9 - desc DBG_WWDG_STOP"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<9> {
        DBG_WWDG_STOP_W::new(self)
    }
    #[doc = "Bit 10 - desc DBG_TIM1_STOP"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<10> {
        DBG_TIM1_STOP_W::new(self)
    }
    #[doc = "Bit 11 - desc DBG_TIM2_STOP"]
    #[inline(always)]
    pub fn dbg_tim2_stop(&mut self) -> DBG_TIM2_STOP_W<11> {
        DBG_TIM2_STOP_W::new(self)
    }
    #[doc = "Bit 12 - desc DBG_TIM3_STOP"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W<12> {
        DBG_TIM3_STOP_W::new(self)
    }
    #[doc = "Bit 13 - desc DBG_TIM4_STOP"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&mut self) -> DBG_TIM4_STOP_W<13> {
        DBG_TIM4_STOP_W::new(self)
    }
    #[doc = "Bit 14 - desc DBG_CAN_STOP"]
    #[inline(always)]
    pub fn dbg_can_stop(&mut self) -> DBG_CAN_STOP_W<14> {
        DBG_CAN_STOP_W::new(self)
    }
    #[doc = "Bit 15 - desc DBG_I2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&mut self) -> DBG_I2C1_SMBUS_TIMEOUT_W<15> {
        DBG_I2C1_SMBUS_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 16 - desc DBG_I2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_i2c2_smbus_timeout(&mut self) -> DBG_I2C2_SMBUS_TIMEOUT_W<16> {
        DBG_I2C2_SMBUS_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 17 - desc DBG_TIM8_STOP"]
    #[inline(always)]
    pub fn dbg_tim8_stop(&mut self) -> DBG_TIM8_STOP_W<17> {
        DBG_TIM8_STOP_W::new(self)
    }
    #[doc = "Bit 18 - desc DBG_TIM5_STOP"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&mut self) -> DBG_TIM5_STOP_W<18> {
        DBG_TIM5_STOP_W::new(self)
    }
    #[doc = "Bit 19 - desc DBG_TIM6_STOP"]
    #[inline(always)]
    pub fn dbg_tim6_stop(&mut self) -> DBG_TIM6_STOP_W<19> {
        DBG_TIM6_STOP_W::new(self)
    }
    #[doc = "Bit 20 - desc DBG_TIM7_STOP"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&mut self) -> DBG_TIM7_STOP_W<20> {
        DBG_TIM7_STOP_W::new(self)
    }
    #[doc = "Bit 25 - desc DBG_TIM12_STOP"]
    #[inline(always)]
    pub fn dbg_tim12_stop(&mut self) -> DBG_TIM12_STOP_W<25> {
        DBG_TIM12_STOP_W::new(self)
    }
    #[doc = "Bit 26 - desc DBG_TIM13_STOP"]
    #[inline(always)]
    pub fn dbg_tim13_stop(&mut self) -> DBG_TIM13_STOP_W<26> {
        DBG_TIM13_STOP_W::new(self)
    }
    #[doc = "Bit 27 - desc DBG_TIM14_STOP"]
    #[inline(always)]
    pub fn dbg_tim14_stop(&mut self) -> DBG_TIM14_STOP_W<27> {
        DBG_TIM14_STOP_W::new(self)
    }
    #[doc = "Bit 28 - desc DBG_TIM9_STOP"]
    #[inline(always)]
    pub fn dbg_tim9_stop(&mut self) -> DBG_TIM9_STOP_W<28> {
        DBG_TIM9_STOP_W::new(self)
    }
    #[doc = "Bit 29 - desc DBG_TIM10_STOP"]
    #[inline(always)]
    pub fn dbg_tim10_stop(&mut self) -> DBG_TIM10_STOP_W<29> {
        DBG_TIM10_STOP_W::new(self)
    }
    #[doc = "Bit 30 - desc DBG_TIM11_STOP"]
    #[inline(always)]
    pub fn dbg_tim11_stop(&mut self) -> DBG_TIM11_STOP_W<30> {
        DBG_TIM11_STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
