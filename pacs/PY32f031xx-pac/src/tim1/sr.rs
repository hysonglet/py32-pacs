#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UIF` reader - Update interrupt flag"]
pub type UIF_R = crate::BitReader<bool>;
#[doc = "Field `UIF` writer - Update interrupt flag"]
pub type UIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CC1IF` reader - Capture/compare 1 interrupt flag"]
pub type CC1IF_R = crate::BitReader<bool>;
#[doc = "Field `CC1IF` writer - Capture/compare 1 interrupt flag"]
pub type CC1IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CC2IF` reader - Capture/Compare 2 interrupt flag"]
pub type CC2IF_R = crate::BitReader<bool>;
#[doc = "Field `CC2IF` writer - Capture/Compare 2 interrupt flag"]
pub type CC2IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CC3IF` reader - Capture/Compare 3 interrupt flag"]
pub type CC3IF_R = crate::BitReader<bool>;
#[doc = "Field `CC3IF` writer - Capture/Compare 3 interrupt flag"]
pub type CC3IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CC4IF` reader - Capture/Compare 4 interrupt flag"]
pub type CC4IF_R = crate::BitReader<bool>;
#[doc = "Field `CC4IF` writer - Capture/Compare 4 interrupt flag"]
pub type CC4IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `COMIF` reader - COM interrupt flag"]
pub type COMIF_R = crate::BitReader<bool>;
#[doc = "Field `COMIF` writer - COM interrupt flag"]
pub type COMIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `TIF` reader - Trigger interrupt flag"]
pub type TIF_R = crate::BitReader<bool>;
#[doc = "Field `TIF` writer - Trigger interrupt flag"]
pub type TIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `BIF` reader - Break interrupt flag"]
pub type BIF_R = crate::BitReader<bool>;
#[doc = "Field `BIF` writer - Break interrupt flag"]
pub type BIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CC1OF` reader - Capture/Compare 1 overcapture flag"]
pub type CC1OF_R = crate::BitReader<bool>;
#[doc = "Field `CC1OF` writer - Capture/Compare 1 overcapture flag"]
pub type CC1OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CC2OF` reader - Capture/compare 2 overcapture flag"]
pub type CC2OF_R = crate::BitReader<bool>;
#[doc = "Field `CC2OF` writer - Capture/compare 2 overcapture flag"]
pub type CC2OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CC3OF` reader - Capture/Compare 3 overcapture flag"]
pub type CC3OF_R = crate::BitReader<bool>;
#[doc = "Field `CC3OF` writer - Capture/Compare 3 overcapture flag"]
pub type CC3OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CC4OF` reader - Capture/Compare 4 overcapture flag"]
pub type CC4OF_R = crate::BitReader<bool>;
#[doc = "Field `CC4OF` writer - Capture/Compare 4 overcapture flag"]
pub type CC4OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `IC1IR` reader - Capture 1 Rise edge capture flag"]
pub type IC1IR_R = crate::BitReader<bool>;
#[doc = "Field `IC1IR` writer - Capture 1 Rise edge capture flag"]
pub type IC1IR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `IC2IR` reader - Capture 2 Rise edge capture flag"]
pub type IC2IR_R = crate::BitReader<bool>;
#[doc = "Field `IC2IR` writer - Capture 2 Rise edge capture flag"]
pub type IC2IR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `IC3IR` reader - Capture 3 Rise edge capture flag"]
pub type IC3IR_R = crate::BitReader<bool>;
#[doc = "Field `IC3IR` writer - Capture 3 Rise edge capture flag"]
pub type IC3IR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `IC4IR` reader - Capture 4 Rise edge capture flag"]
pub type IC4IR_R = crate::BitReader<bool>;
#[doc = "Field `IC4IR` writer - Capture 4 Rise edge capture flag"]
pub type IC4IR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `IC1IF` reader - Capture 1 Fall edge capture flag"]
pub type IC1IF_R = crate::BitReader<bool>;
#[doc = "Field `IC1IF` writer - Capture 1 Fall edge capture flag"]
pub type IC1IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `IC2IF` reader - Capture 2 Fall edge capture flag"]
pub type IC2IF_R = crate::BitReader<bool>;
#[doc = "Field `IC2IF` writer - Capture 2 Fall edge capture flag"]
pub type IC2IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `IC3IF` reader - Capture 3 Fall edge capture flag"]
pub type IC3IF_R = crate::BitReader<bool>;
#[doc = "Field `IC3IF` writer - Capture 3 Fall edge capture flag"]
pub type IC3IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `IC4IF` reader - Capture 4 Fall edge capture flag"]
pub type IC4IF_R = crate::BitReader<bool>;
#[doc = "Field `IC4IF` writer - Capture 4 Fall edge capture flag"]
pub type IC4IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag"]
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt flag"]
    #[inline(always)]
    pub fn cc2if(&self) -> CC2IF_R {
        CC2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt flag"]
    #[inline(always)]
    pub fn cc3if(&self) -> CC3IF_R {
        CC3IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt flag"]
    #[inline(always)]
    pub fn cc4if(&self) -> CC4IF_R {
        CC4IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline(always)]
    pub fn comif(&self) -> COMIF_R {
        COMIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag"]
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/compare 2 overcapture flag"]
    #[inline(always)]
    pub fn cc2of(&self) -> CC2OF_R {
        CC2OF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 overcapture flag"]
    #[inline(always)]
    pub fn cc3of(&self) -> CC3OF_R {
        CC3OF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 4 overcapture flag"]
    #[inline(always)]
    pub fn cc4of(&self) -> CC4OF_R {
        CC4OF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Capture 1 Rise edge capture flag"]
    #[inline(always)]
    pub fn ic1ir(&self) -> IC1IR_R {
        IC1IR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Capture 2 Rise edge capture flag"]
    #[inline(always)]
    pub fn ic2ir(&self) -> IC2IR_R {
        IC2IR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Capture 3 Rise edge capture flag"]
    #[inline(always)]
    pub fn ic3ir(&self) -> IC3IR_R {
        IC3IR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Capture 4 Rise edge capture flag"]
    #[inline(always)]
    pub fn ic4ir(&self) -> IC4IR_R {
        IC4IR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Capture 1 Fall edge capture flag"]
    #[inline(always)]
    pub fn ic1if(&self) -> IC1IF_R {
        IC1IF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Capture 2 Fall edge capture flag"]
    #[inline(always)]
    pub fn ic2if(&self) -> IC2IF_R {
        IC2IF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Capture 3 Fall edge capture flag"]
    #[inline(always)]
    pub fn ic3if(&self) -> IC3IF_R {
        IC3IF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Capture 4 Fall edge capture flag"]
    #[inline(always)]
    pub fn ic4if(&self) -> IC4IF_R {
        IC4IF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn uif(&mut self) -> UIF_W<0> {
        UIF_W::new(self)
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag"]
    #[inline(always)]
    pub fn cc1if(&mut self) -> CC1IF_W<1> {
        CC1IF_W::new(self)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt flag"]
    #[inline(always)]
    pub fn cc2if(&mut self) -> CC2IF_W<2> {
        CC2IF_W::new(self)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt flag"]
    #[inline(always)]
    pub fn cc3if(&mut self) -> CC3IF_W<3> {
        CC3IF_W::new(self)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt flag"]
    #[inline(always)]
    pub fn cc4if(&mut self) -> CC4IF_W<4> {
        CC4IF_W::new(self)
    }
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline(always)]
    pub fn comif(&mut self) -> COMIF_W<5> {
        COMIF_W::new(self)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn tif(&mut self) -> TIF_W<6> {
        TIF_W::new(self)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn bif(&mut self) -> BIF_W<7> {
        BIF_W::new(self)
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag"]
    #[inline(always)]
    pub fn cc1of(&mut self) -> CC1OF_W<9> {
        CC1OF_W::new(self)
    }
    #[doc = "Bit 10 - Capture/compare 2 overcapture flag"]
    #[inline(always)]
    pub fn cc2of(&mut self) -> CC2OF_W<10> {
        CC2OF_W::new(self)
    }
    #[doc = "Bit 11 - Capture/Compare 3 overcapture flag"]
    #[inline(always)]
    pub fn cc3of(&mut self) -> CC3OF_W<11> {
        CC3OF_W::new(self)
    }
    #[doc = "Bit 12 - Capture/Compare 4 overcapture flag"]
    #[inline(always)]
    pub fn cc4of(&mut self) -> CC4OF_W<12> {
        CC4OF_W::new(self)
    }
    #[doc = "Bit 16 - Capture 1 Rise edge capture flag"]
    #[inline(always)]
    pub fn ic1ir(&mut self) -> IC1IR_W<16> {
        IC1IR_W::new(self)
    }
    #[doc = "Bit 17 - Capture 2 Rise edge capture flag"]
    #[inline(always)]
    pub fn ic2ir(&mut self) -> IC2IR_W<17> {
        IC2IR_W::new(self)
    }
    #[doc = "Bit 18 - Capture 3 Rise edge capture flag"]
    #[inline(always)]
    pub fn ic3ir(&mut self) -> IC3IR_W<18> {
        IC3IR_W::new(self)
    }
    #[doc = "Bit 19 - Capture 4 Rise edge capture flag"]
    #[inline(always)]
    pub fn ic4ir(&mut self) -> IC4IR_W<19> {
        IC4IR_W::new(self)
    }
    #[doc = "Bit 20 - Capture 1 Fall edge capture flag"]
    #[inline(always)]
    pub fn ic1if(&mut self) -> IC1IF_W<20> {
        IC1IF_W::new(self)
    }
    #[doc = "Bit 21 - Capture 2 Fall edge capture flag"]
    #[inline(always)]
    pub fn ic2if(&mut self) -> IC2IF_W<21> {
        IC2IF_W::new(self)
    }
    #[doc = "Bit 22 - Capture 3 Fall edge capture flag"]
    #[inline(always)]
    pub fn ic3if(&mut self) -> IC3IF_W<22> {
        IC3IF_W::new(self)
    }
    #[doc = "Bit 23 - Capture 4 Fall edge capture flag"]
    #[inline(always)]
    pub fn ic4if(&mut self) -> IC4IF_W<23> {
        IC4IF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
