#[doc = "Register `BKP_RTCCR` reader"]
pub struct R(crate::R<BKP_RTCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BKP_RTCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BKP_RTCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BKP_RTCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BKP_RTCCR` writer"]
pub struct W(crate::W<BKP_RTCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BKP_RTCCR_SPEC>;
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
impl From<crate::W<BKP_RTCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BKP_RTCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAL` reader - desc CAL"]
pub type CAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAL` writer - desc CAL"]
pub type CAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BKP_RTCCR_SPEC, u8, u8, 7, O>;
#[doc = "Field `CCO` reader - desc CCO"]
pub type CCO_R = crate::BitReader<bool>;
#[doc = "Field `CCO` writer - desc CCO"]
pub type CCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, BKP_RTCCR_SPEC, bool, O>;
#[doc = "Field `ASOE` reader - desc ASOE"]
pub type ASOE_R = crate::BitReader<bool>;
#[doc = "Field `ASOE` writer - desc ASOE"]
pub type ASOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BKP_RTCCR_SPEC, bool, O>;
#[doc = "Field `ASOS` reader - desc ASOS"]
pub type ASOS_R = crate::BitReader<bool>;
#[doc = "Field `ASOS` writer - desc ASOS"]
pub type ASOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, BKP_RTCCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - desc CAL"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - desc CCO"]
    #[inline(always)]
    pub fn cco(&self) -> CCO_R {
        CCO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc ASOE"]
    #[inline(always)]
    pub fn asoe(&self) -> ASOE_R {
        ASOE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc ASOS"]
    #[inline(always)]
    pub fn asos(&self) -> ASOS_R {
        ASOS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - desc CAL"]
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W<0> {
        CAL_W::new(self)
    }
    #[doc = "Bit 7 - desc CCO"]
    #[inline(always)]
    pub fn cco(&mut self) -> CCO_W<7> {
        CCO_W::new(self)
    }
    #[doc = "Bit 8 - desc ASOE"]
    #[inline(always)]
    pub fn asoe(&mut self) -> ASOE_W<8> {
        ASOE_W::new(self)
    }
    #[doc = "Bit 9 - desc ASOS"]
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
#[doc = "desc BKP_RTCCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp_rtccr](index.html) module"]
pub struct BKP_RTCCR_SPEC;
impl crate::RegisterSpec for BKP_RTCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bkp_rtccr::R](R) reader structure"]
impl crate::Readable for BKP_RTCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bkp_rtccr::W](W) writer structure"]
impl crate::Writable for BKP_RTCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BKP_RTCCR to value 0"]
impl crate::Resettable for BKP_RTCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
