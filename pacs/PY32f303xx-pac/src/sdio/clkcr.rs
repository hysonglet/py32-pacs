#[doc = "Register `CLKCR` reader"]
pub struct R(crate::R<CLKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKCR` writer"]
pub struct W(crate::W<CLKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKCR_SPEC>;
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
impl From<crate::W<CLKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKDIV` reader - desc CLKDIV"]
pub type CLKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKDIV` writer - desc CLKDIV"]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `CLKEN` reader - desc CLKEN"]
pub type CLKEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKEN` writer - desc CLKEN"]
pub type CLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, bool, O>;
#[doc = "Field `PWRSAV` reader - desc PWRSAV"]
pub type PWRSAV_R = crate::BitReader<bool>;
#[doc = "Field `PWRSAV` writer - desc PWRSAV"]
pub type PWRSAV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, bool, O>;
#[doc = "Field `WIDBUS` reader - desc WIDBUS"]
pub type WIDBUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WIDBUS` writer - desc WIDBUS"]
pub type WIDBUS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SMPCLKSEL` reader - desc SMPCLKSEL"]
pub type SMPCLKSEL_R = crate::BitReader<bool>;
#[doc = "Field `SMPCLKSEL` writer - desc SMPCLKSEL"]
pub type SMPCLKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, bool, O>;
#[doc = "Field `SMPEN` reader - desc SMPEN"]
pub type SMPEN_R = crate::BitReader<bool>;
#[doc = "Field `SMPEN` writer - desc SMPEN"]
pub type SMPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, bool, O>;
#[doc = "Field `CKSEL` reader - desc CKSEL"]
pub type CKSEL_R = crate::BitReader<bool>;
#[doc = "Field `CKSEL` writer - desc CKSEL"]
pub type CKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - desc CLKDIV"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - desc CLKEN"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc PWRSAV"]
    #[inline(always)]
    pub fn pwrsav(&self) -> PWRSAV_R {
        PWRSAV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - desc WIDBUS"]
    #[inline(always)]
    pub fn widbus(&self) -> WIDBUS_R {
        WIDBUS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - desc SMPCLKSEL"]
    #[inline(always)]
    pub fn smpclksel(&self) -> SMPCLKSEL_R {
        SMPCLKSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc SMPEN"]
    #[inline(always)]
    pub fn smpen(&self) -> SMPEN_R {
        SMPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc CKSEL"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc CLKDIV"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<0> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bit 8 - desc CLKEN"]
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W<8> {
        CLKEN_W::new(self)
    }
    #[doc = "Bit 9 - desc PWRSAV"]
    #[inline(always)]
    pub fn pwrsav(&mut self) -> PWRSAV_W<9> {
        PWRSAV_W::new(self)
    }
    #[doc = "Bits 10:11 - desc WIDBUS"]
    #[inline(always)]
    pub fn widbus(&mut self) -> WIDBUS_W<10> {
        WIDBUS_W::new(self)
    }
    #[doc = "Bit 12 - desc SMPCLKSEL"]
    #[inline(always)]
    pub fn smpclksel(&mut self) -> SMPCLKSEL_W<12> {
        SMPCLKSEL_W::new(self)
    }
    #[doc = "Bit 13 - desc SMPEN"]
    #[inline(always)]
    pub fn smpen(&mut self) -> SMPEN_W<13> {
        SMPEN_W::new(self)
    }
    #[doc = "Bit 14 - desc CKSEL"]
    #[inline(always)]
    pub fn cksel(&mut self) -> CKSEL_W<14> {
        CKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CLKCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkcr](index.html) module"]
pub struct CLKCR_SPEC;
impl crate::RegisterSpec for CLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkcr::R](R) reader structure"]
impl crate::Readable for CLKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkcr::W](W) writer structure"]
impl crate::Writable for CLKCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKCR to value 0x7000"]
impl crate::Resettable for CLKCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7000
    }
}
