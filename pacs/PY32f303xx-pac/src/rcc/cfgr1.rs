#[doc = "Register `CFGR1` reader"]
pub struct R(crate::R<CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR1` writer"]
pub struct W(crate::W<CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR1_SPEC>;
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
impl From<crate::W<CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCOPRE` reader - desc MCOPRE"]
pub type MCOPRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCOPRE` writer - desc MCOPRE"]
pub type MCOPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `HSI48ON` reader - desc HSI48ON"]
pub type HSI48ON_R = crate::BitReader<bool>;
#[doc = "Field `HSI48ON` writer - desc HSI48ON"]
pub type HSI48ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `HSI48RDY` reader - desc HSI48RDY"]
pub type HSI48RDY_R = crate::BitReader<bool>;
#[doc = "Field `HSI48TRIM` reader - desc HSI48TRIM"]
pub type HSI48TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSI48TRIM` writer - desc HSI48TRIM"]
pub type HSI48TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, u8, 7, O>;
#[doc = "Field `HSI48CAL` reader - desc HSI48CAL"]
pub type HSI48CAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `USBSELHSI48` reader - desc USBSELHSI48"]
pub type USBSELHSI48_R = crate::BitReader<bool>;
#[doc = "Field `USBSELHSI48` writer - desc USBSELHSI48"]
pub type USBSELHSI48_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - desc MCOPRE"]
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - desc HSI48ON"]
    #[inline(always)]
    pub fn hsi48on(&self) -> HSI48ON_R {
        HSI48ON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc HSI48RDY"]
    #[inline(always)]
    pub fn hsi48rdy(&self) -> HSI48RDY_R {
        HSI48RDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 9:15 - desc HSI48TRIM"]
    #[inline(always)]
    pub fn hsi48trim(&self) -> HSI48TRIM_R {
        HSI48TRIM_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:28 - desc HSI48CAL"]
    #[inline(always)]
    pub fn hsi48cal(&self) -> HSI48CAL_R {
        HSI48CAL_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    #[doc = "Bit 31 - desc USBSELHSI48"]
    #[inline(always)]
    pub fn usbselhsi48(&self) -> USBSELHSI48_R {
        USBSELHSI48_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc MCOPRE"]
    #[inline(always)]
    pub fn mcopre(&mut self) -> MCOPRE_W<0> {
        MCOPRE_W::new(self)
    }
    #[doc = "Bit 4 - desc HSI48ON"]
    #[inline(always)]
    pub fn hsi48on(&mut self) -> HSI48ON_W<4> {
        HSI48ON_W::new(self)
    }
    #[doc = "Bits 9:15 - desc HSI48TRIM"]
    #[inline(always)]
    pub fn hsi48trim(&mut self) -> HSI48TRIM_W<9> {
        HSI48TRIM_W::new(self)
    }
    #[doc = "Bit 31 - desc USBSELHSI48"]
    #[inline(always)]
    pub fn usbselhsi48(&mut self) -> USBSELHSI48_W<31> {
        USBSELHSI48_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clock configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](index.html) module"]
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr1::R](R) reader structure"]
impl crate::Readable for CFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr1::W](W) writer structure"]
impl crate::Writable for CFGR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR1 to value 0x1080_8000"]
impl crate::Resettable for CFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1080_8000
    }
}
