#[doc = "Register `RTCCR` reader"]
pub struct R(crate::R<RTCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCCR` writer"]
pub struct W(crate::W<RTCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCCR_SPEC>;
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
impl From<crate::W<RTCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAL` reader - Calibration value"]
pub type CAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAL` writer - Calibration value"]
pub type CAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTCCR_SPEC, u8, u8, 7, O>;
#[doc = "Field `CCO` reader - Calibration clock output"]
pub type CCO_R = crate::BitReader<bool>;
#[doc = "Field `CCO` writer - Calibration clock output"]
pub type CCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTCCR_SPEC, bool, O>;
#[doc = "Field `ASOE` reader - Alarm or second output enable"]
pub type ASOE_R = crate::BitReader<bool>;
#[doc = "Field `ASOE` writer - Alarm or second output enable"]
pub type ASOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTCCR_SPEC, bool, O>;
#[doc = "Field `ASOS` reader - Alarm or second output selection"]
pub type ASOS_R = crate::BitReader<bool>;
#[doc = "Field `ASOS` writer - Alarm or second output selection"]
pub type ASOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTCCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Calibration clock output"]
    #[inline(always)]
    pub fn cco(&self) -> CCO_R {
        CCO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm or second output enable"]
    #[inline(always)]
    pub fn asoe(&self) -> ASOE_R {
        ASOE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm or second output selection"]
    #[inline(always)]
    pub fn asos(&self) -> ASOS_R {
        ASOS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W<0> {
        CAL_W::new(self)
    }
    #[doc = "Bit 7 - Calibration clock output"]
    #[inline(always)]
    pub fn cco(&mut self) -> CCO_W<7> {
        CCO_W::new(self)
    }
    #[doc = "Bit 8 - Alarm or second output enable"]
    #[inline(always)]
    pub fn asoe(&mut self) -> ASOE_W<8> {
        ASOE_W::new(self)
    }
    #[doc = "Bit 9 - Alarm or second output selection"]
    #[inline(always)]
    pub fn asos(&mut self) -> ASOS_W<9> {
        ASOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC clock calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccr](index.html) module"]
pub struct RTCCR_SPEC;
impl crate::RegisterSpec for RTCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtccr::R](R) reader structure"]
impl crate::Readable for RTCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtccr::W](W) writer structure"]
impl crate::Writable for RTCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCCR to value 0"]
impl crate::Resettable for RTCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
