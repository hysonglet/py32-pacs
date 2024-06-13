#[doc = "Register `MESR` reader"]
pub struct R(crate::R<MESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MESR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MESR` writer"]
pub struct W(crate::W<MESR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MESR_SPEC>;
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
impl From<crate::W<MESR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MESR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEBP1` reader - desc MEBP1"]
pub type MEBP1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEBP1` writer - desc MEBP1"]
pub type MEBP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MESR_SPEC, u8, u8, 6, O>;
#[doc = "Field `ME1EE` reader - desc ME1EE"]
pub type ME1EE_R = crate::BitReader<bool>;
#[doc = "Field `ME1EE` writer - desc ME1EE"]
pub type ME1EE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MESR_SPEC, bool, O>;
#[doc = "Field `MEAEE` reader - desc MEAEE"]
pub type MEAEE_R = crate::BitReader<bool>;
#[doc = "Field `MEAEE` writer - desc MEAEE"]
pub type MEAEE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MESR_SPEC, bool, O>;
#[doc = "Field `MEBP2` reader - desc MEBP2"]
pub type MEBP2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEBP2` writer - desc MEBP2"]
pub type MEBP2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MESR_SPEC, u8, u8, 6, O>;
#[doc = "Field `ME2EE` reader - desc ME2EE"]
pub type ME2EE_R = crate::BitReader<bool>;
#[doc = "Field `ME2EE` writer - desc ME2EE"]
pub type ME2EE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MESR_SPEC, bool, O>;
#[doc = "Field `MEEEC` reader - desc MEEEC"]
pub type MEEEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEEEC` writer - desc MEEEC"]
pub type MEEEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MESR_SPEC, u8, u8, 4, O>;
#[doc = "Field `MENEC` reader - desc MENEC"]
pub type MENEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MENEC` writer - desc MENEC"]
pub type MENEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MESR_SPEC, u8, u8, 4, O>;
#[doc = "Field `MEL` reader - desc MEL"]
pub type MEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEL` writer - desc MEL"]
pub type MEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MESR_SPEC, u8, u8, 2, O>;
#[doc = "Field `MES` reader - desc MES"]
pub type MES_R = crate::BitReader<bool>;
#[doc = "Field `MES` writer - desc MES"]
pub type MES_W<'a, const O: u8> = crate::BitWriter<'a, u32, MESR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - desc MEBP1"]
    #[inline(always)]
    pub fn mebp1(&self) -> MEBP1_R {
        MEBP1_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - desc ME1EE"]
    #[inline(always)]
    pub fn me1ee(&self) -> ME1EE_R {
        ME1EE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc MEAEE"]
    #[inline(always)]
    pub fn meaee(&self) -> MEAEE_R {
        MEAEE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - desc MEBP2"]
    #[inline(always)]
    pub fn mebp2(&self) -> MEBP2_R {
        MEBP2_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - desc ME2EE"]
    #[inline(always)]
    pub fn me2ee(&self) -> ME2EE_R {
        ME2EE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:19 - desc MEEEC"]
    #[inline(always)]
    pub fn meeec(&self) -> MEEEC_R {
        MEEEC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - desc MENEC"]
    #[inline(always)]
    pub fn menec(&self) -> MENEC_R {
        MENEC_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - desc MEL"]
    #[inline(always)]
    pub fn mel(&self) -> MEL_R {
        MEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - desc MES"]
    #[inline(always)]
    pub fn mes(&self) -> MES_R {
        MES_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - desc MEBP1"]
    #[inline(always)]
    pub fn mebp1(&mut self) -> MEBP1_W<0> {
        MEBP1_W::new(self)
    }
    #[doc = "Bit 6 - desc ME1EE"]
    #[inline(always)]
    pub fn me1ee(&mut self) -> ME1EE_W<6> {
        ME1EE_W::new(self)
    }
    #[doc = "Bit 7 - desc MEAEE"]
    #[inline(always)]
    pub fn meaee(&mut self) -> MEAEE_W<7> {
        MEAEE_W::new(self)
    }
    #[doc = "Bits 8:13 - desc MEBP2"]
    #[inline(always)]
    pub fn mebp2(&mut self) -> MEBP2_W<8> {
        MEBP2_W::new(self)
    }
    #[doc = "Bit 14 - desc ME2EE"]
    #[inline(always)]
    pub fn me2ee(&mut self) -> ME2EE_W<14> {
        ME2EE_W::new(self)
    }
    #[doc = "Bits 16:19 - desc MEEEC"]
    #[inline(always)]
    pub fn meeec(&mut self) -> MEEEC_W<16> {
        MEEEC_W::new(self)
    }
    #[doc = "Bits 20:23 - desc MENEC"]
    #[inline(always)]
    pub fn menec(&mut self) -> MENEC_W<20> {
        MENEC_W::new(self)
    }
    #[doc = "Bits 24:25 - desc MEL"]
    #[inline(always)]
    pub fn mel(&mut self) -> MEL_W<24> {
        MEL_W::new(self)
    }
    #[doc = "Bit 26 - desc MES"]
    #[inline(always)]
    pub fn mes(&mut self) -> MES_W<26> {
        MES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc MESR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mesr](index.html) module"]
pub struct MESR_SPEC;
impl crate::RegisterSpec for MESR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mesr::R](R) reader structure"]
impl crate::Readable for MESR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mesr::W](W) writer structure"]
impl crate::Writable for MESR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MESR to value 0"]
impl crate::Resettable for MESR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
