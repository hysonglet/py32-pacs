#[doc = "Register `TIM_CLK_EXT` reader"]
pub struct R(crate::R<TIM_CLK_EXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM_CLK_EXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM_CLK_EXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM_CLK_EXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM_CLK_EXT` writer"]
pub struct W(crate::W<TIM_CLK_EXT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM_CLK_EXT_SPEC>;
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
impl From<crate::W<TIM_CLK_EXT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM_CLK_EXT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM_PCLK1_SEL` reader - desc TIM_PCLK1_SEL"]
pub type TIM_PCLK1_SEL_R = crate::BitReader<bool>;
#[doc = "Field `TIM_PCLK1_SEL` writer - desc TIM_PCLK1_SEL"]
pub type TIM_PCLK1_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM_CLK_EXT_SPEC, bool, O>;
#[doc = "Field `TIM_PCLK2_SEL` reader - desc TIM_PCLK2_SEL"]
pub type TIM_PCLK2_SEL_R = crate::BitReader<bool>;
#[doc = "Field `TIM_PCLK2_SEL` writer - desc TIM_PCLK2_SEL"]
pub type TIM_PCLK2_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM_CLK_EXT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 6 - desc TIM_PCLK1_SEL"]
    #[inline(always)]
    pub fn tim_pclk1_sel(&self) -> TIM_PCLK1_SEL_R {
        TIM_PCLK1_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc TIM_PCLK2_SEL"]
    #[inline(always)]
    pub fn tim_pclk2_sel(&self) -> TIM_PCLK2_SEL_R {
        TIM_PCLK2_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - desc TIM_PCLK1_SEL"]
    #[inline(always)]
    pub fn tim_pclk1_sel(&mut self) -> TIM_PCLK1_SEL_W<6> {
        TIM_PCLK1_SEL_W::new(self)
    }
    #[doc = "Bit 7 - desc TIM_PCLK2_SEL"]
    #[inline(always)]
    pub fn tim_pclk2_sel(&mut self) -> TIM_PCLK2_SEL_W<7> {
        TIM_PCLK2_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc GPIOENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim_clk_ext](index.html) module"]
pub struct TIM_CLK_EXT_SPEC;
impl crate::RegisterSpec for TIM_CLK_EXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim_clk_ext::R](R) reader structure"]
impl crate::Readable for TIM_CLK_EXT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim_clk_ext::W](W) writer structure"]
impl crate::Writable for TIM_CLK_EXT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM_CLK_EXT to value 0"]
impl crate::Resettable for TIM_CLK_EXT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
