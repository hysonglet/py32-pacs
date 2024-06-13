#[doc = "Register `ICSCR` reader"]
pub struct R(crate::R<ICSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICSCR` writer"]
pub struct W(crate::W<ICSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICSCR_SPEC>;
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
impl From<crate::W<ICSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSI_TRIM` reader - HSI clock trimming"]
pub type HSI_TRIM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HSI_TRIM` writer - HSI clock trimming"]
pub type HSI_TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICSCR_SPEC, u16, u16, 13, O>;
#[doc = "Field `HSI_FS` reader - HSI frequency selection"]
pub type HSI_FS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSI_FS` writer - HSI frequency selection"]
pub type HSI_FS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICSCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `LSI_TRIM` reader - LSI clock trimming"]
pub type LSI_TRIM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LSI_TRIM` writer - LSI clock trimming"]
pub type LSI_TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICSCR_SPEC, u16, u16, 9, O>;
#[doc = "Field `LSI_STARTUP` reader - LSI startup time"]
pub type LSI_STARTUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LSI_STARTUP` writer - LSI startup time"]
pub type LSI_STARTUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICSCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:12 - HSI clock trimming"]
    #[inline(always)]
    pub fn hsi_trim(&self) -> HSI_TRIM_R {
        HSI_TRIM_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:15 - HSI frequency selection"]
    #[inline(always)]
    pub fn hsi_fs(&self) -> HSI_FS_R {
        HSI_FS_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:24 - LSI clock trimming"]
    #[inline(always)]
    pub fn lsi_trim(&self) -> LSI_TRIM_R {
        LSI_TRIM_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 26:27 - LSI startup time"]
    #[inline(always)]
    pub fn lsi_startup(&self) -> LSI_STARTUP_R {
        LSI_STARTUP_R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:12 - HSI clock trimming"]
    #[inline(always)]
    pub fn hsi_trim(&mut self) -> HSI_TRIM_W<0> {
        HSI_TRIM_W::new(self)
    }
    #[doc = "Bits 13:15 - HSI frequency selection"]
    #[inline(always)]
    pub fn hsi_fs(&mut self) -> HSI_FS_W<13> {
        HSI_FS_W::new(self)
    }
    #[doc = "Bits 16:24 - LSI clock trimming"]
    #[inline(always)]
    pub fn lsi_trim(&mut self) -> LSI_TRIM_W<16> {
        LSI_TRIM_W::new(self)
    }
    #[doc = "Bits 26:27 - LSI startup time"]
    #[inline(always)]
    pub fn lsi_startup(&mut self) -> LSI_STARTUP_W<26> {
        LSI_STARTUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal clock sources calibration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icscr](index.html) module"]
pub struct ICSCR_SPEC;
impl crate::RegisterSpec for ICSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icscr::R](R) reader structure"]
impl crate::Readable for ICSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icscr::W](W) writer structure"]
impl crate::Writable for ICSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICSCR to value 0x1000_0000"]
impl crate::Resettable for ICSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000_0000
    }
}
