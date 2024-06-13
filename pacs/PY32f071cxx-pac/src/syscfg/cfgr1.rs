#[doc = "Register `CFGR1` reader"]
pub struct R(crate::R<CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR1` writer"]
pub struct W(crate::W<CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR1_SPEC>;
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
impl From<crate::W<CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_MODE` reader - desc MEM_MODE"]
pub type MEM_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEM_MODE` writer - desc MEM_MODE"]
pub type MEM_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `TIM1_IC1_SRC` reader - desc TIM1_IC1_SRC"]
pub type TIM1_IC1_SRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM1_IC1_SRC` writer - desc TIM1_IC1_SRC"]
pub type TIM1_IC1_SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `TIM2_IC4_SRC` reader - desc TIM2_IC4_SRC"]
pub type TIM2_IC4_SRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM2_IC4_SRC` writer - desc TIM2_IC4_SRC"]
pub type TIM2_IC4_SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `TIM3_IC1_SRC` reader - desc TIM3_IC1_SRC"]
pub type TIM3_IC1_SRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM3_IC1_SRC` writer - desc TIM3_IC1_SRC"]
pub type TIM3_IC1_SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ETR_SRC_TIM1` reader - desc ETR_SRC_TIM1"]
pub type ETR_SRC_TIM1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETR_SRC_TIM1` writer - desc ETR_SRC_TIM1"]
pub type ETR_SRC_TIM1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `ETR_SRC_TIM2` reader - desc ETR_SRC_TIM2"]
pub type ETR_SRC_TIM2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETR_SRC_TIM2` writer - desc ETR_SRC_TIM2"]
pub type ETR_SRC_TIM2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `ETR_SRC_TIM3` reader - desc ETR_SRC_TIM3"]
pub type ETR_SRC_TIM3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETR_SRC_TIM3` writer - desc ETR_SRC_TIM3"]
pub type ETR_SRC_TIM3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `GPIO_AHB_SEL` reader - desc GPIO_AHB_SEL"]
pub type GPIO_AHB_SEL_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_AHB_SEL` writer - desc GPIO_AHB_SEL"]
pub type GPIO_AHB_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - desc MEM_MODE"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc TIM1_IC1_SRC"]
    #[inline(always)]
    pub fn tim1_ic1_src(&self) -> TIM1_IC1_SRC_R {
        TIM1_IC1_SRC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - desc TIM2_IC4_SRC"]
    #[inline(always)]
    pub fn tim2_ic4_src(&self) -> TIM2_IC4_SRC_R {
        TIM2_IC4_SRC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - desc TIM3_IC1_SRC"]
    #[inline(always)]
    pub fn tim3_ic1_src(&self) -> TIM3_IC1_SRC_R {
        TIM3_IC1_SRC_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - desc ETR_SRC_TIM1"]
    #[inline(always)]
    pub fn etr_src_tim1(&self) -> ETR_SRC_TIM1_R {
        ETR_SRC_TIM1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - desc ETR_SRC_TIM2"]
    #[inline(always)]
    pub fn etr_src_tim2(&self) -> ETR_SRC_TIM2_R {
        ETR_SRC_TIM2_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - desc ETR_SRC_TIM3"]
    #[inline(always)]
    pub fn etr_src_tim3(&self) -> ETR_SRC_TIM3_R {
        ETR_SRC_TIM3_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - desc GPIO_AHB_SEL"]
    #[inline(always)]
    pub fn gpio_ahb_sel(&self) -> GPIO_AHB_SEL_R {
        GPIO_AHB_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc MEM_MODE"]
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<0> {
        MEM_MODE_W::new(self)
    }
    #[doc = "Bits 2:3 - desc TIM1_IC1_SRC"]
    #[inline(always)]
    pub fn tim1_ic1_src(&mut self) -> TIM1_IC1_SRC_W<2> {
        TIM1_IC1_SRC_W::new(self)
    }
    #[doc = "Bits 4:5 - desc TIM2_IC4_SRC"]
    #[inline(always)]
    pub fn tim2_ic4_src(&mut self) -> TIM2_IC4_SRC_W<4> {
        TIM2_IC4_SRC_W::new(self)
    }
    #[doc = "Bits 6:7 - desc TIM3_IC1_SRC"]
    #[inline(always)]
    pub fn tim3_ic1_src(&mut self) -> TIM3_IC1_SRC_W<6> {
        TIM3_IC1_SRC_W::new(self)
    }
    #[doc = "Bits 8:10 - desc ETR_SRC_TIM1"]
    #[inline(always)]
    pub fn etr_src_tim1(&mut self) -> ETR_SRC_TIM1_W<8> {
        ETR_SRC_TIM1_W::new(self)
    }
    #[doc = "Bits 12:14 - desc ETR_SRC_TIM2"]
    #[inline(always)]
    pub fn etr_src_tim2(&mut self) -> ETR_SRC_TIM2_W<12> {
        ETR_SRC_TIM2_W::new(self)
    }
    #[doc = "Bits 16:18 - desc ETR_SRC_TIM3"]
    #[inline(always)]
    pub fn etr_src_tim3(&mut self) -> ETR_SRC_TIM3_W<16> {
        ETR_SRC_TIM3_W::new(self)
    }
    #[doc = "Bit 24 - desc GPIO_AHB_SEL"]
    #[inline(always)]
    pub fn gpio_ahb_sel(&mut self) -> GPIO_AHB_SEL_W<24> {
        GPIO_AHB_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CFGR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](index.html) module"]
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr1::R](R) reader structure"]
impl crate::Readable for CFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr1::W](W) writer structure"]
impl crate::Writable for CFGR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR1 to value 0x00ff_0000"]
impl crate::Resettable for CFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00ff_0000
    }
}
