#[doc = "Register `DR0` reader"]
pub struct R(crate::R<DR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DR0` writer"]
pub struct W(crate::W<DR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR0_SPEC>;
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
impl From<crate::W<DR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA0_A` reader - 8-bit data register"]
pub type DATA0_A_R = crate::BitReader<bool>;
#[doc = "Field `DATA0_A` writer - 8-bit data register"]
pub type DATA0_A_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR0_SPEC, bool, O>;
#[doc = "Field `DATA0_B` reader - 8-bit data register"]
pub type DATA0_B_R = crate::BitReader<bool>;
#[doc = "Field `DATA0_B` writer - 8-bit data register"]
pub type DATA0_B_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR0_SPEC, bool, O>;
#[doc = "Field `DATA0_C` reader - 8-bit data register"]
pub type DATA0_C_R = crate::BitReader<bool>;
#[doc = "Field `DATA0_C` writer - 8-bit data register"]
pub type DATA0_C_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR0_SPEC, bool, O>;
#[doc = "Field `DATA0_D` reader - 8-bit data register"]
pub type DATA0_D_R = crate::BitReader<bool>;
#[doc = "Field `DATA0_D` writer - 8-bit data register"]
pub type DATA0_D_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR0_SPEC, bool, O>;
#[doc = "Field `DATA0_E` reader - 8-bit data register"]
pub type DATA0_E_R = crate::BitReader<bool>;
#[doc = "Field `DATA0_E` writer - 8-bit data register"]
pub type DATA0_E_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR0_SPEC, bool, O>;
#[doc = "Field `DATA0_F` reader - 8-bit data register"]
pub type DATA0_F_R = crate::BitReader<bool>;
#[doc = "Field `DATA0_F` writer - 8-bit data register"]
pub type DATA0_F_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR0_SPEC, bool, O>;
#[doc = "Field `DATA0_G` reader - 8-bit data register"]
pub type DATA0_G_R = crate::BitReader<bool>;
#[doc = "Field `DATA0_G` writer - 8-bit data register"]
pub type DATA0_G_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR0_SPEC, bool, O>;
#[doc = "Field `DATA0_DP` reader - 8-bit data register"]
pub type DATA0_DP_R = crate::BitReader<bool>;
#[doc = "Field `DATA0_DP` writer - 8-bit data register"]
pub type DATA0_DP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_a(&self) -> DATA0_A_R {
        DATA0_A_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_b(&self) -> DATA0_B_R {
        DATA0_B_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_c(&self) -> DATA0_C_R {
        DATA0_C_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_d(&self) -> DATA0_D_R {
        DATA0_D_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_e(&self) -> DATA0_E_R {
        DATA0_E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_f(&self) -> DATA0_F_R {
        DATA0_F_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_g(&self) -> DATA0_G_R {
        DATA0_G_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_dp(&self) -> DATA0_DP_R {
        DATA0_DP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_a(&mut self) -> DATA0_A_W<0> {
        DATA0_A_W::new(self)
    }
    #[doc = "Bit 1 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_b(&mut self) -> DATA0_B_W<1> {
        DATA0_B_W::new(self)
    }
    #[doc = "Bit 2 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_c(&mut self) -> DATA0_C_W<2> {
        DATA0_C_W::new(self)
    }
    #[doc = "Bit 3 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_d(&mut self) -> DATA0_D_W<3> {
        DATA0_D_W::new(self)
    }
    #[doc = "Bit 4 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_e(&mut self) -> DATA0_E_W<4> {
        DATA0_E_W::new(self)
    }
    #[doc = "Bit 5 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_f(&mut self) -> DATA0_F_W<5> {
        DATA0_F_W::new(self)
    }
    #[doc = "Bit 6 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_g(&mut self) -> DATA0_G_W<6> {
        DATA0_G_W::new(self)
    }
    #[doc = "Bit 7 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_dp(&mut self) -> DATA0_DP_W<7> {
        DATA0_DP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr0](index.html) module"]
pub struct DR0_SPEC;
impl crate::RegisterSpec for DR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr0::R](R) reader structure"]
impl crate::Readable for DR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dr0::W](W) writer structure"]
impl crate::Writable for DR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DR0 to value 0"]
impl crate::Resettable for DR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
