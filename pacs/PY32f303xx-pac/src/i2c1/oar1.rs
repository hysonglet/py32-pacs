#[doc = "Register `OAR1` reader"]
pub struct R(crate::R<OAR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OAR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OAR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OAR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OAR1` writer"]
pub struct W(crate::W<OAR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OAR1_SPEC>;
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
impl From<crate::W<OAR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OAR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADD0` reader - desc ADD0"]
pub type ADD0_R = crate::BitReader<bool>;
#[doc = "Field `ADD0` writer - desc ADD0"]
pub type ADD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OAR1_SPEC, bool, O>;
#[doc = "Field `ADD1_7` reader - desc ADD"]
pub type ADD1_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADD1_7` writer - desc ADD"]
pub type ADD1_7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OAR1_SPEC, u8, u8, 7, O>;
#[doc = "Field `ADD8_9` reader - desc ADD"]
pub type ADD8_9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADD8_9` writer - desc ADD"]
pub type ADD8_9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OAR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADDMODE` reader - desc ADDMODE"]
pub type ADDMODE_R = crate::BitReader<bool>;
#[doc = "Field `ADDMODE` writer - desc ADDMODE"]
pub type ADDMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OAR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc ADD0"]
    #[inline(always)]
    pub fn add0(&self) -> ADD0_R {
        ADD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - desc ADD"]
    #[inline(always)]
    pub fn add1_7(&self) -> ADD1_7_R {
        ADD1_7_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - desc ADD"]
    #[inline(always)]
    pub fn add8_9(&self) -> ADD8_9_R {
        ADD8_9_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - desc ADDMODE"]
    #[inline(always)]
    pub fn addmode(&self) -> ADDMODE_R {
        ADDMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc ADD0"]
    #[inline(always)]
    pub fn add0(&mut self) -> ADD0_W<0> {
        ADD0_W::new(self)
    }
    #[doc = "Bits 1:7 - desc ADD"]
    #[inline(always)]
    pub fn add1_7(&mut self) -> ADD1_7_W<1> {
        ADD1_7_W::new(self)
    }
    #[doc = "Bits 8:9 - desc ADD"]
    #[inline(always)]
    pub fn add8_9(&mut self) -> ADD8_9_W<8> {
        ADD8_9_W::new(self)
    }
    #[doc = "Bit 15 - desc ADDMODE"]
    #[inline(always)]
    pub fn addmode(&mut self) -> ADDMODE_W<15> {
        ADDMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc OAR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oar1](index.html) module"]
pub struct OAR1_SPEC;
impl crate::RegisterSpec for OAR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oar1::R](R) reader structure"]
impl crate::Readable for OAR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oar1::W](W) writer structure"]
impl crate::Writable for OAR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OAR1 to value 0"]
impl crate::Resettable for OAR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
