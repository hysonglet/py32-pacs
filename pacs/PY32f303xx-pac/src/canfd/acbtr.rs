#[doc = "Register `ACBTR` reader"]
pub struct R(crate::R<ACBTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACBTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACBTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACBTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACBTR` writer"]
pub struct W(crate::W<ACBTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACBTR_SPEC>;
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
impl From<crate::W<ACBTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACBTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AC_SEG_1` reader - desc AC_SEG_1"]
pub type AC_SEG_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `AC_SEG_1` writer - desc AC_SEG_1"]
pub type AC_SEG_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACBTR_SPEC, u16, u16, 9, O>;
#[doc = "Field `AC_SEG_2` reader - desc AC_SEG_2"]
pub type AC_SEG_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AC_SEG_2` writer - desc AC_SEG_2"]
pub type AC_SEG_2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACBTR_SPEC, u8, u8, 7, O>;
#[doc = "Field `AC_SJW` reader - desc AC_SJW"]
pub type AC_SJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AC_SJW` writer - desc AC_SJW"]
pub type AC_SJW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACBTR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:8 - desc AC_SEG_1"]
    #[inline(always)]
    pub fn ac_seg_1(&self) -> AC_SEG_1_R {
        AC_SEG_1_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:22 - desc AC_SEG_2"]
    #[inline(always)]
    pub fn ac_seg_2(&self) -> AC_SEG_2_R {
        AC_SEG_2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - desc AC_SJW"]
    #[inline(always)]
    pub fn ac_sjw(&self) -> AC_SJW_R {
        AC_SJW_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - desc AC_SEG_1"]
    #[inline(always)]
    pub fn ac_seg_1(&mut self) -> AC_SEG_1_W<0> {
        AC_SEG_1_W::new(self)
    }
    #[doc = "Bits 16:22 - desc AC_SEG_2"]
    #[inline(always)]
    pub fn ac_seg_2(&mut self) -> AC_SEG_2_W<16> {
        AC_SEG_2_W::new(self)
    }
    #[doc = "Bits 24:30 - desc AC_SJW"]
    #[inline(always)]
    pub fn ac_sjw(&mut self) -> AC_SJW_W<24> {
        AC_SJW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc ACBTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acbtr](index.html) module"]
pub struct ACBTR_SPEC;
impl crate::RegisterSpec for ACBTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acbtr::R](R) reader structure"]
impl crate::Readable for ACBTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acbtr::W](W) writer structure"]
impl crate::Writable for ACBTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACBTR to value 0x0505_0008"]
impl crate::Resettable for ACBTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0505_0008
    }
}
