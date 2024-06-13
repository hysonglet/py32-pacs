#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PE` reader - desc PE"]
pub type PE_R = crate::BitReader<bool>;
#[doc = "Field `PE` writer - desc PE"]
pub type PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `SMBUS` reader - desc SMBUS"]
pub type SMBUS_R = crate::BitReader<bool>;
#[doc = "Field `SMBUS` writer - desc SMBUS"]
pub type SMBUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `SMBTYPE` reader - desc SMBTYPE"]
pub type SMBTYPE_R = crate::BitReader<bool>;
#[doc = "Field `SMBTYPE` writer - desc SMBTYPE"]
pub type SMBTYPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ENARP` reader - desc ENARP"]
pub type ENARP_R = crate::BitReader<bool>;
#[doc = "Field `ENARP` writer - desc ENARP"]
pub type ENARP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ENPEC` reader - desc ENPEC"]
pub type ENPEC_R = crate::BitReader<bool>;
#[doc = "Field `ENPEC` writer - desc ENPEC"]
pub type ENPEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ENGC` reader - desc ENGC"]
pub type ENGC_R = crate::BitReader<bool>;
#[doc = "Field `ENGC` writer - desc ENGC"]
pub type ENGC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `NOSTRETCH` reader - desc NOSTRETCH"]
pub type NOSTRETCH_R = crate::BitReader<bool>;
#[doc = "Field `NOSTRETCH` writer - desc NOSTRETCH"]
pub type NOSTRETCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `START` reader - desc START"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - desc START"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `STOP` reader - desc STOP"]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - desc STOP"]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ACK` reader - desc ACK"]
pub type ACK_R = crate::BitReader<bool>;
#[doc = "Field `ACK` writer - desc ACK"]
pub type ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `POS` reader - desc POS"]
pub type POS_R = crate::BitReader<bool>;
#[doc = "Field `POS` writer - desc POS"]
pub type POS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `PEC` reader - desc PEC"]
pub type PEC_R = crate::BitReader<bool>;
#[doc = "Field `PEC` writer - desc PEC"]
pub type PEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ALERT` reader - desc ALERT"]
pub type ALERT_R = crate::BitReader<bool>;
#[doc = "Field `ALERT` writer - desc ALERT"]
pub type ALERT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `SWRST` reader - desc SWRST"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `SWRST` writer - desc SWRST"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc PE"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SMBUS"]
    #[inline(always)]
    pub fn smbus(&self) -> SMBUS_R {
        SMBUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - desc SMBTYPE"]
    #[inline(always)]
    pub fn smbtype(&self) -> SMBTYPE_R {
        SMBTYPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc ENARP"]
    #[inline(always)]
    pub fn enarp(&self) -> ENARP_R {
        ENARP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc ENPEC"]
    #[inline(always)]
    pub fn enpec(&self) -> ENPEC_R {
        ENPEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc ENGC"]
    #[inline(always)]
    pub fn engc(&self) -> ENGC_R {
        ENGC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc NOSTRETCH"]
    #[inline(always)]
    pub fn nostretch(&self) -> NOSTRETCH_R {
        NOSTRETCH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc START"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc STOP"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc ACK"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc POS"]
    #[inline(always)]
    pub fn pos(&self) -> POS_R {
        POS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc PEC"]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc ALERT"]
    #[inline(always)]
    pub fn alert(&self) -> ALERT_R {
        ALERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - desc SWRST"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PE"]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W<0> {
        PE_W::new(self)
    }
    #[doc = "Bit 1 - desc SMBUS"]
    #[inline(always)]
    pub fn smbus(&mut self) -> SMBUS_W<1> {
        SMBUS_W::new(self)
    }
    #[doc = "Bit 3 - desc SMBTYPE"]
    #[inline(always)]
    pub fn smbtype(&mut self) -> SMBTYPE_W<3> {
        SMBTYPE_W::new(self)
    }
    #[doc = "Bit 4 - desc ENARP"]
    #[inline(always)]
    pub fn enarp(&mut self) -> ENARP_W<4> {
        ENARP_W::new(self)
    }
    #[doc = "Bit 5 - desc ENPEC"]
    #[inline(always)]
    pub fn enpec(&mut self) -> ENPEC_W<5> {
        ENPEC_W::new(self)
    }
    #[doc = "Bit 6 - desc ENGC"]
    #[inline(always)]
    pub fn engc(&mut self) -> ENGC_W<6> {
        ENGC_W::new(self)
    }
    #[doc = "Bit 7 - desc NOSTRETCH"]
    #[inline(always)]
    pub fn nostretch(&mut self) -> NOSTRETCH_W<7> {
        NOSTRETCH_W::new(self)
    }
    #[doc = "Bit 8 - desc START"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<8> {
        START_W::new(self)
    }
    #[doc = "Bit 9 - desc STOP"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<9> {
        STOP_W::new(self)
    }
    #[doc = "Bit 10 - desc ACK"]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W<10> {
        ACK_W::new(self)
    }
    #[doc = "Bit 11 - desc POS"]
    #[inline(always)]
    pub fn pos(&mut self) -> POS_W<11> {
        POS_W::new(self)
    }
    #[doc = "Bit 12 - desc PEC"]
    #[inline(always)]
    pub fn pec(&mut self) -> PEC_W<12> {
        PEC_W::new(self)
    }
    #[doc = "Bit 13 - desc ALERT"]
    #[inline(always)]
    pub fn alert(&mut self) -> ALERT_W<13> {
        ALERT_W::new(self)
    }
    #[doc = "Bit 15 - desc SWRST"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W<15> {
        SWRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
