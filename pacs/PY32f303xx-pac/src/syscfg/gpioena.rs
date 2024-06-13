#[doc = "Register `GPIOENA` reader"]
pub struct R(crate::R<GPIOENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOENA` writer"]
pub struct W(crate::W<GPIOENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOENA_SPEC>;
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
impl From<crate::W<GPIOENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA_ENA` reader - desc PA_ENA"]
pub type PA_ENA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PA_ENA` writer - desc PA_ENA"]
pub type PA_ENA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOENA_SPEC, u8, u8, 8, O>;
#[doc = "Field `PB_ENA` reader - desc PB_ENA"]
pub type PB_ENA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PB_ENA` writer - desc PB_ENA"]
pub type PB_ENA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOENA_SPEC, u8, u8, 2, O>;
#[doc = "Field `PC_ENA` reader - desc PC_ENA"]
pub type PC_ENA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PC_ENA` writer - desc PC_ENA"]
pub type PC_ENA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOENA_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:7 - desc PA_ENA"]
    #[inline(always)]
    pub fn pa_ena(&self) -> PA_ENA_R {
        PA_ENA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - desc PB_ENA"]
    #[inline(always)]
    pub fn pb_ena(&self) -> PB_ENA_R {
        PB_ENA_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:21 - desc PC_ENA"]
    #[inline(always)]
    pub fn pc_ena(&self) -> PC_ENA_R {
        PC_ENA_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc PA_ENA"]
    #[inline(always)]
    pub fn pa_ena(&mut self) -> PA_ENA_W<0> {
        PA_ENA_W::new(self)
    }
    #[doc = "Bits 8:9 - desc PB_ENA"]
    #[inline(always)]
    pub fn pb_ena(&mut self) -> PB_ENA_W<8> {
        PB_ENA_W::new(self)
    }
    #[doc = "Bits 16:21 - desc PC_ENA"]
    #[inline(always)]
    pub fn pc_ena(&mut self) -> PC_ENA_W<16> {
        PC_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc GPIOENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioena](index.html) module"]
pub struct GPIOENA_SPEC;
impl crate::RegisterSpec for GPIOENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioena::R](R) reader structure"]
impl crate::Readable for GPIOENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpioena::W](W) writer structure"]
impl crate::Writable for GPIOENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOENA to value 0"]
impl crate::Resettable for GPIOENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
