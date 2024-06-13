#[doc = "Register `FDBTR` reader"]
pub struct R(crate::R<FDBTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDBTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDBTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDBTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDBTR` writer"]
pub struct W(crate::W<FDBTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDBTR_SPEC>;
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
impl From<crate::W<FDBTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDBTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FD_SEG_1` reader - desc FD_SEG_1"]
pub type FD_SEG_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FD_SEG_1` writer - desc FD_SEG_1"]
pub type FD_SEG_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDBTR_SPEC, u8, u8, 8, O>;
#[doc = "Field `FD_SEG_2` reader - desc FD_SEG_2"]
pub type FD_SEG_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FD_SEG_2` writer - desc FD_SEG_2"]
pub type FD_SEG_2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDBTR_SPEC, u8, u8, 7, O>;
#[doc = "Field `FD_SJW` reader - desc FD_SJW"]
pub type FD_SJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FD_SJW` writer - desc FD_SJW"]
pub type FD_SJW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDBTR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:7 - desc FD_SEG_1"]
    #[inline(always)]
    pub fn fd_seg_1(&self) -> FD_SEG_1_R {
        FD_SEG_1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - desc FD_SEG_2"]
    #[inline(always)]
    pub fn fd_seg_2(&self) -> FD_SEG_2_R {
        FD_SEG_2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - desc FD_SJW"]
    #[inline(always)]
    pub fn fd_sjw(&self) -> FD_SJW_R {
        FD_SJW_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc FD_SEG_1"]
    #[inline(always)]
    pub fn fd_seg_1(&mut self) -> FD_SEG_1_W<0> {
        FD_SEG_1_W::new(self)
    }
    #[doc = "Bits 16:22 - desc FD_SEG_2"]
    #[inline(always)]
    pub fn fd_seg_2(&mut self) -> FD_SEG_2_W<16> {
        FD_SEG_2_W::new(self)
    }
    #[doc = "Bits 24:30 - desc FD_SJW"]
    #[inline(always)]
    pub fn fd_sjw(&mut self) -> FD_SJW_W<24> {
        FD_SJW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc FDBTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdbtr](index.html) module"]
pub struct FDBTR_SPEC;
impl crate::RegisterSpec for FDBTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdbtr::R](R) reader structure"]
impl crate::Readable for FDBTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdbtr::W](W) writer structure"]
impl crate::Writable for FDBTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDBTR to value 0x0202_0003"]
impl crate::Resettable for FDBTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0202_0003
    }
}
