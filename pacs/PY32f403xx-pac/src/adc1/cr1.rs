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
#[doc = "Field `AWDCH` reader - desc AWDCH"]
pub type AWDCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWDCH` writer - desc AWDCH"]
pub type AWDCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `EOCIE` reader - desc EOCIE"]
pub type EOCIE_R = crate::BitReader<bool>;
#[doc = "Field `EOCIE` writer - desc EOCIE"]
pub type EOCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `AWDIE` reader - desc AWDIE"]
pub type AWDIE_R = crate::BitReader<bool>;
#[doc = "Field `AWDIE` writer - desc AWDIE"]
pub type AWDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `JEOCIE` reader - desc JEOCIE"]
pub type JEOCIE_R = crate::BitReader<bool>;
#[doc = "Field `JEOCIE` writer - desc JEOCIE"]
pub type JEOCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `SCAN` reader - desc SCAN"]
pub type SCAN_R = crate::BitReader<bool>;
#[doc = "Field `SCAN` writer - desc SCAN"]
pub type SCAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `AWDSGL` reader - desc AWDSGL"]
pub type AWDSGL_R = crate::BitReader<bool>;
#[doc = "Field `AWDSGL` writer - desc AWDSGL"]
pub type AWDSGL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `JAUTO` reader - desc JAUTO"]
pub type JAUTO_R = crate::BitReader<bool>;
#[doc = "Field `JAUTO` writer - desc JAUTO"]
pub type JAUTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `DISCEN` reader - desc DISCEN"]
pub type DISCEN_R = crate::BitReader<bool>;
#[doc = "Field `DISCEN` writer - desc DISCEN"]
pub type DISCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `JDISCEN` reader - desc JDISCEN"]
pub type JDISCEN_R = crate::BitReader<bool>;
#[doc = "Field `JDISCEN` writer - desc JDISCEN"]
pub type JDISCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `DISCNUM` reader - desc DISCNUM"]
pub type DISCNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DISCNUM` writer - desc DISCNUM"]
pub type DISCNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `DUALMOD` reader - desc DUALMOD"]
pub type DUALMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DUALMOD` writer - desc DUALMOD"]
pub type DUALMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `JAWDEN` reader - desc JAWDEN"]
pub type JAWDEN_R = crate::BitReader<bool>;
#[doc = "Field `JAWDEN` writer - desc JAWDEN"]
pub type JAWDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `AWDEN` reader - desc AWDEN"]
pub type AWDEN_R = crate::BitReader<bool>;
#[doc = "Field `AWDEN` writer - desc AWDEN"]
pub type AWDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `RESSEL` reader - desc RESSEL"]
pub type RESSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESSEL` writer - desc RESSEL"]
pub type RESSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `WAIT` reader - desc WAIT"]
pub type WAIT_R = crate::BitReader<bool>;
#[doc = "Field `WAIT` writer - desc WAIT"]
pub type WAIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ADSTP` reader - desc ADSTP"]
pub type ADSTP_R = crate::BitReader<bool>;
#[doc = "Field `ADSTP` writer - desc ADSTP"]
pub type ADSTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `MSBSEL` reader - desc MSBSEL"]
pub type MSBSEL_R = crate::BitReader<bool>;
#[doc = "Field `MSBSEL` writer - desc MSBSEL"]
pub type MSBSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - desc AWDCH"]
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - desc EOCIE"]
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc AWDIE"]
    #[inline(always)]
    pub fn awdie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc JEOCIE"]
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc SCAN"]
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc AWDSGL"]
    #[inline(always)]
    pub fn awdsgl(&self) -> AWDSGL_R {
        AWDSGL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc JAUTO"]
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc DISCEN"]
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc JDISCEN"]
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - desc DISCNUM"]
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - desc DUALMOD"]
    #[inline(always)]
    pub fn dualmod(&self) -> DUALMOD_R {
        DUALMOD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - desc JAWDEN"]
    #[inline(always)]
    pub fn jawden(&self) -> JAWDEN_R {
        JAWDEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc AWDEN"]
    #[inline(always)]
    pub fn awden(&self) -> AWDEN_R {
        AWDEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - desc RESSEL"]
    #[inline(always)]
    pub fn ressel(&self) -> RESSEL_R {
        RESSEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - desc WAIT"]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - desc ADSTP"]
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - desc MSBSEL"]
    #[inline(always)]
    pub fn msbsel(&self) -> MSBSEL_R {
        MSBSEL_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - desc AWDCH"]
    #[inline(always)]
    pub fn awdch(&mut self) -> AWDCH_W<0> {
        AWDCH_W::new(self)
    }
    #[doc = "Bit 5 - desc EOCIE"]
    #[inline(always)]
    pub fn eocie(&mut self) -> EOCIE_W<5> {
        EOCIE_W::new(self)
    }
    #[doc = "Bit 6 - desc AWDIE"]
    #[inline(always)]
    pub fn awdie(&mut self) -> AWDIE_W<6> {
        AWDIE_W::new(self)
    }
    #[doc = "Bit 7 - desc JEOCIE"]
    #[inline(always)]
    pub fn jeocie(&mut self) -> JEOCIE_W<7> {
        JEOCIE_W::new(self)
    }
    #[doc = "Bit 8 - desc SCAN"]
    #[inline(always)]
    pub fn scan(&mut self) -> SCAN_W<8> {
        SCAN_W::new(self)
    }
    #[doc = "Bit 9 - desc AWDSGL"]
    #[inline(always)]
    pub fn awdsgl(&mut self) -> AWDSGL_W<9> {
        AWDSGL_W::new(self)
    }
    #[doc = "Bit 10 - desc JAUTO"]
    #[inline(always)]
    pub fn jauto(&mut self) -> JAUTO_W<10> {
        JAUTO_W::new(self)
    }
    #[doc = "Bit 11 - desc DISCEN"]
    #[inline(always)]
    pub fn discen(&mut self) -> DISCEN_W<11> {
        DISCEN_W::new(self)
    }
    #[doc = "Bit 12 - desc JDISCEN"]
    #[inline(always)]
    pub fn jdiscen(&mut self) -> JDISCEN_W<12> {
        JDISCEN_W::new(self)
    }
    #[doc = "Bits 13:15 - desc DISCNUM"]
    #[inline(always)]
    pub fn discnum(&mut self) -> DISCNUM_W<13> {
        DISCNUM_W::new(self)
    }
    #[doc = "Bits 16:19 - desc DUALMOD"]
    #[inline(always)]
    pub fn dualmod(&mut self) -> DUALMOD_W<16> {
        DUALMOD_W::new(self)
    }
    #[doc = "Bit 22 - desc JAWDEN"]
    #[inline(always)]
    pub fn jawden(&mut self) -> JAWDEN_W<22> {
        JAWDEN_W::new(self)
    }
    #[doc = "Bit 23 - desc AWDEN"]
    #[inline(always)]
    pub fn awden(&mut self) -> AWDEN_W<23> {
        AWDEN_W::new(self)
    }
    #[doc = "Bits 24:25 - desc RESSEL"]
    #[inline(always)]
    pub fn ressel(&mut self) -> RESSEL_W<24> {
        RESSEL_W::new(self)
    }
    #[doc = "Bit 26 - desc WAIT"]
    #[inline(always)]
    pub fn wait(&mut self) -> WAIT_W<26> {
        WAIT_W::new(self)
    }
    #[doc = "Bit 27 - desc ADSTP"]
    #[inline(always)]
    pub fn adstp(&mut self) -> ADSTP_W<27> {
        ADSTP_W::new(self)
    }
    #[doc = "Bit 28 - desc MSBSEL"]
    #[inline(always)]
    pub fn msbsel(&mut self) -> MSBSEL_W<28> {
        MSBSEL_W::new(self)
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
