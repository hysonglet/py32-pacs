#[doc = "Register `INTMASK` reader"]
pub struct R(crate::R<INTMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTMASK` writer"]
pub struct W(crate::W<INTMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTMASK_SPEC>;
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
impl From<crate::W<INTMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CADIE` reader - desc CADIE"]
pub type CADIE_R = crate::BitReader<bool>;
#[doc = "Field `CADIE` writer - desc CADIE"]
pub type CADIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `REIE` reader - desc REIE"]
pub type REIE_R = crate::BitReader<bool>;
#[doc = "Field `REIE` writer - desc REIE"]
pub type REIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `CDIE` reader - desc CDIE"]
pub type CDIE_R = crate::BitReader<bool>;
#[doc = "Field `CDIE` writer - desc CDIE"]
pub type CDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `DTOIE` reader - desc DTOIE"]
pub type DTOIE_R = crate::BitReader<bool>;
#[doc = "Field `DTOIE` writer - desc DTOIE"]
pub type DTOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `TXDRIE` reader - desc TXDRIE"]
pub type TXDRIE_R = crate::BitReader<bool>;
#[doc = "Field `TXDRIE` writer - desc TXDRIE"]
pub type TXDRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `RXDRIE` reader - desc RXDRIE"]
pub type RXDRIE_R = crate::BitReader<bool>;
#[doc = "Field `RXDRIE` writer - desc RXDRIE"]
pub type RXDRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `RCRCIE` reader - desc RCRCIE"]
pub type RCRCIE_R = crate::BitReader<bool>;
#[doc = "Field `RCRCIE` writer - desc RCRCIE"]
pub type RCRCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `DCRCIE` reader - desc DCRCIE"]
pub type DCRCIE_R = crate::BitReader<bool>;
#[doc = "Field `DCRCIE` writer - desc DCRCIE"]
pub type DCRCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `RTOIE` reader - desc RTOIE"]
pub type RTOIE_R = crate::BitReader<bool>;
#[doc = "Field `RTOIE` writer - desc RTOIE"]
pub type RTOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `DRTOIE` reader - desc DRTOIE"]
pub type DRTOIE_R = crate::BitReader<bool>;
#[doc = "Field `DRTOIE` writer - desc DRTOIE"]
pub type DRTOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `HTOIE` reader - desc HTOIE"]
pub type HTOIE_R = crate::BitReader<bool>;
#[doc = "Field `HTOIE` writer - desc HTOIE"]
pub type HTOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `FRUNIE` reader - desc FRUNIE"]
pub type FRUNIE_R = crate::BitReader<bool>;
#[doc = "Field `FRUNIE` writer - desc FRUNIE"]
pub type FRUNIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `HLEIE` reader - desc HLEIE"]
pub type HLEIE_R = crate::BitReader<bool>;
#[doc = "Field `HLEIE` writer - desc HLEIE"]
pub type HLEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `SBEIE` reader - desc SBEIE"]
pub type SBEIE_R = crate::BitReader<bool>;
#[doc = "Field `SBEIE` writer - desc SBEIE"]
pub type SBEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `ACDIE` reader - desc ACDIE"]
pub type ACDIE_R = crate::BitReader<bool>;
#[doc = "Field `ACDIE` writer - desc ACDIE"]
pub type ACDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `EBEIE` reader - desc EBEIE"]
pub type EBEIE_R = crate::BitReader<bool>;
#[doc = "Field `EBEIE` writer - desc EBEIE"]
pub type EBEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
#[doc = "Field `SDIOINTIE` reader - desc SDIOINTIE"]
pub type SDIOINTIE_R = crate::BitReader<bool>;
#[doc = "Field `SDIOINTIE` writer - desc SDIOINTIE"]
pub type SDIOINTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc CADIE"]
    #[inline(always)]
    pub fn cadie(&self) -> CADIE_R {
        CADIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc REIE"]
    #[inline(always)]
    pub fn reie(&self) -> REIE_R {
        REIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CDIE"]
    #[inline(always)]
    pub fn cdie(&self) -> CDIE_R {
        CDIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc DTOIE"]
    #[inline(always)]
    pub fn dtoie(&self) -> DTOIE_R {
        DTOIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc TXDRIE"]
    #[inline(always)]
    pub fn txdrie(&self) -> TXDRIE_R {
        TXDRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc RXDRIE"]
    #[inline(always)]
    pub fn rxdrie(&self) -> RXDRIE_R {
        RXDRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc RCRCIE"]
    #[inline(always)]
    pub fn rcrcie(&self) -> RCRCIE_R {
        RCRCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc DCRCIE"]
    #[inline(always)]
    pub fn dcrcie(&self) -> DCRCIE_R {
        DCRCIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc RTOIE"]
    #[inline(always)]
    pub fn rtoie(&self) -> RTOIE_R {
        RTOIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc DRTOIE"]
    #[inline(always)]
    pub fn drtoie(&self) -> DRTOIE_R {
        DRTOIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc HTOIE"]
    #[inline(always)]
    pub fn htoie(&self) -> HTOIE_R {
        HTOIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc FRUNIE"]
    #[inline(always)]
    pub fn frunie(&self) -> FRUNIE_R {
        FRUNIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc HLEIE"]
    #[inline(always)]
    pub fn hleie(&self) -> HLEIE_R {
        HLEIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc SBEIE"]
    #[inline(always)]
    pub fn sbeie(&self) -> SBEIE_R {
        SBEIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc ACDIE"]
    #[inline(always)]
    pub fn acdie(&self) -> ACDIE_R {
        ACDIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc EBEIE"]
    #[inline(always)]
    pub fn ebeie(&self) -> EBEIE_R {
        EBEIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc SDIOINTIE"]
    #[inline(always)]
    pub fn sdiointie(&self) -> SDIOINTIE_R {
        SDIOINTIE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CADIE"]
    #[inline(always)]
    pub fn cadie(&mut self) -> CADIE_W<0> {
        CADIE_W::new(self)
    }
    #[doc = "Bit 1 - desc REIE"]
    #[inline(always)]
    pub fn reie(&mut self) -> REIE_W<1> {
        REIE_W::new(self)
    }
    #[doc = "Bit 2 - desc CDIE"]
    #[inline(always)]
    pub fn cdie(&mut self) -> CDIE_W<2> {
        CDIE_W::new(self)
    }
    #[doc = "Bit 3 - desc DTOIE"]
    #[inline(always)]
    pub fn dtoie(&mut self) -> DTOIE_W<3> {
        DTOIE_W::new(self)
    }
    #[doc = "Bit 4 - desc TXDRIE"]
    #[inline(always)]
    pub fn txdrie(&mut self) -> TXDRIE_W<4> {
        TXDRIE_W::new(self)
    }
    #[doc = "Bit 5 - desc RXDRIE"]
    #[inline(always)]
    pub fn rxdrie(&mut self) -> RXDRIE_W<5> {
        RXDRIE_W::new(self)
    }
    #[doc = "Bit 6 - desc RCRCIE"]
    #[inline(always)]
    pub fn rcrcie(&mut self) -> RCRCIE_W<6> {
        RCRCIE_W::new(self)
    }
    #[doc = "Bit 7 - desc DCRCIE"]
    #[inline(always)]
    pub fn dcrcie(&mut self) -> DCRCIE_W<7> {
        DCRCIE_W::new(self)
    }
    #[doc = "Bit 8 - desc RTOIE"]
    #[inline(always)]
    pub fn rtoie(&mut self) -> RTOIE_W<8> {
        RTOIE_W::new(self)
    }
    #[doc = "Bit 9 - desc DRTOIE"]
    #[inline(always)]
    pub fn drtoie(&mut self) -> DRTOIE_W<9> {
        DRTOIE_W::new(self)
    }
    #[doc = "Bit 10 - desc HTOIE"]
    #[inline(always)]
    pub fn htoie(&mut self) -> HTOIE_W<10> {
        HTOIE_W::new(self)
    }
    #[doc = "Bit 11 - desc FRUNIE"]
    #[inline(always)]
    pub fn frunie(&mut self) -> FRUNIE_W<11> {
        FRUNIE_W::new(self)
    }
    #[doc = "Bit 12 - desc HLEIE"]
    #[inline(always)]
    pub fn hleie(&mut self) -> HLEIE_W<12> {
        HLEIE_W::new(self)
    }
    #[doc = "Bit 13 - desc SBEIE"]
    #[inline(always)]
    pub fn sbeie(&mut self) -> SBEIE_W<13> {
        SBEIE_W::new(self)
    }
    #[doc = "Bit 14 - desc ACDIE"]
    #[inline(always)]
    pub fn acdie(&mut self) -> ACDIE_W<14> {
        ACDIE_W::new(self)
    }
    #[doc = "Bit 15 - desc EBEIE"]
    #[inline(always)]
    pub fn ebeie(&mut self) -> EBEIE_W<15> {
        EBEIE_W::new(self)
    }
    #[doc = "Bit 16 - desc SDIOINTIE"]
    #[inline(always)]
    pub fn sdiointie(&mut self) -> SDIOINTIE_W<16> {
        SDIOINTIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc INTMASK\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intmask](index.html) module"]
pub struct INTMASK_SPEC;
impl crate::RegisterSpec for INTMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intmask::R](R) reader structure"]
impl crate::Readable for INTMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intmask::W](W) writer structure"]
impl crate::Writable for INTMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTMASK to value 0"]
impl crate::Resettable for INTMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
