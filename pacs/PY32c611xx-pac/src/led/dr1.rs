#[doc = "Register `DR1` reader"]
pub struct R(crate::R<DR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DR1` writer"]
pub struct W(crate::W<DR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR1_SPEC>;
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
impl From<crate::W<DR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA1_A` reader - 8-bit data register"]
pub type DATA1_A_R = crate::BitReader<bool>;
#[doc = "Field `DATA1_A` writer - 8-bit data register"]
pub type DATA1_A_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR1_SPEC, bool, O>;
#[doc = "Field `DATA1_B` reader - 8-bit data register"]
pub type DATA1_B_R = crate::BitReader<bool>;
#[doc = "Field `DATA1_B` writer - 8-bit data register"]
pub type DATA1_B_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR1_SPEC, bool, O>;
#[doc = "Field `DATA1_C` reader - 8-bit data register"]
pub type DATA1_C_R = crate::BitReader<bool>;
#[doc = "Field `DATA1_C` writer - 8-bit data register"]
pub type DATA1_C_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR1_SPEC, bool, O>;
#[doc = "Field `DATA1_D` reader - 8-bit data register"]
pub type DATA1_D_R = crate::BitReader<bool>;
#[doc = "Field `DATA1_D` writer - 8-bit data register"]
pub type DATA1_D_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR1_SPEC, bool, O>;
#[doc = "Field `DATA1_E` reader - 8-bit data register"]
pub type DATA1_E_R = crate::BitReader<bool>;
#[doc = "Field `DATA1_E` writer - 8-bit data register"]
pub type DATA1_E_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR1_SPEC, bool, O>;
#[doc = "Field `DATA1_F` reader - 8-bit data register"]
pub type DATA1_F_R = crate::BitReader<bool>;
#[doc = "Field `DATA1_F` writer - 8-bit data register"]
pub type DATA1_F_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR1_SPEC, bool, O>;
#[doc = "Field `DATA1_G` reader - 8-bit data register"]
pub type DATA1_G_R = crate::BitReader<bool>;
#[doc = "Field `DATA1_G` writer - 8-bit data register"]
pub type DATA1_G_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR1_SPEC, bool, O>;
#[doc = "Field `DATA1_DP` reader - 8-bit data register"]
pub type DATA1_DP_R = crate::BitReader<bool>;
#[doc = "Field `DATA1_DP` writer - 8-bit data register"]
pub type DATA1_DP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_a(&self) -> DATA1_A_R {
        DATA1_A_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_b(&self) -> DATA1_B_R {
        DATA1_B_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_c(&self) -> DATA1_C_R {
        DATA1_C_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_d(&self) -> DATA1_D_R {
        DATA1_D_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_e(&self) -> DATA1_E_R {
        DATA1_E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_f(&self) -> DATA1_F_R {
        DATA1_F_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_g(&self) -> DATA1_G_R {
        DATA1_G_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_dp(&self) -> DATA1_DP_R {
        DATA1_DP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_a(&mut self) -> DATA1_A_W<0> {
        DATA1_A_W::new(self)
    }
    #[doc = "Bit 1 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_b(&mut self) -> DATA1_B_W<1> {
        DATA1_B_W::new(self)
    }
    #[doc = "Bit 2 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_c(&mut self) -> DATA1_C_W<2> {
        DATA1_C_W::new(self)
    }
    #[doc = "Bit 3 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_d(&mut self) -> DATA1_D_W<3> {
        DATA1_D_W::new(self)
    }
    #[doc = "Bit 4 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_e(&mut self) -> DATA1_E_W<4> {
        DATA1_E_W::new(self)
    }
    #[doc = "Bit 5 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_f(&mut self) -> DATA1_F_W<5> {
        DATA1_F_W::new(self)
    }
    #[doc = "Bit 6 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_g(&mut self) -> DATA1_G_W<6> {
        DATA1_G_W::new(self)
    }
    #[doc = "Bit 7 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_dp(&mut self) -> DATA1_DP_W<7> {
        DATA1_DP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr1](index.html) module"]
pub struct DR1_SPEC;
impl crate::RegisterSpec for DR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr1::R](R) reader structure"]
impl crate::Readable for DR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dr1::W](W) writer structure"]
impl crate::Writable for DR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DR1 to value 0"]
impl crate::Resettable for DR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
