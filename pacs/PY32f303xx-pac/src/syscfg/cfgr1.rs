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
#[doc = "Field `MEM_MODE` reader - desc MEM_MODE"]
pub type MEM_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEM_MODE` writer - desc MEM_MODE"]
pub type MEM_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `I2C_PB5` reader - desc I2C_PB5"]
pub type I2C_PB5_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PB5` writer - desc I2C_PB5"]
pub type I2C_PB5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `I2C_PB6` reader - desc I2C_PB6"]
pub type I2C_PB6_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PB6` writer - desc I2C_PB6"]
pub type I2C_PB6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `I2C_PB7` reader - desc I2C_PB7"]
pub type I2C_PB7_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PB7` writer - desc I2C_PB7"]
pub type I2C_PB7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `I2C_PB8` reader - desc I2C_PB8"]
pub type I2C_PB8_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PB8` writer - desc I2C_PB8"]
pub type I2C_PB8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `I2C_PB9` reader - desc I2C_PB9"]
pub type I2C_PB9_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PB9` writer - desc I2C_PB9"]
pub type I2C_PB9_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `I2C_PB10` reader - desc I2C_PB10"]
pub type I2C_PB10_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PB10` writer - desc I2C_PB10"]
pub type I2C_PB10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `I2C_PB11` reader - desc I2C_PB11"]
pub type I2C_PB11_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PB11` writer - desc I2C_PB11"]
pub type I2C_PB11_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `I2C_PB12` reader - desc I2C_PB12"]
pub type I2C_PB12_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PB12` writer - desc I2C_PB12"]
pub type I2C_PB12_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - desc MEM_MODE"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 16 - desc I2C_PB5"]
    #[inline(always)]
    pub fn i2c_pb5(&self) -> I2C_PB5_R {
        I2C_PB5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc I2C_PB6"]
    #[inline(always)]
    pub fn i2c_pb6(&self) -> I2C_PB6_R {
        I2C_PB6_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc I2C_PB7"]
    #[inline(always)]
    pub fn i2c_pb7(&self) -> I2C_PB7_R {
        I2C_PB7_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc I2C_PB8"]
    #[inline(always)]
    pub fn i2c_pb8(&self) -> I2C_PB8_R {
        I2C_PB8_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc I2C_PB9"]
    #[inline(always)]
    pub fn i2c_pb9(&self) -> I2C_PB9_R {
        I2C_PB9_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc I2C_PB10"]
    #[inline(always)]
    pub fn i2c_pb10(&self) -> I2C_PB10_R {
        I2C_PB10_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc I2C_PB11"]
    #[inline(always)]
    pub fn i2c_pb11(&self) -> I2C_PB11_R {
        I2C_PB11_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc I2C_PB12"]
    #[inline(always)]
    pub fn i2c_pb12(&self) -> I2C_PB12_R {
        I2C_PB12_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc MEM_MODE"]
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<0> {
        MEM_MODE_W::new(self)
    }
    #[doc = "Bit 16 - desc I2C_PB5"]
    #[inline(always)]
    pub fn i2c_pb5(&mut self) -> I2C_PB5_W<16> {
        I2C_PB5_W::new(self)
    }
    #[doc = "Bit 17 - desc I2C_PB6"]
    #[inline(always)]
    pub fn i2c_pb6(&mut self) -> I2C_PB6_W<17> {
        I2C_PB6_W::new(self)
    }
    #[doc = "Bit 18 - desc I2C_PB7"]
    #[inline(always)]
    pub fn i2c_pb7(&mut self) -> I2C_PB7_W<18> {
        I2C_PB7_W::new(self)
    }
    #[doc = "Bit 19 - desc I2C_PB8"]
    #[inline(always)]
    pub fn i2c_pb8(&mut self) -> I2C_PB8_W<19> {
        I2C_PB8_W::new(self)
    }
    #[doc = "Bit 20 - desc I2C_PB9"]
    #[inline(always)]
    pub fn i2c_pb9(&mut self) -> I2C_PB9_W<20> {
        I2C_PB9_W::new(self)
    }
    #[doc = "Bit 21 - desc I2C_PB10"]
    #[inline(always)]
    pub fn i2c_pb10(&mut self) -> I2C_PB10_W<21> {
        I2C_PB10_W::new(self)
    }
    #[doc = "Bit 22 - desc I2C_PB11"]
    #[inline(always)]
    pub fn i2c_pb11(&mut self) -> I2C_PB11_W<22> {
        I2C_PB11_W::new(self)
    }
    #[doc = "Bit 23 - desc I2C_PB12"]
    #[inline(always)]
    pub fn i2c_pb12(&mut self) -> I2C_PB12_W<23> {
        I2C_PB12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CFGR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](index.html) module"]
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
#[doc = "`reset()` method sets CFGR1 to value 0x00ff_0000"]
impl crate::Resettable for CFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00ff_0000
    }
}
