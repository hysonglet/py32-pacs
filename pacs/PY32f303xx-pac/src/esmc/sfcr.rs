#[doc = "Register `SFCR` reader"]
pub struct R(crate::R<SFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFCR` writer"]
pub struct W(crate::W<SFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFCR_SPEC>;
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
impl From<crate::W<SFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INS0` reader - desc INS0"]
pub type INS0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INS0` writer - desc INS0"]
pub type INS0_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SFCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - desc INS0"]
    #[inline(always)]
    pub fn ins0(&self) -> INS0_R {
        INS0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc INS0"]
    #[inline(always)]
    pub fn ins0(&mut self) -> INS0_W<0> {
        INS0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc SFCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfcr](index.html) module"]
pub struct SFCR_SPEC;
impl crate::RegisterSpec for SFCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sfcr::R](R) reader structure"]
impl crate::Readable for SFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfcr::W](W) writer structure"]
impl crate::Writable for SFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SFCR to value 0x0b"]
impl crate::Resettable for SFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0b
    }
}
