#[doc = "Register `CCIPR` reader"]
pub struct R(crate::R<CCIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCIPR` writer"]
pub struct W(crate::W<CCIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR_SPEC>;
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
impl From<crate::W<CCIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CANSEL` reader - CAN detect clock source selection"]
pub type CANSEL_R = crate::BitReader<bool>;
#[doc = "Field `CANSEL` writer - CAN detect clock source selection"]
pub type CANSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCIPR_SPEC, bool, O>;
#[doc = "Field `PVDSEL` reader - PVD detect clock source selection"]
pub type PVDSEL_R = crate::BitReader<bool>;
#[doc = "Field `PVDSEL` writer - PVD detect clock source selection"]
pub type PVDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCIPR_SPEC, bool, O>;
#[doc = "Field `COMP1SEL` reader - COMP1 clock source selection"]
pub type COMP1SEL_R = crate::BitReader<bool>;
#[doc = "Field `COMP1SEL` writer - COMP1 clock source selection"]
pub type COMP1SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCIPR_SPEC, bool, O>;
#[doc = "Field `COMP2SEL` reader - COMP2 clock source selection"]
pub type COMP2SEL_R = crate::BitReader<bool>;
#[doc = "Field `COMP2SEL` writer - COMP2 clock source selection"]
pub type COMP2SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCIPR_SPEC, bool, O>;
#[doc = "Field `COMP3SEL` reader - COMP3 clock source selection"]
pub type COMP3SEL_R = crate::BitReader<bool>;
#[doc = "Field `COMP3SEL` writer - COMP3 clock source selection"]
pub type COMP3SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCIPR_SPEC, bool, O>;
#[doc = "Field `LPTIM1SEL` reader - LPTIM1 clock source selection"]
pub type LPTIM1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPTIM1SEL` writer - LPTIM1 clock source selection"]
pub type LPTIM1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 6 - CAN detect clock source selection"]
    #[inline(always)]
    pub fn cansel(&self) -> CANSEL_R {
        CANSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PVD detect clock source selection"]
    #[inline(always)]
    pub fn pvdsel(&self) -> PVDSEL_R {
        PVDSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - COMP1 clock source selection"]
    #[inline(always)]
    pub fn comp1sel(&self) -> COMP1SEL_R {
        COMP1SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - COMP2 clock source selection"]
    #[inline(always)]
    pub fn comp2sel(&self) -> COMP2SEL_R {
        COMP2SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - COMP3 clock source selection"]
    #[inline(always)]
    pub fn comp3sel(&self) -> COMP3SEL_R {
        COMP3SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 18:19 - LPTIM1 clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 6 - CAN detect clock source selection"]
    #[inline(always)]
    pub fn cansel(&mut self) -> CANSEL_W<6> {
        CANSEL_W::new(self)
    }
    #[doc = "Bit 7 - PVD detect clock source selection"]
    #[inline(always)]
    pub fn pvdsel(&mut self) -> PVDSEL_W<7> {
        PVDSEL_W::new(self)
    }
    #[doc = "Bit 8 - COMP1 clock source selection"]
    #[inline(always)]
    pub fn comp1sel(&mut self) -> COMP1SEL_W<8> {
        COMP1SEL_W::new(self)
    }
    #[doc = "Bit 9 - COMP2 clock source selection"]
    #[inline(always)]
    pub fn comp2sel(&mut self) -> COMP2SEL_W<9> {
        COMP2SEL_W::new(self)
    }
    #[doc = "Bit 10 - COMP3 clock source selection"]
    #[inline(always)]
    pub fn comp3sel(&mut self) -> COMP3SEL_W<10> {
        COMP3SEL_W::new(self)
    }
    #[doc = "Bits 18:19 - LPTIM1 clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<18> {
        LPTIM1SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripherals independent clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccipr](index.html) module"]
pub struct CCIPR_SPEC;
impl crate::RegisterSpec for CCIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccipr::R](R) reader structure"]
impl crate::Readable for CCIPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccipr::W](W) writer structure"]
impl crate::Writable for CCIPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCIPR to value 0"]
impl crate::Resettable for CCIPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
