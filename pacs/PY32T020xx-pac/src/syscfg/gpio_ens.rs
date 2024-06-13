#[doc = "Register `GPIO_ENS` reader"]
pub struct R(crate::R<GPIO_ENS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_ENS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_ENS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_ENS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_ENS` writer"]
pub struct W(crate::W<GPIO_ENS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_ENS_SPEC>;
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
impl From<crate::W<GPIO_ENS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_ENS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA_ENS` reader - "]
pub type PA_ENS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PA_ENS` writer - "]
pub type PA_ENS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIO_ENS_SPEC, u8, u8, 8, O>;
#[doc = "Field `PB_ENS` reader - "]
pub type PB_ENS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PB_ENS` writer - "]
pub type PB_ENS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIO_ENS_SPEC, u8, u8, 8, O>;
#[doc = "Field `PC_ENS` reader - "]
pub type PC_ENS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PC_ENS` writer - "]
pub type PC_ENS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIO_ENS_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pa_ens(&self) -> PA_ENS_R {
        PA_ENS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn pb_ens(&self) -> PB_ENS_R {
        PB_ENS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pc_ens(&self) -> PC_ENS_R {
        PC_ENS_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pa_ens(&mut self) -> PA_ENS_W<0> {
        PA_ENS_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn pb_ens(&mut self) -> PB_ENS_W<8> {
        PB_ENS_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pc_ens(&mut self) -> PC_ENS_W<16> {
        PC_ENS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_ens](index.html) module"]
pub struct GPIO_ENS_SPEC;
impl crate::RegisterSpec for GPIO_ENS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_ens::R](R) reader structure"]
impl crate::Readable for GPIO_ENS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_ens::W](W) writer structure"]
impl crate::Writable for GPIO_ENS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_ENS to value 0"]
impl crate::Resettable for GPIO_ENS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
