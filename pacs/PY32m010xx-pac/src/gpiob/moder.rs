#[doc = "Register `MODER` reader"]
pub struct R(crate::R<MODER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODER` writer"]
pub struct W(crate::W<MODER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODER_SPEC>;
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
impl From<crate::W<MODER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE0` reader - Port x configuration bits (y = 0..15)"]
pub type MODE0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE0` writer - Port x configuration bits (y = 0..15)"]
pub type MODE0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODER_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE1` reader - Port x configuration bits (y = 0..15)"]
pub type MODE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE1` writer - Port x configuration bits (y = 0..15)"]
pub type MODE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODER_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE2` reader - Port x configuration bits (y = 0..15)"]
pub type MODE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE2` writer - Port x configuration bits (y = 0..15)"]
pub type MODE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODER_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE3` reader - Port x configuration bits (y = 0..15)"]
pub type MODE3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE3` writer - Port x configuration bits (y = 0..15)"]
pub type MODE3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODER_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE4` reader - Port x configuration bits (y = 0..15)"]
pub type MODE4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE4` writer - Port x configuration bits (y = 0..15)"]
pub type MODE4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODER_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE5` reader - Port x configuration bits (y = 0..15)"]
pub type MODE5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE5` writer - Port x configuration bits (y = 0..15)"]
pub type MODE5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODER_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE6` reader - Port x configuration bits (y = 0..15)"]
pub type MODE6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE6` writer - Port x configuration bits (y = 0..15)"]
pub type MODE6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODER_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE7` reader - Port x configuration bits (y = 0..15)"]
pub type MODE7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE7` writer - Port x configuration bits (y = 0..15)"]
pub type MODE7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODER_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE8` reader - Port x configuration bits (y = 0..15)"]
pub type MODE8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE8` writer - Port x configuration bits (y = 0..15)"]
pub type MODE8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODER_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode4(&self) -> MODE4_R {
        MODE4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode5(&self) -> MODE5_R {
        MODE5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode6(&self) -> MODE6_R {
        MODE6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode7(&self) -> MODE7_R {
        MODE7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode8(&self) -> MODE8_R {
        MODE8_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode0(&mut self) -> MODE0_W<0> {
        MODE0_W::new(self)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W<2> {
        MODE1_W::new(self)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE2_W<4> {
        MODE2_W::new(self)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode3(&mut self) -> MODE3_W<6> {
        MODE3_W::new(self)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode4(&mut self) -> MODE4_W<8> {
        MODE4_W::new(self)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode5(&mut self) -> MODE5_W<10> {
        MODE5_W::new(self)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode6(&mut self) -> MODE6_W<12> {
        MODE6_W::new(self)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode7(&mut self) -> MODE7_W<14> {
        MODE7_W::new(self)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode8(&mut self) -> MODE8_W<16> {
        MODE8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [moder](index.html) module"]
pub struct MODER_SPEC;
impl crate::RegisterSpec for MODER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [moder::R](R) reader structure"]
impl crate::Readable for MODER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [moder::W](W) writer structure"]
impl crate::Writable for MODER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODER to value 0xffff_ffff"]
impl crate::Resettable for MODER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
