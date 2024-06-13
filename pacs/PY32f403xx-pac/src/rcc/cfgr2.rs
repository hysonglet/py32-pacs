#[doc = "Register `CFGR2` reader"]
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR2` writer"]
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CANCKSEL` reader - desc CANCKSEL"]
pub type CANCKSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CANCKSEL` writer - desc CANCKSEL"]
pub type CANCKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `HSEDRV` reader - desc HSEDRV"]
pub type HSEDRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSEDRV` writer - desc HSEDRV"]
pub type HSEDRV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `HSESTART` reader - desc HSESTART"]
pub type HSESTART_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSESTART` writer - desc HSESTART"]
pub type HSESTART_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `HSERFSEL` reader - desc HSERFSEL"]
pub type HSERFSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSERFSEL` writer - desc HSERFSEL"]
pub type HSERFSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:3 - desc CANCKSEL"]
    #[inline(always)]
    pub fn cancksel(&self) -> CANCKSEL_R {
        CANCKSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - desc HSEDRV"]
    #[inline(always)]
    pub fn hsedrv(&self) -> HSEDRV_R {
        HSEDRV_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - desc HSESTART"]
    #[inline(always)]
    pub fn hsestart(&self) -> HSESTART_R {
        HSESTART_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - desc HSERFSEL"]
    #[inline(always)]
    pub fn hserfsel(&self) -> HSERFSEL_R {
        HSERFSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc CANCKSEL"]
    #[inline(always)]
    pub fn cancksel(&mut self) -> CANCKSEL_W<0> {
        CANCKSEL_W::new(self)
    }
    #[doc = "Bits 8:9 - desc HSEDRV"]
    #[inline(always)]
    pub fn hsedrv(&mut self) -> HSEDRV_W<8> {
        HSEDRV_W::new(self)
    }
    #[doc = "Bits 10:11 - desc HSESTART"]
    #[inline(always)]
    pub fn hsestart(&mut self) -> HSESTART_W<10> {
        HSESTART_W::new(self)
    }
    #[doc = "Bits 12:13 - desc HSERFSEL"]
    #[inline(always)]
    pub fn hserfsel(&mut self) -> HSERFSEL_W<12> {
        HSERFSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CFGR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr2::R](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR2 to value 0x08"]
impl crate::Resettable for CFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
