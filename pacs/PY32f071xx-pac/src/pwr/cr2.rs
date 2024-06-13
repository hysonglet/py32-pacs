#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PVDE` reader - Power voltage detector enable"]
pub type PVDE_R = crate::BitReader<bool>;
#[doc = "Field `PVDE` writer - Power voltage detector enable"]
pub type PVDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `SRCSEL` reader - Power voltage detector volatage selection"]
pub type SRCSEL_R = crate::BitReader<bool>;
#[doc = "Field `SRCSEL` writer - Power voltage detector volatage selection"]
pub type SRCSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `PVDT` reader - Power voltage detector threshold selection"]
pub type PVDT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PVDT` writer - Power voltage detector threshold selection"]
pub type PVDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `FLTEN` reader - Digital filter enable"]
pub type FLTEN_R = crate::BitReader<bool>;
#[doc = "Field `FLTEN` writer - Digital filter enable"]
pub type FLTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `FLT_TIME` reader - Digital filter time configuration"]
pub type FLT_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLT_TIME` writer - Digital filter time configuration"]
pub type FLT_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Power voltage detector volatage selection"]
    #[inline(always)]
    pub fn srcsel(&self) -> SRCSEL_R {
        SRCSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Power voltage detector threshold selection"]
    #[inline(always)]
    pub fn pvdt(&self) -> PVDT_R {
        PVDT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Digital filter enable"]
    #[inline(always)]
    pub fn flten(&self) -> FLTEN_R {
        FLTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Digital filter time configuration"]
    #[inline(always)]
    pub fn flt_time(&self) -> FLT_TIME_R {
        FLT_TIME_R::new(((self.bits >> 9) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W<0> {
        PVDE_W::new(self)
    }
    #[doc = "Bit 2 - Power voltage detector volatage selection"]
    #[inline(always)]
    pub fn srcsel(&mut self) -> SRCSEL_W<2> {
        SRCSEL_W::new(self)
    }
    #[doc = "Bits 4:6 - Power voltage detector threshold selection"]
    #[inline(always)]
    pub fn pvdt(&mut self) -> PVDT_W<4> {
        PVDT_W::new(self)
    }
    #[doc = "Bit 8 - Digital filter enable"]
    #[inline(always)]
    pub fn flten(&mut self) -> FLTEN_W<8> {
        FLTEN_W::new(self)
    }
    #[doc = "Bits 9:11 - Digital filter time configuration"]
    #[inline(always)]
    pub fn flt_time(&mut self) -> FLT_TIME_W<9> {
        FLT_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0x0500"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0500
    }
}
