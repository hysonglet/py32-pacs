#[doc = "Register `ADDR32` reader"]
pub struct R(crate::R<ADDR32_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR32_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR32_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR32_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR32` writer"]
pub struct W(crate::W<ADDR32_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR32_SPEC>;
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
impl From<crate::W<ADDR32_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR32_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR3` reader - desc ADDR3"]
pub type ADDR3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR3` writer - desc ADDR3"]
pub type ADDR3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR32_SPEC, u8, u8, 8, O>;
#[doc = "Field `MREG` reader - desc MREG"]
pub type MREG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MREG` writer - desc MREG"]
pub type MREG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR32_SPEC, u8, u8, 8, O>;
#[doc = "Field `SS` reader - desc SS"]
pub type SS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SS` writer - desc SS"]
pub type SS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR32_SPEC, u8, u8, 4, O>;
#[doc = "Field `XSS` reader - desc XSS"]
pub type XSS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XSS` writer - desc XSS"]
pub type XSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR32_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - desc ADDR3"]
    #[inline(always)]
    pub fn addr3(&self) -> ADDR3_R {
        ADDR3_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - desc MREG"]
    #[inline(always)]
    pub fn mreg(&self) -> MREG_R {
        MREG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - desc SS"]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - desc XSS"]
    #[inline(always)]
    pub fn xss(&self) -> XSS_R {
        XSS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc ADDR3"]
    #[inline(always)]
    pub fn addr3(&mut self) -> ADDR3_W<0> {
        ADDR3_W::new(self)
    }
    #[doc = "Bits 8:15 - desc MREG"]
    #[inline(always)]
    pub fn mreg(&mut self) -> MREG_W<8> {
        MREG_W::new(self)
    }
    #[doc = "Bits 16:19 - desc SS"]
    #[inline(always)]
    pub fn ss(&mut self) -> SS_W<16> {
        SS_W::new(self)
    }
    #[doc = "Bits 24:27 - desc XSS"]
    #[inline(always)]
    pub fn xss(&mut self) -> XSS_W<24> {
        XSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc ADDR32\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr32](index.html) module"]
pub struct ADDR32_SPEC;
impl crate::RegisterSpec for ADDR32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr32::R](R) reader structure"]
impl crate::Readable for ADDR32_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr32::W](W) writer structure"]
impl crate::Writable for ADDR32_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDR32 to value 0x0101_0000"]
impl crate::Resettable for ADDR32_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0101_0000
    }
}
