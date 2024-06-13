#[doc = "Register `TMOUT` reader"]
pub struct R(crate::R<TMOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMOUT` writer"]
pub struct W(crate::W<TMOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMOUT_SPEC>;
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
impl From<crate::W<TMOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESPTIME` reader - desc RESPTIME"]
pub type RESPTIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESPTIME` writer - desc RESPTIME"]
pub type RESPTIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMOUT_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATATIME` reader - desc DATATIME"]
pub type DATATIME_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATATIME` writer - desc DATATIME"]
pub type DATATIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMOUT_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - desc RESPTIME"]
    #[inline(always)]
    pub fn resptime(&self) -> RESPTIME_R {
        RESPTIME_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - desc DATATIME"]
    #[inline(always)]
    pub fn datatime(&self) -> DATATIME_R {
        DATATIME_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc RESPTIME"]
    #[inline(always)]
    pub fn resptime(&mut self) -> RESPTIME_W<0> {
        RESPTIME_W::new(self)
    }
    #[doc = "Bits 8:31 - desc DATATIME"]
    #[inline(always)]
    pub fn datatime(&mut self) -> DATATIME_W<8> {
        DATATIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc TMOUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmout](index.html) module"]
pub struct TMOUT_SPEC;
impl crate::RegisterSpec for TMOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmout::R](R) reader structure"]
impl crate::Readable for TMOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmout::W](W) writer structure"]
impl crate::Writable for TMOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMOUT to value 0xffff_ff40"]
impl crate::Resettable for TMOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ff40
    }
}
