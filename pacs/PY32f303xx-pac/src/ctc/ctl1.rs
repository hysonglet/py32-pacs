#[doc = "Register `CTL1` reader"]
pub struct R(crate::R<CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL1` writer"]
pub struct W(crate::W<CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL1_SPEC>;
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
impl From<crate::W<CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RLVALUE` reader - desc RLVALUE"]
pub type RLVALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RLVALUE` writer - desc RLVALUE"]
pub type RLVALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL1_SPEC, u16, u16, 16, O>;
#[doc = "Field `CKLIM` reader - desc CKLIM"]
pub type CKLIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKLIM` writer - desc CKLIM"]
pub type CKLIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL1_SPEC, u8, u8, 8, O>;
#[doc = "Field `REFPSC` reader - desc REFPSC"]
pub type REFPSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFPSC` writer - desc REFPSC"]
pub type REFPSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL1_SPEC, u8, u8, 3, O>;
#[doc = "Field `REFSEL` reader - desc REFSEL"]
pub type REFSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFSEL` writer - desc REFSEL"]
pub type REFSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL1_SPEC, u8, u8, 2, O>;
#[doc = "Field `REFPOL` reader - desc REFPOL"]
pub type REFPOL_R = crate::BitReader<bool>;
#[doc = "Field `REFPOL` writer - desc REFPOL"]
pub type REFPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - desc RLVALUE"]
    #[inline(always)]
    pub fn rlvalue(&self) -> RLVALUE_R {
        RLVALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - desc CKLIM"]
    #[inline(always)]
    pub fn cklim(&self) -> CKLIM_R {
        CKLIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - desc REFPSC"]
    #[inline(always)]
    pub fn refpsc(&self) -> REFPSC_R {
        REFPSC_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - desc REFSEL"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - desc REFPOL"]
    #[inline(always)]
    pub fn refpol(&self) -> REFPOL_R {
        REFPOL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc RLVALUE"]
    #[inline(always)]
    pub fn rlvalue(&mut self) -> RLVALUE_W<0> {
        RLVALUE_W::new(self)
    }
    #[doc = "Bits 16:23 - desc CKLIM"]
    #[inline(always)]
    pub fn cklim(&mut self) -> CKLIM_W<16> {
        CKLIM_W::new(self)
    }
    #[doc = "Bits 24:26 - desc REFPSC"]
    #[inline(always)]
    pub fn refpsc(&mut self) -> REFPSC_W<24> {
        REFPSC_W::new(self)
    }
    #[doc = "Bits 28:29 - desc REFSEL"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W<28> {
        REFSEL_W::new(self)
    }
    #[doc = "Bit 31 - desc REFPOL"]
    #[inline(always)]
    pub fn refpol(&mut self) -> REFPOL_W<31> {
        REFPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CTL1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](index.html) module"]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl1::R](R) reader structure"]
impl crate::Readable for CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl1::W](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL1 to value 0x2022_bb7f"]
impl crate::Resettable for CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2022_bb7f
    }
}
