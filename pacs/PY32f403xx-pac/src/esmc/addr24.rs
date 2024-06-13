#[doc = "Register `ADDR24` reader"]
pub struct R(crate::R<ADDR24_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR24_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR24_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR24_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR24` writer"]
pub struct W(crate::W<ADDR24_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR24_SPEC>;
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
impl From<crate::W<ADDR24_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR24_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INS` reader - desc INS"]
pub type INS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INS` writer - desc INS"]
pub type INS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR24_SPEC, u8, u8, 8, O>;
#[doc = "Field `ADDR0` reader - desc ADDR0"]
pub type ADDR0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR0` writer - desc ADDR0"]
pub type ADDR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR24_SPEC, u8, u8, 8, O>;
#[doc = "Field `ADDR1` reader - desc ADDR1"]
pub type ADDR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR1` writer - desc ADDR1"]
pub type ADDR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR24_SPEC, u8, u8, 8, O>;
#[doc = "Field `ADDR2` reader - desc ADDR2"]
pub type ADDR2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR2` writer - desc ADDR2"]
pub type ADDR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR24_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - desc INS"]
    #[inline(always)]
    pub fn ins(&self) -> INS_R {
        INS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - desc ADDR0"]
    #[inline(always)]
    pub fn addr0(&self) -> ADDR0_R {
        ADDR0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - desc ADDR1"]
    #[inline(always)]
    pub fn addr1(&self) -> ADDR1_R {
        ADDR1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - desc ADDR2"]
    #[inline(always)]
    pub fn addr2(&self) -> ADDR2_R {
        ADDR2_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc INS"]
    #[inline(always)]
    pub fn ins(&mut self) -> INS_W<0> {
        INS_W::new(self)
    }
    #[doc = "Bits 8:15 - desc ADDR0"]
    #[inline(always)]
    pub fn addr0(&mut self) -> ADDR0_W<8> {
        ADDR0_W::new(self)
    }
    #[doc = "Bits 16:23 - desc ADDR1"]
    #[inline(always)]
    pub fn addr1(&mut self) -> ADDR1_W<16> {
        ADDR1_W::new(self)
    }
    #[doc = "Bits 24:31 - desc ADDR2"]
    #[inline(always)]
    pub fn addr2(&mut self) -> ADDR2_W<24> {
        ADDR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc ADDR24\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr24](index.html) module"]
pub struct ADDR24_SPEC;
impl crate::RegisterSpec for ADDR24_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr24::R](R) reader structure"]
impl crate::Readable for ADDR24_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr24::W](W) writer structure"]
impl crate::Writable for ADDR24_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDR24 to value 0x0b"]
impl crate::Resettable for ADDR24_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0b
    }
}
