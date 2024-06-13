#[doc = "Register `RAM8` reader"]
pub struct R(crate::R<RAM8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAM8` writer"]
pub struct W(crate::W<RAM8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM8_SPEC>;
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
impl From<crate::W<RAM8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D` reader - des D"]
pub type D_R = crate::FieldReader<u8, u8>;
#[doc = "Field `D` writer - des D"]
pub type D_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RAM8_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - des D"]
    #[inline(always)]
    pub fn d(&self) -> D_R {
        D_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - des D"]
    #[inline(always)]
    pub fn d(&mut self) -> D_W<0> {
        D_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "des RAM8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram8](index.html) module"]
pub struct RAM8_SPEC;
impl crate::RegisterSpec for RAM8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram8::R](R) reader structure"]
impl crate::Readable for RAM8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram8::W](W) writer structure"]
impl crate::Writable for RAM8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAM8 to value 0"]
impl crate::Resettable for RAM8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
