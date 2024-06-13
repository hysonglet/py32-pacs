#[doc = "Register `CFGR1` reader"]
pub struct R(crate::R<CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR1` writer"]
pub struct W(crate::W<CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR1_SPEC>;
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
impl From<crate::W<CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_MODE` reader - Memory mapping selection bits"]
pub type MEM_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEM_MODE` writer - Memory mapping selection bits"]
pub type MEM_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `I2C_PA2_FMP` reader - desc I2C_PA2_FMP"]
pub type I2C_PA2_FMP_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PA2_FMP` writer - desc I2C_PA2_FMP"]
pub type I2C_PA2_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `I2C_PB3_FMP` reader - desc I2C_PB3_FMP"]
pub type I2C_PB3_FMP_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PB3_FMP` writer - desc I2C_PB3_FMP"]
pub type I2C_PB3_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `I2C_PB4_FMP` reader - desc I2C_PB4_FMP"]
pub type I2C_PB4_FMP_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PB4_FMP` writer - desc I2C_PB4_FMP"]
pub type I2C_PB4_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `I2C_PB6_FMP` reader - desc I2C_PB6_FMP"]
pub type I2C_PB6_FMP_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PB6_FMP` writer - desc I2C_PB6_FMP"]
pub type I2C_PB6_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 16 - desc I2C_PA2_FMP"]
    #[inline(always)]
    pub fn i2c_pa2_fmp(&self) -> I2C_PA2_FMP_R {
        I2C_PA2_FMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc I2C_PB3_FMP"]
    #[inline(always)]
    pub fn i2c_pb3_fmp(&self) -> I2C_PB3_FMP_R {
        I2C_PB3_FMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc I2C_PB4_FMP"]
    #[inline(always)]
    pub fn i2c_pb4_fmp(&self) -> I2C_PB4_FMP_R {
        I2C_PB4_FMP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc I2C_PB6_FMP"]
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<0> {
        MEM_MODE_W::new(self)
    }
    #[doc = "Bit 16 - desc I2C_PA2_FMP"]
    #[inline(always)]
    pub fn i2c_pa2_fmp(&mut self) -> I2C_PA2_FMP_W<16> {
        I2C_PA2_FMP_W::new(self)
    }
    #[doc = "Bit 17 - desc I2C_PB3_FMP"]
    #[inline(always)]
    pub fn i2c_pb3_fmp(&mut self) -> I2C_PB3_FMP_W<17> {
        I2C_PB3_FMP_W::new(self)
    }
    #[doc = "Bit 18 - desc I2C_PB4_FMP"]
    #[inline(always)]
    pub fn i2c_pb4_fmp(&mut self) -> I2C_PB4_FMP_W<18> {
        I2C_PB4_FMP_W::new(self)
    }
    #[doc = "Bit 19 - desc I2C_PB6_FMP"]
    #[inline(always)]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W<19> {
        I2C_PB6_FMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](index.html) module"]
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr1::R](R) reader structure"]
impl crate::Readable for CFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr1::W](W) writer structure"]
impl crate::Writable for CFGR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
