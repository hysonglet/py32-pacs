#[doc = "Register `XCR3` reader"]
pub struct R(crate::R<XCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XCR3` writer"]
pub struct W(crate::W<XCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XCR3_SPEC>;
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
impl From<crate::W<XCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR8BIT` reader - desc ADDR8BIT"]
pub type ADDR8BIT_R = crate::BitReader<bool>;
#[doc = "Field `ADDR8BIT` writer - desc ADDR8BIT"]
pub type ADDR8BIT_W<'a, const O: u8> = crate::BitWriter<'a, u8, XCR3_SPEC, bool, O>;
#[doc = "Field `ADDR16BIT` reader - desc ADDR16BIT"]
pub type ADDR16BIT_R = crate::BitReader<bool>;
#[doc = "Field `ADDR16BIT` writer - desc ADDR16BIT"]
pub type ADDR16BIT_W<'a, const O: u8> = crate::BitWriter<'a, u8, XCR3_SPEC, bool, O>;
#[doc = "Field `ADDR32BIT` reader - desc ADDR32BIT"]
pub type ADDR32BIT_R = crate::BitReader<bool>;
#[doc = "Field `ADDR32BIT` writer - desc ADDR32BIT"]
pub type ADDR32BIT_W<'a, const O: u8> = crate::BitWriter<'a, u8, XCR3_SPEC, bool, O>;
#[doc = "Field `SSCLRRQ` reader - desc SSCLRRQ"]
pub type SSCLRRQ_R = crate::BitReader<bool>;
#[doc = "Field `SSCLRRQ` writer - desc SSCLRRQ"]
pub type SSCLRRQ_W<'a, const O: u8> = crate::BitWriter<'a, u8, XCR3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc ADDR8BIT"]
    #[inline(always)]
    pub fn addr8bit(&self) -> ADDR8BIT_R {
        ADDR8BIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc ADDR16BIT"]
    #[inline(always)]
    pub fn addr16bit(&self) -> ADDR16BIT_R {
        ADDR16BIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc ADDR32BIT"]
    #[inline(always)]
    pub fn addr32bit(&self) -> ADDR32BIT_R {
        ADDR32BIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - desc SSCLRRQ"]
    #[inline(always)]
    pub fn ssclrrq(&self) -> SSCLRRQ_R {
        SSCLRRQ_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc ADDR8BIT"]
    #[inline(always)]
    pub fn addr8bit(&mut self) -> ADDR8BIT_W<0> {
        ADDR8BIT_W::new(self)
    }
    #[doc = "Bit 1 - desc ADDR16BIT"]
    #[inline(always)]
    pub fn addr16bit(&mut self) -> ADDR16BIT_W<1> {
        ADDR16BIT_W::new(self)
    }
    #[doc = "Bit 2 - desc ADDR32BIT"]
    #[inline(always)]
    pub fn addr32bit(&mut self) -> ADDR32BIT_W<2> {
        ADDR32BIT_W::new(self)
    }
    #[doc = "Bit 6 - desc SSCLRRQ"]
    #[inline(always)]
    pub fn ssclrrq(&mut self) -> SSCLRRQ_W<6> {
        SSCLRRQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc XCR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xcr3](index.html) module"]
pub struct XCR3_SPEC;
impl crate::RegisterSpec for XCR3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [xcr3::R](R) reader structure"]
impl crate::Readable for XCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xcr3::W](W) writer structure"]
impl crate::Writable for XCR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XCR3 to value 0"]
impl crate::Resettable for XCR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
