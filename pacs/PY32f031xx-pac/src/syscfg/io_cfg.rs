#[doc = "Register `IO_CFG` reader"]
pub struct R(crate::R<IO_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IO_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IO_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IO_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IO_CFG` writer"]
pub struct W(crate::W<IO_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IO_CFG_SPEC>;
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
impl From<crate::W<IO_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IO_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA_EI2C` reader - desc PA_EI2C"]
pub type PA_EI2C_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PA_EI2C` writer - desc PA_EI2C"]
pub type PA_EI2C_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IO_CFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `PB_EI2C` reader - desc PB_EI2C"]
pub type PB_EI2C_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PB_EI2C` writer - desc PB_EI2C"]
pub type PB_EI2C_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IO_CFG_SPEC, u8, u8, 5, O>;
#[doc = "Field `PA_EHS` reader - desc PA_EHS"]
pub type PA_EHS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PA_EHS` writer - desc PA_EHS"]
pub type PA_EHS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IO_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `PB_EHS` reader - desc PB_EHS"]
pub type PB_EHS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PB_EHS` writer - desc PB_EHS"]
pub type PB_EHS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IO_CFG_SPEC, u8, u8, 6, O>;
#[doc = "Field `PF_EI2C` reader - desc PF_EI2C"]
pub type PF_EI2C_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PF_EI2C` writer - desc PF_EI2C"]
pub type PF_EI2C_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IO_CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `PF_PU_I2C` reader - desc PF_PU_I2C"]
pub type PF_PU_I2C_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PF_PU_I2C` writer - desc PF_PU_I2C"]
pub type PF_PU_I2C_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IO_CFG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:7 - desc PA_EI2C"]
    #[inline(always)]
    pub fn pa_ei2c(&self) -> PA_EI2C_R {
        PA_EI2C_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - desc PB_EI2C"]
    #[inline(always)]
    pub fn pb_ei2c(&self) -> PB_EI2C_R {
        PB_EI2C_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - desc PA_EHS"]
    #[inline(always)]
    pub fn pa_ehs(&self) -> PA_EHS_R {
        PA_EHS_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 16:21 - desc PB_EHS"]
    #[inline(always)]
    pub fn pb_ehs(&self) -> PB_EHS_R {
        PB_EHS_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - desc PF_EI2C"]
    #[inline(always)]
    pub fn pf_ei2c(&self) -> PF_EI2C_R {
        PF_EI2C_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - desc PF_PU_I2C"]
    #[inline(always)]
    pub fn pf_pu_i2c(&self) -> PF_PU_I2C_R {
        PF_PU_I2C_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc PA_EI2C"]
    #[inline(always)]
    pub fn pa_ei2c(&mut self) -> PA_EI2C_W<0> {
        PA_EI2C_W::new(self)
    }
    #[doc = "Bits 8:12 - desc PB_EI2C"]
    #[inline(always)]
    pub fn pb_ei2c(&mut self) -> PB_EI2C_W<8> {
        PB_EI2C_W::new(self)
    }
    #[doc = "Bits 13:14 - desc PA_EHS"]
    #[inline(always)]
    pub fn pa_ehs(&mut self) -> PA_EHS_W<13> {
        PA_EHS_W::new(self)
    }
    #[doc = "Bits 16:21 - desc PB_EHS"]
    #[inline(always)]
    pub fn pb_ehs(&mut self) -> PB_EHS_W<16> {
        PB_EHS_W::new(self)
    }
    #[doc = "Bits 24:27 - desc PF_EI2C"]
    #[inline(always)]
    pub fn pf_ei2c(&mut self) -> PF_EI2C_W<24> {
        PF_EI2C_W::new(self)
    }
    #[doc = "Bits 28:29 - desc PF_PU_I2C"]
    #[inline(always)]
    pub fn pf_pu_i2c(&mut self) -> PF_PU_I2C_W<28> {
        PF_PU_I2C_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C type IO configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io_cfg](index.html) module"]
pub struct IO_CFG_SPEC;
impl crate::RegisterSpec for IO_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [io_cfg::R](R) reader structure"]
impl crate::Readable for IO_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [io_cfg::W](W) writer structure"]
impl crate::Writable for IO_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IO_CFG to value 0"]
impl crate::Resettable for IO_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
