#[doc = "Register `AHB2ENR` reader"]
pub struct R(crate::R<AHB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2ENR` writer"]
pub struct W(crate::W<AHB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2ENR_SPEC>;
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
impl From<crate::W<AHB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOPAEN` reader - desc IOPAEN"]
pub type IOPAEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPAEN` writer - desc IOPAEN"]
pub type IOPAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `IOPBEN` reader - desc IOPBEN"]
pub type IOPBEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPBEN` writer - desc IOPBEN"]
pub type IOPBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `IOPCEN` reader - desc IOPCEN"]
pub type IOPCEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPCEN` writer - desc IOPCEN"]
pub type IOPCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `IOPDEN` reader - desc IOPDEN"]
pub type IOPDEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPDEN` writer - desc IOPDEN"]
pub type IOPDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `IOPEEN` reader - desc IOPEEN"]
pub type IOPEEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPEEN` writer - desc IOPEEN"]
pub type IOPEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - desc IOPAEN"]
    #[inline(always)]
    pub fn iopaen(&self) -> IOPAEN_R {
        IOPAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc IOPBEN"]
    #[inline(always)]
    pub fn iopben(&self) -> IOPBEN_R {
        IOPBEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc IOPCEN"]
    #[inline(always)]
    pub fn iopcen(&self) -> IOPCEN_R {
        IOPCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc IOPDEN"]
    #[inline(always)]
    pub fn iopden(&self) -> IOPDEN_R {
        IOPDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc IOPEEN"]
    #[inline(always)]
    pub fn iopeen(&self) -> IOPEEN_R {
        IOPEEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - desc IOPAEN"]
    #[inline(always)]
    pub fn iopaen(&mut self) -> IOPAEN_W<2> {
        IOPAEN_W::new(self)
    }
    #[doc = "Bit 3 - desc IOPBEN"]
    #[inline(always)]
    pub fn iopben(&mut self) -> IOPBEN_W<3> {
        IOPBEN_W::new(self)
    }
    #[doc = "Bit 4 - desc IOPCEN"]
    #[inline(always)]
    pub fn iopcen(&mut self) -> IOPCEN_W<4> {
        IOPCEN_W::new(self)
    }
    #[doc = "Bit 5 - desc IOPDEN"]
    #[inline(always)]
    pub fn iopden(&mut self) -> IOPDEN_W<5> {
        IOPDEN_W::new(self)
    }
    #[doc = "Bit 6 - desc IOPEEN"]
    #[inline(always)]
    pub fn iopeen(&mut self) -> IOPEEN_W<6> {
        IOPEEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB2 peripheral clock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2enr](index.html) module"]
pub struct AHB2ENR_SPEC;
impl crate::RegisterSpec for AHB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2enr::R](R) reader structure"]
impl crate::Readable for AHB2ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2enr::W](W) writer structure"]
impl crate::Writable for AHB2ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB2ENR to value 0"]
impl crate::Resettable for AHB2ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
