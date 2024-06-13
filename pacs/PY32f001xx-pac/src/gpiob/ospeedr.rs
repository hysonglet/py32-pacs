#[doc = "Register `OSPEEDR` reader"]
pub struct R(crate::R<OSPEEDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSPEEDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSPEEDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSPEEDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSPEEDR` writer"]
pub struct W(crate::W<OSPEEDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSPEEDR_SPEC>;
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
impl From<crate::W<OSPEEDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSPEEDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSPEED0` reader - Port x configuration bits (y = 0..15)"]
pub type OSPEED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSPEED0` writer - Port x configuration bits (y = 0..15)"]
pub type OSPEED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSPEEDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `OSPEED1` reader - Port x configuration bits (y = 0..15)"]
pub type OSPEED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSPEED1` writer - Port x configuration bits (y = 0..15)"]
pub type OSPEED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSPEEDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `OSPEED2` reader - Port x configuration bits (y = 0..15)"]
pub type OSPEED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSPEED2` writer - Port x configuration bits (y = 0..15)"]
pub type OSPEED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSPEEDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `OSPEED3` reader - Port x configuration bits (y = 0..15)"]
pub type OSPEED3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSPEED3` writer - Port x configuration bits (y = 0..15)"]
pub type OSPEED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSPEEDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `OSPEED4` reader - Port x configuration bits (y = 0..15)"]
pub type OSPEED4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSPEED4` writer - Port x configuration bits (y = 0..15)"]
pub type OSPEED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSPEEDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `OSPEED5` reader - Port x configuration bits (y = 0..15)"]
pub type OSPEED5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSPEED5` writer - Port x configuration bits (y = 0..15)"]
pub type OSPEED5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSPEEDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `OSPEED6` reader - Port x configuration bits (y = 0..15)"]
pub type OSPEED6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSPEED6` writer - Port x configuration bits (y = 0..15)"]
pub type OSPEED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSPEEDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `OSPEED7` reader - Port x configuration bits (y = 0..15)"]
pub type OSPEED7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSPEED7` writer - Port x configuration bits (y = 0..15)"]
pub type OSPEED7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSPEEDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `OSPEED8` reader - Port x configuration bits (y = 0..15)"]
pub type OSPEED8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSPEED8` writer - Port x configuration bits (y = 0..15)"]
pub type OSPEED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSPEEDR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed0(&self) -> OSPEED0_R {
        OSPEED0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed1(&self) -> OSPEED1_R {
        OSPEED1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed2(&self) -> OSPEED2_R {
        OSPEED2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed3(&self) -> OSPEED3_R {
        OSPEED3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed4(&self) -> OSPEED4_R {
        OSPEED4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed5(&self) -> OSPEED5_R {
        OSPEED5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed6(&self) -> OSPEED6_R {
        OSPEED6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed7(&self) -> OSPEED7_R {
        OSPEED7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed8(&self) -> OSPEED8_R {
        OSPEED8_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed0(&mut self) -> OSPEED0_W<0> {
        OSPEED0_W::new(self)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed1(&mut self) -> OSPEED1_W<2> {
        OSPEED1_W::new(self)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed2(&mut self) -> OSPEED2_W<4> {
        OSPEED2_W::new(self)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed3(&mut self) -> OSPEED3_W<6> {
        OSPEED3_W::new(self)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed4(&mut self) -> OSPEED4_W<8> {
        OSPEED4_W::new(self)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed5(&mut self) -> OSPEED5_W<10> {
        OSPEED5_W::new(self)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed6(&mut self) -> OSPEED6_W<12> {
        OSPEED6_W::new(self)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed7(&mut self) -> OSPEED7_W<14> {
        OSPEED7_W::new(self)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed8(&mut self) -> OSPEED8_W<16> {
        OSPEED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ospeedr](index.html) module"]
pub struct OSPEEDR_SPEC;
impl crate::RegisterSpec for OSPEEDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ospeedr::R](R) reader structure"]
impl crate::Readable for OSPEEDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ospeedr::W](W) writer structure"]
impl crate::Writable for OSPEEDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSPEEDR to value 0"]
impl crate::Resettable for OSPEEDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
