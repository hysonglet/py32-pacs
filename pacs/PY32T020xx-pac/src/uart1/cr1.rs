#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M` reader - Word length"]
pub type M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M` writer - Word length"]
pub type M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `STOP` reader - STOP bits"]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - STOP bits"]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `PCE` reader - Parity control enable"]
pub type PCE_R = crate::BitReader<bool>;
#[doc = "Field `PCE` writer - Parity control enable"]
pub type PCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `PS` reader - Parity selection"]
pub type PS_R = crate::BitReader<bool>;
#[doc = "Field `PS` writer - Parity selection"]
pub type PS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `SP` reader - "]
pub type SP_R = crate::BitReader<bool>;
#[doc = "Field `SP` writer - "]
pub type SP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `SBK` reader - "]
pub type SBK_R = crate::BitReader<bool>;
#[doc = "Field `SBK` writer - "]
pub type SBK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `SWAP` reader - "]
pub type SWAP_R = crate::BitReader<bool>;
#[doc = "Field `SWAP` writer - "]
pub type SWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `MSBFIRST` reader - "]
pub type MSBFIRST_R = crate::BitReader<bool>;
#[doc = "Field `MSBFIRST` writer - "]
pub type MSBFIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Word length"]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - STOP bits"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity control enable"]
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Parity selection"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sbk(&self) -> SBK_R {
        SBK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn msbfirst(&self) -> MSBFIRST_R {
        MSBFIRST_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Word length"]
    #[inline(always)]
    pub fn m(&mut self) -> M_W<0> {
        M_W::new(self)
    }
    #[doc = "Bit 2 - STOP bits"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<2> {
        STOP_W::new(self)
    }
    #[doc = "Bit 3 - Parity control enable"]
    #[inline(always)]
    pub fn pce(&mut self) -> PCE_W<3> {
        PCE_W::new(self)
    }
    #[doc = "Bit 4 - Parity selection"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W<4> {
        PS_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sp(&mut self) -> SP_W<5> {
        SP_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sbk(&mut self) -> SBK_W<6> {
        SBK_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swap(&mut self) -> SWAP_W<8> {
        SWAP_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn msbfirst(&mut self) -> MSBFIRST_W<9> {
        MSBFIRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
