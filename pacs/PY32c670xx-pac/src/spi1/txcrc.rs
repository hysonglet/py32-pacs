#[doc = "Register `TXCRC` reader"]
pub struct R(crate::R<TXCRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXCRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXCRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXCRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXCRC` writer"]
pub struct W(crate::W<TXCRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXCRC_SPEC>;
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
impl From<crate::W<TXCRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXCRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXCRC` reader - TX CRC register"]
pub type TXCRC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TXCRC` writer - TX CRC register"]
pub type TXCRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXCRC_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - TX CRC register"]
    #[inline(always)]
    pub fn txcrc(&self) -> TXCRC_R {
        TXCRC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TX CRC register"]
    #[inline(always)]
    pub fn txcrc(&mut self) -> TXCRC_W<0> {
        TXCRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX CRC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcrc](index.html) module"]
pub struct TXCRC_SPEC;
impl crate::RegisterSpec for TXCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txcrc::R](R) reader structure"]
impl crate::Readable for TXCRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txcrc::W](W) writer structure"]
impl crate::Writable for TXCRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXCRC to value 0"]
impl crate::Resettable for TXCRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
