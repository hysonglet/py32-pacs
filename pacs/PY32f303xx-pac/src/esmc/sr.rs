#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSACTIVE` reader - desc SSACTIVE"]
pub type SSACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `SSACTIVE` writer - desc SSACTIVE"]
pub type SSACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SR_SPEC, bool, O>;
#[doc = "Field `RXBUSY` reader - desc RXBUSY"]
pub type RXBUSY_R = crate::BitReader<bool>;
#[doc = "Field `RXBUSY` writer - desc RXBUSY"]
pub type RXBUSY_W<'a, const O: u8> = crate::BitWriter<'a, u8, SR_SPEC, bool, O>;
#[doc = "Field `TXBUSY` reader - desc TXBUSY"]
pub type TXBUSY_R = crate::BitReader<bool>;
#[doc = "Field `TXBUSY` writer - desc TXBUSY"]
pub type TXBUSY_W<'a, const O: u8> = crate::BitWriter<'a, u8, SR_SPEC, bool, O>;
#[doc = "Field `IDLE` reader - desc IDLE"]
pub type IDLE_R = crate::BitReader<bool>;
#[doc = "Field `IDLE` writer - desc IDLE"]
pub type IDLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SR_SPEC, bool, O>;
#[doc = "Field `FIFIOOVERFOLOW` reader - desc FIFIOOVERFOLOW"]
pub type FIFIOOVERFOLOW_R = crate::BitReader<bool>;
#[doc = "Field `FIFIOOVERFOLOW` writer - desc FIFIOOVERFOLOW"]
pub type FIFIOOVERFOLOW_W<'a, const O: u8> = crate::BitWriter<'a, u8, SR_SPEC, bool, O>;
#[doc = "Field `SPIF` reader - desc SPIF"]
pub type SPIF_R = crate::BitReader<bool>;
#[doc = "Field `SPIF` writer - desc SPIF"]
pub type SPIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, SR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc SSACTIVE"]
    #[inline(always)]
    pub fn ssactive(&self) -> SSACTIVE_R {
        SSACTIVE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RXBUSY"]
    #[inline(always)]
    pub fn rxbusy(&self) -> RXBUSY_R {
        RXBUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TXBUSY"]
    #[inline(always)]
    pub fn txbusy(&self) -> TXBUSY_R {
        TXBUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc IDLE"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - desc FIFIOOVERFOLOW"]
    #[inline(always)]
    pub fn fifiooverfolow(&self) -> FIFIOOVERFOLOW_R {
        FIFIOOVERFOLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc SPIF"]
    #[inline(always)]
    pub fn spif(&self) -> SPIF_R {
        SPIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SSACTIVE"]
    #[inline(always)]
    pub fn ssactive(&mut self) -> SSACTIVE_W<0> {
        SSACTIVE_W::new(self)
    }
    #[doc = "Bit 1 - desc RXBUSY"]
    #[inline(always)]
    pub fn rxbusy(&mut self) -> RXBUSY_W<1> {
        RXBUSY_W::new(self)
    }
    #[doc = "Bit 2 - desc TXBUSY"]
    #[inline(always)]
    pub fn txbusy(&mut self) -> TXBUSY_W<2> {
        TXBUSY_W::new(self)
    }
    #[doc = "Bit 4 - desc IDLE"]
    #[inline(always)]
    pub fn idle(&mut self) -> IDLE_W<4> {
        IDLE_W::new(self)
    }
    #[doc = "Bit 6 - desc FIFIOOVERFOLOW"]
    #[inline(always)]
    pub fn fifiooverfolow(&mut self) -> FIFIOOVERFOLOW_W<6> {
        FIFIOOVERFOLOW_W::new(self)
    }
    #[doc = "Bit 7 - desc SPIF"]
    #[inline(always)]
    pub fn spif(&mut self) -> SPIF_W<7> {
        SPIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc SR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0x10"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
