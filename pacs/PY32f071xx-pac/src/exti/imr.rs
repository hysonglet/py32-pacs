#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR` writer"]
pub struct W(crate::W<IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR_SPEC>;
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
impl From<crate::W<IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IM0` reader - CPU wakeup with interrupt mask on event input"]
pub type IM0_R = crate::BitReader<bool>;
#[doc = "Field `IM0` writer - CPU wakeup with interrupt mask on event input"]
pub type IM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IM1` reader - CPU wakeup with interrupt mask on event input"]
pub type IM1_R = crate::BitReader<bool>;
#[doc = "Field `IM1` writer - CPU wakeup with interrupt mask on event input"]
pub type IM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IM2` reader - CPU wakeup with interrupt mask on event input"]
pub type IM2_R = crate::BitReader<bool>;
#[doc = "Field `IM2` writer - CPU wakeup with interrupt mask on event input"]
pub type IM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IM3` reader - CPU wakeup with interrupt mask on event input"]
pub type IM3_R = crate::BitReader<bool>;
#[doc = "Field `IM3` writer - CPU wakeup with interrupt mask on event input"]
pub type IM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IM4` reader - CPU wakeup with interrupt mask on event input"]
pub type IM4_R = crate::BitReader<bool>;
#[doc = "Field `IM4` writer - CPU wakeup with interrupt mask on event input"]
pub type IM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IM5` reader - CPU wakeup with interrupt mask on event input"]
pub type IM5_R = crate::BitReader<bool>;
#[doc = "Field `IM5` writer - CPU wakeup with interrupt mask on event input"]
pub type IM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IM6` reader - CPU wakeup with interrupt mask on event input"]
pub type IM6_R = crate::BitReader<bool>;
#[doc = "Field `IM6` writer - CPU wakeup with interrupt mask on event input"]
pub type IM6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IM7` reader - CPU wakeup with interrupt mask on event input"]
pub type IM7_R = crate::BitReader<bool>;
#[doc = "Field `IM7` writer - CPU wakeup with interrupt mask on event input"]
pub type IM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IM8` reader - CPU wakeup with interrupt mask on event input"]
pub type IM8_R = crate::BitReader<bool>;
#[doc = "Field `IM8` writer - CPU wakeup with interrupt mask on event input"]
pub type IM8_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IM9` reader - CPU wakeup with interrupt mask on event input"]
pub type IM9_R = crate::BitReader<bool>;
#[doc = "Field `IM9` writer - CPU wakeup with interrupt mask on event input"]
pub type IM9_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IM10` reader - CPU wakeup with interrupt mask on event input"]
pub type IM10_R = crate::BitReader<bool>;
#[doc = "Field `IM10` writer - CPU wakeup with interrupt mask on event input"]
pub type IM10_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IM11` reader - CPU wakeup with interrupt mask on event input"]
pub type IM11_R = crate::BitReader<bool>;
#[doc = "Field `IM11` writer - CPU wakeup with interrupt mask on event input"]
pub type IM11_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IM12` reader - CPU wakeup with interrupt mask on event input"]
pub type IM12_R = crate::BitReader<bool>;
#[doc = "Field `IM12` writer - CPU wakeup with interrupt mask on event input"]
pub type IM12_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IM13` reader - CPU wakeup with interrupt mask on event input"]
pub type IM13_R = crate::BitReader<bool>;
#[doc = "Field `IM13` writer - CPU wakeup with interrupt mask on event input"]
pub type IM13_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IM14` reader - CPU wakeup with interrupt mask on event input"]
pub type IM14_R = crate::BitReader<bool>;
#[doc = "Field `IM14` writer - CPU wakeup with interrupt mask on event input"]
pub type IM14_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IM15` reader - CPU wakeup with interrupt mask on event input"]
pub type IM15_R = crate::BitReader<bool>;
#[doc = "Field `IM15` writer - CPU wakeup with interrupt mask on event input"]
pub type IM15_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IM16` reader - CPU wakeup with interrupt mask on event input"]
pub type IM16_R = crate::BitReader<bool>;
#[doc = "Field `IM16` writer - CPU wakeup with interrupt mask on event input"]
pub type IM16_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IM17` reader - CPU wakeup with interrupt mask on event input"]
pub type IM17_R = crate::BitReader<bool>;
#[doc = "Field `IM17` writer - CPU wakeup with interrupt mask on event input"]
pub type IM17_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IM18` reader - CPU wakeup with interrupt mask on event input"]
pub type IM18_R = crate::BitReader<bool>;
#[doc = "Field `IM18` writer - CPU wakeup with interrupt mask on event input"]
pub type IM18_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IM19` reader - CPU wakeup with interrupt mask on event input"]
pub type IM19_R = crate::BitReader<bool>;
#[doc = "Field `IM19` writer - CPU wakeup with interrupt mask on event input"]
pub type IM19_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IM20` reader - CPU wakeup with interrupt mask on event input"]
pub type IM20_R = crate::BitReader<bool>;
#[doc = "Field `IM20` writer - CPU wakeup with interrupt mask on event input"]
pub type IM20_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IM29` reader - CPU wakeup with interrupt mask on event input"]
pub type IM29_R = crate::BitReader<bool>;
#[doc = "Field `IM29` writer - CPU wakeup with interrupt mask on event input"]
pub type IM29_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im0(&self) -> IM0_R {
        IM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im1(&self) -> IM1_R {
        IM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im2(&self) -> IM2_R {
        IM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im3(&self) -> IM3_R {
        IM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im4(&self) -> IM4_R {
        IM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im5(&self) -> IM5_R {
        IM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im6(&self) -> IM6_R {
        IM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im7(&self) -> IM7_R {
        IM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im8(&self) -> IM8_R {
        IM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im9(&self) -> IM9_R {
        IM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im10(&self) -> IM10_R {
        IM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im11(&self) -> IM11_R {
        IM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im12(&self) -> IM12_R {
        IM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im13(&self) -> IM13_R {
        IM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im14(&self) -> IM14_R {
        IM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im15(&self) -> IM15_R {
        IM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im16(&self) -> IM16_R {
        IM16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im17(&self) -> IM17_R {
        IM17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im18(&self) -> IM18_R {
        IM18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im19(&self) -> IM19_R {
        IM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im20(&self) -> IM20_R {
        IM20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 29 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im29(&self) -> IM29_R {
        IM29_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im0(&mut self) -> IM0_W<0> {
        IM0_W::new(self)
    }
    #[doc = "Bit 1 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im1(&mut self) -> IM1_W<1> {
        IM1_W::new(self)
    }
    #[doc = "Bit 2 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im2(&mut self) -> IM2_W<2> {
        IM2_W::new(self)
    }
    #[doc = "Bit 3 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im3(&mut self) -> IM3_W<3> {
        IM3_W::new(self)
    }
    #[doc = "Bit 4 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im4(&mut self) -> IM4_W<4> {
        IM4_W::new(self)
    }
    #[doc = "Bit 5 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im5(&mut self) -> IM5_W<5> {
        IM5_W::new(self)
    }
    #[doc = "Bit 6 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im6(&mut self) -> IM6_W<6> {
        IM6_W::new(self)
    }
    #[doc = "Bit 7 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im7(&mut self) -> IM7_W<7> {
        IM7_W::new(self)
    }
    #[doc = "Bit 8 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im8(&mut self) -> IM8_W<8> {
        IM8_W::new(self)
    }
    #[doc = "Bit 9 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im9(&mut self) -> IM9_W<9> {
        IM9_W::new(self)
    }
    #[doc = "Bit 10 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im10(&mut self) -> IM10_W<10> {
        IM10_W::new(self)
    }
    #[doc = "Bit 11 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im11(&mut self) -> IM11_W<11> {
        IM11_W::new(self)
    }
    #[doc = "Bit 12 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im12(&mut self) -> IM12_W<12> {
        IM12_W::new(self)
    }
    #[doc = "Bit 13 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im13(&mut self) -> IM13_W<13> {
        IM13_W::new(self)
    }
    #[doc = "Bit 14 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im14(&mut self) -> IM14_W<14> {
        IM14_W::new(self)
    }
    #[doc = "Bit 15 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im15(&mut self) -> IM15_W<15> {
        IM15_W::new(self)
    }
    #[doc = "Bit 16 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im16(&mut self) -> IM16_W<16> {
        IM16_W::new(self)
    }
    #[doc = "Bit 17 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im17(&mut self) -> IM17_W<17> {
        IM17_W::new(self)
    }
    #[doc = "Bit 18 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im18(&mut self) -> IM18_W<18> {
        IM18_W::new(self)
    }
    #[doc = "Bit 19 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im19(&mut self) -> IM19_W<19> {
        IM19_W::new(self)
    }
    #[doc = "Bit 20 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im20(&mut self) -> IM20_W<20> {
        IM20_W::new(self)
    }
    #[doc = "Bit 29 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im29(&mut self) -> IM29_W<29> {
        IM29_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI CPU wakeup with interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr::W](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMR to value 0xfff8_0000"]
impl crate::Resettable for IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfff8_0000
    }
}
