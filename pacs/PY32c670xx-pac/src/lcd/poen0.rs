#[doc = "Register `POEN0` reader"]
pub struct R(crate::R<POEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POEN0` writer"]
pub struct W(crate::W<POEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POEN0_SPEC>;
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
impl From<crate::W<POEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `S0` reader - S0"]
pub type S0_R = crate::BitReader<bool>;
#[doc = "Field `S0` writer - S0"]
pub type S0_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN0_SPEC, bool, O>;
#[doc = "Field `S1` reader - S1"]
pub type S1_R = crate::BitReader<bool>;
#[doc = "Field `S1` writer - S1"]
pub type S1_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN0_SPEC, bool, O>;
#[doc = "Field `S2` reader - S2"]
pub type S2_R = crate::BitReader<bool>;
#[doc = "Field `S2` writer - S2"]
pub type S2_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN0_SPEC, bool, O>;
#[doc = "Field `S3` reader - S3"]
pub type S3_R = crate::BitReader<bool>;
#[doc = "Field `S3` writer - S3"]
pub type S3_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN0_SPEC, bool, O>;
#[doc = "Field `S4` reader - S4"]
pub type S4_R = crate::BitReader<bool>;
#[doc = "Field `S4` writer - S4"]
pub type S4_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN0_SPEC, bool, O>;
#[doc = "Field `S5` reader - S5"]
pub type S5_R = crate::BitReader<bool>;
#[doc = "Field `S5` writer - S5"]
pub type S5_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN0_SPEC, bool, O>;
#[doc = "Field `S6` reader - S6"]
pub type S6_R = crate::BitReader<bool>;
#[doc = "Field `S6` writer - S6"]
pub type S6_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN0_SPEC, bool, O>;
#[doc = "Field `S7` reader - S7"]
pub type S7_R = crate::BitReader<bool>;
#[doc = "Field `S7` writer - S7"]
pub type S7_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN0_SPEC, bool, O>;
#[doc = "Field `S8` reader - S8"]
pub type S8_R = crate::BitReader<bool>;
#[doc = "Field `S8` writer - S8"]
pub type S8_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN0_SPEC, bool, O>;
#[doc = "Field `S9` reader - S9"]
pub type S9_R = crate::BitReader<bool>;
#[doc = "Field `S9` writer - S9"]
pub type S9_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN0_SPEC, bool, O>;
#[doc = "Field `S10` reader - S10"]
pub type S10_R = crate::BitReader<bool>;
#[doc = "Field `S10` writer - S10"]
pub type S10_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN0_SPEC, bool, O>;
#[doc = "Field `S11` reader - S11"]
pub type S11_R = crate::BitReader<bool>;
#[doc = "Field `S11` writer - S11"]
pub type S11_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN0_SPEC, bool, O>;
#[doc = "Field `S12` reader - S12"]
pub type S12_R = crate::BitReader<bool>;
#[doc = "Field `S12` writer - S12"]
pub type S12_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN0_SPEC, bool, O>;
#[doc = "Field `S13` reader - S13"]
pub type S13_R = crate::BitReader<bool>;
#[doc = "Field `S13` writer - S13"]
pub type S13_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - S0"]
    #[inline(always)]
    pub fn s0(&self) -> S0_R {
        S0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - S1"]
    #[inline(always)]
    pub fn s1(&self) -> S1_R {
        S1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - S2"]
    #[inline(always)]
    pub fn s2(&self) -> S2_R {
        S2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - S3"]
    #[inline(always)]
    pub fn s3(&self) -> S3_R {
        S3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - S4"]
    #[inline(always)]
    pub fn s4(&self) -> S4_R {
        S4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - S5"]
    #[inline(always)]
    pub fn s5(&self) -> S5_R {
        S5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - S6"]
    #[inline(always)]
    pub fn s6(&self) -> S6_R {
        S6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - S7"]
    #[inline(always)]
    pub fn s7(&self) -> S7_R {
        S7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - S8"]
    #[inline(always)]
    pub fn s8(&self) -> S8_R {
        S8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - S9"]
    #[inline(always)]
    pub fn s9(&self) -> S9_R {
        S9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - S10"]
    #[inline(always)]
    pub fn s10(&self) -> S10_R {
        S10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - S11"]
    #[inline(always)]
    pub fn s11(&self) -> S11_R {
        S11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - S12"]
    #[inline(always)]
    pub fn s12(&self) -> S12_R {
        S12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - S13"]
    #[inline(always)]
    pub fn s13(&self) -> S13_R {
        S13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - S0"]
    #[inline(always)]
    pub fn s0(&mut self) -> S0_W<0> {
        S0_W::new(self)
    }
    #[doc = "Bit 1 - S1"]
    #[inline(always)]
    pub fn s1(&mut self) -> S1_W<1> {
        S1_W::new(self)
    }
    #[doc = "Bit 2 - S2"]
    #[inline(always)]
    pub fn s2(&mut self) -> S2_W<2> {
        S2_W::new(self)
    }
    #[doc = "Bit 3 - S3"]
    #[inline(always)]
    pub fn s3(&mut self) -> S3_W<3> {
        S3_W::new(self)
    }
    #[doc = "Bit 4 - S4"]
    #[inline(always)]
    pub fn s4(&mut self) -> S4_W<4> {
        S4_W::new(self)
    }
    #[doc = "Bit 5 - S5"]
    #[inline(always)]
    pub fn s5(&mut self) -> S5_W<5> {
        S5_W::new(self)
    }
    #[doc = "Bit 6 - S6"]
    #[inline(always)]
    pub fn s6(&mut self) -> S6_W<6> {
        S6_W::new(self)
    }
    #[doc = "Bit 7 - S7"]
    #[inline(always)]
    pub fn s7(&mut self) -> S7_W<7> {
        S7_W::new(self)
    }
    #[doc = "Bit 8 - S8"]
    #[inline(always)]
    pub fn s8(&mut self) -> S8_W<8> {
        S8_W::new(self)
    }
    #[doc = "Bit 9 - S9"]
    #[inline(always)]
    pub fn s9(&mut self) -> S9_W<9> {
        S9_W::new(self)
    }
    #[doc = "Bit 10 - S10"]
    #[inline(always)]
    pub fn s10(&mut self) -> S10_W<10> {
        S10_W::new(self)
    }
    #[doc = "Bit 11 - S11"]
    #[inline(always)]
    pub fn s11(&mut self) -> S11_W<11> {
        S11_W::new(self)
    }
    #[doc = "Bit 12 - S12"]
    #[inline(always)]
    pub fn s12(&mut self) -> S12_W<12> {
        S12_W::new(self)
    }
    #[doc = "Bit 13 - S13"]
    #[inline(always)]
    pub fn s13(&mut self) -> S13_W<13> {
        S13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "POEN0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [poen0](index.html) module"]
pub struct POEN0_SPEC;
impl crate::RegisterSpec for POEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [poen0::R](R) reader structure"]
impl crate::Readable for POEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [poen0::W](W) writer structure"]
impl crate::Writable for POEN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POEN0 to value 0"]
impl crate::Resettable for POEN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
