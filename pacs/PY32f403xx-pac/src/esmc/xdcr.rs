#[doc = "Register `XDCR` reader"]
pub struct R(crate::R<XDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XDCR` writer"]
pub struct W(crate::W<XDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XDCR_SPEC>;
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
impl From<crate::W<XDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XDUMMY` reader - desc XDUMMY"]
pub type XDUMMY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XDUMMY` writer - desc XDUMMY"]
pub type XDUMMY_W<'a, const O: u8> = crate::FieldWriter<'a, u8, XDCR_SPEC, u8, u8, 5, O>;
#[doc = "Field `NOADDR` reader - desc NOADDR"]
pub type NOADDR_R = crate::BitReader<bool>;
#[doc = "Field `NOADDR` writer - desc NOADDR"]
pub type NOADDR_W<'a, const O: u8> = crate::BitWriter<'a, u8, XDCR_SPEC, bool, O>;
#[doc = "Field `NOCMD` reader - desc NOCMD"]
pub type NOCMD_R = crate::BitReader<bool>;
#[doc = "Field `NOCMD` writer - desc NOCMD"]
pub type NOCMD_W<'a, const O: u8> = crate::BitWriter<'a, u8, XDCR_SPEC, bool, O>;
#[doc = "Field `REC` reader - desc REC"]
pub type REC_R = crate::BitReader<bool>;
#[doc = "Field `REC` writer - desc REC"]
pub type REC_W<'a, const O: u8> = crate::BitWriter<'a, u8, XDCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - desc XDUMMY"]
    #[inline(always)]
    pub fn xdummy(&self) -> XDUMMY_R {
        XDUMMY_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - desc NOADDR"]
    #[inline(always)]
    pub fn noaddr(&self) -> NOADDR_R {
        NOADDR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc NOCMD"]
    #[inline(always)]
    pub fn nocmd(&self) -> NOCMD_R {
        NOCMD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc REC"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - desc XDUMMY"]
    #[inline(always)]
    pub fn xdummy(&mut self) -> XDUMMY_W<0> {
        XDUMMY_W::new(self)
    }
    #[doc = "Bit 5 - desc NOADDR"]
    #[inline(always)]
    pub fn noaddr(&mut self) -> NOADDR_W<5> {
        NOADDR_W::new(self)
    }
    #[doc = "Bit 6 - desc NOCMD"]
    #[inline(always)]
    pub fn nocmd(&mut self) -> NOCMD_W<6> {
        NOCMD_W::new(self)
    }
    #[doc = "Bit 7 - desc REC"]
    #[inline(always)]
    pub fn rec(&mut self) -> REC_W<7> {
        REC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc DCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdcr](index.html) module"]
pub struct XDCR_SPEC;
impl crate::RegisterSpec for XDCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [xdcr::R](R) reader structure"]
impl crate::Readable for XDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xdcr::W](W) writer structure"]
impl crate::Writable for XDCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XDCR to value 0"]
impl crate::Resettable for XDCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
