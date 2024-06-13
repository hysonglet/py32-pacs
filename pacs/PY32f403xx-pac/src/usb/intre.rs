#[doc = "Register `INTRE` reader"]
pub struct R(crate::R<INTRE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTRE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTRE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTRE` writer"]
pub struct W(crate::W<INTRE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTRE_SPEC>;
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
impl From<crate::W<INTRE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTRE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN_SUSPEND` reader - desc EN_SUSPEND"]
pub type EN_SUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `EN_SUSPEND` writer - desc EN_SUSPEND"]
pub type EN_SUSPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRE_SPEC, bool, O>;
#[doc = "Field `EN_RESUME` reader - desc EN_RESUME"]
pub type EN_RESUME_R = crate::BitReader<bool>;
#[doc = "Field `EN_RESUME` writer - desc EN_RESUME"]
pub type EN_RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRE_SPEC, bool, O>;
#[doc = "Field `EN_RESET` reader - desc EN_RESET"]
pub type EN_RESET_R = crate::BitReader<bool>;
#[doc = "Field `EN_RESET` writer - desc EN_RESET"]
pub type EN_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRE_SPEC, bool, O>;
#[doc = "Field `EN_SOF` reader - desc EN_SOF"]
pub type EN_SOF_R = crate::BitReader<bool>;
#[doc = "Field `EN_SOF` writer - desc EN_SOF"]
pub type EN_SOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRE_SPEC, bool, O>;
#[doc = "Field `EP1OUTE` reader - desc EP1OUTE"]
pub type EP1OUTE_R = crate::BitReader<bool>;
#[doc = "Field `EP1OUTE` writer - desc EP1OUTE"]
pub type EP1OUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRE_SPEC, bool, O>;
#[doc = "Field `EP2OUTE` reader - desc EP2OUTE"]
pub type EP2OUTE_R = crate::BitReader<bool>;
#[doc = "Field `EP2OUTE` writer - desc EP2OUTE"]
pub type EP2OUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRE_SPEC, bool, O>;
#[doc = "Field `EP3OUTE` reader - desc EP3OUTE"]
pub type EP3OUTE_R = crate::BitReader<bool>;
#[doc = "Field `EP3OUTE` writer - desc EP3OUTE"]
pub type EP3OUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRE_SPEC, bool, O>;
#[doc = "Field `EP4OUTE` reader - desc EP4OUTE"]
pub type EP4OUTE_R = crate::BitReader<bool>;
#[doc = "Field `EP4OUTE` writer - desc EP4OUTE"]
pub type EP4OUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRE_SPEC, bool, O>;
#[doc = "Field `EP5OUTE` reader - desc EP5OUTE"]
pub type EP5OUTE_R = crate::BitReader<bool>;
#[doc = "Field `EP5OUTE` writer - desc EP5OUTE"]
pub type EP5OUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRE_SPEC, bool, O>;
#[doc = "Field `EP6OUTE` reader - desc EP6OUTE"]
pub type EP6OUTE_R = crate::BitReader<bool>;
#[doc = "Field `EP6OUTE` writer - desc EP6OUTE"]
pub type EP6OUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRE_SPEC, bool, O>;
#[doc = "Field `EP7OUTE` reader - desc EP7OUTE"]
pub type EP7OUTE_R = crate::BitReader<bool>;
#[doc = "Field `EP7OUTE` writer - desc EP7OUTE"]
pub type EP7OUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRE_SPEC, bool, O>;
#[doc = "Field `EP0E` reader - desc EP0E"]
pub type EP0E_R = crate::BitReader<bool>;
#[doc = "Field `EP0E` writer - desc EP0E"]
pub type EP0E_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRE_SPEC, bool, O>;
#[doc = "Field `EP1INE` reader - desc EP1INE"]
pub type EP1INE_R = crate::BitReader<bool>;
#[doc = "Field `EP1INE` writer - desc EP1INE"]
pub type EP1INE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRE_SPEC, bool, O>;
#[doc = "Field `EP2INE` reader - desc EP2INE"]
pub type EP2INE_R = crate::BitReader<bool>;
#[doc = "Field `EP2INE` writer - desc EP2INE"]
pub type EP2INE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRE_SPEC, bool, O>;
#[doc = "Field `EP3INE` reader - desc EP3INE"]
pub type EP3INE_R = crate::BitReader<bool>;
#[doc = "Field `EP3INE` writer - desc EP3INE"]
pub type EP3INE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRE_SPEC, bool, O>;
#[doc = "Field `EP4INE` reader - desc EP4INE"]
pub type EP4INE_R = crate::BitReader<bool>;
#[doc = "Field `EP4INE` writer - desc EP4INE"]
pub type EP4INE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRE_SPEC, bool, O>;
#[doc = "Field `EP5INE` reader - desc EP5INE"]
pub type EP5INE_R = crate::BitReader<bool>;
#[doc = "Field `EP5INE` writer - desc EP5INE"]
pub type EP5INE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRE_SPEC, bool, O>;
#[doc = "Field `EP6INE` reader - desc EP6INE"]
pub type EP6INE_R = crate::BitReader<bool>;
#[doc = "Field `EP6INE` writer - desc EP6INE"]
pub type EP6INE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRE_SPEC, bool, O>;
#[doc = "Field `EP7INE` reader - desc EP7INE"]
pub type EP7INE_R = crate::BitReader<bool>;
#[doc = "Field `EP7INE` writer - desc EP7INE"]
pub type EP7INE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc EN_SUSPEND"]
    #[inline(always)]
    pub fn en_suspend(&self) -> EN_SUSPEND_R {
        EN_SUSPEND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EN_RESUME"]
    #[inline(always)]
    pub fn en_resume(&self) -> EN_RESUME_R {
        EN_RESUME_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc EN_RESET"]
    #[inline(always)]
    pub fn en_reset(&self) -> EN_RESET_R {
        EN_RESET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc EN_SOF"]
    #[inline(always)]
    pub fn en_sof(&self) -> EN_SOF_R {
        EN_SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - desc EP1OUTE"]
    #[inline(always)]
    pub fn ep1oute(&self) -> EP1OUTE_R {
        EP1OUTE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc EP2OUTE"]
    #[inline(always)]
    pub fn ep2oute(&self) -> EP2OUTE_R {
        EP2OUTE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc EP3OUTE"]
    #[inline(always)]
    pub fn ep3oute(&self) -> EP3OUTE_R {
        EP3OUTE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc EP4OUTE"]
    #[inline(always)]
    pub fn ep4oute(&self) -> EP4OUTE_R {
        EP4OUTE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc EP5OUTE"]
    #[inline(always)]
    pub fn ep5oute(&self) -> EP5OUTE_R {
        EP5OUTE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc EP6OUTE"]
    #[inline(always)]
    pub fn ep6oute(&self) -> EP6OUTE_R {
        EP6OUTE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc EP7OUTE"]
    #[inline(always)]
    pub fn ep7oute(&self) -> EP7OUTE_R {
        EP7OUTE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc EP0E"]
    #[inline(always)]
    pub fn ep0e(&self) -> EP0E_R {
        EP0E_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc EP1INE"]
    #[inline(always)]
    pub fn ep1ine(&self) -> EP1INE_R {
        EP1INE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc EP2INE"]
    #[inline(always)]
    pub fn ep2ine(&self) -> EP2INE_R {
        EP2INE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc EP3INE"]
    #[inline(always)]
    pub fn ep3ine(&self) -> EP3INE_R {
        EP3INE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc EP4INE"]
    #[inline(always)]
    pub fn ep4ine(&self) -> EP4INE_R {
        EP4INE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc EP5INE"]
    #[inline(always)]
    pub fn ep5ine(&self) -> EP5INE_R {
        EP5INE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc EP6INE"]
    #[inline(always)]
    pub fn ep6ine(&self) -> EP6INE_R {
        EP6INE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc EP7INE"]
    #[inline(always)]
    pub fn ep7ine(&self) -> EP7INE_R {
        EP7INE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN_SUSPEND"]
    #[inline(always)]
    pub fn en_suspend(&mut self) -> EN_SUSPEND_W<0> {
        EN_SUSPEND_W::new(self)
    }
    #[doc = "Bit 1 - desc EN_RESUME"]
    #[inline(always)]
    pub fn en_resume(&mut self) -> EN_RESUME_W<1> {
        EN_RESUME_W::new(self)
    }
    #[doc = "Bit 2 - desc EN_RESET"]
    #[inline(always)]
    pub fn en_reset(&mut self) -> EN_RESET_W<2> {
        EN_RESET_W::new(self)
    }
    #[doc = "Bit 3 - desc EN_SOF"]
    #[inline(always)]
    pub fn en_sof(&mut self) -> EN_SOF_W<3> {
        EN_SOF_W::new(self)
    }
    #[doc = "Bit 9 - desc EP1OUTE"]
    #[inline(always)]
    pub fn ep1oute(&mut self) -> EP1OUTE_W<9> {
        EP1OUTE_W::new(self)
    }
    #[doc = "Bit 10 - desc EP2OUTE"]
    #[inline(always)]
    pub fn ep2oute(&mut self) -> EP2OUTE_W<10> {
        EP2OUTE_W::new(self)
    }
    #[doc = "Bit 11 - desc EP3OUTE"]
    #[inline(always)]
    pub fn ep3oute(&mut self) -> EP3OUTE_W<11> {
        EP3OUTE_W::new(self)
    }
    #[doc = "Bit 12 - desc EP4OUTE"]
    #[inline(always)]
    pub fn ep4oute(&mut self) -> EP4OUTE_W<12> {
        EP4OUTE_W::new(self)
    }
    #[doc = "Bit 13 - desc EP5OUTE"]
    #[inline(always)]
    pub fn ep5oute(&mut self) -> EP5OUTE_W<13> {
        EP5OUTE_W::new(self)
    }
    #[doc = "Bit 14 - desc EP6OUTE"]
    #[inline(always)]
    pub fn ep6oute(&mut self) -> EP6OUTE_W<14> {
        EP6OUTE_W::new(self)
    }
    #[doc = "Bit 15 - desc EP7OUTE"]
    #[inline(always)]
    pub fn ep7oute(&mut self) -> EP7OUTE_W<15> {
        EP7OUTE_W::new(self)
    }
    #[doc = "Bit 16 - desc EP0E"]
    #[inline(always)]
    pub fn ep0e(&mut self) -> EP0E_W<16> {
        EP0E_W::new(self)
    }
    #[doc = "Bit 17 - desc EP1INE"]
    #[inline(always)]
    pub fn ep1ine(&mut self) -> EP1INE_W<17> {
        EP1INE_W::new(self)
    }
    #[doc = "Bit 18 - desc EP2INE"]
    #[inline(always)]
    pub fn ep2ine(&mut self) -> EP2INE_W<18> {
        EP2INE_W::new(self)
    }
    #[doc = "Bit 19 - desc EP3INE"]
    #[inline(always)]
    pub fn ep3ine(&mut self) -> EP3INE_W<19> {
        EP3INE_W::new(self)
    }
    #[doc = "Bit 20 - desc EP4INE"]
    #[inline(always)]
    pub fn ep4ine(&mut self) -> EP4INE_W<20> {
        EP4INE_W::new(self)
    }
    #[doc = "Bit 21 - desc EP5INE"]
    #[inline(always)]
    pub fn ep5ine(&mut self) -> EP5INE_W<21> {
        EP5INE_W::new(self)
    }
    #[doc = "Bit 22 - desc EP6INE"]
    #[inline(always)]
    pub fn ep6ine(&mut self) -> EP6INE_W<22> {
        EP6INE_W::new(self)
    }
    #[doc = "Bit 23 - desc EP7INE"]
    #[inline(always)]
    pub fn ep7ine(&mut self) -> EP7INE_W<23> {
        EP7INE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc INTRE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intre](index.html) module"]
pub struct INTRE_SPEC;
impl crate::RegisterSpec for INTRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intre::R](R) reader structure"]
impl crate::Readable for INTRE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intre::W](W) writer structure"]
impl crate::Writable for INTRE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTRE to value 0x00ff_fe06"]
impl crate::Resettable for INTRE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00ff_fe06
    }
}
