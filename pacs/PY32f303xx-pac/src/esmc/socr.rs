#[doc = "Register `SOCR` reader"]
pub struct R(crate::R<SOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOCR` writer"]
pub struct W(crate::W<SOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOCR_SPEC>;
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
impl From<crate::W<SOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPIMODE` reader - desc SPIMODE"]
pub type SPIMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPIMODE` writer - desc SPIMODE"]
pub type SPIMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SOCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SENM` reader - desc SENM"]
pub type SENM_R = crate::BitReader<bool>;
#[doc = "Field `SENM` writer - desc SENM"]
pub type SENM_W<'a, const O: u8> = crate::BitWriter<'a, u8, SOCR_SPEC, bool, O>;
#[doc = "Field `MULTICMD` reader - desc MULTICMD"]
pub type MULTICMD_R = crate::BitReader<bool>;
#[doc = "Field `MULTICMD` writer - desc MULTICMD"]
pub type MULTICMD_W<'a, const O: u8> = crate::BitWriter<'a, u8, SOCR_SPEC, bool, O>;
#[doc = "Field `MULTIADDR` reader - desc MULTIADDR"]
pub type MULTIADDR_R = crate::BitReader<bool>;
#[doc = "Field `MULTIADDR` writer - desc MULTIADDR"]
pub type MULTIADDR_W<'a, const O: u8> = crate::BitWriter<'a, u8, SOCR_SPEC, bool, O>;
#[doc = "Field `DDRCMD` reader - desc DDRCMD"]
pub type DDRCMD_R = crate::BitReader<bool>;
#[doc = "Field `DDRCMD` writer - desc DDRCMD"]
pub type DDRCMD_W<'a, const O: u8> = crate::BitWriter<'a, u8, SOCR_SPEC, bool, O>;
#[doc = "Field `DDRADDR` reader - desc DDRADDR"]
pub type DDRADDR_R = crate::BitReader<bool>;
#[doc = "Field `DDRADDR` writer - desc DDRADDR"]
pub type DDRADDR_W<'a, const O: u8> = crate::BitWriter<'a, u8, SOCR_SPEC, bool, O>;
#[doc = "Field `DDRDATA` reader - desc DDRDATA"]
pub type DDRDATA_R = crate::BitReader<bool>;
#[doc = "Field `DDRDATA` writer - desc DDRDATA"]
pub type DDRDATA_W<'a, const O: u8> = crate::BitWriter<'a, u8, SOCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - desc SPIMODE"]
    #[inline(always)]
    pub fn spimode(&self) -> SPIMODE_R {
        SPIMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - desc SENM"]
    #[inline(always)]
    pub fn senm(&self) -> SENM_R {
        SENM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc MULTICMD"]
    #[inline(always)]
    pub fn multicmd(&self) -> MULTICMD_R {
        MULTICMD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc MULTIADDR"]
    #[inline(always)]
    pub fn multiaddr(&self) -> MULTIADDR_R {
        MULTIADDR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc DDRCMD"]
    #[inline(always)]
    pub fn ddrcmd(&self) -> DDRCMD_R {
        DDRCMD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc DDRADDR"]
    #[inline(always)]
    pub fn ddraddr(&self) -> DDRADDR_R {
        DDRADDR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc DDRDATA"]
    #[inline(always)]
    pub fn ddrdata(&self) -> DDRDATA_R {
        DDRDATA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc SPIMODE"]
    #[inline(always)]
    pub fn spimode(&mut self) -> SPIMODE_W<0> {
        SPIMODE_W::new(self)
    }
    #[doc = "Bit 2 - desc SENM"]
    #[inline(always)]
    pub fn senm(&mut self) -> SENM_W<2> {
        SENM_W::new(self)
    }
    #[doc = "Bit 3 - desc MULTICMD"]
    #[inline(always)]
    pub fn multicmd(&mut self) -> MULTICMD_W<3> {
        MULTICMD_W::new(self)
    }
    #[doc = "Bit 4 - desc MULTIADDR"]
    #[inline(always)]
    pub fn multiaddr(&mut self) -> MULTIADDR_W<4> {
        MULTIADDR_W::new(self)
    }
    #[doc = "Bit 5 - desc DDRCMD"]
    #[inline(always)]
    pub fn ddrcmd(&mut self) -> DDRCMD_W<5> {
        DDRCMD_W::new(self)
    }
    #[doc = "Bit 6 - desc DDRADDR"]
    #[inline(always)]
    pub fn ddraddr(&mut self) -> DDRADDR_W<6> {
        DDRADDR_W::new(self)
    }
    #[doc = "Bit 7 - desc DDRDATA"]
    #[inline(always)]
    pub fn ddrdata(&mut self) -> DDRDATA_W<7> {
        DDRDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc SOCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [socr](index.html) module"]
pub struct SOCR_SPEC;
impl crate::RegisterSpec for SOCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [socr::R](R) reader structure"]
impl crate::Readable for SOCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [socr::W](W) writer structure"]
impl crate::Writable for SOCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOCR to value 0x02"]
impl crate::Resettable for SOCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
