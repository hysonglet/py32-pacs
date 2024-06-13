#[doc = "Register `SWIER` reader"]
pub struct R(crate::R<SWIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWIER` writer"]
pub struct W(crate::W<SWIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWIER_SPEC>;
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
impl From<crate::W<SWIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWI0` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type SWI0_R = crate::BitReader<bool>;
#[doc = "Field `SWI0` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type SWI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER_SPEC, bool, O>;
#[doc = "Field `SWI1` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type SWI1_R = crate::BitReader<bool>;
#[doc = "Field `SWI1` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type SWI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER_SPEC, bool, O>;
#[doc = "Field `SWI2` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type SWI2_R = crate::BitReader<bool>;
#[doc = "Field `SWI2` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type SWI2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER_SPEC, bool, O>;
#[doc = "Field `SWI3` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type SWI3_R = crate::BitReader<bool>;
#[doc = "Field `SWI3` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type SWI3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER_SPEC, bool, O>;
#[doc = "Field `SWI4` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type SWI4_R = crate::BitReader<bool>;
#[doc = "Field `SWI4` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type SWI4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER_SPEC, bool, O>;
#[doc = "Field `SWI5` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type SWI5_R = crate::BitReader<bool>;
#[doc = "Field `SWI5` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type SWI5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER_SPEC, bool, O>;
#[doc = "Field `SWI6` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type SWI6_R = crate::BitReader<bool>;
#[doc = "Field `SWI6` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type SWI6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER_SPEC, bool, O>;
#[doc = "Field `SWI7` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type SWI7_R = crate::BitReader<bool>;
#[doc = "Field `SWI7` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type SWI7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi0(&self) -> SWI0_R {
        SWI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi1(&self) -> SWI1_R {
        SWI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi3(&self) -> SWI3_R {
        SWI3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi4(&self) -> SWI4_R {
        SWI4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi5(&self) -> SWI5_R {
        SWI5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi6(&self) -> SWI6_R {
        SWI6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi7(&self) -> SWI7_R {
        SWI7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi0(&mut self) -> SWI0_W<0> {
        SWI0_W::new(self)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi1(&mut self) -> SWI1_W<1> {
        SWI1_W::new(self)
    }
    #[doc = "Bit 2 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi2(&mut self) -> SWI2_W<2> {
        SWI2_W::new(self)
    }
    #[doc = "Bit 3 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi3(&mut self) -> SWI3_W<3> {
        SWI3_W::new(self)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi4(&mut self) -> SWI4_W<4> {
        SWI4_W::new(self)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi5(&mut self) -> SWI5_W<5> {
        SWI5_W::new(self)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi6(&mut self) -> SWI6_W<6> {
        SWI6_W::new(self)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi7(&mut self) -> SWI7_W<7> {
        SWI7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI software interrupt event register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier](index.html) module"]
pub struct SWIER_SPEC;
impl crate::RegisterSpec for SWIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swier::R](R) reader structure"]
impl crate::Readable for SWIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swier::W](W) writer structure"]
impl crate::Writable for SWIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWIER to value 0"]
impl crate::Resettable for SWIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
