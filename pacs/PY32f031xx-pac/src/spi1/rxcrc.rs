#[doc = "Register `RXCRC` reader"]
pub struct R(crate::R<RXCRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXCRC` writer"]
pub struct W(crate::W<RXCRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXCRC_SPEC>;
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
impl From<crate::W<RXCRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXCRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXCRC` reader - RX CRC register"]
pub type RXCRC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RXCRC` writer - RX CRC register"]
pub type RXCRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXCRC_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - RX CRC register"]
    #[inline(always)]
    pub fn rxcrc(&self) -> RXCRC_R {
        RXCRC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RX CRC register"]
    #[inline(always)]
    pub fn rxcrc(&mut self) -> RXCRC_W<0> {
        RXCRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX CRC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcrc](index.html) module"]
pub struct RXCRC_SPEC;
impl crate::RegisterSpec for RXCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxcrc::R](R) reader structure"]
impl crate::Readable for RXCRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxcrc::W](W) writer structure"]
impl crate::Writable for RXCRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXCRC to value 0"]
impl crate::Resettable for RXCRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
