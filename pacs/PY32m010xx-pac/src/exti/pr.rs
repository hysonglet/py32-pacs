#[doc = "Register `PR` reader"]
pub struct R(crate::R<PR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PR` writer"]
pub struct W(crate::W<PR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR_SPEC>;
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
impl From<crate::W<PR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PR0` reader - configurable event inputs x rising edge Pending bit."]
pub type PR0_R = crate::BitReader<bool>;
#[doc = "Field `PR0` writer - configurable event inputs x rising edge Pending bit."]
pub type PR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR1` reader - configurable event inputs x rising edge Pending bit."]
pub type PR1_R = crate::BitReader<bool>;
#[doc = "Field `PR1` writer - configurable event inputs x rising edge Pending bit."]
pub type PR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR2` reader - configurable event inputs x rising edge Pending bit."]
pub type PR2_R = crate::BitReader<bool>;
#[doc = "Field `PR2` writer - configurable event inputs x rising edge Pending bit."]
pub type PR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR3` reader - configurable event inputs x rising edge Pending bit."]
pub type PR3_R = crate::BitReader<bool>;
#[doc = "Field `PR3` writer - configurable event inputs x rising edge Pending bit."]
pub type PR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR4` reader - configurable event inputs x rising edge Pending bit."]
pub type PR4_R = crate::BitReader<bool>;
#[doc = "Field `PR4` writer - configurable event inputs x rising edge Pending bit."]
pub type PR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR5` reader - configurable event inputs x rising edge Pending bit."]
pub type PR5_R = crate::BitReader<bool>;
#[doc = "Field `PR5` writer - configurable event inputs x rising edge Pending bit."]
pub type PR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR6` reader - configurable event inputs x rising edge Pending bit."]
pub type PR6_R = crate::BitReader<bool>;
#[doc = "Field `PR6` writer - configurable event inputs x rising edge Pending bit."]
pub type PR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR7` reader - configurable event inputs x rising edge Pending bit."]
pub type PR7_R = crate::BitReader<bool>;
#[doc = "Field `PR7` writer - configurable event inputs x rising edge Pending bit."]
pub type PR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn pr0(&self) -> PR0_R {
        PR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn pr1(&self) -> PR1_R {
        PR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn pr2(&self) -> PR2_R {
        PR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn pr3(&self) -> PR3_R {
        PR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn pr4(&self) -> PR4_R {
        PR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn pr5(&self) -> PR5_R {
        PR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn pr6(&self) -> PR6_R {
        PR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn pr7(&self) -> PR7_R {
        PR7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn pr0(&mut self) -> PR0_W<0> {
        PR0_W::new(self)
    }
    #[doc = "Bit 1 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn pr1(&mut self) -> PR1_W<1> {
        PR1_W::new(self)
    }
    #[doc = "Bit 2 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn pr2(&mut self) -> PR2_W<2> {
        PR2_W::new(self)
    }
    #[doc = "Bit 3 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn pr3(&mut self) -> PR3_W<3> {
        PR3_W::new(self)
    }
    #[doc = "Bit 4 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn pr4(&mut self) -> PR4_W<4> {
        PR4_W::new(self)
    }
    #[doc = "Bit 5 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn pr5(&mut self) -> PR5_W<5> {
        PR5_W::new(self)
    }
    #[doc = "Bit 6 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn pr6(&mut self) -> PR6_W<6> {
        PR6_W::new(self)
    }
    #[doc = "Bit 7 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn pr7(&mut self) -> PR7_W<7> {
        PR7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr](index.html) module"]
pub struct PR_SPEC;
impl crate::RegisterSpec for PR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pr::R](R) reader structure"]
impl crate::Readable for PR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pr::W](W) writer structure"]
impl crate::Writable for PR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PR to value 0"]
impl crate::Resettable for PR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
