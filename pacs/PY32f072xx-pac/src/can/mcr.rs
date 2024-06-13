#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSOFF` reader - desc BUSOFF"]
pub type BUSOFF_R = crate::BitReader<bool>;
#[doc = "Field `BUSOFF` writer - desc BUSOFF"]
pub type BUSOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `LBMI` reader - desc LBMI"]
pub type LBMI_R = crate::BitReader<bool>;
#[doc = "Field `LBMI` writer - desc LBMI"]
pub type LBMI_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `LBME` reader - desc LBME"]
pub type LBME_R = crate::BitReader<bool>;
#[doc = "Field `LBME` writer - desc LBME"]
pub type LBME_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `RESET` reader - desc RESET"]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - desc RESET"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `TSA` reader - desc TSA"]
pub type TSA_R = crate::BitReader<bool>;
#[doc = "Field `TSA` writer - desc TSA"]
pub type TSA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `TSALL` reader - desc TSALL"]
pub type TSALL_R = crate::BitReader<bool>;
#[doc = "Field `TSALL` writer - desc TSALL"]
pub type TSALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `TSONE` reader - desc TSONE"]
pub type TSONE_R = crate::BitReader<bool>;
#[doc = "Field `TSONE` writer - desc TSONE"]
pub type TSONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `TPA` reader - desc TPA"]
pub type TPA_R = crate::BitReader<bool>;
#[doc = "Field `TPA` writer - desc TPA"]
pub type TPA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `TPE` reader - desc TPE"]
pub type TPE_R = crate::BitReader<bool>;
#[doc = "Field `TPE` writer - desc TPE"]
pub type TPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `STBY` reader - desc STBY"]
pub type STBY_R = crate::BitReader<bool>;
#[doc = "Field `STBY` writer - desc STBY"]
pub type STBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `LOM` reader - desc LOM"]
pub type LOM_R = crate::BitReader<bool>;
#[doc = "Field `LOM` writer - desc LOM"]
pub type LOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `TBSEL` reader - desc TBSEL"]
pub type TBSEL_R = crate::BitReader<bool>;
#[doc = "Field `TBSEL` writer - desc TBSEL"]
pub type TBSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `TSSTAT` reader - desc TSSTAT"]
pub type TSSTAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSFF` reader - desc TSFF"]
pub type TSFF_R = crate::BitReader<bool>;
#[doc = "Field `TTTBM` reader - desc TTTBM"]
pub type TTTBM_R = crate::BitReader<bool>;
#[doc = "Field `TTTBM` writer - desc TTTBM"]
pub type TTTBM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `TSMODE` reader - desc TSMODE"]
pub type TSMODE_R = crate::BitReader<bool>;
#[doc = "Field `TSMODE` writer - desc TSMODE"]
pub type TSMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `TSNEXT` reader - desc TSNEXT"]
pub type TSNEXT_R = crate::BitReader<bool>;
#[doc = "Field `TSNEXT` writer - desc TSNEXT"]
pub type TSNEXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `RSTAT` reader - desc RSTAT"]
pub type RSTAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RBALL` reader - desc RBALL"]
pub type RBALL_R = crate::BitReader<bool>;
#[doc = "Field `RBALL` writer - desc RBALL"]
pub type RBALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `RREL` reader - desc RREL"]
pub type RREL_R = crate::BitReader<bool>;
#[doc = "Field `RREL` writer - desc RREL"]
pub type RREL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `ROV` reader - desc ROV"]
pub type ROV_R = crate::BitReader<bool>;
#[doc = "Field `ROM` reader - desc ROM"]
pub type ROM_R = crate::BitReader<bool>;
#[doc = "Field `ROM` writer - desc ROM"]
pub type ROM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `SACK` reader - desc SACK"]
pub type SACK_R = crate::BitReader<bool>;
#[doc = "Field `SACK` writer - desc SACK"]
pub type SACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc BUSOFF"]
    #[inline(always)]
    pub fn busoff(&self) -> BUSOFF_R {
        BUSOFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - desc LBMI"]
    #[inline(always)]
    pub fn lbmi(&self) -> LBMI_R {
        LBMI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc LBME"]
    #[inline(always)]
    pub fn lbme(&self) -> LBME_R {
        LBME_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc RESET"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc TSA"]
    #[inline(always)]
    pub fn tsa(&self) -> TSA_R {
        TSA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc TSALL"]
    #[inline(always)]
    pub fn tsall(&self) -> TSALL_R {
        TSALL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc TSONE"]
    #[inline(always)]
    pub fn tsone(&self) -> TSONE_R {
        TSONE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc TPA"]
    #[inline(always)]
    pub fn tpa(&self) -> TPA_R {
        TPA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc TPE"]
    #[inline(always)]
    pub fn tpe(&self) -> TPE_R {
        TPE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc STBY"]
    #[inline(always)]
    pub fn stby(&self) -> STBY_R {
        STBY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc LOM"]
    #[inline(always)]
    pub fn lom(&self) -> LOM_R {
        LOM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc TBSEL"]
    #[inline(always)]
    pub fn tbsel(&self) -> TBSEL_R {
        TBSEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - desc TSSTAT"]
    #[inline(always)]
    pub fn tsstat(&self) -> TSSTAT_R {
        TSSTAT_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - desc TSFF"]
    #[inline(always)]
    pub fn tsff(&self) -> TSFF_R {
        TSFF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - desc TTTBM"]
    #[inline(always)]
    pub fn tttbm(&self) -> TTTBM_R {
        TTTBM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc TSMODE"]
    #[inline(always)]
    pub fn tsmode(&self) -> TSMODE_R {
        TSMODE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc TSNEXT"]
    #[inline(always)]
    pub fn tsnext(&self) -> TSNEXT_R {
        TSNEXT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:25 - desc RSTAT"]
    #[inline(always)]
    pub fn rstat(&self) -> RSTAT_R {
        RSTAT_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 27 - desc RBALL"]
    #[inline(always)]
    pub fn rball(&self) -> RBALL_R {
        RBALL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - desc RREL"]
    #[inline(always)]
    pub fn rrel(&self) -> RREL_R {
        RREL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc ROV"]
    #[inline(always)]
    pub fn rov(&self) -> ROV_R {
        ROV_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc ROM"]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc SACK"]
    #[inline(always)]
    pub fn sack(&self) -> SACK_R {
        SACK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc BUSOFF"]
    #[inline(always)]
    pub fn busoff(&mut self) -> BUSOFF_W<0> {
        BUSOFF_W::new(self)
    }
    #[doc = "Bit 5 - desc LBMI"]
    #[inline(always)]
    pub fn lbmi(&mut self) -> LBMI_W<5> {
        LBMI_W::new(self)
    }
    #[doc = "Bit 6 - desc LBME"]
    #[inline(always)]
    pub fn lbme(&mut self) -> LBME_W<6> {
        LBME_W::new(self)
    }
    #[doc = "Bit 7 - desc RESET"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<7> {
        RESET_W::new(self)
    }
    #[doc = "Bit 8 - desc TSA"]
    #[inline(always)]
    pub fn tsa(&mut self) -> TSA_W<8> {
        TSA_W::new(self)
    }
    #[doc = "Bit 9 - desc TSALL"]
    #[inline(always)]
    pub fn tsall(&mut self) -> TSALL_W<9> {
        TSALL_W::new(self)
    }
    #[doc = "Bit 10 - desc TSONE"]
    #[inline(always)]
    pub fn tsone(&mut self) -> TSONE_W<10> {
        TSONE_W::new(self)
    }
    #[doc = "Bit 11 - desc TPA"]
    #[inline(always)]
    pub fn tpa(&mut self) -> TPA_W<11> {
        TPA_W::new(self)
    }
    #[doc = "Bit 12 - desc TPE"]
    #[inline(always)]
    pub fn tpe(&mut self) -> TPE_W<12> {
        TPE_W::new(self)
    }
    #[doc = "Bit 13 - desc STBY"]
    #[inline(always)]
    pub fn stby(&mut self) -> STBY_W<13> {
        STBY_W::new(self)
    }
    #[doc = "Bit 14 - desc LOM"]
    #[inline(always)]
    pub fn lom(&mut self) -> LOM_W<14> {
        LOM_W::new(self)
    }
    #[doc = "Bit 15 - desc TBSEL"]
    #[inline(always)]
    pub fn tbsel(&mut self) -> TBSEL_W<15> {
        TBSEL_W::new(self)
    }
    #[doc = "Bit 20 - desc TTTBM"]
    #[inline(always)]
    pub fn tttbm(&mut self) -> TTTBM_W<20> {
        TTTBM_W::new(self)
    }
    #[doc = "Bit 21 - desc TSMODE"]
    #[inline(always)]
    pub fn tsmode(&mut self) -> TSMODE_W<21> {
        TSMODE_W::new(self)
    }
    #[doc = "Bit 22 - desc TSNEXT"]
    #[inline(always)]
    pub fn tsnext(&mut self) -> TSNEXT_W<22> {
        TSNEXT_W::new(self)
    }
    #[doc = "Bit 27 - desc RBALL"]
    #[inline(always)]
    pub fn rball(&mut self) -> RBALL_W<27> {
        RBALL_W::new(self)
    }
    #[doc = "Bit 28 - desc RREL"]
    #[inline(always)]
    pub fn rrel(&mut self) -> RREL_W<28> {
        RREL_W::new(self)
    }
    #[doc = "Bit 30 - desc ROM"]
    #[inline(always)]
    pub fn rom(&mut self) -> ROM_W<30> {
        ROM_W::new(self)
    }
    #[doc = "Bit 31 - desc SACK"]
    #[inline(always)]
    pub fn sack(&mut self) -> SACK_W<31> {
        SACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc MCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCR to value 0x0090_0080"]
impl crate::Resettable for MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0090_0080
    }
}
