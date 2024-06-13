#[doc = "Register `XLBTR` reader"]
pub struct R(crate::R<XLBTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XLBTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XLBTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XLBTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XLBTR` writer"]
pub struct W(crate::W<XLBTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XLBTR_SPEC>;
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
impl From<crate::W<XLBTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XLBTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XL_SEG_1` reader - desc XL_SEG_1"]
pub type XL_SEG_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XL_SEG_1` writer - desc XL_SEG_1"]
pub type XL_SEG_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XLBTR_SPEC, u8, u8, 8, O>;
#[doc = "Field `XL_SEG_2` reader - desc XL_SEG_2"]
pub type XL_SEG_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XL_SEG_2` writer - desc XL_SEG_2"]
pub type XL_SEG_2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XLBTR_SPEC, u8, u8, 7, O>;
#[doc = "Field `XL_SJW` reader - desc XL_SJW"]
pub type XL_SJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XL_SJW` writer - desc XL_SJW"]
pub type XL_SJW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XLBTR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:7 - desc XL_SEG_1"]
    #[inline(always)]
    pub fn xl_seg_1(&self) -> XL_SEG_1_R {
        XL_SEG_1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - desc XL_SEG_2"]
    #[inline(always)]
    pub fn xl_seg_2(&self) -> XL_SEG_2_R {
        XL_SEG_2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - desc XL_SJW"]
    #[inline(always)]
    pub fn xl_sjw(&self) -> XL_SJW_R {
        XL_SJW_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc XL_SEG_1"]
    #[inline(always)]
    pub fn xl_seg_1(&mut self) -> XL_SEG_1_W<0> {
        XL_SEG_1_W::new(self)
    }
    #[doc = "Bits 16:22 - desc XL_SEG_2"]
    #[inline(always)]
    pub fn xl_seg_2(&mut self) -> XL_SEG_2_W<16> {
        XL_SEG_2_W::new(self)
    }
    #[doc = "Bits 24:30 - desc XL_SJW"]
    #[inline(always)]
    pub fn xl_sjw(&mut self) -> XL_SJW_W<24> {
        XL_SJW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc XLBTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xlbtr](index.html) module"]
pub struct XLBTR_SPEC;
impl crate::RegisterSpec for XLBTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xlbtr::R](R) reader structure"]
impl crate::Readable for XLBTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xlbtr::W](W) writer structure"]
impl crate::Writable for XLBTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XLBTR to value 0x0202_0003"]
impl crate::Resettable for XLBTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0202_0003
    }
}
