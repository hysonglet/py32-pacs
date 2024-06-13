#[doc = "Register `FRAME` writer"]
pub struct W(crate::W<FRAME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAME_SPEC>;
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
impl From<crate::W<FRAME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FramNUM` writer - FramNUM"]
pub type FRAM_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRAME_SPEC, u16, u16, 11, O>;
#[doc = "Field `INDEX` writer - INDEX"]
pub type INDEX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRAME_SPEC, u8, u8, 4, O>;
impl W {
    #[doc = "Bits 0:10 - FramNUM"]
    #[inline(always)]
    pub fn fram_num(&mut self) -> FRAM_NUM_W<0> {
        FRAM_NUM_W::new(self)
    }
    #[doc = "Bits 16:19 - INDEX"]
    #[inline(always)]
    pub fn index(&mut self) -> INDEX_W<16> {
        INDEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FRAME\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frame](index.html) module"]
pub struct FRAME_SPEC;
impl crate::RegisterSpec for FRAME_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [frame::W](W) writer structure"]
impl crate::Writable for FRAME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRAME to value 0"]
impl crate::Resettable for FRAME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
