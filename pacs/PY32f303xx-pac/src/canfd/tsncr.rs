#[doc = "Register `TSNCR` reader"]
pub struct R(crate::R<TSNCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSNCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSNCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSNCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSNCR` writer"]
pub struct W(crate::W<TSNCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSNCR_SPEC>;
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
impl From<crate::W<TSNCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSNCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VERSION` reader - desc VERSION"]
pub type VERSION_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CES` reader - desc CES"]
pub type CES_R = crate::BitReader<bool>;
#[doc = "Field `CES` writer - desc CES"]
pub type CES_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSNCR_SPEC, bool, O>;
#[doc = "Field `ROP` reader - desc ROP"]
pub type ROP_R = crate::BitReader<bool>;
#[doc = "Field `ROP` writer - desc ROP"]
pub type ROP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSNCR_SPEC, bool, O>;
#[doc = "Field `TMSE` reader - desc TMSE"]
pub type TMSE_R = crate::BitReader<bool>;
#[doc = "Field `TMSE` writer - desc TMSE"]
pub type TMSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSNCR_SPEC, bool, O>;
#[doc = "Field `TSEN` reader - desc TSEN"]
pub type TSEN_R = crate::BitReader<bool>;
#[doc = "Field `TSEN` writer - desc TSEN"]
pub type TSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSNCR_SPEC, bool, O>;
#[doc = "Field `TSPOS` reader - desc TSPOS"]
pub type TSPOS_R = crate::BitReader<bool>;
#[doc = "Field `TSPOS` writer - desc TSPOS"]
pub type TSPOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSNCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - desc VERSION"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - desc CES"]
    #[inline(always)]
    pub fn ces(&self) -> CES_R {
        CES_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc ROP"]
    #[inline(always)]
    pub fn rop(&self) -> ROP_R {
        ROP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc TMSE"]
    #[inline(always)]
    pub fn tmse(&self) -> TMSE_R {
        TMSE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - desc TSEN"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc TSPOS"]
    #[inline(always)]
    pub fn tspos(&self) -> TSPOS_R {
        TSPOS_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - desc CES"]
    #[inline(always)]
    pub fn ces(&mut self) -> CES_W<16> {
        CES_W::new(self)
    }
    #[doc = "Bit 17 - desc ROP"]
    #[inline(always)]
    pub fn rop(&mut self) -> ROP_W<17> {
        ROP_W::new(self)
    }
    #[doc = "Bit 18 - desc TMSE"]
    #[inline(always)]
    pub fn tmse(&mut self) -> TMSE_W<18> {
        TMSE_W::new(self)
    }
    #[doc = "Bit 24 - desc TSEN"]
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W<24> {
        TSEN_W::new(self)
    }
    #[doc = "Bit 25 - desc TSPOS"]
    #[inline(always)]
    pub fn tspos(&mut self) -> TSPOS_W<25> {
        TSPOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc TSNCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsncr](index.html) module"]
pub struct TSNCR_SPEC;
impl crate::RegisterSpec for TSNCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsncr::R](R) reader structure"]
impl crate::Readable for TSNCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsncr::W](W) writer structure"]
impl crate::Writable for TSNCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSNCR to value 0x0201_0801"]
impl crate::Resettable for TSNCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0201_0801
    }
}
