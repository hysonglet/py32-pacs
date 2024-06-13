#[doc = "Register `CCER` reader"]
pub struct R(crate::R<CCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCER` writer"]
pub struct W(crate::W<CCER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCER_SPEC>;
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
impl From<crate::W<CCER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC1E` reader - desc CC1E"]
pub type CC1E_R = crate::BitReader<bool>;
#[doc = "Field `CC1E` writer - desc CC1E"]
pub type CC1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
#[doc = "Field `CC1P` reader - desc CC1P"]
pub type CC1P_R = crate::BitReader<bool>;
#[doc = "Field `CC1P` writer - desc CC1P"]
pub type CC1P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
#[doc = "Field `CC1NE` reader - desc CC1NE"]
pub type CC1NE_R = crate::BitReader<bool>;
#[doc = "Field `CC1NE` writer - desc CC1NE"]
pub type CC1NE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
#[doc = "Field `CC1NP` reader - desc CC1NP"]
pub type CC1NP_R = crate::BitReader<bool>;
#[doc = "Field `CC1NP` writer - desc CC1NP"]
pub type CC1NP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
#[doc = "Field `CC2E` reader - desc CC2E"]
pub type CC2E_R = crate::BitReader<bool>;
#[doc = "Field `CC2E` writer - desc CC2E"]
pub type CC2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
#[doc = "Field `CC2P` reader - desc CC2P"]
pub type CC2P_R = crate::BitReader<bool>;
#[doc = "Field `CC2P` writer - desc CC2P"]
pub type CC2P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
#[doc = "Field `CC2NE` reader - desc CC2NE"]
pub type CC2NE_R = crate::BitReader<bool>;
#[doc = "Field `CC2NE` writer - desc CC2NE"]
pub type CC2NE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
#[doc = "Field `CC2NP` reader - desc CC2NP"]
pub type CC2NP_R = crate::BitReader<bool>;
#[doc = "Field `CC2NP` writer - desc CC2NP"]
pub type CC2NP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
#[doc = "Field `CC3E` reader - desc CC3E"]
pub type CC3E_R = crate::BitReader<bool>;
#[doc = "Field `CC3E` writer - desc CC3E"]
pub type CC3E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
#[doc = "Field `CC3P` reader - desc CC3P"]
pub type CC3P_R = crate::BitReader<bool>;
#[doc = "Field `CC3P` writer - desc CC3P"]
pub type CC3P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
#[doc = "Field `CC3NE` reader - desc CC3NE"]
pub type CC3NE_R = crate::BitReader<bool>;
#[doc = "Field `CC3NE` writer - desc CC3NE"]
pub type CC3NE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
#[doc = "Field `CC3NP` reader - desc CC3NP"]
pub type CC3NP_R = crate::BitReader<bool>;
#[doc = "Field `CC3NP` writer - desc CC3NP"]
pub type CC3NP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
#[doc = "Field `CC4E` reader - desc CC4E"]
pub type CC4E_R = crate::BitReader<bool>;
#[doc = "Field `CC4E` writer - desc CC4E"]
pub type CC4E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
#[doc = "Field `CC4P` reader - desc CC4P"]
pub type CC4P_R = crate::BitReader<bool>;
#[doc = "Field `CC4P` writer - desc CC4P"]
pub type CC4P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc CC1E"]
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CC1P"]
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CC1NE"]
    #[inline(always)]
    pub fn cc1ne(&self) -> CC1NE_R {
        CC1NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CC1NP"]
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CC2E"]
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc CC2P"]
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc CC2NE"]
    #[inline(always)]
    pub fn cc2ne(&self) -> CC2NE_R {
        CC2NE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc CC2NP"]
    #[inline(always)]
    pub fn cc2np(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc CC3E"]
    #[inline(always)]
    pub fn cc3e(&self) -> CC3E_R {
        CC3E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc CC3P"]
    #[inline(always)]
    pub fn cc3p(&self) -> CC3P_R {
        CC3P_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc CC3NE"]
    #[inline(always)]
    pub fn cc3ne(&self) -> CC3NE_R {
        CC3NE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc CC3NP"]
    #[inline(always)]
    pub fn cc3np(&self) -> CC3NP_R {
        CC3NP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc CC4E"]
    #[inline(always)]
    pub fn cc4e(&self) -> CC4E_R {
        CC4E_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc CC4P"]
    #[inline(always)]
    pub fn cc4p(&self) -> CC4P_R {
        CC4P_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CC1E"]
    #[inline(always)]
    pub fn cc1e(&mut self) -> CC1E_W<0> {
        CC1E_W::new(self)
    }
    #[doc = "Bit 1 - desc CC1P"]
    #[inline(always)]
    pub fn cc1p(&mut self) -> CC1P_W<1> {
        CC1P_W::new(self)
    }
    #[doc = "Bit 2 - desc CC1NE"]
    #[inline(always)]
    pub fn cc1ne(&mut self) -> CC1NE_W<2> {
        CC1NE_W::new(self)
    }
    #[doc = "Bit 3 - desc CC1NP"]
    #[inline(always)]
    pub fn cc1np(&mut self) -> CC1NP_W<3> {
        CC1NP_W::new(self)
    }
    #[doc = "Bit 4 - desc CC2E"]
    #[inline(always)]
    pub fn cc2e(&mut self) -> CC2E_W<4> {
        CC2E_W::new(self)
    }
    #[doc = "Bit 5 - desc CC2P"]
    #[inline(always)]
    pub fn cc2p(&mut self) -> CC2P_W<5> {
        CC2P_W::new(self)
    }
    #[doc = "Bit 6 - desc CC2NE"]
    #[inline(always)]
    pub fn cc2ne(&mut self) -> CC2NE_W<6> {
        CC2NE_W::new(self)
    }
    #[doc = "Bit 7 - desc CC2NP"]
    #[inline(always)]
    pub fn cc2np(&mut self) -> CC2NP_W<7> {
        CC2NP_W::new(self)
    }
    #[doc = "Bit 8 - desc CC3E"]
    #[inline(always)]
    pub fn cc3e(&mut self) -> CC3E_W<8> {
        CC3E_W::new(self)
    }
    #[doc = "Bit 9 - desc CC3P"]
    #[inline(always)]
    pub fn cc3p(&mut self) -> CC3P_W<9> {
        CC3P_W::new(self)
    }
    #[doc = "Bit 10 - desc CC3NE"]
    #[inline(always)]
    pub fn cc3ne(&mut self) -> CC3NE_W<10> {
        CC3NE_W::new(self)
    }
    #[doc = "Bit 11 - desc CC3NP"]
    #[inline(always)]
    pub fn cc3np(&mut self) -> CC3NP_W<11> {
        CC3NP_W::new(self)
    }
    #[doc = "Bit 12 - desc CC4E"]
    #[inline(always)]
    pub fn cc4e(&mut self) -> CC4E_W<12> {
        CC4E_W::new(self)
    }
    #[doc = "Bit 13 - desc CC4P"]
    #[inline(always)]
    pub fn cc4p(&mut self) -> CC4P_W<13> {
        CC4P_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CCER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccer](index.html) module"]
pub struct CCER_SPEC;
impl crate::RegisterSpec for CCER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccer::R](R) reader structure"]
impl crate::Readable for CCER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccer::W](W) writer structure"]
impl crate::Writable for CCER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCER to value 0"]
impl crate::Resettable for CCER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
