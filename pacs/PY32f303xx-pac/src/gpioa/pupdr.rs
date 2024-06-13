#[doc = "Register `PUPDR` reader"]
pub struct R(crate::R<PUPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUPDR` writer"]
pub struct W(crate::W<PUPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUPDR_SPEC>;
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
impl From<crate::W<PUPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PUPD0` reader - desc PUPD0"]
pub type PUPD0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPD0` writer - desc PUPD0"]
pub type PUPD0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPD1` reader - desc PUPD1"]
pub type PUPD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPD1` writer - desc PUPD1"]
pub type PUPD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPD2` reader - desc PUPD2"]
pub type PUPD2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPD2` writer - desc PUPD2"]
pub type PUPD2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPD3` reader - desc PUPD3"]
pub type PUPD3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPD3` writer - desc PUPD3"]
pub type PUPD3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPD4` reader - desc PUPD4"]
pub type PUPD4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPD4` writer - desc PUPD4"]
pub type PUPD4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPD5` reader - desc PUPD5"]
pub type PUPD5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPD5` writer - desc PUPD5"]
pub type PUPD5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPD6` reader - desc PUPD6"]
pub type PUPD6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPD6` writer - desc PUPD6"]
pub type PUPD6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPD7` reader - desc PUPD7"]
pub type PUPD7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPD7` writer - desc PUPD7"]
pub type PUPD7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPD8` reader - desc PUPD8"]
pub type PUPD8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPD8` writer - desc PUPD8"]
pub type PUPD8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPD9` reader - desc PUPD9"]
pub type PUPD9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPD9` writer - desc PUPD9"]
pub type PUPD9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPD10` reader - desc PUPD10"]
pub type PUPD10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPD10` writer - desc PUPD10"]
pub type PUPD10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPD11` reader - desc PUPD11"]
pub type PUPD11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPD11` writer - desc PUPD11"]
pub type PUPD11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPD12` reader - desc PUPD12"]
pub type PUPD12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPD12` writer - desc PUPD12"]
pub type PUPD12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPD13` reader - desc PUPD13"]
pub type PUPD13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPD13` writer - desc PUPD13"]
pub type PUPD13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPD14` reader - desc PUPD14"]
pub type PUPD14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPD14` writer - desc PUPD14"]
pub type PUPD14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUPD15` reader - desc PUPD15"]
pub type PUPD15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUPD15` writer - desc PUPD15"]
pub type PUPD15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - desc PUPD0"]
    #[inline(always)]
    pub fn pupd0(&self) -> PUPD0_R {
        PUPD0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc PUPD1"]
    #[inline(always)]
    pub fn pupd1(&self) -> PUPD1_R {
        PUPD1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - desc PUPD2"]
    #[inline(always)]
    pub fn pupd2(&self) -> PUPD2_R {
        PUPD2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - desc PUPD3"]
    #[inline(always)]
    pub fn pupd3(&self) -> PUPD3_R {
        PUPD3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - desc PUPD4"]
    #[inline(always)]
    pub fn pupd4(&self) -> PUPD4_R {
        PUPD4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - desc PUPD5"]
    #[inline(always)]
    pub fn pupd5(&self) -> PUPD5_R {
        PUPD5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - desc PUPD6"]
    #[inline(always)]
    pub fn pupd6(&self) -> PUPD6_R {
        PUPD6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - desc PUPD7"]
    #[inline(always)]
    pub fn pupd7(&self) -> PUPD7_R {
        PUPD7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - desc PUPD8"]
    #[inline(always)]
    pub fn pupd8(&self) -> PUPD8_R {
        PUPD8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - desc PUPD9"]
    #[inline(always)]
    pub fn pupd9(&self) -> PUPD9_R {
        PUPD9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - desc PUPD10"]
    #[inline(always)]
    pub fn pupd10(&self) -> PUPD10_R {
        PUPD10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - desc PUPD11"]
    #[inline(always)]
    pub fn pupd11(&self) -> PUPD11_R {
        PUPD11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - desc PUPD12"]
    #[inline(always)]
    pub fn pupd12(&self) -> PUPD12_R {
        PUPD12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - desc PUPD13"]
    #[inline(always)]
    pub fn pupd13(&self) -> PUPD13_R {
        PUPD13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - desc PUPD14"]
    #[inline(always)]
    pub fn pupd14(&self) -> PUPD14_R {
        PUPD14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - desc PUPD15"]
    #[inline(always)]
    pub fn pupd15(&self) -> PUPD15_R {
        PUPD15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc PUPD0"]
    #[inline(always)]
    pub fn pupd0(&mut self) -> PUPD0_W<0> {
        PUPD0_W::new(self)
    }
    #[doc = "Bits 2:3 - desc PUPD1"]
    #[inline(always)]
    pub fn pupd1(&mut self) -> PUPD1_W<2> {
        PUPD1_W::new(self)
    }
    #[doc = "Bits 4:5 - desc PUPD2"]
    #[inline(always)]
    pub fn pupd2(&mut self) -> PUPD2_W<4> {
        PUPD2_W::new(self)
    }
    #[doc = "Bits 6:7 - desc PUPD3"]
    #[inline(always)]
    pub fn pupd3(&mut self) -> PUPD3_W<6> {
        PUPD3_W::new(self)
    }
    #[doc = "Bits 8:9 - desc PUPD4"]
    #[inline(always)]
    pub fn pupd4(&mut self) -> PUPD4_W<8> {
        PUPD4_W::new(self)
    }
    #[doc = "Bits 10:11 - desc PUPD5"]
    #[inline(always)]
    pub fn pupd5(&mut self) -> PUPD5_W<10> {
        PUPD5_W::new(self)
    }
    #[doc = "Bits 12:13 - desc PUPD6"]
    #[inline(always)]
    pub fn pupd6(&mut self) -> PUPD6_W<12> {
        PUPD6_W::new(self)
    }
    #[doc = "Bits 14:15 - desc PUPD7"]
    #[inline(always)]
    pub fn pupd7(&mut self) -> PUPD7_W<14> {
        PUPD7_W::new(self)
    }
    #[doc = "Bits 16:17 - desc PUPD8"]
    #[inline(always)]
    pub fn pupd8(&mut self) -> PUPD8_W<16> {
        PUPD8_W::new(self)
    }
    #[doc = "Bits 18:19 - desc PUPD9"]
    #[inline(always)]
    pub fn pupd9(&mut self) -> PUPD9_W<18> {
        PUPD9_W::new(self)
    }
    #[doc = "Bits 20:21 - desc PUPD10"]
    #[inline(always)]
    pub fn pupd10(&mut self) -> PUPD10_W<20> {
        PUPD10_W::new(self)
    }
    #[doc = "Bits 22:23 - desc PUPD11"]
    #[inline(always)]
    pub fn pupd11(&mut self) -> PUPD11_W<22> {
        PUPD11_W::new(self)
    }
    #[doc = "Bits 24:25 - desc PUPD12"]
    #[inline(always)]
    pub fn pupd12(&mut self) -> PUPD12_W<24> {
        PUPD12_W::new(self)
    }
    #[doc = "Bits 26:27 - desc PUPD13"]
    #[inline(always)]
    pub fn pupd13(&mut self) -> PUPD13_W<26> {
        PUPD13_W::new(self)
    }
    #[doc = "Bits 28:29 - desc PUPD14"]
    #[inline(always)]
    pub fn pupd14(&mut self) -> PUPD14_W<28> {
        PUPD14_W::new(self)
    }
    #[doc = "Bits 30:31 - desc PUPD15"]
    #[inline(always)]
    pub fn pupd15(&mut self) -> PUPD15_W<30> {
        PUPD15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc PUPDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pupdr](index.html) module"]
pub struct PUPDR_SPEC;
impl crate::RegisterSpec for PUPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pupdr::R](R) reader structure"]
impl crate::Readable for PUPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pupdr::W](W) writer structure"]
impl crate::Writable for PUPDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PUPDR to value 0"]
impl crate::Resettable for PUPDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
