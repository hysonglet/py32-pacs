#[doc = "Register `POEN1` reader"]
pub struct R(crate::R<POEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POEN1` writer"]
pub struct W(crate::W<POEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POEN1_SPEC>;
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
impl From<crate::W<POEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `S17` reader - S17"]
pub type S17_R = crate::BitReader<bool>;
#[doc = "Field `S17` writer - S17"]
pub type S17_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S16` reader - S16"]
pub type S16_R = crate::BitReader<bool>;
#[doc = "Field `S16` writer - S16"]
pub type S16_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S15` reader - S15"]
pub type S15_R = crate::BitReader<bool>;
#[doc = "Field `S15` writer - S15"]
pub type S15_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S14` reader - S14"]
pub type S14_R = crate::BitReader<bool>;
#[doc = "Field `S14` writer - S14"]
pub type S14_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `C0` reader - C0"]
pub type C0_R = crate::BitReader<bool>;
#[doc = "Field `C0` writer - C0"]
pub type C0_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `C1` reader - C1"]
pub type C1_R = crate::BitReader<bool>;
#[doc = "Field `C1` writer - C1"]
pub type C1_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `C2` reader - C2"]
pub type C2_R = crate::BitReader<bool>;
#[doc = "Field `C2` writer - C2"]
pub type C2_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `C3` reader - C3"]
pub type C3_R = crate::BitReader<bool>;
#[doc = "Field `C3` writer - C3"]
pub type C3_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 4 - S17"]
    #[inline(always)]
    pub fn s17(&self) -> S17_R {
        S17_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - S16"]
    #[inline(always)]
    pub fn s16(&self) -> S16_R {
        S16_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - S15"]
    #[inline(always)]
    pub fn s15(&self) -> S15_R {
        S15_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - S14"]
    #[inline(always)]
    pub fn s14(&self) -> S14_R {
        S14_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - C0"]
    #[inline(always)]
    pub fn c0(&self) -> C0_R {
        C0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - C1"]
    #[inline(always)]
    pub fn c1(&self) -> C1_R {
        C1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - C2"]
    #[inline(always)]
    pub fn c2(&self) -> C2_R {
        C2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - C3"]
    #[inline(always)]
    pub fn c3(&self) -> C3_R {
        C3_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - S17"]
    #[inline(always)]
    pub fn s17(&mut self) -> S17_W<4> {
        S17_W::new(self)
    }
    #[doc = "Bit 5 - S16"]
    #[inline(always)]
    pub fn s16(&mut self) -> S16_W<5> {
        S16_W::new(self)
    }
    #[doc = "Bit 6 - S15"]
    #[inline(always)]
    pub fn s15(&mut self) -> S15_W<6> {
        S15_W::new(self)
    }
    #[doc = "Bit 7 - S14"]
    #[inline(always)]
    pub fn s14(&mut self) -> S14_W<7> {
        S14_W::new(self)
    }
    #[doc = "Bit 8 - C0"]
    #[inline(always)]
    pub fn c0(&mut self) -> C0_W<8> {
        C0_W::new(self)
    }
    #[doc = "Bit 9 - C1"]
    #[inline(always)]
    pub fn c1(&mut self) -> C1_W<9> {
        C1_W::new(self)
    }
    #[doc = "Bit 10 - C2"]
    #[inline(always)]
    pub fn c2(&mut self) -> C2_W<10> {
        C2_W::new(self)
    }
    #[doc = "Bit 11 - C3"]
    #[inline(always)]
    pub fn c3(&mut self) -> C3_W<11> {
        C3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "POEN1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [poen1](index.html) module"]
pub struct POEN1_SPEC;
impl crate::RegisterSpec for POEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [poen1::R](R) reader structure"]
impl crate::Readable for POEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [poen1::W](W) writer structure"]
impl crate::Writable for POEN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POEN1 to value 0"]
impl crate::Resettable for POEN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
