#[doc = "Register `INTSTS` reader"]
pub struct R(crate::R<INTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTSTS` writer"]
pub struct W(crate::W<INTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSTS_SPEC>;
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
impl From<crate::W<INTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAD` reader - desc CAD"]
pub type CAD_R = crate::BitReader<bool>;
#[doc = "Field `CAD` writer - desc CAD"]
pub type CAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `RE` reader - desc RE"]
pub type RE_R = crate::BitReader<bool>;
#[doc = "Field `RE` writer - desc RE"]
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `CD` reader - desc CD"]
pub type CD_R = crate::BitReader<bool>;
#[doc = "Field `CD` writer - desc CD"]
pub type CD_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `DTO` reader - desc DTO"]
pub type DTO_R = crate::BitReader<bool>;
#[doc = "Field `DTO` writer - desc DTO"]
pub type DTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `TXDR` reader - desc TXDR"]
pub type TXDR_R = crate::BitReader<bool>;
#[doc = "Field `TXDR` writer - desc TXDR"]
pub type TXDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `RXDR` reader - desc RXDR"]
pub type RXDR_R = crate::BitReader<bool>;
#[doc = "Field `RXDR` writer - desc RXDR"]
pub type RXDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `RCRC` reader - desc RCRC"]
pub type RCRC_R = crate::BitReader<bool>;
#[doc = "Field `RCRC` writer - desc RCRC"]
pub type RCRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `DCRC` reader - desc DCRC"]
pub type DCRC_R = crate::BitReader<bool>;
#[doc = "Field `DCRC` writer - desc DCRC"]
pub type DCRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `RTO_BAR` reader - desc RTO_BAR"]
pub type RTO_BAR_R = crate::BitReader<bool>;
#[doc = "Field `RTO_BAR` writer - desc RTO_BAR"]
pub type RTO_BAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `DRTO_BDS` reader - desc DRTO_BDS"]
pub type DRTO_BDS_R = crate::BitReader<bool>;
#[doc = "Field `DRTO_BDS` writer - desc DRTO_BDS"]
pub type DRTO_BDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `HTO` reader - desc HTO"]
pub type HTO_R = crate::BitReader<bool>;
#[doc = "Field `HTO` writer - desc HTO"]
pub type HTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `FRUN` reader - desc FRUN"]
pub type FRUN_R = crate::BitReader<bool>;
#[doc = "Field `FRUN` writer - desc FRUN"]
pub type FRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `HLE` reader - desc HLE"]
pub type HLE_R = crate::BitReader<bool>;
#[doc = "Field `HLE` writer - desc HLE"]
pub type HLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `SBE` reader - desc SBE"]
pub type SBE_R = crate::BitReader<bool>;
#[doc = "Field `SBE` writer - desc SBE"]
pub type SBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `ACD` reader - desc ACD"]
pub type ACD_R = crate::BitReader<bool>;
#[doc = "Field `ACD` writer - desc ACD"]
pub type ACD_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `EBE` reader - desc EBE"]
pub type EBE_R = crate::BitReader<bool>;
#[doc = "Field `EBE` writer - desc EBE"]
pub type EBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `SDIOINT` reader - desc SDIOINT"]
pub type SDIOINT_R = crate::BitReader<bool>;
#[doc = "Field `SDIOINT` writer - desc SDIOINT"]
pub type SDIOINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc CAD"]
    #[inline(always)]
    pub fn cad(&self) -> CAD_R {
        CAD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RE"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CD"]
    #[inline(always)]
    pub fn cd(&self) -> CD_R {
        CD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc DTO"]
    #[inline(always)]
    pub fn dto(&self) -> DTO_R {
        DTO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc TXDR"]
    #[inline(always)]
    pub fn txdr(&self) -> TXDR_R {
        TXDR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc RXDR"]
    #[inline(always)]
    pub fn rxdr(&self) -> RXDR_R {
        RXDR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc RCRC"]
    #[inline(always)]
    pub fn rcrc(&self) -> RCRC_R {
        RCRC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc DCRC"]
    #[inline(always)]
    pub fn dcrc(&self) -> DCRC_R {
        DCRC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc RTO_BAR"]
    #[inline(always)]
    pub fn rto_bar(&self) -> RTO_BAR_R {
        RTO_BAR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc DRTO_BDS"]
    #[inline(always)]
    pub fn drto_bds(&self) -> DRTO_BDS_R {
        DRTO_BDS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc HTO"]
    #[inline(always)]
    pub fn hto(&self) -> HTO_R {
        HTO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc FRUN"]
    #[inline(always)]
    pub fn frun(&self) -> FRUN_R {
        FRUN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc HLE"]
    #[inline(always)]
    pub fn hle(&self) -> HLE_R {
        HLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc SBE"]
    #[inline(always)]
    pub fn sbe(&self) -> SBE_R {
        SBE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc ACD"]
    #[inline(always)]
    pub fn acd(&self) -> ACD_R {
        ACD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc EBE"]
    #[inline(always)]
    pub fn ebe(&self) -> EBE_R {
        EBE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc SDIOINT"]
    #[inline(always)]
    pub fn sdioint(&self) -> SDIOINT_R {
        SDIOINT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CAD"]
    #[inline(always)]
    pub fn cad(&mut self) -> CAD_W<0> {
        CAD_W::new(self)
    }
    #[doc = "Bit 1 - desc RE"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W<1> {
        RE_W::new(self)
    }
    #[doc = "Bit 2 - desc CD"]
    #[inline(always)]
    pub fn cd(&mut self) -> CD_W<2> {
        CD_W::new(self)
    }
    #[doc = "Bit 3 - desc DTO"]
    #[inline(always)]
    pub fn dto(&mut self) -> DTO_W<3> {
        DTO_W::new(self)
    }
    #[doc = "Bit 4 - desc TXDR"]
    #[inline(always)]
    pub fn txdr(&mut self) -> TXDR_W<4> {
        TXDR_W::new(self)
    }
    #[doc = "Bit 5 - desc RXDR"]
    #[inline(always)]
    pub fn rxdr(&mut self) -> RXDR_W<5> {
        RXDR_W::new(self)
    }
    #[doc = "Bit 6 - desc RCRC"]
    #[inline(always)]
    pub fn rcrc(&mut self) -> RCRC_W<6> {
        RCRC_W::new(self)
    }
    #[doc = "Bit 7 - desc DCRC"]
    #[inline(always)]
    pub fn dcrc(&mut self) -> DCRC_W<7> {
        DCRC_W::new(self)
    }
    #[doc = "Bit 8 - desc RTO_BAR"]
    #[inline(always)]
    pub fn rto_bar(&mut self) -> RTO_BAR_W<8> {
        RTO_BAR_W::new(self)
    }
    #[doc = "Bit 9 - desc DRTO_BDS"]
    #[inline(always)]
    pub fn drto_bds(&mut self) -> DRTO_BDS_W<9> {
        DRTO_BDS_W::new(self)
    }
    #[doc = "Bit 10 - desc HTO"]
    #[inline(always)]
    pub fn hto(&mut self) -> HTO_W<10> {
        HTO_W::new(self)
    }
    #[doc = "Bit 11 - desc FRUN"]
    #[inline(always)]
    pub fn frun(&mut self) -> FRUN_W<11> {
        FRUN_W::new(self)
    }
    #[doc = "Bit 12 - desc HLE"]
    #[inline(always)]
    pub fn hle(&mut self) -> HLE_W<12> {
        HLE_W::new(self)
    }
    #[doc = "Bit 13 - desc SBE"]
    #[inline(always)]
    pub fn sbe(&mut self) -> SBE_W<13> {
        SBE_W::new(self)
    }
    #[doc = "Bit 14 - desc ACD"]
    #[inline(always)]
    pub fn acd(&mut self) -> ACD_W<14> {
        ACD_W::new(self)
    }
    #[doc = "Bit 15 - desc EBE"]
    #[inline(always)]
    pub fn ebe(&mut self) -> EBE_W<15> {
        EBE_W::new(self)
    }
    #[doc = "Bit 16 - desc SDIOINT"]
    #[inline(always)]
    pub fn sdioint(&mut self) -> SDIOINT_W<16> {
        SDIOINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc INTSTS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intsts](index.html) module"]
pub struct INTSTS_SPEC;
impl crate::RegisterSpec for INTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intsts::R](R) reader structure"]
impl crate::Readable for INTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intsts::W](W) writer structure"]
impl crate::Writable for INTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTSTS to value 0"]
impl crate::Resettable for INTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
