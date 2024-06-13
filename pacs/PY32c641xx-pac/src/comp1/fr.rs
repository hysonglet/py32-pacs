#[doc = "Register `FR` reader"]
pub struct R(crate::R<FR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FR` writer"]
pub struct W(crate::W<FR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FR_SPEC>;
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
impl From<crate::W<FR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLTEN1` reader - Filter enable bit"]
pub type FLTEN1_R = crate::BitReader<bool>;
#[doc = "Field `FLTEN1` writer - Filter enable bit"]
pub type FLTEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR_SPEC, bool, O>;
#[doc = "Field `FLTCNT1` reader - Comparator filter and counter"]
pub type FLTCNT1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FLTCNT1` writer - Comparator filter and counter"]
pub type FLTCNT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - Filter enable bit"]
    #[inline(always)]
    pub fn flten1(&self) -> FLTEN1_R {
        FLTEN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:31 - Comparator filter and counter"]
    #[inline(always)]
    pub fn fltcnt1(&self) -> FLTCNT1_R {
        FLTCNT1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Filter enable bit"]
    #[inline(always)]
    pub fn flten1(&mut self) -> FLTEN1_W<0> {
        FLTEN1_W::new(self)
    }
    #[doc = "Bits 16:31 - Comparator filter and counter"]
    #[inline(always)]
    pub fn fltcnt1(&mut self) -> FLTCNT1_W<16> {
        FLTCNT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator Filter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fr](index.html) module"]
pub struct FR_SPEC;
impl crate::RegisterSpec for FR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fr::R](R) reader structure"]
impl crate::Readable for FR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fr::W](W) writer structure"]
impl crate::Writable for FR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FR to value 0"]
impl crate::Resettable for FR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
