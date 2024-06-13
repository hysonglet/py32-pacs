#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFTRST` reader - desc SOFTRST"]
pub type SOFTRST_R = crate::BitReader<bool>;
#[doc = "Field `SOFTRST` writer - desc SOFTRST"]
pub type SOFTRST_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR_SPEC, bool, O>;
#[doc = "Field `2XQSPI` reader - desc 2XQSPI"]
pub type _2XQSPI_R = crate::BitReader<bool>;
#[doc = "Field `2XQSPI` writer - desc 2XQSPI"]
pub type _2XQSPI_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR_SPEC, bool, O>;
#[doc = "Field `DMAEN` reader - desc DMAEN"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - desc DMAEN"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR_SPEC, bool, O>;
#[doc = "Field `GIE` reader - desc GIE"]
pub type GIE_R = crate::BitReader<bool>;
#[doc = "Field `GIE` writer - desc GIE"]
pub type GIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR_SPEC, bool, O>;
#[doc = "Field `XIPEN` reader - desc XIPEN"]
pub type XIPEN_R = crate::BitReader<bool>;
#[doc = "Field `XIPEN` writer - desc XIPEN"]
pub type XIPEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR_SPEC, bool, O>;
#[doc = "Field `SPIEN` reader - desc SPIEN"]
pub type SPIEN_R = crate::BitReader<bool>;
#[doc = "Field `SPIEN` writer - desc SPIEN"]
pub type SPIEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc SOFTRST"]
    #[inline(always)]
    pub fn softrst(&self) -> SOFTRST_R {
        SOFTRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - desc 2XQSPI"]
    #[inline(always)]
    pub fn _2xqspi(&self) -> _2XQSPI_R {
        _2XQSPI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc GIE"]
    #[inline(always)]
    pub fn gie(&self) -> GIE_R {
        GIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - desc XIPEN"]
    #[inline(always)]
    pub fn xipen(&self) -> XIPEN_R {
        XIPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc SPIEN"]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SOFTRST"]
    #[inline(always)]
    pub fn softrst(&mut self) -> SOFTRST_W<0> {
        SOFTRST_W::new(self)
    }
    #[doc = "Bit 2 - desc 2XQSPI"]
    #[inline(always)]
    pub fn _2xqspi(&mut self) -> _2XQSPI_W<2> {
        _2XQSPI_W::new(self)
    }
    #[doc = "Bit 3 - desc DMAEN"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<3> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 4 - desc GIE"]
    #[inline(always)]
    pub fn gie(&mut self) -> GIE_W<4> {
        GIE_W::new(self)
    }
    #[doc = "Bit 6 - desc XIPEN"]
    #[inline(always)]
    pub fn xipen(&mut self) -> XIPEN_W<6> {
        XIPEN_W::new(self)
    }
    #[doc = "Bit 7 - desc SPIEN"]
    #[inline(always)]
    pub fn spien(&mut self) -> SPIEN_W<7> {
        SPIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0xf8"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf8
    }
}
