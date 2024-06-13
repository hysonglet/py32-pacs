#[doc = "Register `ACFCR` reader"]
pub struct R(crate::R<ACFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACFCR` writer"]
pub struct W(crate::W<ACFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACFCR_SPEC>;
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
impl From<crate::W<ACFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACFADR` reader - desc ACFADR"]
pub type ACFADR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACFADR` writer - desc ACFADR"]
pub type ACFADR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACFCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `AE_0` reader - desc AE_0"]
pub type AE_0_R = crate::BitReader<bool>;
#[doc = "Field `AE_0` writer - desc AE_0"]
pub type AE_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACFCR_SPEC, bool, O>;
#[doc = "Field `AE_1` reader - desc AE_1"]
pub type AE_1_R = crate::BitReader<bool>;
#[doc = "Field `AE_1` writer - desc AE_1"]
pub type AE_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACFCR_SPEC, bool, O>;
#[doc = "Field `AE_2` reader - desc AE_2"]
pub type AE_2_R = crate::BitReader<bool>;
#[doc = "Field `AE_2` writer - desc AE_2"]
pub type AE_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACFCR_SPEC, bool, O>;
#[doc = "Field `AE_3` reader - desc AE_3"]
pub type AE_3_R = crate::BitReader<bool>;
#[doc = "Field `AE_3` writer - desc AE_3"]
pub type AE_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACFCR_SPEC, bool, O>;
#[doc = "Field `AE_4` reader - desc AE_4"]
pub type AE_4_R = crate::BitReader<bool>;
#[doc = "Field `AE_4` writer - desc AE_4"]
pub type AE_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACFCR_SPEC, bool, O>;
#[doc = "Field `AE_5` reader - desc AE_5"]
pub type AE_5_R = crate::BitReader<bool>;
#[doc = "Field `AE_5` writer - desc AE_5"]
pub type AE_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACFCR_SPEC, bool, O>;
#[doc = "Field `AE_6` reader - desc AE_6"]
pub type AE_6_R = crate::BitReader<bool>;
#[doc = "Field `AE_6` writer - desc AE_6"]
pub type AE_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACFCR_SPEC, bool, O>;
#[doc = "Field `AE_7` reader - desc AE_7"]
pub type AE_7_R = crate::BitReader<bool>;
#[doc = "Field `AE_7` writer - desc AE_7"]
pub type AE_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACFCR_SPEC, bool, O>;
#[doc = "Field `AE_8` reader - desc AE_8"]
pub type AE_8_R = crate::BitReader<bool>;
#[doc = "Field `AE_8` writer - desc AE_8"]
pub type AE_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACFCR_SPEC, bool, O>;
#[doc = "Field `AE_9` reader - desc AE_9"]
pub type AE_9_R = crate::BitReader<bool>;
#[doc = "Field `AE_9` writer - desc AE_9"]
pub type AE_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACFCR_SPEC, bool, O>;
#[doc = "Field `AE_10` reader - desc AE_10"]
pub type AE_10_R = crate::BitReader<bool>;
#[doc = "Field `AE_10` writer - desc AE_10"]
pub type AE_10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACFCR_SPEC, bool, O>;
#[doc = "Field `AE_11` reader - desc AE_11"]
pub type AE_11_R = crate::BitReader<bool>;
#[doc = "Field `AE_11` writer - desc AE_11"]
pub type AE_11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACFCR_SPEC, bool, O>;
#[doc = "Field `AE_12` reader - desc AE_12"]
pub type AE_12_R = crate::BitReader<bool>;
#[doc = "Field `AE_12` writer - desc AE_12"]
pub type AE_12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACFCR_SPEC, bool, O>;
#[doc = "Field `AE_13` reader - desc AE_13"]
pub type AE_13_R = crate::BitReader<bool>;
#[doc = "Field `AE_13` writer - desc AE_13"]
pub type AE_13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACFCR_SPEC, bool, O>;
#[doc = "Field `AE_14` reader - desc AE_14"]
pub type AE_14_R = crate::BitReader<bool>;
#[doc = "Field `AE_14` writer - desc AE_14"]
pub type AE_14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACFCR_SPEC, bool, O>;
#[doc = "Field `AE_15` reader - desc AE_15"]
pub type AE_15_R = crate::BitReader<bool>;
#[doc = "Field `AE_15` writer - desc AE_15"]
pub type AE_15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACFCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - desc ACFADR"]
    #[inline(always)]
    pub fn acfadr(&self) -> ACFADR_R {
        ACFADR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 16 - desc AE_0"]
    #[inline(always)]
    pub fn ae_0(&self) -> AE_0_R {
        AE_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc AE_1"]
    #[inline(always)]
    pub fn ae_1(&self) -> AE_1_R {
        AE_1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc AE_2"]
    #[inline(always)]
    pub fn ae_2(&self) -> AE_2_R {
        AE_2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc AE_3"]
    #[inline(always)]
    pub fn ae_3(&self) -> AE_3_R {
        AE_3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc AE_4"]
    #[inline(always)]
    pub fn ae_4(&self) -> AE_4_R {
        AE_4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc AE_5"]
    #[inline(always)]
    pub fn ae_5(&self) -> AE_5_R {
        AE_5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc AE_6"]
    #[inline(always)]
    pub fn ae_6(&self) -> AE_6_R {
        AE_6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc AE_7"]
    #[inline(always)]
    pub fn ae_7(&self) -> AE_7_R {
        AE_7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - desc AE_8"]
    #[inline(always)]
    pub fn ae_8(&self) -> AE_8_R {
        AE_8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc AE_9"]
    #[inline(always)]
    pub fn ae_9(&self) -> AE_9_R {
        AE_9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - desc AE_10"]
    #[inline(always)]
    pub fn ae_10(&self) -> AE_10_R {
        AE_10_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - desc AE_11"]
    #[inline(always)]
    pub fn ae_11(&self) -> AE_11_R {
        AE_11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - desc AE_12"]
    #[inline(always)]
    pub fn ae_12(&self) -> AE_12_R {
        AE_12_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc AE_13"]
    #[inline(always)]
    pub fn ae_13(&self) -> AE_13_R {
        AE_13_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc AE_14"]
    #[inline(always)]
    pub fn ae_14(&self) -> AE_14_R {
        AE_14_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc AE_15"]
    #[inline(always)]
    pub fn ae_15(&self) -> AE_15_R {
        AE_15_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc ACFADR"]
    #[inline(always)]
    pub fn acfadr(&mut self) -> ACFADR_W<0> {
        ACFADR_W::new(self)
    }
    #[doc = "Bit 16 - desc AE_0"]
    #[inline(always)]
    pub fn ae_0(&mut self) -> AE_0_W<16> {
        AE_0_W::new(self)
    }
    #[doc = "Bit 17 - desc AE_1"]
    #[inline(always)]
    pub fn ae_1(&mut self) -> AE_1_W<17> {
        AE_1_W::new(self)
    }
    #[doc = "Bit 18 - desc AE_2"]
    #[inline(always)]
    pub fn ae_2(&mut self) -> AE_2_W<18> {
        AE_2_W::new(self)
    }
    #[doc = "Bit 19 - desc AE_3"]
    #[inline(always)]
    pub fn ae_3(&mut self) -> AE_3_W<19> {
        AE_3_W::new(self)
    }
    #[doc = "Bit 20 - desc AE_4"]
    #[inline(always)]
    pub fn ae_4(&mut self) -> AE_4_W<20> {
        AE_4_W::new(self)
    }
    #[doc = "Bit 21 - desc AE_5"]
    #[inline(always)]
    pub fn ae_5(&mut self) -> AE_5_W<21> {
        AE_5_W::new(self)
    }
    #[doc = "Bit 22 - desc AE_6"]
    #[inline(always)]
    pub fn ae_6(&mut self) -> AE_6_W<22> {
        AE_6_W::new(self)
    }
    #[doc = "Bit 23 - desc AE_7"]
    #[inline(always)]
    pub fn ae_7(&mut self) -> AE_7_W<23> {
        AE_7_W::new(self)
    }
    #[doc = "Bit 24 - desc AE_8"]
    #[inline(always)]
    pub fn ae_8(&mut self) -> AE_8_W<24> {
        AE_8_W::new(self)
    }
    #[doc = "Bit 25 - desc AE_9"]
    #[inline(always)]
    pub fn ae_9(&mut self) -> AE_9_W<25> {
        AE_9_W::new(self)
    }
    #[doc = "Bit 26 - desc AE_10"]
    #[inline(always)]
    pub fn ae_10(&mut self) -> AE_10_W<26> {
        AE_10_W::new(self)
    }
    #[doc = "Bit 27 - desc AE_11"]
    #[inline(always)]
    pub fn ae_11(&mut self) -> AE_11_W<27> {
        AE_11_W::new(self)
    }
    #[doc = "Bit 28 - desc AE_12"]
    #[inline(always)]
    pub fn ae_12(&mut self) -> AE_12_W<28> {
        AE_12_W::new(self)
    }
    #[doc = "Bit 29 - desc AE_13"]
    #[inline(always)]
    pub fn ae_13(&mut self) -> AE_13_W<29> {
        AE_13_W::new(self)
    }
    #[doc = "Bit 30 - desc AE_14"]
    #[inline(always)]
    pub fn ae_14(&mut self) -> AE_14_W<30> {
        AE_14_W::new(self)
    }
    #[doc = "Bit 31 - desc AE_15"]
    #[inline(always)]
    pub fn ae_15(&mut self) -> AE_15_W<31> {
        AE_15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc ACFCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acfcr](index.html) module"]
pub struct ACFCR_SPEC;
impl crate::RegisterSpec for ACFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acfcr::R](R) reader structure"]
impl crate::Readable for ACFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acfcr::W](W) writer structure"]
impl crate::Writable for ACFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACFCR to value 0x0001_0000"]
impl crate::Resettable for ACFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0000
    }
}
