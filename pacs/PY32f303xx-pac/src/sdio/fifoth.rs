#[doc = "Register `FIFOTH` reader"]
pub struct R(crate::R<FIFOTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOTH` writer"]
pub struct W(crate::W<FIFOTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOTH_SPEC>;
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
impl From<crate::W<FIFOTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXWMARK` reader - desc TXWMARK"]
pub type TXWMARK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TXWMARK` writer - desc TXWMARK"]
pub type TXWMARK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFOTH_SPEC, u16, u16, 12, O>;
#[doc = "Field `RXWMARK` reader - desc RXWMARK"]
pub type RXWMARK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RXWMARK` writer - desc RXWMARK"]
pub type RXWMARK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFOTH_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - desc TXWMARK"]
    #[inline(always)]
    pub fn txwmark(&self) -> TXWMARK_R {
        TXWMARK_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - desc RXWMARK"]
    #[inline(always)]
    pub fn rxwmark(&self) -> RXWMARK_R {
        RXWMARK_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc TXWMARK"]
    #[inline(always)]
    pub fn txwmark(&mut self) -> TXWMARK_W<0> {
        TXWMARK_W::new(self)
    }
    #[doc = "Bits 16:27 - desc RXWMARK"]
    #[inline(always)]
    pub fn rxwmark(&mut self) -> RXWMARK_W<16> {
        RXWMARK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc FIFOTH\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoth](index.html) module"]
pub struct FIFOTH_SPEC;
impl crate::RegisterSpec for FIFOTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifoth::R](R) reader structure"]
impl crate::Readable for FIFOTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifoth::W](W) writer structure"]
impl crate::Writable for FIFOTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOTH to value 0"]
impl crate::Resettable for FIFOTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
