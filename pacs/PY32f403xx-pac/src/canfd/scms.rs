#[doc = "Register `SCMS` reader"]
pub struct R(crate::R<SCMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCMS` writer"]
pub struct W(crate::W<SCMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCMS_SPEC>;
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
impl From<crate::W<SCMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCMS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XMREN` reader - desc XMREN"]
pub type XMREN_R = crate::BitReader<bool>;
#[doc = "Field `XMREN` writer - desc XMREN"]
pub type XMREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCMS_SPEC, bool, O>;
#[doc = "Field `FSTIM` reader - desc FSTIM"]
pub type FSTIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FSTIM` writer - desc FSTIM"]
pub type FSTIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCMS_SPEC, u8, u8, 3, O>;
#[doc = "Field `ACFA` reader - desc ACFA"]
pub type ACFA_R = crate::BitReader<bool>;
#[doc = "Field `ACFA` writer - desc ACFA"]
pub type ACFA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCMS_SPEC, bool, O>;
#[doc = "Field `TXS` reader - desc TXS"]
pub type TXS_R = crate::BitReader<bool>;
#[doc = "Field `TXB` reader - desc TXB"]
pub type TXB_R = crate::BitReader<bool>;
#[doc = "Field `HELOC` reader - desc HELOC"]
pub type HELOC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MPEN` reader - desc MPEN"]
pub type MPEN_R = crate::BitReader<bool>;
#[doc = "Field `MPEN` writer - desc MPEN"]
pub type MPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCMS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc XMREN"]
    #[inline(always)]
    pub fn xmren(&self) -> XMREN_R {
        XMREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - desc FSTIM"]
    #[inline(always)]
    pub fn fstim(&self) -> FSTIM_R {
        FSTIM_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 24 - desc ACFA"]
    #[inline(always)]
    pub fn acfa(&self) -> ACFA_R {
        ACFA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc TXS"]
    #[inline(always)]
    pub fn txs(&self) -> TXS_R {
        TXS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - desc TXB"]
    #[inline(always)]
    pub fn txb(&self) -> TXB_R {
        TXB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - desc HELOC"]
    #[inline(always)]
    pub fn heloc(&self) -> HELOC_R {
        HELOC_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 31 - desc MPEN"]
    #[inline(always)]
    pub fn mpen(&self) -> MPEN_R {
        MPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc XMREN"]
    #[inline(always)]
    pub fn xmren(&mut self) -> XMREN_W<0> {
        XMREN_W::new(self)
    }
    #[doc = "Bits 1:3 - desc FSTIM"]
    #[inline(always)]
    pub fn fstim(&mut self) -> FSTIM_W<1> {
        FSTIM_W::new(self)
    }
    #[doc = "Bit 24 - desc ACFA"]
    #[inline(always)]
    pub fn acfa(&mut self) -> ACFA_W<24> {
        ACFA_W::new(self)
    }
    #[doc = "Bit 31 - desc MPEN"]
    #[inline(always)]
    pub fn mpen(&mut self) -> MPEN_W<31> {
        MPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc SCMS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scms](index.html) module"]
pub struct SCMS_SPEC;
impl crate::RegisterSpec for SCMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scms::R](R) reader structure"]
impl crate::Readable for SCMS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scms::W](W) writer structure"]
impl crate::Writable for SCMS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCMS to value 0"]
impl crate::Resettable for SCMS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
