#[doc = "Register `EMR` reader"]
pub struct R(crate::R<EMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMR` writer"]
pub struct W(crate::W<EMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMR_SPEC>;
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
impl From<crate::W<EMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM0` reader - CPU wakeup with event mask on event input"]
pub type EM0_R = crate::BitReader<bool>;
#[doc = "Field `EM0` writer - CPU wakeup with event mask on event input"]
pub type EM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `EM1` reader - CPU wakeup with event mask on event input"]
pub type EM1_R = crate::BitReader<bool>;
#[doc = "Field `EM1` writer - CPU wakeup with event mask on event input"]
pub type EM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `EM2` reader - CPU wakeup with event mask on event input"]
pub type EM2_R = crate::BitReader<bool>;
#[doc = "Field `EM2` writer - CPU wakeup with event mask on event input"]
pub type EM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `EM3` reader - CPU wakeup with event mask on event input"]
pub type EM3_R = crate::BitReader<bool>;
#[doc = "Field `EM3` writer - CPU wakeup with event mask on event input"]
pub type EM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `EM4` reader - CPU wakeup with event mask on event input"]
pub type EM4_R = crate::BitReader<bool>;
#[doc = "Field `EM4` writer - CPU wakeup with event mask on event input"]
pub type EM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `EM5` reader - CPU wakeup with event mask on event input"]
pub type EM5_R = crate::BitReader<bool>;
#[doc = "Field `EM5` writer - CPU wakeup with event mask on event input"]
pub type EM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `EM6` reader - CPU wakeup with event mask on event input"]
pub type EM6_R = crate::BitReader<bool>;
#[doc = "Field `EM6` writer - CPU wakeup with event mask on event input"]
pub type EM6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `EM7` reader - CPU wakeup with event mask on event input"]
pub type EM7_R = crate::BitReader<bool>;
#[doc = "Field `EM7` writer - CPU wakeup with event mask on event input"]
pub type EM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `EM29` reader - CPU wakeup with event mask on event input"]
pub type EM29_R = crate::BitReader<bool>;
#[doc = "Field `EM29` writer - CPU wakeup with event mask on event input"]
pub type EM29_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em0(&self) -> EM0_R {
        EM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em1(&self) -> EM1_R {
        EM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em2(&self) -> EM2_R {
        EM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em3(&self) -> EM3_R {
        EM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em4(&self) -> EM4_R {
        EM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em5(&self) -> EM5_R {
        EM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em6(&self) -> EM6_R {
        EM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em7(&self) -> EM7_R {
        EM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 29 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em29(&self) -> EM29_R {
        EM29_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em0(&mut self) -> EM0_W<0> {
        EM0_W::new(self)
    }
    #[doc = "Bit 1 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em1(&mut self) -> EM1_W<1> {
        EM1_W::new(self)
    }
    #[doc = "Bit 2 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em2(&mut self) -> EM2_W<2> {
        EM2_W::new(self)
    }
    #[doc = "Bit 3 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em3(&mut self) -> EM3_W<3> {
        EM3_W::new(self)
    }
    #[doc = "Bit 4 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em4(&mut self) -> EM4_W<4> {
        EM4_W::new(self)
    }
    #[doc = "Bit 5 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em5(&mut self) -> EM5_W<5> {
        EM5_W::new(self)
    }
    #[doc = "Bit 6 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em6(&mut self) -> EM6_W<6> {
        EM6_W::new(self)
    }
    #[doc = "Bit 7 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em7(&mut self) -> EM7_W<7> {
        EM7_W::new(self)
    }
    #[doc = "Bit 29 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em29(&mut self) -> EM29_W<29> {
        EM29_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI CPU wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr](index.html) module"]
pub struct EMR_SPEC;
impl crate::RegisterSpec for EMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emr::R](R) reader structure"]
impl crate::Readable for EMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emr::W](W) writer structure"]
impl crate::Writable for EMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
