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
#[doc = "Field `EN` reader - "]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - "]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `MODE` reader - "]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - "]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `EXTCMODEN` reader - "]
pub type EXTCMODEN_R = crate::BitReader<bool>;
#[doc = "Field `EXTCMODEN` writer - "]
pub type EXTCMODEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ICOPA` reader - "]
pub type ICOPA_R = crate::BitReader<bool>;
#[doc = "Field `ICOPA` writer - "]
pub type ICOPA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `COPAMODE` reader - "]
pub type COPAMODE_R = crate::BitReader<bool>;
#[doc = "Field `COPAMODE` writer - "]
pub type COPAMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `COPADATA` reader - "]
pub type COPADATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COPADATA` writer - "]
pub type COPADATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `VCCTKCR` reader - "]
pub type VCCTKCR_R = crate::BitReader<bool>;
#[doc = "Field `VCCTKCR` writer - "]
pub type VCCTKCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `VCC1CR` reader - "]
pub type VCC1CR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VCC1CR` writer - "]
pub type VCC1CR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `MIDACSTEP` reader - "]
pub type MIDACSTEP_R = crate::BitReader<bool>;
#[doc = "Field `MIDACSTEP` writer - "]
pub type MIDACSTEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `SIDACSTEP` reader - "]
pub type SIDACSTEP_R = crate::BitReader<bool>;
#[doc = "Field `SIDACSTEP` writer - "]
pub type SIDACSTEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `CFT` reader - "]
pub type CFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFT` writer - "]
pub type CFT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `VCCTKEN` reader - "]
pub type VCCTKEN_R = crate::BitReader<bool>;
#[doc = "Field `VCCTKEN` writer - "]
pub type VCCTKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `INTCMODEN` reader - "]
pub type INTCMODEN_R = crate::BitReader<bool>;
#[doc = "Field `INTCMODEN` writer - "]
pub type INTCMODEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `CMPHYSEN` reader - "]
pub type CMPHYSEN_R = crate::BitReader<bool>;
#[doc = "Field `CMPHYSEN` writer - "]
pub type CMPHYSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `BUFFEREN` reader - "]
pub type BUFFEREN_R = crate::BitReader<bool>;
#[doc = "Field `BUFFEREN` writer - "]
pub type BUFFEREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `DIV` reader - "]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - "]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `STDIV` reader - "]
pub type STDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STDIV` writer - "]
pub type STDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn extcmoden(&self) -> EXTCMODEN_R {
        EXTCMODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn icopa(&self) -> ICOPA_R {
        ICOPA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn copamode(&self) -> COPAMODE_R {
        COPAMODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn copadata(&self) -> COPADATA_R {
        COPADATA_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn vcctkcr(&self) -> VCCTKCR_R {
        VCCTKCR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn vcc1cr(&self) -> VCC1CR_R {
        VCC1CR_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn midacstep(&self) -> MIDACSTEP_R {
        MIDACSTEP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn sidacstep(&self) -> SIDACSTEP_R {
        SIDACSTEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn cft(&self) -> CFT_R {
        CFT_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn vcctken(&self) -> VCCTKEN_R {
        VCCTKEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn intcmoden(&self) -> INTCMODEN_R {
        INTCMODEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cmphysen(&self) -> CMPHYSEN_R {
        CMPHYSEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn bufferen(&self) -> BUFFEREN_R {
        BUFFEREN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29"]
    #[inline(always)]
    pub fn stdiv(&self) -> STDIV_R {
        STDIV_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<1> {
        MODE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn extcmoden(&mut self) -> EXTCMODEN_W<3> {
        EXTCMODEN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn icopa(&mut self) -> ICOPA_W<4> {
        ICOPA_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn copamode(&mut self) -> COPAMODE_W<5> {
        COPAMODE_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn copadata(&mut self) -> COPADATA_W<6> {
        COPADATA_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn vcctkcr(&mut self) -> VCCTKCR_W<8> {
        VCCTKCR_W::new(self)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn vcc1cr(&mut self) -> VCC1CR_W<11> {
        VCC1CR_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn midacstep(&mut self) -> MIDACSTEP_W<14> {
        MIDACSTEP_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn sidacstep(&mut self) -> SIDACSTEP_W<15> {
        SIDACSTEP_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn cft(&mut self) -> CFT_W<16> {
        CFT_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn vcctken(&mut self) -> VCCTKEN_W<19> {
        VCCTKEN_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn intcmoden(&mut self) -> INTCMODEN_W<20> {
        INTCMODEN_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cmphysen(&mut self) -> CMPHYSEN_W<21> {
        CMPHYSEN_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn bufferen(&mut self) -> BUFFEREN_W<22> {
        BUFFEREN_W::new(self)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<24> {
        DIV_W::new(self)
    }
    #[doc = "Bits 27:29"]
    #[inline(always)]
    pub fn stdiv(&mut self) -> STDIV_W<27> {
        STDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
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
