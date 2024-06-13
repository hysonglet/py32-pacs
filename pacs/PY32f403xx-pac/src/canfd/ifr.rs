#[doc = "Register `IFR` reader"]
pub struct R(crate::R<IFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFR` writer"]
pub struct W(crate::W<IFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFR_SPEC>;
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
impl From<crate::W<IFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AIF` reader - desc AIF"]
pub type AIF_R = crate::BitReader<bool>;
#[doc = "Field `AIF` writer - desc AIF"]
pub type AIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
#[doc = "Field `EIF` reader - desc EIF"]
pub type EIF_R = crate::BitReader<bool>;
#[doc = "Field `EIF` writer - desc EIF"]
pub type EIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
#[doc = "Field `TSIF` reader - desc TSIF"]
pub type TSIF_R = crate::BitReader<bool>;
#[doc = "Field `TSIF` writer - desc TSIF"]
pub type TSIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
#[doc = "Field `TPIF` reader - desc TPIF"]
pub type TPIF_R = crate::BitReader<bool>;
#[doc = "Field `TPIF` writer - desc TPIF"]
pub type TPIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
#[doc = "Field `RAFIF` reader - desc RAFIF"]
pub type RAFIF_R = crate::BitReader<bool>;
#[doc = "Field `RAFIF` writer - desc RAFIF"]
pub type RAFIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
#[doc = "Field `RFIF` reader - desc RFIF"]
pub type RFIF_R = crate::BitReader<bool>;
#[doc = "Field `RFIF` writer - desc RFIF"]
pub type RFIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
#[doc = "Field `ROIF` reader - desc ROIF"]
pub type ROIF_R = crate::BitReader<bool>;
#[doc = "Field `ROIF` writer - desc ROIF"]
pub type ROIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
#[doc = "Field `RIF` reader - desc RIF"]
pub type RIF_R = crate::BitReader<bool>;
#[doc = "Field `RIF` writer - desc RIF"]
pub type RIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
#[doc = "Field `BEIF` reader - desc BEIF"]
pub type BEIF_R = crate::BitReader<bool>;
#[doc = "Field `BEIF` writer - desc BEIF"]
pub type BEIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
#[doc = "Field `ALIF` reader - desc ALIF"]
pub type ALIF_R = crate::BitReader<bool>;
#[doc = "Field `ALIF` writer - desc ALIF"]
pub type ALIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
#[doc = "Field `EPIF` reader - desc EPIF"]
pub type EPIF_R = crate::BitReader<bool>;
#[doc = "Field `EPIF` writer - desc EPIF"]
pub type EPIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
#[doc = "Field `TTIF` reader - desc TTIF"]
pub type TTIF_R = crate::BitReader<bool>;
#[doc = "Field `TTIF` writer - desc TTIF"]
pub type TTIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
#[doc = "Field `TEIF` reader - desc TEIF"]
pub type TEIF_R = crate::BitReader<bool>;
#[doc = "Field `TEIF` writer - desc TEIF"]
pub type TEIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
#[doc = "Field `WTIF` reader - desc WTIF"]
pub type WTIF_R = crate::BitReader<bool>;
#[doc = "Field `WTIF` writer - desc WTIF"]
pub type WTIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
#[doc = "Field `MDWIF` reader - desc MDWIF"]
pub type MDWIF_R = crate::BitReader<bool>;
#[doc = "Field `MDWIF` writer - desc MDWIF"]
pub type MDWIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
#[doc = "Field `MDEIF` reader - desc MDEIF"]
pub type MDEIF_R = crate::BitReader<bool>;
#[doc = "Field `MDEIF` writer - desc MDEIF"]
pub type MDEIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
#[doc = "Field `MAEIF` reader - desc MAEIF"]
pub type MAEIF_R = crate::BitReader<bool>;
#[doc = "Field `MAEIF` writer - desc MAEIF"]
pub type MAEIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
#[doc = "Field `SEIF` reader - desc SEIF"]
pub type SEIF_R = crate::BitReader<bool>;
#[doc = "Field `SEIF` writer - desc SEIF"]
pub type SEIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
#[doc = "Field `SWIF` reader - desc SWIF"]
pub type SWIF_R = crate::BitReader<bool>;
#[doc = "Field `SWIF` writer - desc SWIF"]
pub type SWIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
#[doc = "Field `EPASS` reader - desc EPASS"]
pub type EPASS_R = crate::BitReader<bool>;
#[doc = "Field `EWARN` reader - desc EWARN"]
pub type EWARN_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - desc AIF"]
    #[inline(always)]
    pub fn aif(&self) -> AIF_R {
        AIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EIF"]
    #[inline(always)]
    pub fn eif(&self) -> EIF_R {
        EIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TSIF"]
    #[inline(always)]
    pub fn tsif(&self) -> TSIF_R {
        TSIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TPIF"]
    #[inline(always)]
    pub fn tpif(&self) -> TPIF_R {
        TPIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc RAFIF"]
    #[inline(always)]
    pub fn rafif(&self) -> RAFIF_R {
        RAFIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc RFIF"]
    #[inline(always)]
    pub fn rfif(&self) -> RFIF_R {
        RFIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc ROIF"]
    #[inline(always)]
    pub fn roif(&self) -> ROIF_R {
        ROIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc RIF"]
    #[inline(always)]
    pub fn rif(&self) -> RIF_R {
        RIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc BEIF"]
    #[inline(always)]
    pub fn beif(&self) -> BEIF_R {
        BEIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc ALIF"]
    #[inline(always)]
    pub fn alif(&self) -> ALIF_R {
        ALIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc EPIF"]
    #[inline(always)]
    pub fn epif(&self) -> EPIF_R {
        EPIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc TTIF"]
    #[inline(always)]
    pub fn ttif(&self) -> TTIF_R {
        TTIF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc TEIF"]
    #[inline(always)]
    pub fn teif(&self) -> TEIF_R {
        TEIF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc WTIF"]
    #[inline(always)]
    pub fn wtif(&self) -> WTIF_R {
        WTIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc MDWIF"]
    #[inline(always)]
    pub fn mdwif(&self) -> MDWIF_R {
        MDWIF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc MDEIF"]
    #[inline(always)]
    pub fn mdeif(&self) -> MDEIF_R {
        MDEIF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc MAEIF"]
    #[inline(always)]
    pub fn maeif(&self) -> MAEIF_R {
        MAEIF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc SEIF"]
    #[inline(always)]
    pub fn seif(&self) -> SEIF_R {
        SEIF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc SWIF"]
    #[inline(always)]
    pub fn swif(&self) -> SWIF_R {
        SWIF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 30 - desc EPASS"]
    #[inline(always)]
    pub fn epass(&self) -> EPASS_R {
        EPASS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc EWARN"]
    #[inline(always)]
    pub fn ewarn(&self) -> EWARN_R {
        EWARN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc AIF"]
    #[inline(always)]
    pub fn aif(&mut self) -> AIF_W<0> {
        AIF_W::new(self)
    }
    #[doc = "Bit 1 - desc EIF"]
    #[inline(always)]
    pub fn eif(&mut self) -> EIF_W<1> {
        EIF_W::new(self)
    }
    #[doc = "Bit 2 - desc TSIF"]
    #[inline(always)]
    pub fn tsif(&mut self) -> TSIF_W<2> {
        TSIF_W::new(self)
    }
    #[doc = "Bit 3 - desc TPIF"]
    #[inline(always)]
    pub fn tpif(&mut self) -> TPIF_W<3> {
        TPIF_W::new(self)
    }
    #[doc = "Bit 4 - desc RAFIF"]
    #[inline(always)]
    pub fn rafif(&mut self) -> RAFIF_W<4> {
        RAFIF_W::new(self)
    }
    #[doc = "Bit 5 - desc RFIF"]
    #[inline(always)]
    pub fn rfif(&mut self) -> RFIF_W<5> {
        RFIF_W::new(self)
    }
    #[doc = "Bit 6 - desc ROIF"]
    #[inline(always)]
    pub fn roif(&mut self) -> ROIF_W<6> {
        ROIF_W::new(self)
    }
    #[doc = "Bit 7 - desc RIF"]
    #[inline(always)]
    pub fn rif(&mut self) -> RIF_W<7> {
        RIF_W::new(self)
    }
    #[doc = "Bit 8 - desc BEIF"]
    #[inline(always)]
    pub fn beif(&mut self) -> BEIF_W<8> {
        BEIF_W::new(self)
    }
    #[doc = "Bit 9 - desc ALIF"]
    #[inline(always)]
    pub fn alif(&mut self) -> ALIF_W<9> {
        ALIF_W::new(self)
    }
    #[doc = "Bit 10 - desc EPIF"]
    #[inline(always)]
    pub fn epif(&mut self) -> EPIF_W<10> {
        EPIF_W::new(self)
    }
    #[doc = "Bit 11 - desc TTIF"]
    #[inline(always)]
    pub fn ttif(&mut self) -> TTIF_W<11> {
        TTIF_W::new(self)
    }
    #[doc = "Bit 12 - desc TEIF"]
    #[inline(always)]
    pub fn teif(&mut self) -> TEIF_W<12> {
        TEIF_W::new(self)
    }
    #[doc = "Bit 13 - desc WTIF"]
    #[inline(always)]
    pub fn wtif(&mut self) -> WTIF_W<13> {
        WTIF_W::new(self)
    }
    #[doc = "Bit 14 - desc MDWIF"]
    #[inline(always)]
    pub fn mdwif(&mut self) -> MDWIF_W<14> {
        MDWIF_W::new(self)
    }
    #[doc = "Bit 15 - desc MDEIF"]
    #[inline(always)]
    pub fn mdeif(&mut self) -> MDEIF_W<15> {
        MDEIF_W::new(self)
    }
    #[doc = "Bit 16 - desc MAEIF"]
    #[inline(always)]
    pub fn maeif(&mut self) -> MAEIF_W<16> {
        MAEIF_W::new(self)
    }
    #[doc = "Bit 17 - desc SEIF"]
    #[inline(always)]
    pub fn seif(&mut self) -> SEIF_W<17> {
        SEIF_W::new(self)
    }
    #[doc = "Bit 18 - desc SWIF"]
    #[inline(always)]
    pub fn swif(&mut self) -> SWIF_W<18> {
        SWIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc IFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifr](index.html) module"]
pub struct IFR_SPEC;
impl crate::RegisterSpec for IFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifr::R](R) reader structure"]
impl crate::Readable for IFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifr::W](W) writer structure"]
impl crate::Writable for IFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFR to value 0"]
impl crate::Resettable for IFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
