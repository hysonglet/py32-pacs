#[doc = "Register `WT` reader"]
pub struct R(crate::R<WT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WT` writer"]
pub struct W(crate::W<WT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WT_SPEC>;
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
impl From<crate::W<WT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - "]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - "]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WT_SPEC, u8, u8, 2, O>;
#[doc = "Field `CNT` reader - "]
pub type CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNT` writer - "]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WT_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<2> {
        CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wakeup Time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wt](index.html) module"]
pub struct WT_SPEC;
impl crate::RegisterSpec for WT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wt::R](R) reader structure"]
impl crate::Readable for WT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wt::W](W) writer structure"]
impl crate::Writable for WT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WT to value 0x82"]
impl crate::Resettable for WT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x82
    }
}
