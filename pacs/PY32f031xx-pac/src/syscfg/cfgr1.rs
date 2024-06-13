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
#[doc = "Field `I2C_PA2_ANF` reader - Analog filter enable control driving capability activation bits PA2"]
pub type I2C_PA2_ANF_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PA2_ANF` writer - Analog filter enable control driving capability activation bits PA2"]
pub type I2C_PA2_ANF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `I2C_PA3_ANF` reader - Analog filter enable control driving capability activation bits PA3"]
pub type I2C_PA3_ANF_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PA3_ANF` writer - Analog filter enable control driving capability activation bits PA3"]
pub type I2C_PA3_ANF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `I2C_PA7_ANF` reader - Analog filter enable control driving capability activation bits PA7"]
pub type I2C_PA7_ANF_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PA7_ANF` writer - Analog filter enable control driving capability activation bits PA7"]
pub type I2C_PA7_ANF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `I2C_PA8_ANF` reader - Analog filter enable control driving capability activation bits PA8"]
pub type I2C_PA8_ANF_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PA8_ANF` writer - Analog filter enable control driving capability activation bits PA8"]
pub type I2C_PA8_ANF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `I2C_PA9_ANF` reader - Analog filter enable control driving capability activation bits PA9"]
pub type I2C_PA9_ANF_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PA9_ANF` writer - Analog filter enable control driving capability activation bits PA9"]
pub type I2C_PA9_ANF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `I2C_PA10_ANF` reader - Analog filter enable control driving capability activation bits PA10"]
pub type I2C_PA10_ANF_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PA10_ANF` writer - Analog filter enable control driving capability activation bits PA10"]
pub type I2C_PA10_ANF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `I2C_PA11_ANF` reader - Analog filter enable control driving capability activation bits PA11"]
pub type I2C_PA11_ANF_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PA11_ANF` writer - Analog filter enable control driving capability activation bits PA11"]
pub type I2C_PA11_ANF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `I2C_PA12_ANF` reader - Analog filter enable control driving capability activation bits PA12"]
pub type I2C_PA12_ANF_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PA12_ANF` writer - Analog filter enable control driving capability activation bits PA12"]
pub type I2C_PA12_ANF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `I2C_PB6_ANF` reader - Analog filter enable control driving capability activation bits PB6"]
pub type I2C_PB6_ANF_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PB6_ANF` writer - Analog filter enable control driving capability activation bits PB6"]
pub type I2C_PB6_ANF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `I2C_PB7_ANF` reader - Analog filter enable control driving capability activation bits PB7"]
pub type I2C_PB7_ANF_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PB7_ANF` writer - Analog filter enable control driving capability activation bits PB7"]
pub type I2C_PB7_ANF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `I2C_PB8_ANF` reader - Analog filter enable control driving capability activation bits PB8"]
pub type I2C_PB8_ANF_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PB8_ANF` writer - Analog filter enable control driving capability activation bits PB8"]
pub type I2C_PB8_ANF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `I2C_PF0_ANF` reader - Analog filter enable control driving capability activation bits PF0"]
pub type I2C_PF0_ANF_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PF0_ANF` writer - Analog filter enable control driving capability activation bits PF0"]
pub type I2C_PF0_ANF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `I2C_PF1_ANF` reader - Analog filter enable control driving capability activation bits PF1"]
pub type I2C_PF1_ANF_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PF1_ANF` writer - Analog filter enable control driving capability activation bits PF1"]
pub type I2C_PF1_ANF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 18 - Analog filter enable control driving capability activation bits PA2"]
    #[inline(always)]
    pub fn i2c_pa2_anf(&self) -> I2C_PA2_ANF_R {
        I2C_PA2_ANF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Analog filter enable control driving capability activation bits PA3"]
    #[inline(always)]
    pub fn i2c_pa3_anf(&self) -> I2C_PA3_ANF_R {
        I2C_PA3_ANF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Analog filter enable control driving capability activation bits PA7"]
    #[inline(always)]
    pub fn i2c_pa7_anf(&self) -> I2C_PA7_ANF_R {
        I2C_PA7_ANF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Analog filter enable control driving capability activation bits PA8"]
    #[inline(always)]
    pub fn i2c_pa8_anf(&self) -> I2C_PA8_ANF_R {
        I2C_PA8_ANF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Analog filter enable control driving capability activation bits PA9"]
    #[inline(always)]
    pub fn i2c_pa9_anf(&self) -> I2C_PA9_ANF_R {
        I2C_PA9_ANF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Analog filter enable control driving capability activation bits PA10"]
    #[inline(always)]
    pub fn i2c_pa10_anf(&self) -> I2C_PA10_ANF_R {
        I2C_PA10_ANF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Analog filter enable control driving capability activation bits PA11"]
    #[inline(always)]
    pub fn i2c_pa11_anf(&self) -> I2C_PA11_ANF_R {
        I2C_PA11_ANF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Analog filter enable control driving capability activation bits PA12"]
    #[inline(always)]
    pub fn i2c_pa12_anf(&self) -> I2C_PA12_ANF_R {
        I2C_PA12_ANF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Analog filter enable control driving capability activation bits PB6"]
    #[inline(always)]
    pub fn i2c_pb6_anf(&self) -> I2C_PB6_ANF_R {
        I2C_PB6_ANF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Analog filter enable control driving capability activation bits PB7"]
    #[inline(always)]
    pub fn i2c_pb7_anf(&self) -> I2C_PB7_ANF_R {
        I2C_PB7_ANF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Analog filter enable control driving capability activation bits PB8"]
    #[inline(always)]
    pub fn i2c_pb8_anf(&self) -> I2C_PB8_ANF_R {
        I2C_PB8_ANF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Analog filter enable control driving capability activation bits PF0"]
    #[inline(always)]
    pub fn i2c_pf0_anf(&self) -> I2C_PF0_ANF_R {
        I2C_PF0_ANF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Analog filter enable control driving capability activation bits PF1"]
    #[inline(always)]
    pub fn i2c_pf1_anf(&self) -> I2C_PF1_ANF_R {
        I2C_PF1_ANF_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<0> {
        MEM_MODE_W::new(self)
    }
    #[doc = "Bit 18 - Analog filter enable control driving capability activation bits PA2"]
    #[inline(always)]
    pub fn i2c_pa2_anf(&mut self) -> I2C_PA2_ANF_W<18> {
        I2C_PA2_ANF_W::new(self)
    }
    #[doc = "Bit 19 - Analog filter enable control driving capability activation bits PA3"]
    #[inline(always)]
    pub fn i2c_pa3_anf(&mut self) -> I2C_PA3_ANF_W<19> {
        I2C_PA3_ANF_W::new(self)
    }
    #[doc = "Bit 20 - Analog filter enable control driving capability activation bits PA7"]
    #[inline(always)]
    pub fn i2c_pa7_anf(&mut self) -> I2C_PA7_ANF_W<20> {
        I2C_PA7_ANF_W::new(self)
    }
    #[doc = "Bit 21 - Analog filter enable control driving capability activation bits PA8"]
    #[inline(always)]
    pub fn i2c_pa8_anf(&mut self) -> I2C_PA8_ANF_W<21> {
        I2C_PA8_ANF_W::new(self)
    }
    #[doc = "Bit 22 - Analog filter enable control driving capability activation bits PA9"]
    #[inline(always)]
    pub fn i2c_pa9_anf(&mut self) -> I2C_PA9_ANF_W<22> {
        I2C_PA9_ANF_W::new(self)
    }
    #[doc = "Bit 23 - Analog filter enable control driving capability activation bits PA10"]
    #[inline(always)]
    pub fn i2c_pa10_anf(&mut self) -> I2C_PA10_ANF_W<23> {
        I2C_PA10_ANF_W::new(self)
    }
    #[doc = "Bit 24 - Analog filter enable control driving capability activation bits PA11"]
    #[inline(always)]
    pub fn i2c_pa11_anf(&mut self) -> I2C_PA11_ANF_W<24> {
        I2C_PA11_ANF_W::new(self)
    }
    #[doc = "Bit 25 - Analog filter enable control driving capability activation bits PA12"]
    #[inline(always)]
    pub fn i2c_pa12_anf(&mut self) -> I2C_PA12_ANF_W<25> {
        I2C_PA12_ANF_W::new(self)
    }
    #[doc = "Bit 26 - Analog filter enable control driving capability activation bits PB6"]
    #[inline(always)]
    pub fn i2c_pb6_anf(&mut self) -> I2C_PB6_ANF_W<26> {
        I2C_PB6_ANF_W::new(self)
    }
    #[doc = "Bit 27 - Analog filter enable control driving capability activation bits PB7"]
    #[inline(always)]
    pub fn i2c_pb7_anf(&mut self) -> I2C_PB7_ANF_W<27> {
        I2C_PB7_ANF_W::new(self)
    }
    #[doc = "Bit 28 - Analog filter enable control driving capability activation bits PB8"]
    #[inline(always)]
    pub fn i2c_pb8_anf(&mut self) -> I2C_PB8_ANF_W<28> {
        I2C_PB8_ANF_W::new(self)
    }
    #[doc = "Bit 29 - Analog filter enable control driving capability activation bits PF0"]
    #[inline(always)]
    pub fn i2c_pf0_anf(&mut self) -> I2C_PF0_ANF_W<29> {
        I2C_PF0_ANF_W::new(self)
    }
    #[doc = "Bit 30 - Analog filter enable control driving capability activation bits PF1"]
    #[inline(always)]
    pub fn i2c_pf1_anf(&mut self) -> I2C_PF1_ANF_W<30> {
        I2C_PF1_ANF_W::new(self)
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
