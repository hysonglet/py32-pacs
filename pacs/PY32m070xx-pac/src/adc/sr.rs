#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AWD` reader - desc AWD"]
pub type AWD_R = crate::BitReader<bool>;
#[doc = "Field `AWD` writer - desc AWD"]
pub type AWD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `EOC` reader - desc EOC"]
pub type EOC_R = crate::BitReader<bool>;
#[doc = "Field `EOC` writer - desc EOC"]
pub type EOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `JEOC` reader - desc JEOC"]
pub type JEOC_R = crate::BitReader<bool>;
#[doc = "Field `JEOC` writer - desc JEOC"]
pub type JEOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `JSTRT` reader - desc JSTRT"]
pub type JSTRT_R = crate::BitReader<bool>;
#[doc = "Field `JSTRT` writer - desc JSTRT"]
pub type JSTRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `STRT` reader - desc STRT"]
pub type STRT_R = crate::BitReader<bool>;
#[doc = "Field `STRT` writer - desc STRT"]
pub type STRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `OVER` reader - desc OVER"]
pub type OVER_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - desc AWD"]
    #[inline(always)]
    pub fn awd(&self) -> AWD_R {
        AWD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EOC"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc JEOC"]
    #[inline(always)]
    pub fn jeoc(&self) -> JEOC_R {
        JEOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc JSTRT"]
    #[inline(always)]
    pub fn jstrt(&self) -> JSTRT_R {
        JSTRT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc STRT"]
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc OVER"]
    #[inline(always)]
    pub fn over(&self) -> OVER_R {
        OVER_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc AWD"]
    #[inline(always)]
    pub fn awd(&mut self) -> AWD_W<0> {
        AWD_W::new(self)
    }
    #[doc = "Bit 1 - desc EOC"]
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W<1> {
        EOC_W::new(self)
    }
    #[doc = "Bit 2 - desc JEOC"]
    #[inline(always)]
    pub fn jeoc(&mut self) -> JEOC_W<2> {
        JEOC_W::new(self)
    }
    #[doc = "Bit 3 - desc JSTRT"]
    #[inline(always)]
    pub fn jstrt(&mut self) -> JSTRT_W<3> {
        JSTRT_W::new(self)
    }
    #[doc = "Bit 4 - desc STRT"]
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W<4> {
        STRT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc SR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
