#[doc = "Register `BDTR` reader"]
pub struct R(crate::R<BDTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDTR` writer"]
pub struct W(crate::W<BDTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDTR_SPEC>;
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
impl From<crate::W<BDTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTG` reader - desc DTG"]
pub type DTG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTG` writer - desc DTG"]
pub type DTG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BDTR_SPEC, u8, u8, 8, O>;
#[doc = "Field `LOCK` reader - desc LOCK"]
pub type LOCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOCK` writer - desc LOCK"]
pub type LOCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BDTR_SPEC, u8, u8, 2, O>;
#[doc = "Field `OSSI` reader - desc OSSI"]
pub type OSSI_R = crate::BitReader<bool>;
#[doc = "Field `OSSI` writer - desc OSSI"]
pub type OSSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, bool, O>;
#[doc = "Field `OSSR` reader - desc OSSR"]
pub type OSSR_R = crate::BitReader<bool>;
#[doc = "Field `OSSR` writer - desc OSSR"]
pub type OSSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, bool, O>;
#[doc = "Field `BKE` reader - desc BKE"]
pub type BKE_R = crate::BitReader<bool>;
#[doc = "Field `BKE` writer - desc BKE"]
pub type BKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, bool, O>;
#[doc = "Field `BKP` reader - desc BKP"]
pub type BKP_R = crate::BitReader<bool>;
#[doc = "Field `BKP` writer - desc BKP"]
pub type BKP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, bool, O>;
#[doc = "Field `AOE` reader - desc AOE"]
pub type AOE_R = crate::BitReader<bool>;
#[doc = "Field `AOE` writer - desc AOE"]
pub type AOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, bool, O>;
#[doc = "Field `MOE` reader - desc MOE"]
pub type MOE_R = crate::BitReader<bool>;
#[doc = "Field `MOE` writer - desc MOE"]
pub type MOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - desc DTG"]
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - desc LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - desc OSSI"]
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc OSSR"]
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc BKE"]
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc BKP"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc AOE"]
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc MOE"]
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc DTG"]
    #[inline(always)]
    pub fn dtg(&mut self) -> DTG_W<0> {
        DTG_W::new(self)
    }
    #[doc = "Bits 8:9 - desc LOCK"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<8> {
        LOCK_W::new(self)
    }
    #[doc = "Bit 10 - desc OSSI"]
    #[inline(always)]
    pub fn ossi(&mut self) -> OSSI_W<10> {
        OSSI_W::new(self)
    }
    #[doc = "Bit 11 - desc OSSR"]
    #[inline(always)]
    pub fn ossr(&mut self) -> OSSR_W<11> {
        OSSR_W::new(self)
    }
    #[doc = "Bit 12 - desc BKE"]
    #[inline(always)]
    pub fn bke(&mut self) -> BKE_W<12> {
        BKE_W::new(self)
    }
    #[doc = "Bit 13 - desc BKP"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<13> {
        BKP_W::new(self)
    }
    #[doc = "Bit 14 - desc AOE"]
    #[inline(always)]
    pub fn aoe(&mut self) -> AOE_W<14> {
        AOE_W::new(self)
    }
    #[doc = "Bit 15 - desc MOE"]
    #[inline(always)]
    pub fn moe(&mut self) -> MOE_W<15> {
        MOE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc BDTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdtr](index.html) module"]
pub struct BDTR_SPEC;
impl crate::RegisterSpec for BDTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdtr::R](R) reader structure"]
impl crate::Readable for BDTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdtr::W](W) writer structure"]
impl crate::Writable for BDTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BDTR to value 0"]
impl crate::Resettable for BDTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
