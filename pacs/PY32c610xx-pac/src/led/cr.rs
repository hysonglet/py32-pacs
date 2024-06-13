#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEDON` reader - LED enable"]
pub type LEDON_R = crate::BitReader<bool>;
#[doc = "Field `LEDON` writer - LED enable"]
pub type LEDON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `LED_COM_SEL` reader - LED COM Selection"]
pub type LED_COM_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LED_COM_SEL` writer - LED COM Selection"]
pub type LED_COM_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `IE` reader - LED interrupt enable"]
pub type IE_R = crate::BitReader<bool>;
#[doc = "Field `IE` writer - LED interrupt enable"]
pub type IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EHS` reader - Light control"]
pub type EHS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EHS` writer - Light control"]
pub type EHS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - LED enable"]
    #[inline(always)]
    pub fn ledon(&self) -> LEDON_R {
        LEDON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - LED COM Selection"]
    #[inline(always)]
    pub fn led_com_sel(&self) -> LED_COM_SEL_R {
        LED_COM_SEL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - LED interrupt enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Light control"]
    #[inline(always)]
    pub fn ehs(&self) -> EHS_R {
        EHS_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LED enable"]
    #[inline(always)]
    pub fn ledon(&mut self) -> LEDON_W<0> {
        LEDON_W::new(self)
    }
    #[doc = "Bits 1:2 - LED COM Selection"]
    #[inline(always)]
    pub fn led_com_sel(&mut self) -> LED_COM_SEL_W<1> {
        LED_COM_SEL_W::new(self)
    }
    #[doc = "Bit 3 - LED interrupt enable"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W<3> {
        IE_W::new(self)
    }
    #[doc = "Bits 12:13 - Light control"]
    #[inline(always)]
    pub fn ehs(&mut self) -> EHS_W<12> {
        EHS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
