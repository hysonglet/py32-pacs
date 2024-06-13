#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW` reader - desc SW"]
pub type SW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW` writer - desc SW"]
pub type SW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SWS` reader - desc SWS"]
pub type SWS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SWS` writer - desc SWS"]
pub type SWS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `HPRE` reader - desc HPRE"]
pub type HPRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HPRE` writer - desc HPRE"]
pub type HPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 4, O>;
#[doc = "Field `PPRE1` reader - APB Low speed prescaler (APB1)"]
pub type PPRE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPRE1` writer - APB Low speed prescaler (APB1)"]
pub type PPRE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `PPRE2` reader - APB high-speed prescaler(APB2)"]
pub type PPRE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPRE2` writer - APB high-speed prescaler(APB2)"]
pub type PPRE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `ADCPRE` reader - desc ADCPRE"]
pub type ADCPRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCPRE` writer - desc ADCPRE"]
pub type ADCPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PLLSRC` reader - desc PLLSRC"]
pub type PLLSRC_R = crate::BitReader<bool>;
#[doc = "Field `PLLSRC` writer - desc PLLSRC"]
pub type PLLSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `PLLXTPRE` reader - desc PLLXTPRE"]
pub type PLLXTPRE_R = crate::BitReader<bool>;
#[doc = "Field `PLLXTPRE` writer - desc PLLXTPRE"]
pub type PLLXTPRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `PLLMULL` reader - desc PLLMULL"]
pub type PLLMULL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLMULL` writer - desc PLLMULL"]
pub type PLLMULL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 4, O>;
#[doc = "Field `USBPRE` reader - desc USBPRE"]
pub type USBPRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBPRE` writer - desc USBPRE"]
pub type USBPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `MCO` reader - desc MCO"]
pub type MCO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCO` writer - desc MCO"]
pub type MCO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADCPRE_` reader - desc ADCPRE"]
pub type ADCPRE__R = crate::BitReader<bool>;
#[doc = "Field `ADCPRE_` writer - desc ADCPRE"]
pub type ADCPRE__W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `PLLMULL_` reader - desc PLLMULL"]
pub type PLLMULL__R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLMULL_` writer - desc PLLMULL"]
pub type PLLMULL__W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `USBPRE_` reader - desc USBPRE"]
pub type USBPRE__R = crate::BitReader<bool>;
#[doc = "Field `USBPRE_` writer - desc USBPRE"]
pub type USBPRE__W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - desc SW"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc SWS"]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - desc HPRE"]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - APB high-speed prescaler(APB2)"]
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:15 - desc ADCPRE"]
    #[inline(always)]
    pub fn adcpre(&self) -> ADCPRE_R {
        ADCPRE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - desc PLLSRC"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc PLLXTPRE"]
    #[inline(always)]
    pub fn pllxtpre(&self) -> PLLXTPRE_R {
        PLLXTPRE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - desc PLLMULL"]
    #[inline(always)]
    pub fn pllmull(&self) -> PLLMULL_R {
        PLLMULL_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - desc USBPRE"]
    #[inline(always)]
    pub fn usbpre(&self) -> USBPRE_R {
        USBPRE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - desc MCO"]
    #[inline(always)]
    pub fn mco(&self) -> MCO_R {
        MCO_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - desc ADCPRE"]
    #[inline(always)]
    pub fn adcpre_(&self) -> ADCPRE__R {
        ADCPRE__R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - desc PLLMULL"]
    #[inline(always)]
    pub fn pllmull_(&self) -> PLLMULL__R {
        PLLMULL__R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - desc USBPRE"]
    #[inline(always)]
    pub fn usbpre_(&self) -> USBPRE__R {
        USBPRE__R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc SW"]
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W<0> {
        SW_W::new(self)
    }
    #[doc = "Bits 2:3 - desc SWS"]
    #[inline(always)]
    pub fn sws(&mut self) -> SWS_W<2> {
        SWS_W::new(self)
    }
    #[doc = "Bits 4:7 - desc HPRE"]
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W<4> {
        HPRE_W::new(self)
    }
    #[doc = "Bits 8:10 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&mut self) -> PPRE1_W<8> {
        PPRE1_W::new(self)
    }
    #[doc = "Bits 11:13 - APB high-speed prescaler(APB2)"]
    #[inline(always)]
    pub fn ppre2(&mut self) -> PPRE2_W<11> {
        PPRE2_W::new(self)
    }
    #[doc = "Bits 14:15 - desc ADCPRE"]
    #[inline(always)]
    pub fn adcpre(&mut self) -> ADCPRE_W<14> {
        ADCPRE_W::new(self)
    }
    #[doc = "Bit 16 - desc PLLSRC"]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W<16> {
        PLLSRC_W::new(self)
    }
    #[doc = "Bit 17 - desc PLLXTPRE"]
    #[inline(always)]
    pub fn pllxtpre(&mut self) -> PLLXTPRE_W<17> {
        PLLXTPRE_W::new(self)
    }
    #[doc = "Bits 18:21 - desc PLLMULL"]
    #[inline(always)]
    pub fn pllmull(&mut self) -> PLLMULL_W<18> {
        PLLMULL_W::new(self)
    }
    #[doc = "Bits 22:23 - desc USBPRE"]
    #[inline(always)]
    pub fn usbpre(&mut self) -> USBPRE_W<22> {
        USBPRE_W::new(self)
    }
    #[doc = "Bits 24:27 - desc MCO"]
    #[inline(always)]
    pub fn mco(&mut self) -> MCO_W<24> {
        MCO_W::new(self)
    }
    #[doc = "Bit 28 - desc ADCPRE"]
    #[inline(always)]
    pub fn adcpre_(&mut self) -> ADCPRE__W<28> {
        ADCPRE__W::new(self)
    }
    #[doc = "Bits 29:30 - desc PLLMULL"]
    #[inline(always)]
    pub fn pllmull_(&mut self) -> PLLMULL__W<29> {
        PLLMULL__W::new(self)
    }
    #[doc = "Bit 31 - desc USBPRE"]
    #[inline(always)]
    pub fn usbpre_(&mut self) -> USBPRE__W<31> {
        USBPRE__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
