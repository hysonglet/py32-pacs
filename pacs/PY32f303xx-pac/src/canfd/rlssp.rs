#[doc = "Register `RLSSP` reader"]
pub struct R(crate::R<RLSSP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RLSSP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RLSSP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RLSSP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RLSSP` writer"]
pub struct W(crate::W<RLSSP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RLSSP_SPEC>;
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
impl From<crate::W<RLSSP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RLSSP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESC` reader - desc PRESC"]
pub type PRESC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRESC` writer - desc PRESC"]
pub type PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RLSSP_SPEC, u8, u8, 5, O>;
#[doc = "Field `FD_SSPOFF` reader - desc FD_SSPOFF"]
pub type FD_SSPOFF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FD_SSPOFF` writer - desc FD_SSPOFF"]
pub type FD_SSPOFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RLSSP_SPEC, u8, u8, 8, O>;
#[doc = "Field `XL_SSPOFF` reader - desc XL_SSPOFF"]
pub type XL_SSPOFF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XL_SSPOFF` writer - desc XL_SSPOFF"]
pub type XL_SSPOFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RLSSP_SPEC, u8, u8, 8, O>;
#[doc = "Field `REALIM` reader - desc REALIM"]
pub type REALIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REALIM` writer - desc REALIM"]
pub type REALIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RLSSP_SPEC, u8, u8, 3, O>;
#[doc = "Field `RETLIM` reader - desc RETLIM"]
pub type RETLIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RETLIM` writer - desc RETLIM"]
pub type RETLIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RLSSP_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:4 - desc PRESC"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - desc FD_SSPOFF"]
    #[inline(always)]
    pub fn fd_sspoff(&self) -> FD_SSPOFF_R {
        FD_SSPOFF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - desc XL_SSPOFF"]
    #[inline(always)]
    pub fn xl_sspoff(&self) -> XL_SSPOFF_R {
        XL_SSPOFF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - desc REALIM"]
    #[inline(always)]
    pub fn realim(&self) -> REALIM_R {
        REALIM_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - desc RETLIM"]
    #[inline(always)]
    pub fn retlim(&self) -> RETLIM_R {
        RETLIM_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - desc PRESC"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W<0> {
        PRESC_W::new(self)
    }
    #[doc = "Bits 8:15 - desc FD_SSPOFF"]
    #[inline(always)]
    pub fn fd_sspoff(&mut self) -> FD_SSPOFF_W<8> {
        FD_SSPOFF_W::new(self)
    }
    #[doc = "Bits 16:23 - desc XL_SSPOFF"]
    #[inline(always)]
    pub fn xl_sspoff(&mut self) -> XL_SSPOFF_W<16> {
        XL_SSPOFF_W::new(self)
    }
    #[doc = "Bits 24:26 - desc REALIM"]
    #[inline(always)]
    pub fn realim(&mut self) -> REALIM_W<24> {
        REALIM_W::new(self)
    }
    #[doc = "Bits 28:30 - desc RETLIM"]
    #[inline(always)]
    pub fn retlim(&mut self) -> RETLIM_W<28> {
        RETLIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc RLSSP\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlssp](index.html) module"]
pub struct RLSSP_SPEC;
impl crate::RegisterSpec for RLSSP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rlssp::R](R) reader structure"]
impl crate::Readable for RLSSP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rlssp::W](W) writer structure"]
impl crate::Writable for RLSSP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RLSSP to value 0x7700_0000"]
impl crate::Resettable for RLSSP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7700_0000
    }
}
