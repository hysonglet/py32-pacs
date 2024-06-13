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
#[doc = "Field `PE` reader - desc PE"]
pub type PE_R = crate::BitReader<bool>;
#[doc = "Field `FE` reader - desc FE"]
pub type FE_R = crate::BitReader<bool>;
#[doc = "Field `NE` reader - desc NE"]
pub type NE_R = crate::BitReader<bool>;
#[doc = "Field `ORE` reader - desc ORE"]
pub type ORE_R = crate::BitReader<bool>;
#[doc = "Field `IDLE` reader - desc IDLE"]
pub type IDLE_R = crate::BitReader<bool>;
#[doc = "Field `RXNE` reader - desc RXNE"]
pub type RXNE_R = crate::BitReader<bool>;
#[doc = "Field `RXNE` writer - desc RXNE"]
pub type RXNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `TC` reader - desc TC"]
pub type TC_R = crate::BitReader<bool>;
#[doc = "Field `TC` writer - desc TC"]
pub type TC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `TXE` reader - desc TXE"]
pub type TXE_R = crate::BitReader<bool>;
#[doc = "Field `LBD` reader - desc LBD"]
pub type LBD_R = crate::BitReader<bool>;
#[doc = "Field `LBD` writer - desc LBD"]
pub type LBD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CTS` reader - desc CTS"]
pub type CTS_R = crate::BitReader<bool>;
#[doc = "Field `CTS` writer - desc CTS"]
pub type CTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `ABRF` reader - desc ABRF"]
pub type ABRF_R = crate::BitReader<bool>;
#[doc = "Field `ABRE` reader - desc ABRE"]
pub type ABRE_R = crate::BitReader<bool>;
#[doc = "Field `ABRRQ` writer - desc ABRRQ"]
pub type ABRRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc PE"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc FE"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc NE"]
    #[inline(always)]
    pub fn ne(&self) -> NE_R {
        NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc ORE"]
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc IDLE"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc RXNE"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc TC"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc TXE"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc LBD"]
    #[inline(always)]
    pub fn lbd(&self) -> LBD_R {
        LBD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc CTS"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc ABRF"]
    #[inline(always)]
    pub fn abrf(&self) -> ABRF_R {
        ABRF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc ABRE"]
    #[inline(always)]
    pub fn abre(&self) -> ABRE_R {
        ABRE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - desc RXNE"]
    #[inline(always)]
    pub fn rxne(&mut self) -> RXNE_W<5> {
        RXNE_W::new(self)
    }
    #[doc = "Bit 6 - desc TC"]
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W<6> {
        TC_W::new(self)
    }
    #[doc = "Bit 8 - desc LBD"]
    #[inline(always)]
    pub fn lbd(&mut self) -> LBD_W<8> {
        LBD_W::new(self)
    }
    #[doc = "Bit 9 - desc CTS"]
    #[inline(always)]
    pub fn cts(&mut self) -> CTS_W<9> {
        CTS_W::new(self)
    }
    #[doc = "Bit 12 - desc ABRRQ"]
    #[inline(always)]
    pub fn abrrq(&mut self) -> ABRRQ_W<12> {
        ABRRQ_W::new(self)
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
#[doc = "`reset()` method sets SR to value 0xc0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc0
    }
}
