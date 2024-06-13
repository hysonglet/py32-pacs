#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Suspend` reader - Suspend"]
pub type SUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `Suspend` writer - Suspend"]
pub type SUSPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `Resume` reader - Resume"]
pub type RESUME_R = crate::BitReader<bool>;
#[doc = "Field `Resume` writer - Resume"]
pub type RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `Reset` reader - Reset"]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `Reset` writer - Reset"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `SOF` reader - SOF"]
pub type SOF_R = crate::BitReader<bool>;
#[doc = "Field `SOF` writer - SOF"]
pub type SOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `EP1OUT` reader - EP1OUT"]
pub type EP1OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP1OUT` writer - EP1OUT"]
pub type EP1OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `EP2OUT` reader - EP2OUT"]
pub type EP2OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP2OUT` writer - EP2OUT"]
pub type EP2OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `EP3OUT` reader - EP3OUT"]
pub type EP3OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP3OUT` writer - EP3OUT"]
pub type EP3OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `EP4OUT` reader - EP4OUT"]
pub type EP4OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP4OUT` writer - EP4OUT"]
pub type EP4OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `EP5OUT` reader - EP5OUT"]
pub type EP5OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP5OUT` writer - EP5OUT"]
pub type EP5OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `EP0` reader - EP0"]
pub type EP0_R = crate::BitReader<bool>;
#[doc = "Field `EP0` writer - EP0"]
pub type EP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `EP1IN` reader - EP1IN"]
pub type EP1IN_R = crate::BitReader<bool>;
#[doc = "Field `EP1IN` writer - EP1IN"]
pub type EP1IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `EP2IN` reader - EP2IN"]
pub type EP2IN_R = crate::BitReader<bool>;
#[doc = "Field `EP2IN` writer - EP2IN"]
pub type EP2IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `EP3IN` reader - EP3IN"]
pub type EP3IN_R = crate::BitReader<bool>;
#[doc = "Field `EP3IN` writer - EP3IN"]
pub type EP3IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `EP4IN` reader - EP4IN"]
pub type EP4IN_R = crate::BitReader<bool>;
#[doc = "Field `EP4IN` writer - EP4IN"]
pub type EP4IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `EP5IN` reader - EP5IN"]
pub type EP5IN_R = crate::BitReader<bool>;
#[doc = "Field `EP5IN` writer - EP5IN"]
pub type EP5IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Suspend"]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Resume"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SOF"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - EP1OUT"]
    #[inline(always)]
    pub fn ep1out(&self) -> EP1OUT_R {
        EP1OUT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EP2OUT"]
    #[inline(always)]
    pub fn ep2out(&self) -> EP2OUT_R {
        EP2OUT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EP3OUT"]
    #[inline(always)]
    pub fn ep3out(&self) -> EP3OUT_R {
        EP3OUT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EP4OUT"]
    #[inline(always)]
    pub fn ep4out(&self) -> EP4OUT_R {
        EP4OUT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EP5OUT"]
    #[inline(always)]
    pub fn ep5out(&self) -> EP5OUT_R {
        EP5OUT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - EP0"]
    #[inline(always)]
    pub fn ep0(&self) -> EP0_R {
        EP0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - EP1IN"]
    #[inline(always)]
    pub fn ep1in(&self) -> EP1IN_R {
        EP1IN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - EP2IN"]
    #[inline(always)]
    pub fn ep2in(&self) -> EP2IN_R {
        EP2IN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - EP3IN"]
    #[inline(always)]
    pub fn ep3in(&self) -> EP3IN_R {
        EP3IN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - EP4IN"]
    #[inline(always)]
    pub fn ep4in(&self) -> EP4IN_R {
        EP4IN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - EP5IN"]
    #[inline(always)]
    pub fn ep5in(&self) -> EP5IN_R {
        EP5IN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Suspend"]
    #[inline(always)]
    pub fn suspend(&mut self) -> SUSPEND_W<0> {
        SUSPEND_W::new(self)
    }
    #[doc = "Bit 1 - Resume"]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W<1> {
        RESUME_W::new(self)
    }
    #[doc = "Bit 2 - Reset"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<2> {
        RESET_W::new(self)
    }
    #[doc = "Bit 3 - SOF"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W<3> {
        SOF_W::new(self)
    }
    #[doc = "Bit 9 - EP1OUT"]
    #[inline(always)]
    pub fn ep1out(&mut self) -> EP1OUT_W<9> {
        EP1OUT_W::new(self)
    }
    #[doc = "Bit 10 - EP2OUT"]
    #[inline(always)]
    pub fn ep2out(&mut self) -> EP2OUT_W<10> {
        EP2OUT_W::new(self)
    }
    #[doc = "Bit 11 - EP3OUT"]
    #[inline(always)]
    pub fn ep3out(&mut self) -> EP3OUT_W<11> {
        EP3OUT_W::new(self)
    }
    #[doc = "Bit 12 - EP4OUT"]
    #[inline(always)]
    pub fn ep4out(&mut self) -> EP4OUT_W<12> {
        EP4OUT_W::new(self)
    }
    #[doc = "Bit 13 - EP5OUT"]
    #[inline(always)]
    pub fn ep5out(&mut self) -> EP5OUT_W<13> {
        EP5OUT_W::new(self)
    }
    #[doc = "Bit 16 - EP0"]
    #[inline(always)]
    pub fn ep0(&mut self) -> EP0_W<16> {
        EP0_W::new(self)
    }
    #[doc = "Bit 17 - EP1IN"]
    #[inline(always)]
    pub fn ep1in(&mut self) -> EP1IN_W<17> {
        EP1IN_W::new(self)
    }
    #[doc = "Bit 18 - EP2IN"]
    #[inline(always)]
    pub fn ep2in(&mut self) -> EP2IN_W<18> {
        EP2IN_W::new(self)
    }
    #[doc = "Bit 19 - EP3IN"]
    #[inline(always)]
    pub fn ep3in(&mut self) -> EP3IN_W<19> {
        EP3IN_W::new(self)
    }
    #[doc = "Bit 20 - EP4IN"]
    #[inline(always)]
    pub fn ep4in(&mut self) -> EP4IN_W<20> {
        EP4IN_W::new(self)
    }
    #[doc = "Bit 21 - EP5IN"]
    #[inline(always)]
    pub fn ep5in(&mut self) -> EP5IN_W<21> {
        EP5IN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
