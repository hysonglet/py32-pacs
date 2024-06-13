#[doc = "Register `AHB2RSTR` reader"]
pub struct R(crate::R<AHB2RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2RSTR` writer"]
pub struct W(crate::W<AHB2RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2RSTR_SPEC>;
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
impl From<crate::W<AHB2RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOPARST` reader - desc IOPARST"]
pub type IOPARST_R = crate::BitReader<bool>;
#[doc = "Field `IOPARST` writer - desc IOPARST"]
pub type IOPARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
#[doc = "Field `IOPBRST` reader - desc IOPBRST"]
pub type IOPBRST_R = crate::BitReader<bool>;
#[doc = "Field `IOPBRST` writer - desc IOPBRST"]
pub type IOPBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
#[doc = "Field `IOPCRST` reader - desc IOPCRST"]
pub type IOPCRST_R = crate::BitReader<bool>;
#[doc = "Field `IOPCRST` writer - desc IOPCRST"]
pub type IOPCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
#[doc = "Field `IOPDRST` reader - desc IOPDRST"]
pub type IOPDRST_R = crate::BitReader<bool>;
#[doc = "Field `IOPDRST` writer - desc IOPDRST"]
pub type IOPDRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
#[doc = "Field `IOPERST` reader - desc IOPERST"]
pub type IOPERST_R = crate::BitReader<bool>;
#[doc = "Field `IOPERST` writer - desc IOPERST"]
pub type IOPERST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - desc IOPARST"]
    #[inline(always)]
    pub fn ioparst(&self) -> IOPARST_R {
        IOPARST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc IOPBRST"]
    #[inline(always)]
    pub fn iopbrst(&self) -> IOPBRST_R {
        IOPBRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc IOPCRST"]
    #[inline(always)]
    pub fn iopcrst(&self) -> IOPCRST_R {
        IOPCRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc IOPDRST"]
    #[inline(always)]
    pub fn iopdrst(&self) -> IOPDRST_R {
        IOPDRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc IOPERST"]
    #[inline(always)]
    pub fn ioperst(&self) -> IOPERST_R {
        IOPERST_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - desc IOPARST"]
    #[inline(always)]
    pub fn ioparst(&mut self) -> IOPARST_W<2> {
        IOPARST_W::new(self)
    }
    #[doc = "Bit 3 - desc IOPBRST"]
    #[inline(always)]
    pub fn iopbrst(&mut self) -> IOPBRST_W<3> {
        IOPBRST_W::new(self)
    }
    #[doc = "Bit 4 - desc IOPCRST"]
    #[inline(always)]
    pub fn iopcrst(&mut self) -> IOPCRST_W<4> {
        IOPCRST_W::new(self)
    }
    #[doc = "Bit 5 - desc IOPDRST"]
    #[inline(always)]
    pub fn iopdrst(&mut self) -> IOPDRST_W<5> {
        IOPDRST_W::new(self)
    }
    #[doc = "Bit 6 - desc IOPERST"]
    #[inline(always)]
    pub fn ioperst(&mut self) -> IOPERST_W<6> {
        IOPERST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB2 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2rstr](index.html) module"]
pub struct AHB2RSTR_SPEC;
impl crate::RegisterSpec for AHB2RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2rstr::R](R) reader structure"]
impl crate::Readable for AHB2RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2rstr::W](W) writer structure"]
impl crate::Writable for AHB2RSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB2RSTR to value 0"]
impl crate::Resettable for AHB2RSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
