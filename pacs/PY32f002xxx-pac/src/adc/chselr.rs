#[doc = "Register `CHSELR` reader"]
pub struct R(crate::R<CHSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHSELR` writer"]
pub struct W(crate::W<CHSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHSELR_SPEC>;
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
impl From<crate::W<CHSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHSEL0` reader - Channel-0 selection"]
pub type CHSEL0_R = crate::BitReader<bool>;
#[doc = "Field `CHSEL0` writer - Channel-0 selection"]
pub type CHSEL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHSELR_SPEC, bool, O>;
#[doc = "Field `CHSEL1` reader - Channel-1 selection"]
pub type CHSEL1_R = crate::BitReader<bool>;
#[doc = "Field `CHSEL1` writer - Channel-1 selection"]
pub type CHSEL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHSELR_SPEC, bool, O>;
#[doc = "Field `CHSEL2` reader - Channel-2 selection"]
pub type CHSEL2_R = crate::BitReader<bool>;
#[doc = "Field `CHSEL2` writer - Channel-2 selection"]
pub type CHSEL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHSELR_SPEC, bool, O>;
#[doc = "Field `CHSEL3` reader - Channel-3 selection"]
pub type CHSEL3_R = crate::BitReader<bool>;
#[doc = "Field `CHSEL3` writer - Channel-3 selection"]
pub type CHSEL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHSELR_SPEC, bool, O>;
#[doc = "Field `CHSEL4` reader - Channel-4 selection"]
pub type CHSEL4_R = crate::BitReader<bool>;
#[doc = "Field `CHSEL4` writer - Channel-4 selection"]
pub type CHSEL4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHSELR_SPEC, bool, O>;
#[doc = "Field `CHSEL5` reader - Channel-5 selection"]
pub type CHSEL5_R = crate::BitReader<bool>;
#[doc = "Field `CHSEL5` writer - Channel-5 selection"]
pub type CHSEL5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHSELR_SPEC, bool, O>;
#[doc = "Field `CHSEL6` reader - Channel-6 selection"]
pub type CHSEL6_R = crate::BitReader<bool>;
#[doc = "Field `CHSEL6` writer - Channel-6 selection"]
pub type CHSEL6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHSELR_SPEC, bool, O>;
#[doc = "Field `CHSEL7` reader - Channel-7 selection"]
pub type CHSEL7_R = crate::BitReader<bool>;
#[doc = "Field `CHSEL7` writer - Channel-7 selection"]
pub type CHSEL7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHSELR_SPEC, bool, O>;
#[doc = "Field `CHSEL8` reader - Channel-8 selection"]
pub type CHSEL8_R = crate::BitReader<bool>;
#[doc = "Field `CHSEL8` writer - Channel-8 selection"]
pub type CHSEL8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHSELR_SPEC, bool, O>;
#[doc = "Field `CHSEL9` reader - Channel-9 selection"]
pub type CHSEL9_R = crate::BitReader<bool>;
#[doc = "Field `CHSEL9` writer - Channel-9 selection"]
pub type CHSEL9_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHSELR_SPEC, bool, O>;
#[doc = "Field `CHSEL10` reader - Channel-10 selection"]
pub type CHSEL10_R = crate::BitReader<bool>;
#[doc = "Field `CHSEL10` writer - Channel-10 selection"]
pub type CHSEL10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHSELR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Channel-0 selection"]
    #[inline(always)]
    pub fn chsel0(&self) -> CHSEL0_R {
        CHSEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel-1 selection"]
    #[inline(always)]
    pub fn chsel1(&self) -> CHSEL1_R {
        CHSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel-2 selection"]
    #[inline(always)]
    pub fn chsel2(&self) -> CHSEL2_R {
        CHSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel-3 selection"]
    #[inline(always)]
    pub fn chsel3(&self) -> CHSEL3_R {
        CHSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel-4 selection"]
    #[inline(always)]
    pub fn chsel4(&self) -> CHSEL4_R {
        CHSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel-5 selection"]
    #[inline(always)]
    pub fn chsel5(&self) -> CHSEL5_R {
        CHSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel-6 selection"]
    #[inline(always)]
    pub fn chsel6(&self) -> CHSEL6_R {
        CHSEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel-7 selection"]
    #[inline(always)]
    pub fn chsel7(&self) -> CHSEL7_R {
        CHSEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel-8 selection"]
    #[inline(always)]
    pub fn chsel8(&self) -> CHSEL8_R {
        CHSEL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel-9 selection"]
    #[inline(always)]
    pub fn chsel9(&self) -> CHSEL9_R {
        CHSEL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel-10 selection"]
    #[inline(always)]
    pub fn chsel10(&self) -> CHSEL10_R {
        CHSEL10_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel-0 selection"]
    #[inline(always)]
    pub fn chsel0(&mut self) -> CHSEL0_W<0> {
        CHSEL0_W::new(self)
    }
    #[doc = "Bit 1 - Channel-1 selection"]
    #[inline(always)]
    pub fn chsel1(&mut self) -> CHSEL1_W<1> {
        CHSEL1_W::new(self)
    }
    #[doc = "Bit 2 - Channel-2 selection"]
    #[inline(always)]
    pub fn chsel2(&mut self) -> CHSEL2_W<2> {
        CHSEL2_W::new(self)
    }
    #[doc = "Bit 3 - Channel-3 selection"]
    #[inline(always)]
    pub fn chsel3(&mut self) -> CHSEL3_W<3> {
        CHSEL3_W::new(self)
    }
    #[doc = "Bit 4 - Channel-4 selection"]
    #[inline(always)]
    pub fn chsel4(&mut self) -> CHSEL4_W<4> {
        CHSEL4_W::new(self)
    }
    #[doc = "Bit 5 - Channel-5 selection"]
    #[inline(always)]
    pub fn chsel5(&mut self) -> CHSEL5_W<5> {
        CHSEL5_W::new(self)
    }
    #[doc = "Bit 6 - Channel-6 selection"]
    #[inline(always)]
    pub fn chsel6(&mut self) -> CHSEL6_W<6> {
        CHSEL6_W::new(self)
    }
    #[doc = "Bit 7 - Channel-7 selection"]
    #[inline(always)]
    pub fn chsel7(&mut self) -> CHSEL7_W<7> {
        CHSEL7_W::new(self)
    }
    #[doc = "Bit 8 - Channel-8 selection"]
    #[inline(always)]
    pub fn chsel8(&mut self) -> CHSEL8_W<8> {
        CHSEL8_W::new(self)
    }
    #[doc = "Bit 9 - Channel-9 selection"]
    #[inline(always)]
    pub fn chsel9(&mut self) -> CHSEL9_W<9> {
        CHSEL9_W::new(self)
    }
    #[doc = "Bit 10 - Channel-10 selection"]
    #[inline(always)]
    pub fn chsel10(&mut self) -> CHSEL10_W<10> {
        CHSEL10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC group regular sequencer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chselr](index.html) module"]
pub struct CHSELR_SPEC;
impl crate::RegisterSpec for CHSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chselr::R](R) reader structure"]
impl crate::Readable for CHSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chselr::W](W) writer structure"]
impl crate::Writable for CHSELR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHSELR to value 0x0fff_0000"]
impl crate::Resettable for CHSELR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_0000
    }
}
