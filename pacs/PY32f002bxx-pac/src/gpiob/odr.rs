#[doc = "Register `ODR` reader"]
pub struct R(crate::R<ODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ODR` writer"]
pub struct W(crate::W<ODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ODR_SPEC>;
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
impl From<crate::W<ODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OD0` reader - Port output data (y = 0..15)"]
pub type OD0_R = crate::BitReader<bool>;
#[doc = "Field `OD0` writer - Port output data (y = 0..15)"]
pub type OD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `OD1` reader - Port output data (y = 0..15)"]
pub type OD1_R = crate::BitReader<bool>;
#[doc = "Field `OD1` writer - Port output data (y = 0..15)"]
pub type OD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `OD2` reader - Port output data (y = 0..15)"]
pub type OD2_R = crate::BitReader<bool>;
#[doc = "Field `OD2` writer - Port output data (y = 0..15)"]
pub type OD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `OD3` reader - Port output data (y = 0..15)"]
pub type OD3_R = crate::BitReader<bool>;
#[doc = "Field `OD3` writer - Port output data (y = 0..15)"]
pub type OD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `OD4` reader - Port output data (y = 0..15)"]
pub type OD4_R = crate::BitReader<bool>;
#[doc = "Field `OD4` writer - Port output data (y = 0..15)"]
pub type OD4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `OD5` reader - Port output data (y = 0..15)"]
pub type OD5_R = crate::BitReader<bool>;
#[doc = "Field `OD5` writer - Port output data (y = 0..15)"]
pub type OD5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `OD6` reader - Port output data (y = 0..15)"]
pub type OD6_R = crate::BitReader<bool>;
#[doc = "Field `OD6` writer - Port output data (y = 0..15)"]
pub type OD6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `OD7` reader - Port output data (y = 0..15)"]
pub type OD7_R = crate::BitReader<bool>;
#[doc = "Field `OD7` writer - Port output data (y = 0..15)"]
pub type OD7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `OD8` reader - Port output data (y = 0..15)"]
pub type OD8_R = crate::BitReader<bool>;
#[doc = "Field `OD8` writer - Port output data (y = 0..15)"]
pub type OD8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od0(&self) -> OD0_R {
        OD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od1(&self) -> OD1_R {
        OD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od2(&self) -> OD2_R {
        OD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od3(&self) -> OD3_R {
        OD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od4(&self) -> OD4_R {
        OD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od5(&self) -> OD5_R {
        OD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od6(&self) -> OD6_R {
        OD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od7(&self) -> OD7_R {
        OD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od8(&self) -> OD8_R {
        OD8_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od0(&mut self) -> OD0_W<0> {
        OD0_W::new(self)
    }
    #[doc = "Bit 1 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od1(&mut self) -> OD1_W<1> {
        OD1_W::new(self)
    }
    #[doc = "Bit 2 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od2(&mut self) -> OD2_W<2> {
        OD2_W::new(self)
    }
    #[doc = "Bit 3 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od3(&mut self) -> OD3_W<3> {
        OD3_W::new(self)
    }
    #[doc = "Bit 4 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od4(&mut self) -> OD4_W<4> {
        OD4_W::new(self)
    }
    #[doc = "Bit 5 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od5(&mut self) -> OD5_W<5> {
        OD5_W::new(self)
    }
    #[doc = "Bit 6 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od6(&mut self) -> OD6_W<6> {
        OD6_W::new(self)
    }
    #[doc = "Bit 7 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od7(&mut self) -> OD7_W<7> {
        OD7_W::new(self)
    }
    #[doc = "Bit 8 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od8(&mut self) -> OD8_W<8> {
        OD8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odr](index.html) module"]
pub struct ODR_SPEC;
impl crate::RegisterSpec for ODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [odr::R](R) reader structure"]
impl crate::Readable for ODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [odr::W](W) writer structure"]
impl crate::Writable for ODR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ODR to value 0"]
impl crate::Resettable for ODR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
