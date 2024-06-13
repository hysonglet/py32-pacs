#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITERATION` reader - desc ITERATION"]
pub type ITERATION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ITERATION` writer - desc ITERATION"]
pub type ITERATION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 5, O>;
#[doc = "Field `CORDIC_INT` reader - desc CORDIC_INT"]
pub type CORDIC_INT_R = crate::BitReader<bool>;
#[doc = "Field `CORDIC_INT` writer - desc CORDIC_INT"]
pub type CORDIC_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SQRT_INT` reader - desc SQRT_INT"]
pub type SQRT_INT_R = crate::BitReader<bool>;
#[doc = "Field `SQRT_INT` writer - desc SQRT_INT"]
pub type SQRT_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CORDIC_MODE` reader - desc CORDIC_MODE"]
pub type CORDIC_MODE_R = crate::BitReader<bool>;
#[doc = "Field `CORDIC_MODE` writer - desc CORDIC_MODE"]
pub type CORDIC_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `START` reader - desc START"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - desc START"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `START_MODE` reader - desc START_MODE"]
pub type START_MODE_R = crate::BitReader<bool>;
#[doc = "Field `START_MODE` writer - desc START_MODE"]
pub type START_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `VECSIZE` reader - desc VECSIZE"]
pub type VECSIZE_R = crate::BitReader<bool>;
#[doc = "Field `VECSIZE` writer - desc VECSIZE"]
pub type VECSIZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RESSIZE` reader - desc RESSIZE"]
pub type RESSIZE_R = crate::BitReader<bool>;
#[doc = "Field `RESSIZE` writer - desc RESSIZE"]
pub type RESSIZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ARGSIZE` reader - desc ARGSIZE"]
pub type ARGSIZE_R = crate::BitReader<bool>;
#[doc = "Field `ARGSIZE` writer - desc ARGSIZE"]
pub type ARGSIZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CORDIC_INT_MASK` reader - desc CORDIC_INT_MASK"]
pub type CORDIC_INT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `CORDIC_INT_MASK` writer - desc CORDIC_INT_MASK"]
pub type CORDIC_INT_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SQRT_INT_MASK` reader - desc SQRT_INT_MASK"]
pub type SQRT_INT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `SQRT_INT_MASK` writer - desc SQRT_INT_MASK"]
pub type SQRT_INT_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CORDIC_ERROR_INT_MASK` reader - desc CORDIC_ERROR_INT_MASK"]
pub type CORDIC_ERROR_INT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `CORDIC_ERROR_INT_MASK` writer - desc CORDIC_ERROR_INT_MASK"]
pub type CORDIC_ERROR_INT_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ARCTAN_MOD_OV_MASK` reader - desc ARCTAN_MOD_OV_MASK"]
pub type ARCTAN_MOD_OV_MASK_R = crate::BitReader<bool>;
#[doc = "Field `ARCTAN_MOD_OV_MASK` writer - desc ARCTAN_MOD_OV_MASK"]
pub type ARCTAN_MOD_OV_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - desc ITERATION"]
    #[inline(always)]
    pub fn iteration(&self) -> ITERATION_R {
        ITERATION_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - desc CORDIC_INT"]
    #[inline(always)]
    pub fn cordic_int(&self) -> CORDIC_INT_R {
        CORDIC_INT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc SQRT_INT"]
    #[inline(always)]
    pub fn sqrt_int(&self) -> SQRT_INT_R {
        SQRT_INT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc CORDIC_MODE"]
    #[inline(always)]
    pub fn cordic_mode(&self) -> CORDIC_MODE_R {
        CORDIC_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc START"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc START_MODE"]
    #[inline(always)]
    pub fn start_mode(&self) -> START_MODE_R {
        START_MODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 20 - desc VECSIZE"]
    #[inline(always)]
    pub fn vecsize(&self) -> VECSIZE_R {
        VECSIZE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc RESSIZE"]
    #[inline(always)]
    pub fn ressize(&self) -> RESSIZE_R {
        RESSIZE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc ARGSIZE"]
    #[inline(always)]
    pub fn argsize(&self) -> ARGSIZE_R {
        ARGSIZE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 28 - desc CORDIC_INT_MASK"]
    #[inline(always)]
    pub fn cordic_int_mask(&self) -> CORDIC_INT_MASK_R {
        CORDIC_INT_MASK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc SQRT_INT_MASK"]
    #[inline(always)]
    pub fn sqrt_int_mask(&self) -> SQRT_INT_MASK_R {
        SQRT_INT_MASK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc CORDIC_ERROR_INT_MASK"]
    #[inline(always)]
    pub fn cordic_error_int_mask(&self) -> CORDIC_ERROR_INT_MASK_R {
        CORDIC_ERROR_INT_MASK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc ARCTAN_MOD_OV_MASK"]
    #[inline(always)]
    pub fn arctan_mod_ov_mask(&self) -> ARCTAN_MOD_OV_MASK_R {
        ARCTAN_MOD_OV_MASK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - desc ITERATION"]
    #[inline(always)]
    pub fn iteration(&mut self) -> ITERATION_W<0> {
        ITERATION_W::new(self)
    }
    #[doc = "Bit 5 - desc CORDIC_INT"]
    #[inline(always)]
    pub fn cordic_int(&mut self) -> CORDIC_INT_W<5> {
        CORDIC_INT_W::new(self)
    }
    #[doc = "Bit 6 - desc SQRT_INT"]
    #[inline(always)]
    pub fn sqrt_int(&mut self) -> SQRT_INT_W<6> {
        SQRT_INT_W::new(self)
    }
    #[doc = "Bit 7 - desc CORDIC_MODE"]
    #[inline(always)]
    pub fn cordic_mode(&mut self) -> CORDIC_MODE_W<7> {
        CORDIC_MODE_W::new(self)
    }
    #[doc = "Bit 8 - desc START"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<8> {
        START_W::new(self)
    }
    #[doc = "Bit 9 - desc START_MODE"]
    #[inline(always)]
    pub fn start_mode(&mut self) -> START_MODE_W<9> {
        START_MODE_W::new(self)
    }
    #[doc = "Bit 20 - desc VECSIZE"]
    #[inline(always)]
    pub fn vecsize(&mut self) -> VECSIZE_W<20> {
        VECSIZE_W::new(self)
    }
    #[doc = "Bit 21 - desc RESSIZE"]
    #[inline(always)]
    pub fn ressize(&mut self) -> RESSIZE_W<21> {
        RESSIZE_W::new(self)
    }
    #[doc = "Bit 22 - desc ARGSIZE"]
    #[inline(always)]
    pub fn argsize(&mut self) -> ARGSIZE_W<22> {
        ARGSIZE_W::new(self)
    }
    #[doc = "Bit 28 - desc CORDIC_INT_MASK"]
    #[inline(always)]
    pub fn cordic_int_mask(&mut self) -> CORDIC_INT_MASK_W<28> {
        CORDIC_INT_MASK_W::new(self)
    }
    #[doc = "Bit 29 - desc SQRT_INT_MASK"]
    #[inline(always)]
    pub fn sqrt_int_mask(&mut self) -> SQRT_INT_MASK_W<29> {
        SQRT_INT_MASK_W::new(self)
    }
    #[doc = "Bit 30 - desc CORDIC_ERROR_INT_MASK"]
    #[inline(always)]
    pub fn cordic_error_int_mask(&mut self) -> CORDIC_ERROR_INT_MASK_W<30> {
        CORDIC_ERROR_INT_MASK_W::new(self)
    }
    #[doc = "Bit 31 - desc ARCTAN_MOD_OV_MASK"]
    #[inline(always)]
    pub fn arctan_mod_ov_mask(&mut self) -> ARCTAN_MOD_OV_MASK_W<31> {
        ARCTAN_MOD_OV_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CORDIC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
