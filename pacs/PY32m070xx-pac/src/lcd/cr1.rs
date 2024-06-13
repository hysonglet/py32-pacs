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
#[doc = "Field `BLINKCNT` reader - BLINKCNT"]
pub type BLINKCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BLINKCNT` writer - BLINKCNT"]
pub type BLINKCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 6, O>;
#[doc = "Field `BLINKEN` reader - BLINKEN"]
pub type BLINKEN_R = crate::BitReader<bool>;
#[doc = "Field `BLINKEN` writer - BLINKEN"]
pub type BLINKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `MODE` reader - MODE"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - MODE"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `IE` reader - IE"]
pub type IE_R = crate::BitReader<bool>;
#[doc = "Field `IE` writer - IE"]
pub type IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `DMAEN` reader - DMAEN"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - DMAEN"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `INTF` reader - INTF"]
pub type INTF_R = crate::BitReader<bool>;
#[doc = "Field `INTF` writer - INTF"]
pub type INTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - BLINKCNT"]
    #[inline(always)]
    pub fn blinkcnt(&self) -> BLINKCNT_R {
        BLINKCNT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - BLINKEN"]
    #[inline(always)]
    pub fn blinken(&self) -> BLINKEN_R {
        BLINKEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IE"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - INTF"]
    #[inline(always)]
    pub fn intf(&self) -> INTF_R {
        INTF_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - BLINKCNT"]
    #[inline(always)]
    pub fn blinkcnt(&mut self) -> BLINKCNT_W<0> {
        BLINKCNT_W::new(self)
    }
    #[doc = "Bit 6 - BLINKEN"]
    #[inline(always)]
    pub fn blinken(&mut self) -> BLINKEN_W<6> {
        BLINKEN_W::new(self)
    }
    #[doc = "Bit 8 - MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<8> {
        MODE_W::new(self)
    }
    #[doc = "Bit 9 - IE"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W<9> {
        IE_W::new(self)
    }
    #[doc = "Bit 10 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<10> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 11 - INTF"]
    #[inline(always)]
    pub fn intf(&mut self) -> INTF_W<11> {
        INTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
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
