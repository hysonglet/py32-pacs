#[doc = "Register `FTSR` reader"]
pub struct R(crate::R<FTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FTSR` writer"]
pub struct W(crate::W<FTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTSR_SPEC>;
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
impl From<crate::W<FTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FT0` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT0_R = crate::BitReader<bool>;
#[doc = "Field `FT0` writer - Falling trigger event configuration bit of Configurable Event input"]
pub type FT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR_SPEC, bool, O>;
#[doc = "Field `FT1` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT1_R = crate::BitReader<bool>;
#[doc = "Field `FT1` writer - Falling trigger event configuration bit of Configurable Event input"]
pub type FT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR_SPEC, bool, O>;
#[doc = "Field `FT2` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT2_R = crate::BitReader<bool>;
#[doc = "Field `FT2` writer - Falling trigger event configuration bit of Configurable Event input"]
pub type FT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR_SPEC, bool, O>;
#[doc = "Field `FT3` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT3_R = crate::BitReader<bool>;
#[doc = "Field `FT3` writer - Falling trigger event configuration bit of Configurable Event input"]
pub type FT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR_SPEC, bool, O>;
#[doc = "Field `FT4` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT4_R = crate::BitReader<bool>;
#[doc = "Field `FT4` writer - Falling trigger event configuration bit of Configurable Event input"]
pub type FT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR_SPEC, bool, O>;
#[doc = "Field `FT5` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT5_R = crate::BitReader<bool>;
#[doc = "Field `FT5` writer - Falling trigger event configuration bit of Configurable Event input"]
pub type FT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR_SPEC, bool, O>;
#[doc = "Field `FT6` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT6_R = crate::BitReader<bool>;
#[doc = "Field `FT6` writer - Falling trigger event configuration bit of Configurable Event input"]
pub type FT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR_SPEC, bool, O>;
#[doc = "Field `FT7` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT7_R = crate::BitReader<bool>;
#[doc = "Field `FT7` writer - Falling trigger event configuration bit of Configurable Event input"]
pub type FT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft0(&self) -> FT0_R {
        FT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft1(&self) -> FT1_R {
        FT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft2(&self) -> FT2_R {
        FT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft3(&self) -> FT3_R {
        FT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft4(&self) -> FT4_R {
        FT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft5(&self) -> FT5_R {
        FT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft6(&self) -> FT6_R {
        FT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft7(&self) -> FT7_R {
        FT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft0(&mut self) -> FT0_W<0> {
        FT0_W::new(self)
    }
    #[doc = "Bit 1 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft1(&mut self) -> FT1_W<1> {
        FT1_W::new(self)
    }
    #[doc = "Bit 2 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft2(&mut self) -> FT2_W<2> {
        FT2_W::new(self)
    }
    #[doc = "Bit 3 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft3(&mut self) -> FT3_W<3> {
        FT3_W::new(self)
    }
    #[doc = "Bit 4 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft4(&mut self) -> FT4_W<4> {
        FT4_W::new(self)
    }
    #[doc = "Bit 5 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft5(&mut self) -> FT5_W<5> {
        FT5_W::new(self)
    }
    #[doc = "Bit 6 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft6(&mut self) -> FT6_W<6> {
        FT6_W::new(self)
    }
    #[doc = "Bit 7 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft7(&mut self) -> FT7_W<7> {
        FT7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI falling trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftsr](index.html) module"]
pub struct FTSR_SPEC;
impl crate::RegisterSpec for FTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ftsr::R](R) reader structure"]
impl crate::Readable for FTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ftsr::W](W) writer structure"]
impl crate::Writable for FTSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FTSR to value 0"]
impl crate::Resettable for FTSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
