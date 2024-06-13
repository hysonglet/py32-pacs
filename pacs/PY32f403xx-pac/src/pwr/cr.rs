#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPDS` reader - desc LPDS"]
pub type LPDS_R = crate::BitReader<bool>;
#[doc = "Field `LPDS` writer - desc LPDS"]
pub type LPDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PDDS` reader - desc PDDS"]
pub type PDDS_R = crate::BitReader<bool>;
#[doc = "Field `PDDS` writer - desc PDDS"]
pub type PDDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CWUF` writer - desc CWUF"]
pub type CWUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CSBF` writer - desc CSBF"]
pub type CSBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PVDE` reader - desc PVDE"]
pub type PVDE_R = crate::BitReader<bool>;
#[doc = "Field `PVDE` writer - desc PVDE"]
pub type PVDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PLS` reader - desc PLS"]
pub type PLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLS` writer - desc PLS"]
pub type PLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DBP` reader - desc DBP"]
pub type DBP_R = crate::BitReader<bool>;
#[doc = "Field `DBP` writer - desc DBP"]
pub type DBP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `VOS` reader - desc VOS"]
pub type VOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VOS` writer - desc VOS"]
pub type VOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `BKPVR_VOS` reader - desc BKPVR_VOS"]
pub type BKPVR_VOS_R = crate::BitReader<bool>;
#[doc = "Field `BKPVR_VOS` writer - desc BKPVR_VOS"]
pub type BKPVR_VOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `HSION_CTRL` reader - desc HSION_CTRL"]
pub type HSION_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `HSION_CTRL` writer - desc HSION_CTRL"]
pub type HSION_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `FLS_WUPT` reader - desc FLS_WUPT"]
pub type FLS_WUPT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLS_WUPT` writer - desc FLS_WUPT"]
pub type FLS_WUPT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `STDBY_MRRDY_WAIT` reader - desc STDBY_MRRDY_WAIT"]
pub type STDBY_MRRDY_WAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STDBY_MRRDY_WAIT` writer - desc STDBY_MRRDY_WAIT"]
pub type STDBY_MRRDY_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - desc LPDS"]
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PDDS"]
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PVDE"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - desc PLS"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - desc DBP"]
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 13:14 - desc VOS"]
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - desc BKPVR_VOS"]
    #[inline(always)]
    pub fn bkpvr_vos(&self) -> BKPVR_VOS_R {
        BKPVR_VOS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc HSION_CTRL"]
    #[inline(always)]
    pub fn hsion_ctrl(&self) -> HSION_CTRL_R {
        HSION_CTRL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - desc FLS_WUPT"]
    #[inline(always)]
    pub fn fls_wupt(&self) -> FLS_WUPT_R {
        FLS_WUPT_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 20:21 - desc STDBY_MRRDY_WAIT"]
    #[inline(always)]
    pub fn stdby_mrrdy_wait(&self) -> STDBY_MRRDY_WAIT_R {
        STDBY_MRRDY_WAIT_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc LPDS"]
    #[inline(always)]
    pub fn lpds(&mut self) -> LPDS_W<0> {
        LPDS_W::new(self)
    }
    #[doc = "Bit 1 - desc PDDS"]
    #[inline(always)]
    pub fn pdds(&mut self) -> PDDS_W<1> {
        PDDS_W::new(self)
    }
    #[doc = "Bit 2 - desc CWUF"]
    #[inline(always)]
    pub fn cwuf(&mut self) -> CWUF_W<2> {
        CWUF_W::new(self)
    }
    #[doc = "Bit 3 - desc CSBF"]
    #[inline(always)]
    pub fn csbf(&mut self) -> CSBF_W<3> {
        CSBF_W::new(self)
    }
    #[doc = "Bit 4 - desc PVDE"]
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W<4> {
        PVDE_W::new(self)
    }
    #[doc = "Bits 5:7 - desc PLS"]
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W<5> {
        PLS_W::new(self)
    }
    #[doc = "Bit 8 - desc DBP"]
    #[inline(always)]
    pub fn dbp(&mut self) -> DBP_W<8> {
        DBP_W::new(self)
    }
    #[doc = "Bits 13:14 - desc VOS"]
    #[inline(always)]
    pub fn vos(&mut self) -> VOS_W<13> {
        VOS_W::new(self)
    }
    #[doc = "Bit 15 - desc BKPVR_VOS"]
    #[inline(always)]
    pub fn bkpvr_vos(&mut self) -> BKPVR_VOS_W<15> {
        BKPVR_VOS_W::new(self)
    }
    #[doc = "Bit 16 - desc HSION_CTRL"]
    #[inline(always)]
    pub fn hsion_ctrl(&mut self) -> HSION_CTRL_W<16> {
        HSION_CTRL_W::new(self)
    }
    #[doc = "Bits 17:18 - desc FLS_WUPT"]
    #[inline(always)]
    pub fn fls_wupt(&mut self) -> FLS_WUPT_W<17> {
        FLS_WUPT_W::new(self)
    }
    #[doc = "Bits 20:21 - desc STDBY_MRRDY_WAIT"]
    #[inline(always)]
    pub fn stdby_mrrdy_wait(&mut self) -> STDBY_MRRDY_WAIT_W<20> {
        STDBY_MRRDY_WAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0x0011_0000"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0011_0000
    }
}
