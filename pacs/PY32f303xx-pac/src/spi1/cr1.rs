#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPHA` reader - desc CPHA"]
pub type CPHA_R = crate::BitReader<bool>;
#[doc = "Field `CPHA` writer - desc CPHA"]
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `CPOL` reader - desc CPOL"]
pub type CPOL_R = crate::BitReader<bool>;
#[doc = "Field `CPOL` writer - desc CPOL"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `MSTR` reader - desc MSTR"]
pub type MSTR_R = crate::BitReader<bool>;
#[doc = "Field `MSTR` writer - desc MSTR"]
pub type MSTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `BR` reader - desc BR"]
pub type BR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BR` writer - desc BR"]
pub type BR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPE` reader - desc SPE"]
pub type SPE_R = crate::BitReader<bool>;
#[doc = "Field `SPE` writer - desc SPE"]
pub type SPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `LSBFIRST` reader - desc LSBFIRST"]
pub type LSBFIRST_R = crate::BitReader<bool>;
#[doc = "Field `LSBFIRST` writer - desc LSBFIRST"]
pub type LSBFIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `SSI` reader - desc SSI"]
pub type SSI_R = crate::BitReader<bool>;
#[doc = "Field `SSI` writer - desc SSI"]
pub type SSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `SSM` reader - desc SSM"]
pub type SSM_R = crate::BitReader<bool>;
#[doc = "Field `SSM` writer - desc SSM"]
pub type SSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `RXONLY` reader - desc RXONLY"]
pub type RXONLY_R = crate::BitReader<bool>;
#[doc = "Field `RXONLY` writer - desc RXONLY"]
pub type RXONLY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `DFF` reader - desc DFF"]
pub type DFF_R = crate::BitReader<bool>;
#[doc = "Field `DFF` writer - desc DFF"]
pub type DFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `CRCNEXT` reader - desc CRCNEXT"]
pub type CRCNEXT_R = crate::BitReader<bool>;
#[doc = "Field `CRCNEXT` writer - desc CRCNEXT"]
pub type CRCNEXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `CRCEN` reader - desc CRCEN"]
pub type CRCEN_R = crate::BitReader<bool>;
#[doc = "Field `CRCEN` writer - desc CRCEN"]
pub type CRCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `BIDIOE` reader - desc BIDIOE"]
pub type BIDIOE_R = crate::BitReader<bool>;
#[doc = "Field `BIDIOE` writer - desc BIDIOE"]
pub type BIDIOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `BIDIMODE` reader - desc BIDIMODE"]
pub type BIDIMODE_R = crate::BitReader<bool>;
#[doc = "Field `BIDIMODE` writer - desc BIDIMODE"]
pub type BIDIMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc CPHA"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CPOL"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc MSTR"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - desc BR"]
    #[inline(always)]
    pub fn br(&self) -> BR_R {
        BR_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - desc SPE"]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc LSBFIRST"]
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc SSI"]
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc SSM"]
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc RXONLY"]
    #[inline(always)]
    pub fn rxonly(&self) -> RXONLY_R {
        RXONLY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc DFF"]
    #[inline(always)]
    pub fn dff(&self) -> DFF_R {
        DFF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc CRCNEXT"]
    #[inline(always)]
    pub fn crcnext(&self) -> CRCNEXT_R {
        CRCNEXT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc CRCEN"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc BIDIOE"]
    #[inline(always)]
    pub fn bidioe(&self) -> BIDIOE_R {
        BIDIOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc BIDIMODE"]
    #[inline(always)]
    pub fn bidimode(&self) -> BIDIMODE_R {
        BIDIMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CPHA"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W<0> {
        CPHA_W::new(self)
    }
    #[doc = "Bit 1 - desc CPOL"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W<1> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 2 - desc MSTR"]
    #[inline(always)]
    pub fn mstr(&mut self) -> MSTR_W<2> {
        MSTR_W::new(self)
    }
    #[doc = "Bits 3:5 - desc BR"]
    #[inline(always)]
    pub fn br(&mut self) -> BR_W<3> {
        BR_W::new(self)
    }
    #[doc = "Bit 6 - desc SPE"]
    #[inline(always)]
    pub fn spe(&mut self) -> SPE_W<6> {
        SPE_W::new(self)
    }
    #[doc = "Bit 7 - desc LSBFIRST"]
    #[inline(always)]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W<7> {
        LSBFIRST_W::new(self)
    }
    #[doc = "Bit 8 - desc SSI"]
    #[inline(always)]
    pub fn ssi(&mut self) -> SSI_W<8> {
        SSI_W::new(self)
    }
    #[doc = "Bit 9 - desc SSM"]
    #[inline(always)]
    pub fn ssm(&mut self) -> SSM_W<9> {
        SSM_W::new(self)
    }
    #[doc = "Bit 10 - desc RXONLY"]
    #[inline(always)]
    pub fn rxonly(&mut self) -> RXONLY_W<10> {
        RXONLY_W::new(self)
    }
    #[doc = "Bit 11 - desc DFF"]
    #[inline(always)]
    pub fn dff(&mut self) -> DFF_W<11> {
        DFF_W::new(self)
    }
    #[doc = "Bit 12 - desc CRCNEXT"]
    #[inline(always)]
    pub fn crcnext(&mut self) -> CRCNEXT_W<12> {
        CRCNEXT_W::new(self)
    }
    #[doc = "Bit 13 - desc CRCEN"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<13> {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 14 - desc BIDIOE"]
    #[inline(always)]
    pub fn bidioe(&mut self) -> BIDIOE_W<14> {
        BIDIOE_W::new(self)
    }
    #[doc = "Bit 15 - desc BIDIMODE"]
    #[inline(always)]
    pub fn bidimode(&mut self) -> BIDIMODE_W<15> {
        BIDIMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
