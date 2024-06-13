#[doc = "Register `REFMSG` reader"]
pub struct R(crate::R<REFMSG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REFMSG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REFMSG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REFMSG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REFMSG` writer"]
pub struct W(crate::W<REFMSG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REFMSG_SPEC>;
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
impl From<crate::W<REFMSG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REFMSG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REF_ID` reader - desc REF_ID"]
pub type REF_ID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REF_ID` writer - desc REF_ID"]
pub type REF_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REFMSG_SPEC, u32, u32, 29, O>;
#[doc = "Field `REF_IDE` reader - desc REF_IDE"]
pub type REF_IDE_R = crate::BitReader<bool>;
#[doc = "Field `REF_IDE` writer - desc REF_IDE"]
pub type REF_IDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFMSG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:28 - desc REF_ID"]
    #[inline(always)]
    pub fn ref_id(&self) -> REF_ID_R {
        REF_ID_R::new((self.bits & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 31 - desc REF_IDE"]
    #[inline(always)]
    pub fn ref_ide(&self) -> REF_IDE_R {
        REF_IDE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - desc REF_ID"]
    #[inline(always)]
    pub fn ref_id(&mut self) -> REF_ID_W<0> {
        REF_ID_W::new(self)
    }
    #[doc = "Bit 31 - desc REF_IDE"]
    #[inline(always)]
    pub fn ref_ide(&mut self) -> REF_IDE_W<31> {
        REF_IDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc REFMSG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refmsg](index.html) module"]
pub struct REFMSG_SPEC;
impl crate::RegisterSpec for REFMSG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [refmsg::R](R) reader structure"]
impl crate::Readable for REFMSG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [refmsg::W](W) writer structure"]
impl crate::Writable for REFMSG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REFMSG to value 0"]
impl crate::Resettable for REFMSG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
