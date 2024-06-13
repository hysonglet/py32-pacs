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
#[doc = "Field `EM8` reader - CPU wakeup with event mask on event input"]
pub type EM8_R = crate::BitReader<bool>;
#[doc = "Field `EM8` writer - CPU wakeup with event mask on event input"]
pub type EM8_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `EM9` reader - CPU wakeup with event mask on event input"]
pub type EM9_R = crate::BitReader<bool>;
#[doc = "Field `EM9` writer - CPU wakeup with event mask on event input"]
pub type EM9_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `EM10` reader - CPU wakeup with event mask on event input"]
pub type EM10_R = crate::BitReader<bool>;
#[doc = "Field `EM10` writer - CPU wakeup with event mask on event input"]
pub type EM10_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `EM11` reader - CPU wakeup with event mask on event input"]
pub type EM11_R = crate::BitReader<bool>;
#[doc = "Field `EM11` writer - CPU wakeup with event mask on event input"]
pub type EM11_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `EM12` reader - CPU wakeup with event mask on event input"]
pub type EM12_R = crate::BitReader<bool>;
#[doc = "Field `EM12` writer - CPU wakeup with event mask on event input"]
pub type EM12_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `EM13` reader - CPU wakeup with event mask on event input"]
pub type EM13_R = crate::BitReader<bool>;
#[doc = "Field `EM13` writer - CPU wakeup with event mask on event input"]
pub type EM13_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `EM14` reader - CPU wakeup with event mask on event input"]
pub type EM14_R = crate::BitReader<bool>;
#[doc = "Field `EM14` writer - CPU wakeup with event mask on event input"]
pub type EM14_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `EM15` reader - CPU wakeup with event mask on event input"]
pub type EM15_R = crate::BitReader<bool>;
#[doc = "Field `EM15` writer - CPU wakeup with event mask on event input"]
pub type EM15_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `EM16` reader - CPU wakeup with event mask on event input"]
pub type EM16_R = crate::BitReader<bool>;
#[doc = "Field `EM16` writer - CPU wakeup with event mask on event input"]
pub type EM16_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `EM17` reader - CPU wakeup with event mask on event input"]
pub type EM17_R = crate::BitReader<bool>;
#[doc = "Field `EM17` writer - CPU wakeup with event mask on event input"]
pub type EM17_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `EM18` reader - CPU wakeup with event mask on event input"]
pub type EM18_R = crate::BitReader<bool>;
#[doc = "Field `EM18` writer - CPU wakeup with event mask on event input"]
pub type EM18_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `EM19` reader - CPU wakeup with event mask on event input"]
pub type EM19_R = crate::BitReader<bool>;
#[doc = "Field `EM19` writer - CPU wakeup with event mask on event input"]
pub type EM19_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `EM20` reader - CPU wakeup with event mask on event input"]
pub type EM20_R = crate::BitReader<bool>;
#[doc = "Field `EM20` writer - CPU wakeup with event mask on event input"]
pub type EM20_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
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
    #[doc = "Bit 8 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em8(&self) -> EM8_R {
        EM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em9(&self) -> EM9_R {
        EM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em10(&self) -> EM10_R {
        EM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em11(&self) -> EM11_R {
        EM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em12(&self) -> EM12_R {
        EM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em13(&self) -> EM13_R {
        EM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em14(&self) -> EM14_R {
        EM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em15(&self) -> EM15_R {
        EM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em16(&self) -> EM16_R {
        EM16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em17(&self) -> EM17_R {
        EM17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em18(&self) -> EM18_R {
        EM18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em19(&self) -> EM19_R {
        EM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em20(&self) -> EM20_R {
        EM20_R::new(((self.bits >> 20) & 1) != 0)
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
    #[doc = "Bit 8 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em8(&mut self) -> EM8_W<8> {
        EM8_W::new(self)
    }
    #[doc = "Bit 9 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em9(&mut self) -> EM9_W<9> {
        EM9_W::new(self)
    }
    #[doc = "Bit 10 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em10(&mut self) -> EM10_W<10> {
        EM10_W::new(self)
    }
    #[doc = "Bit 11 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em11(&mut self) -> EM11_W<11> {
        EM11_W::new(self)
    }
    #[doc = "Bit 12 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em12(&mut self) -> EM12_W<12> {
        EM12_W::new(self)
    }
    #[doc = "Bit 13 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em13(&mut self) -> EM13_W<13> {
        EM13_W::new(self)
    }
    #[doc = "Bit 14 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em14(&mut self) -> EM14_W<14> {
        EM14_W::new(self)
    }
    #[doc = "Bit 15 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em15(&mut self) -> EM15_W<15> {
        EM15_W::new(self)
    }
    #[doc = "Bit 16 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em16(&mut self) -> EM16_W<16> {
        EM16_W::new(self)
    }
    #[doc = "Bit 17 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em17(&mut self) -> EM17_W<17> {
        EM17_W::new(self)
    }
    #[doc = "Bit 18 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em18(&mut self) -> EM18_W<18> {
        EM18_W::new(self)
    }
    #[doc = "Bit 19 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em19(&mut self) -> EM19_W<19> {
        EM19_W::new(self)
    }
    #[doc = "Bit 20 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em20(&mut self) -> EM20_W<20> {
        EM20_W::new(self)
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
