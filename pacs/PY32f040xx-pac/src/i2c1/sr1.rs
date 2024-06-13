#[doc = "Register `SR1` reader"]
pub struct R(crate::R<SR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR1` writer"]
pub struct W(crate::W<SR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR1_SPEC>;
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
impl From<crate::W<SR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SB` reader - desc SB"]
pub type SB_R = crate::BitReader<bool>;
#[doc = "Field `ADDR` reader - desc ADDR"]
pub type ADDR_R = crate::BitReader<bool>;
#[doc = "Field `BTF` reader - desc BTF"]
pub type BTF_R = crate::BitReader<bool>;
#[doc = "Field `ADD10` reader - desc ADD10"]
pub type ADD10_R = crate::BitReader<bool>;
#[doc = "Field `STOPF` reader - desc STOPF"]
pub type STOPF_R = crate::BitReader<bool>;
#[doc = "Field `RXNE` reader - desc RXNE"]
pub type RXNE_R = crate::BitReader<bool>;
#[doc = "Field `TXE` reader - desc TXE"]
pub type TXE_R = crate::BitReader<bool>;
#[doc = "Field `BERR` reader - desc BERR"]
pub type BERR_R = crate::BitReader<bool>;
#[doc = "Field `BERR` writer - desc BERR"]
pub type BERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR1_SPEC, bool, O>;
#[doc = "Field `ARLO` reader - desc ARLO"]
pub type ARLO_R = crate::BitReader<bool>;
#[doc = "Field `ARLO` writer - desc ARLO"]
pub type ARLO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR1_SPEC, bool, O>;
#[doc = "Field `AF` reader - desc AF"]
pub type AF_R = crate::BitReader<bool>;
#[doc = "Field `AF` writer - desc AF"]
pub type AF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR1_SPEC, bool, O>;
#[doc = "Field `OVR` reader - desc OVR"]
pub type OVR_R = crate::BitReader<bool>;
#[doc = "Field `OVR` writer - desc OVR"]
pub type OVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR1_SPEC, bool, O>;
#[doc = "Field `PECERR` reader - desc PECERR"]
pub type PECERR_R = crate::BitReader<bool>;
#[doc = "Field `PECERR` writer - desc PECERR"]
pub type PECERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR1_SPEC, bool, O>;
#[doc = "Field `TIMEOUT` reader - desc TIMEOUT"]
pub type TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `TIMEOUT` writer - desc TIMEOUT"]
pub type TIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR1_SPEC, bool, O>;
#[doc = "Field `SMBALERT` reader - desc SMBALERT"]
pub type SMBALERT_R = crate::BitReader<bool>;
#[doc = "Field `SMBALERT` writer - desc SMBALERT"]
pub type SMBALERT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc SB"]
    #[inline(always)]
    pub fn sb(&self) -> SB_R {
        SB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc ADDR"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc BTF"]
    #[inline(always)]
    pub fn btf(&self) -> BTF_R {
        BTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc ADD10"]
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc STOPF"]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - desc RXNE"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc TXE"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc BERR"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc ARLO"]
    #[inline(always)]
    pub fn arlo(&self) -> ARLO_R {
        ARLO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc AF"]
    #[inline(always)]
    pub fn af(&self) -> AF_R {
        AF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc OVR"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc PECERR"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - desc TIMEOUT"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc SMBALERT"]
    #[inline(always)]
    pub fn smbalert(&self) -> SMBALERT_R {
        SMBALERT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - desc BERR"]
    #[inline(always)]
    pub fn berr(&mut self) -> BERR_W<8> {
        BERR_W::new(self)
    }
    #[doc = "Bit 9 - desc ARLO"]
    #[inline(always)]
    pub fn arlo(&mut self) -> ARLO_W<9> {
        ARLO_W::new(self)
    }
    #[doc = "Bit 10 - desc AF"]
    #[inline(always)]
    pub fn af(&mut self) -> AF_W<10> {
        AF_W::new(self)
    }
    #[doc = "Bit 11 - desc OVR"]
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W<11> {
        OVR_W::new(self)
    }
    #[doc = "Bit 12 - desc PECERR"]
    #[inline(always)]
    pub fn pecerr(&mut self) -> PECERR_W<12> {
        PECERR_W::new(self)
    }
    #[doc = "Bit 14 - desc TIMEOUT"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W<14> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 15 - desc SMBALERT"]
    #[inline(always)]
    pub fn smbalert(&mut self) -> SMBALERT_W<15> {
        SMBALERT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc SR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr1](index.html) module"]
pub struct SR1_SPEC;
impl crate::RegisterSpec for SR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr1::R](R) reader structure"]
impl crate::Readable for SR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr1::W](W) writer structure"]
impl crate::Writable for SR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR1 to value 0"]
impl crate::Resettable for SR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
