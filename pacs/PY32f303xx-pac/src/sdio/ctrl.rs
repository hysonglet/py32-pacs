#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDIORST` reader - desc SDIORST"]
pub type SDIORST_R = crate::BitReader<bool>;
#[doc = "Field `SDIORST` writer - desc SDIORST"]
pub type SDIORST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `FIFORST` reader - desc FIFORST"]
pub type FIFORST_R = crate::BitReader<bool>;
#[doc = "Field `FIFORST` writer - desc FIFORST"]
pub type FIFORST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `INTEN` reader - desc INTEN"]
pub type INTEN_R = crate::BitReader<bool>;
#[doc = "Field `INTEN` writer - desc INTEN"]
pub type INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DMAEN` reader - desc DMAEN"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - desc DMAEN"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `READWAIT` reader - desc READWAIT"]
pub type READWAIT_R = crate::BitReader<bool>;
#[doc = "Field `READWAIT` writer - desc READWAIT"]
pub type READWAIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `AUTOIRQRESP` reader - desc AUTOIRQRESP"]
pub type AUTOIRQRESP_R = crate::BitReader<bool>;
#[doc = "Field `AUTOIRQRESP` writer - desc AUTOIRQRESP"]
pub type AUTOIRQRESP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ABORTRD` reader - desc ABORTRD"]
pub type ABORTRD_R = crate::BitReader<bool>;
#[doc = "Field `ABORTRD` writer - desc ABORTRD"]
pub type ABORTRD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CCSDEN` reader - desc CCSDEN"]
pub type CCSDEN_R = crate::BitReader<bool>;
#[doc = "Field `CCSDEN` writer - desc CCSDEN"]
pub type CCSDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `AUTOSTOPCCSD` reader - desc AUTOSTOPCCSD"]
pub type AUTOSTOPCCSD_R = crate::BitReader<bool>;
#[doc = "Field `AUTOSTOPCCSD` writer - desc AUTOSTOPCCSD"]
pub type AUTOSTOPCCSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CEATAINTEN` reader - desc CEATAINTEN"]
pub type CEATAINTEN_R = crate::BitReader<bool>;
#[doc = "Field `CEATAINTEN` writer - desc CEATAINTEN"]
pub type CEATAINTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ODPUEN` reader - desc ODPUEN"]
pub type ODPUEN_R = crate::BitReader<bool>;
#[doc = "Field `ODPUEN` writer - desc ODPUEN"]
pub type ODPUEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc SDIORST"]
    #[inline(always)]
    pub fn sdiorst(&self) -> SDIORST_R {
        SDIORST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc FIFORST"]
    #[inline(always)]
    pub fn fiforst(&self) -> FIFORST_R {
        FIFORST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - desc INTEN"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc READWAIT"]
    #[inline(always)]
    pub fn readwait(&self) -> READWAIT_R {
        READWAIT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc AUTOIRQRESP"]
    #[inline(always)]
    pub fn autoirqresp(&self) -> AUTOIRQRESP_R {
        AUTOIRQRESP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc ABORTRD"]
    #[inline(always)]
    pub fn abortrd(&self) -> ABORTRD_R {
        ABORTRD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc CCSDEN"]
    #[inline(always)]
    pub fn ccsden(&self) -> CCSDEN_R {
        CCSDEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc AUTOSTOPCCSD"]
    #[inline(always)]
    pub fn autostopccsd(&self) -> AUTOSTOPCCSD_R {
        AUTOSTOPCCSD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc CEATAINTEN"]
    #[inline(always)]
    pub fn ceatainten(&self) -> CEATAINTEN_R {
        CEATAINTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 24 - desc ODPUEN"]
    #[inline(always)]
    pub fn odpuen(&self) -> ODPUEN_R {
        ODPUEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SDIORST"]
    #[inline(always)]
    pub fn sdiorst(&mut self) -> SDIORST_W<0> {
        SDIORST_W::new(self)
    }
    #[doc = "Bit 1 - desc FIFORST"]
    #[inline(always)]
    pub fn fiforst(&mut self) -> FIFORST_W<1> {
        FIFORST_W::new(self)
    }
    #[doc = "Bit 4 - desc INTEN"]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W<4> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 5 - desc DMAEN"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<5> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 6 - desc READWAIT"]
    #[inline(always)]
    pub fn readwait(&mut self) -> READWAIT_W<6> {
        READWAIT_W::new(self)
    }
    #[doc = "Bit 7 - desc AUTOIRQRESP"]
    #[inline(always)]
    pub fn autoirqresp(&mut self) -> AUTOIRQRESP_W<7> {
        AUTOIRQRESP_W::new(self)
    }
    #[doc = "Bit 8 - desc ABORTRD"]
    #[inline(always)]
    pub fn abortrd(&mut self) -> ABORTRD_W<8> {
        ABORTRD_W::new(self)
    }
    #[doc = "Bit 9 - desc CCSDEN"]
    #[inline(always)]
    pub fn ccsden(&mut self) -> CCSDEN_W<9> {
        CCSDEN_W::new(self)
    }
    #[doc = "Bit 10 - desc AUTOSTOPCCSD"]
    #[inline(always)]
    pub fn autostopccsd(&mut self) -> AUTOSTOPCCSD_W<10> {
        AUTOSTOPCCSD_W::new(self)
    }
    #[doc = "Bit 11 - desc CEATAINTEN"]
    #[inline(always)]
    pub fn ceatainten(&mut self) -> CEATAINTEN_W<11> {
        CEATAINTEN_W::new(self)
    }
    #[doc = "Bit 24 - desc ODPUEN"]
    #[inline(always)]
    pub fn odpuen(&mut self) -> ODPUEN_W<24> {
        ODPUEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x0100_0ff3"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0ff3
    }
}
