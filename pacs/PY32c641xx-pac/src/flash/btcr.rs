#[doc = "Register `BTCR` reader"]
pub struct R(crate::R<BTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BTCR` writer"]
pub struct W(crate::W<BTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BTCR_SPEC>;
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
impl From<crate::W<BTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOT_SIZE` reader - desc BOOT_SIZE"]
pub type BOOT_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOOT_SIZE` writer - desc BOOT_SIZE"]
pub type BOOT_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `BOOT0` reader - desc BOOT0"]
pub type BOOT0_R = crate::BitReader<bool>;
#[doc = "Field `BOOT0` writer - desc BOOT0"]
pub type BOOT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BTCR_SPEC, bool, O>;
#[doc = "Field `NBOOT1` reader - desc NBOOT1"]
pub type NBOOT1_R = crate::BitReader<bool>;
#[doc = "Field `NBOOT1` writer - desc NBOOT1"]
pub type NBOOT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BTCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - desc BOOT_SIZE"]
    #[inline(always)]
    pub fn boot_size(&self) -> BOOT_SIZE_R {
        BOOT_SIZE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 14 - desc BOOT0"]
    #[inline(always)]
    pub fn boot0(&self) -> BOOT0_R {
        BOOT0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc NBOOT1"]
    #[inline(always)]
    pub fn nboot1(&self) -> NBOOT1_R {
        NBOOT1_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc BOOT_SIZE"]
    #[inline(always)]
    pub fn boot_size(&mut self) -> BOOT_SIZE_W<0> {
        BOOT_SIZE_W::new(self)
    }
    #[doc = "Bit 14 - desc BOOT0"]
    #[inline(always)]
    pub fn boot0(&mut self) -> BOOT0_W<14> {
        BOOT0_W::new(self)
    }
    #[doc = "Bit 15 - desc NBOOT1"]
    #[inline(always)]
    pub fn nboot1(&mut self) -> NBOOT1_W<15> {
        NBOOT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH boot control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btcr](index.html) module"]
pub struct BTCR_SPEC;
impl crate::RegisterSpec for BTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [btcr::R](R) reader structure"]
impl crate::Readable for BTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [btcr::W](W) writer structure"]
impl crate::Writable for BTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BTCR to value 0"]
impl crate::Resettable for BTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
