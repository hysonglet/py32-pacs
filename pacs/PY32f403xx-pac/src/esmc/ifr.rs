#[doc = "Register `IFR` reader"]
pub struct R(crate::R<IFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFR` writer"]
pub struct W(crate::W<IFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFR_SPEC>;
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
impl From<crate::W<IFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDIF` reader - desc CMDIF"]
pub type CMDIF_R = crate::BitReader<bool>;
#[doc = "Field `CMDIF` writer - desc CMDIF"]
pub type CMDIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, IFR_SPEC, bool, O>;
#[doc = "Field `FIFOEIF` reader - desc FIFOEIF"]
pub type FIFOEIF_R = crate::BitReader<bool>;
#[doc = "Field `FIFOEIF` writer - desc FIFOEIF"]
pub type FIFOEIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, IFR_SPEC, bool, O>;
#[doc = "Field `FIFOHIF` reader - desc FIFOHIF"]
pub type FIFOHIF_R = crate::BitReader<bool>;
#[doc = "Field `FIFOHIF` writer - desc FIFOHIF"]
pub type FIFOHIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, IFR_SPEC, bool, O>;
#[doc = "Field `FIFOFIF` reader - desc FIFOFIF"]
pub type FIFOFIF_R = crate::BitReader<bool>;
#[doc = "Field `FIFOFIF` writer - desc FIFOFIF"]
pub type FIFOFIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, IFR_SPEC, bool, O>;
#[doc = "Field `IDLEIF` reader - desc IDLEIF"]
pub type IDLEIF_R = crate::BitReader<bool>;
#[doc = "Field `IDLEIF` writer - desc IDLEIF"]
pub type IDLEIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, IFR_SPEC, bool, O>;
#[doc = "Field `DATAWAITIF` reader - desc DATAWAITIF"]
pub type DATAWAITIF_R = crate::BitReader<bool>;
#[doc = "Field `DATAWAITIF` writer - desc DATAWAITIF"]
pub type DATAWAITIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, IFR_SPEC, bool, O>;
#[doc = "Field `FIFOOIF` reader - desc FIFOOIF"]
pub type FIFOOIF_R = crate::BitReader<bool>;
#[doc = "Field `FIFOOIF` writer - desc FIFOOIF"]
pub type FIFOOIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, IFR_SPEC, bool, O>;
#[doc = "Field `ADDRDONEIF` reader - desc ADDRDONEIF"]
pub type ADDRDONEIF_R = crate::BitReader<bool>;
#[doc = "Field `ADDRDONEIF` writer - desc ADDRDONEIF"]
pub type ADDRDONEIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, IFR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc CMDIF"]
    #[inline(always)]
    pub fn cmdif(&self) -> CMDIF_R {
        CMDIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc FIFOEIF"]
    #[inline(always)]
    pub fn fifoeif(&self) -> FIFOEIF_R {
        FIFOEIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc FIFOHIF"]
    #[inline(always)]
    pub fn fifohif(&self) -> FIFOHIF_R {
        FIFOHIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc FIFOFIF"]
    #[inline(always)]
    pub fn fifofif(&self) -> FIFOFIF_R {
        FIFOFIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc IDLEIF"]
    #[inline(always)]
    pub fn idleif(&self) -> IDLEIF_R {
        IDLEIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc DATAWAITIF"]
    #[inline(always)]
    pub fn datawaitif(&self) -> DATAWAITIF_R {
        DATAWAITIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc FIFOOIF"]
    #[inline(always)]
    pub fn fifooif(&self) -> FIFOOIF_R {
        FIFOOIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc ADDRDONEIF"]
    #[inline(always)]
    pub fn addrdoneif(&self) -> ADDRDONEIF_R {
        ADDRDONEIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CMDIF"]
    #[inline(always)]
    pub fn cmdif(&mut self) -> CMDIF_W<0> {
        CMDIF_W::new(self)
    }
    #[doc = "Bit 1 - desc FIFOEIF"]
    #[inline(always)]
    pub fn fifoeif(&mut self) -> FIFOEIF_W<1> {
        FIFOEIF_W::new(self)
    }
    #[doc = "Bit 2 - desc FIFOHIF"]
    #[inline(always)]
    pub fn fifohif(&mut self) -> FIFOHIF_W<2> {
        FIFOHIF_W::new(self)
    }
    #[doc = "Bit 3 - desc FIFOFIF"]
    #[inline(always)]
    pub fn fifofif(&mut self) -> FIFOFIF_W<3> {
        FIFOFIF_W::new(self)
    }
    #[doc = "Bit 4 - desc IDLEIF"]
    #[inline(always)]
    pub fn idleif(&mut self) -> IDLEIF_W<4> {
        IDLEIF_W::new(self)
    }
    #[doc = "Bit 5 - desc DATAWAITIF"]
    #[inline(always)]
    pub fn datawaitif(&mut self) -> DATAWAITIF_W<5> {
        DATAWAITIF_W::new(self)
    }
    #[doc = "Bit 6 - desc FIFOOIF"]
    #[inline(always)]
    pub fn fifooif(&mut self) -> FIFOOIF_W<6> {
        FIFOOIF_W::new(self)
    }
    #[doc = "Bit 7 - desc ADDRDONEIF"]
    #[inline(always)]
    pub fn addrdoneif(&mut self) -> ADDRDONEIF_W<7> {
        ADDRDONEIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc IFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifr](index.html) module"]
pub struct IFR_SPEC;
impl crate::RegisterSpec for IFR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ifr::R](R) reader structure"]
impl crate::Readable for IFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifr::W](W) writer structure"]
impl crate::Writable for IFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFR to value 0x02"]
impl crate::Resettable for IFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
