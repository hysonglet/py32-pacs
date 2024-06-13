#[doc = "Register `BLKSIZ` reader"]
pub struct R(crate::R<BLKSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLKSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLKSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLKSIZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLKSIZ` writer"]
pub struct W(crate::W<BLKSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLKSIZ_SPEC>;
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
impl From<crate::W<BLKSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLKSIZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBLOCKSIZE` reader - desc DBLOCKSIZE"]
pub type DBLOCKSIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DBLOCKSIZE` writer - desc DBLOCKSIZE"]
pub type DBLOCKSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BLKSIZ_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - desc DBLOCKSIZE"]
    #[inline(always)]
    pub fn dblocksize(&self) -> DBLOCKSIZE_R {
        DBLOCKSIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc DBLOCKSIZE"]
    #[inline(always)]
    pub fn dblocksize(&mut self) -> DBLOCKSIZE_W<0> {
        DBLOCKSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc BLKSIZ\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blksiz](index.html) module"]
pub struct BLKSIZ_SPEC;
impl crate::RegisterSpec for BLKSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blksiz::R](R) reader structure"]
impl crate::Readable for BLKSIZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blksiz::W](W) writer structure"]
impl crate::Writable for BLKSIZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLKSIZ to value 0x0200"]
impl crate::Resettable for BLKSIZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
