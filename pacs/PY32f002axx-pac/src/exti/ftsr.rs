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
#[doc = "Field `FT8` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT8_R = crate::BitReader<bool>;
#[doc = "Field `FT8` writer - Falling trigger event configuration bit of Configurable Event input"]
pub type FT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR_SPEC, bool, O>;
#[doc = "Field `FT9` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT9_R = crate::BitReader<bool>;
#[doc = "Field `FT9` writer - Falling trigger event configuration bit of Configurable Event input"]
pub type FT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR_SPEC, bool, O>;
#[doc = "Field `FT10` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT10_R = crate::BitReader<bool>;
#[doc = "Field `FT10` writer - Falling trigger event configuration bit of Configurable Event input"]
pub type FT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR_SPEC, bool, O>;
#[doc = "Field `FT11` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT11_R = crate::BitReader<bool>;
#[doc = "Field `FT11` writer - Falling trigger event configuration bit of Configurable Event input"]
pub type FT11_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR_SPEC, bool, O>;
#[doc = "Field `FT12` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT12_R = crate::BitReader<bool>;
#[doc = "Field `FT12` writer - Falling trigger event configuration bit of Configurable Event input"]
pub type FT12_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR_SPEC, bool, O>;
#[doc = "Field `FT13` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT13_R = crate::BitReader<bool>;
#[doc = "Field `FT13` writer - Falling trigger event configuration bit of Configurable Event input"]
pub type FT13_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR_SPEC, bool, O>;
#[doc = "Field `FT14` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT14_R = crate::BitReader<bool>;
#[doc = "Field `FT14` writer - Falling trigger event configuration bit of Configurable Event input"]
pub type FT14_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR_SPEC, bool, O>;
#[doc = "Field `FT15` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT15_R = crate::BitReader<bool>;
#[doc = "Field `FT15` writer - Falling trigger event configuration bit of Configurable Event input"]
pub type FT15_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR_SPEC, bool, O>;
#[doc = "Field `FT16` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT16_R = crate::BitReader<bool>;
#[doc = "Field `FT16` writer - Falling trigger event configuration bit of Configurable Event input"]
pub type FT16_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR_SPEC, bool, O>;
#[doc = "Field `FT17` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT17_R = crate::BitReader<bool>;
#[doc = "Field `FT17` writer - Falling trigger event configuration bit of Configurable Event input"]
pub type FT17_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR_SPEC, bool, O>;
#[doc = "Field `FT18` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT18_R = crate::BitReader<bool>;
#[doc = "Field `FT18` writer - Falling trigger event configuration bit of Configurable Event input"]
pub type FT18_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR_SPEC, bool, O>;
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
    #[doc = "Bit 8 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft8(&self) -> FT8_R {
        FT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft9(&self) -> FT9_R {
        FT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft10(&self) -> FT10_R {
        FT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft11(&self) -> FT11_R {
        FT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft12(&self) -> FT12_R {
        FT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft13(&self) -> FT13_R {
        FT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft14(&self) -> FT14_R {
        FT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft15(&self) -> FT15_R {
        FT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft16(&self) -> FT16_R {
        FT16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft17(&self) -> FT17_R {
        FT17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft18(&self) -> FT18_R {
        FT18_R::new(((self.bits >> 18) & 1) != 0)
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
    #[doc = "Bit 8 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft8(&mut self) -> FT8_W<8> {
        FT8_W::new(self)
    }
    #[doc = "Bit 9 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft9(&mut self) -> FT9_W<9> {
        FT9_W::new(self)
    }
    #[doc = "Bit 10 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft10(&mut self) -> FT10_W<10> {
        FT10_W::new(self)
    }
    #[doc = "Bit 11 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft11(&mut self) -> FT11_W<11> {
        FT11_W::new(self)
    }
    #[doc = "Bit 12 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft12(&mut self) -> FT12_W<12> {
        FT12_W::new(self)
    }
    #[doc = "Bit 13 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft13(&mut self) -> FT13_W<13> {
        FT13_W::new(self)
    }
    #[doc = "Bit 14 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft14(&mut self) -> FT14_W<14> {
        FT14_W::new(self)
    }
    #[doc = "Bit 15 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft15(&mut self) -> FT15_W<15> {
        FT15_W::new(self)
    }
    #[doc = "Bit 16 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft16(&mut self) -> FT16_W<16> {
        FT16_W::new(self)
    }
    #[doc = "Bit 17 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft17(&mut self) -> FT17_W<17> {
        FT17_W::new(self)
    }
    #[doc = "Bit 18 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft18(&mut self) -> FT18_W<18> {
        FT18_W::new(self)
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
