#[doc = "Register `DIER` reader"]
pub struct R(crate::R<DIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIER` writer"]
pub struct W(crate::W<DIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIER_SPEC>;
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
impl From<crate::W<DIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UIE` reader - desc UIE"]
pub type UIE_R = crate::BitReader<bool>;
#[doc = "Field `UIE` writer - desc UIE"]
pub type UIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, bool, O>;
#[doc = "Field `CC1IE` reader - desc CC1IE"]
pub type CC1IE_R = crate::BitReader<bool>;
#[doc = "Field `CC1IE` writer - desc CC1IE"]
pub type CC1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, bool, O>;
#[doc = "Field `CC2IE` reader - desc CC2IE"]
pub type CC2IE_R = crate::BitReader<bool>;
#[doc = "Field `CC2IE` writer - desc CC2IE"]
pub type CC2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, bool, O>;
#[doc = "Field `CC3IE` reader - desc CC3IE"]
pub type CC3IE_R = crate::BitReader<bool>;
#[doc = "Field `CC3IE` writer - desc CC3IE"]
pub type CC3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, bool, O>;
#[doc = "Field `CC4IE` reader - desc CC4IE"]
pub type CC4IE_R = crate::BitReader<bool>;
#[doc = "Field `CC4IE` writer - desc CC4IE"]
pub type CC4IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, bool, O>;
#[doc = "Field `TIE` reader - desc TIE"]
pub type TIE_R = crate::BitReader<bool>;
#[doc = "Field `TIE` writer - desc TIE"]
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, bool, O>;
#[doc = "Field `UDE` reader - desc UDE"]
pub type UDE_R = crate::BitReader<bool>;
#[doc = "Field `UDE` writer - desc UDE"]
pub type UDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, bool, O>;
#[doc = "Field `CC1DE` reader - desc CC1DE"]
pub type CC1DE_R = crate::BitReader<bool>;
#[doc = "Field `CC1DE` writer - desc CC1DE"]
pub type CC1DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, bool, O>;
#[doc = "Field `CC2DE` reader - desc CC2DE"]
pub type CC2DE_R = crate::BitReader<bool>;
#[doc = "Field `CC2DE` writer - desc CC2DE"]
pub type CC2DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, bool, O>;
#[doc = "Field `CC3DE` reader - desc CC3DE"]
pub type CC3DE_R = crate::BitReader<bool>;
#[doc = "Field `CC3DE` writer - desc CC3DE"]
pub type CC3DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, bool, O>;
#[doc = "Field `CC4DE` reader - desc CC4DE"]
pub type CC4DE_R = crate::BitReader<bool>;
#[doc = "Field `CC4DE` writer - desc CC4DE"]
pub type CC4DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, bool, O>;
#[doc = "Field `TDE` reader - desc TDE"]
pub type TDE_R = crate::BitReader<bool>;
#[doc = "Field `TDE` writer - desc TDE"]
pub type TDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc UIE"]
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CC1IE"]
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CC2IE"]
    #[inline(always)]
    pub fn cc2ie(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CC3IE"]
    #[inline(always)]
    pub fn cc3ie(&self) -> CC3IE_R {
        CC3IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CC4IE"]
    #[inline(always)]
    pub fn cc4ie(&self) -> CC4IE_R {
        CC4IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - desc TIE"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - desc UDE"]
    #[inline(always)]
    pub fn ude(&self) -> UDE_R {
        UDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc CC1DE"]
    #[inline(always)]
    pub fn cc1de(&self) -> CC1DE_R {
        CC1DE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc CC2DE"]
    #[inline(always)]
    pub fn cc2de(&self) -> CC2DE_R {
        CC2DE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc CC3DE"]
    #[inline(always)]
    pub fn cc3de(&self) -> CC3DE_R {
        CC3DE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc CC4DE"]
    #[inline(always)]
    pub fn cc4de(&self) -> CC4DE_R {
        CC4DE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - desc TDE"]
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc UIE"]
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W<0> {
        UIE_W::new(self)
    }
    #[doc = "Bit 1 - desc CC1IE"]
    #[inline(always)]
    pub fn cc1ie(&mut self) -> CC1IE_W<1> {
        CC1IE_W::new(self)
    }
    #[doc = "Bit 2 - desc CC2IE"]
    #[inline(always)]
    pub fn cc2ie(&mut self) -> CC2IE_W<2> {
        CC2IE_W::new(self)
    }
    #[doc = "Bit 3 - desc CC3IE"]
    #[inline(always)]
    pub fn cc3ie(&mut self) -> CC3IE_W<3> {
        CC3IE_W::new(self)
    }
    #[doc = "Bit 4 - desc CC4IE"]
    #[inline(always)]
    pub fn cc4ie(&mut self) -> CC4IE_W<4> {
        CC4IE_W::new(self)
    }
    #[doc = "Bit 6 - desc TIE"]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<6> {
        TIE_W::new(self)
    }
    #[doc = "Bit 8 - desc UDE"]
    #[inline(always)]
    pub fn ude(&mut self) -> UDE_W<8> {
        UDE_W::new(self)
    }
    #[doc = "Bit 9 - desc CC1DE"]
    #[inline(always)]
    pub fn cc1de(&mut self) -> CC1DE_W<9> {
        CC1DE_W::new(self)
    }
    #[doc = "Bit 10 - desc CC2DE"]
    #[inline(always)]
    pub fn cc2de(&mut self) -> CC2DE_W<10> {
        CC2DE_W::new(self)
    }
    #[doc = "Bit 11 - desc CC3DE"]
    #[inline(always)]
    pub fn cc3de(&mut self) -> CC3DE_W<11> {
        CC3DE_W::new(self)
    }
    #[doc = "Bit 12 - desc CC4DE"]
    #[inline(always)]
    pub fn cc4de(&mut self) -> CC4DE_W<12> {
        CC4DE_W::new(self)
    }
    #[doc = "Bit 14 - desc TDE"]
    #[inline(always)]
    pub fn tde(&mut self) -> TDE_W<14> {
        TDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc DIER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dier](index.html) module"]
pub struct DIER_SPEC;
impl crate::RegisterSpec for DIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dier::R](R) reader structure"]
impl crate::Readable for DIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dier::W](W) writer structure"]
impl crate::Writable for DIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIER to value 0"]
impl crate::Resettable for DIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
