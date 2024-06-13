#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUF` reader - desc WUF"]
pub type WUF_R = crate::BitReader<bool>;
#[doc = "Field `SBF` reader - desc SBF"]
pub type SBF_R = crate::BitReader<bool>;
#[doc = "Field `PVDO` reader - desc PVDO"]
pub type PVDO_R = crate::BitReader<bool>;
#[doc = "Field `EWUP1` reader - desc EWUP1"]
pub type EWUP1_R = crate::BitReader<bool>;
#[doc = "Field `EWUP1` writer - desc EWUP1"]
pub type EWUP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `EWUP2` reader - desc EWUP2"]
pub type EWUP2_R = crate::BitReader<bool>;
#[doc = "Field `EWUP2` writer - desc EWUP2"]
pub type EWUP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `EWUP3` reader - desc EWUP3"]
pub type EWUP3_R = crate::BitReader<bool>;
#[doc = "Field `EWUP3` writer - desc EWUP3"]
pub type EWUP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `EWUP4` reader - desc EWUP4"]
pub type EWUP4_R = crate::BitReader<bool>;
#[doc = "Field `EWUP4` writer - desc EWUP4"]
pub type EWUP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `EWUP5` reader - desc EWUP5"]
pub type EWUP5_R = crate::BitReader<bool>;
#[doc = "Field `EWUP5` writer - desc EWUP5"]
pub type EWUP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `FLTDIS` reader - desc FLTDIS"]
pub type FLTDIS_R = crate::BitReader<bool>;
#[doc = "Field `FLTDIS` writer - desc FLTDIS"]
pub type FLTDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `FLT_CTRL` reader - desc FLT_CTRL"]
pub type FLT_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLT_CTRL` writer - desc FLT_CTRL"]
pub type FLT_CTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - desc WUF"]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SBF"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PVDO"]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - desc EWUP1"]
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc EWUP2"]
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc EWUP3"]
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc EWUP4"]
    #[inline(always)]
    pub fn ewup4(&self) -> EWUP4_R {
        EWUP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc EWUP5"]
    #[inline(always)]
    pub fn ewup5(&self) -> EWUP5_R {
        EWUP5_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - desc FLTDIS"]
    #[inline(always)]
    pub fn fltdis(&self) -> FLTDIS_R {
        FLTDIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - desc FLT_CTRL"]
    #[inline(always)]
    pub fn flt_ctrl(&self) -> FLT_CTRL_R {
        FLT_CTRL_R::new(((self.bits >> 17) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - desc EWUP1"]
    #[inline(always)]
    pub fn ewup1(&mut self) -> EWUP1_W<8> {
        EWUP1_W::new(self)
    }
    #[doc = "Bit 9 - desc EWUP2"]
    #[inline(always)]
    pub fn ewup2(&mut self) -> EWUP2_W<9> {
        EWUP2_W::new(self)
    }
    #[doc = "Bit 10 - desc EWUP3"]
    #[inline(always)]
    pub fn ewup3(&mut self) -> EWUP3_W<10> {
        EWUP3_W::new(self)
    }
    #[doc = "Bit 11 - desc EWUP4"]
    #[inline(always)]
    pub fn ewup4(&mut self) -> EWUP4_W<11> {
        EWUP4_W::new(self)
    }
    #[doc = "Bit 12 - desc EWUP5"]
    #[inline(always)]
    pub fn ewup5(&mut self) -> EWUP5_W<12> {
        EWUP5_W::new(self)
    }
    #[doc = "Bit 16 - desc FLTDIS"]
    #[inline(always)]
    pub fn fltdis(&mut self) -> FLTDIS_W<16> {
        FLTDIS_W::new(self)
    }
    #[doc = "Bits 17:19 - desc FLT_CTRL"]
    #[inline(always)]
    pub fn flt_ctrl(&mut self) -> FLT_CTRL_W<17> {
        FLT_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR to value 0x83"]
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x83
    }
}
