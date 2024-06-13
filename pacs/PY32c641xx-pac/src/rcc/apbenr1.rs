#[doc = "Register `APBENR1` reader"]
pub struct R(crate::R<APBENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBENR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBENR1` writer"]
pub struct W(crate::W<APBENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBENR1_SPEC>;
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
impl From<crate::W<APBENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBENR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2CEN` reader - I2C clock enable"]
pub type I2CEN_R = crate::BitReader<bool>;
#[doc = "Field `I2CEN` writer - I2C clock enable"]
pub type I2CEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `DBGEN` reader - Debug support clock enable"]
pub type DBGEN_R = crate::BitReader<bool>;
#[doc = "Field `DBGEN` writer - Debug support clock enable"]
pub type DBGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `PWREN` reader - Power interface clock enable"]
pub type PWREN_R = crate::BitReader<bool>;
#[doc = "Field `PWREN` writer - Power interface clock enable"]
pub type PWREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `LPTIMEN` reader - LPTIM clock enable"]
pub type LPTIMEN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIMEN` writer - LPTIM clock enable"]
pub type LPTIMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 21 - I2C clock enable"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2CEN_R {
        I2CEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 27 - Debug support clock enable"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - LPTIM clock enable"]
    #[inline(always)]
    pub fn lptimen(&self) -> LPTIMEN_R {
        LPTIMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - I2C clock enable"]
    #[inline(always)]
    pub fn i2cen(&mut self) -> I2CEN_W<21> {
        I2CEN_W::new(self)
    }
    #[doc = "Bit 27 - Debug support clock enable"]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DBGEN_W<27> {
        DBGEN_W::new(self)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pwren(&mut self) -> PWREN_W<28> {
        PWREN_W::new(self)
    }
    #[doc = "Bit 31 - LPTIM clock enable"]
    #[inline(always)]
    pub fn lptimen(&mut self) -> LPTIMEN_W<31> {
        LPTIMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB peripheral clock enable register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbenr1](index.html) module"]
pub struct APBENR1_SPEC;
impl crate::RegisterSpec for APBENR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbenr1::R](R) reader structure"]
impl crate::Readable for APBENR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbenr1::W](W) writer structure"]
impl crate::Writable for APBENR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APBENR1 to value 0"]
impl crate::Resettable for APBENR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
