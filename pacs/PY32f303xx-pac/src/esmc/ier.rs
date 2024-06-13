#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDIE` reader - desc CMDIE"]
pub type CMDIE_R = crate::BitReader<bool>;
#[doc = "Field `CMDIE` writer - desc CMDIE"]
pub type CMDIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, IER_SPEC, bool, O>;
#[doc = "Field `FIFOEIE` reader - desc FIFOEIE"]
pub type FIFOEIE_R = crate::BitReader<bool>;
#[doc = "Field `FIFOEIE` writer - desc FIFOEIE"]
pub type FIFOEIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, IER_SPEC, bool, O>;
#[doc = "Field `FIFOHIE` reader - desc FIFOHIE"]
pub type FIFOHIE_R = crate::BitReader<bool>;
#[doc = "Field `FIFOHIE` writer - desc FIFOHIE"]
pub type FIFOHIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, IER_SPEC, bool, O>;
#[doc = "Field `FIFOFIE` reader - desc FIFOFIE"]
pub type FIFOFIE_R = crate::BitReader<bool>;
#[doc = "Field `FIFOFIE` writer - desc FIFOFIE"]
pub type FIFOFIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, IER_SPEC, bool, O>;
#[doc = "Field `IDLEIE` reader - desc IDLEIE"]
pub type IDLEIE_R = crate::BitReader<bool>;
#[doc = "Field `IDLEIE` writer - desc IDLEIE"]
pub type IDLEIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, IER_SPEC, bool, O>;
#[doc = "Field `DATAWAITIE` reader - desc DATAWAITIE"]
pub type DATAWAITIE_R = crate::BitReader<bool>;
#[doc = "Field `DATAWAITIE` writer - desc DATAWAITIE"]
pub type DATAWAITIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, IER_SPEC, bool, O>;
#[doc = "Field `FIFOOIE` reader - desc FIFOOIE"]
pub type FIFOOIE_R = crate::BitReader<bool>;
#[doc = "Field `FIFOOIE` writer - desc FIFOOIE"]
pub type FIFOOIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, IER_SPEC, bool, O>;
#[doc = "Field `ADDRDONEIE` reader - desc ADDRDONEIE"]
pub type ADDRDONEIE_R = crate::BitReader<bool>;
#[doc = "Field `ADDRDONEIE` writer - desc ADDRDONEIE"]
pub type ADDRDONEIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc CMDIE"]
    #[inline(always)]
    pub fn cmdie(&self) -> CMDIE_R {
        CMDIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc FIFOEIE"]
    #[inline(always)]
    pub fn fifoeie(&self) -> FIFOEIE_R {
        FIFOEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc FIFOHIE"]
    #[inline(always)]
    pub fn fifohie(&self) -> FIFOHIE_R {
        FIFOHIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc FIFOFIE"]
    #[inline(always)]
    pub fn fifofie(&self) -> FIFOFIE_R {
        FIFOFIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc IDLEIE"]
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc DATAWAITIE"]
    #[inline(always)]
    pub fn datawaitie(&self) -> DATAWAITIE_R {
        DATAWAITIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc FIFOOIE"]
    #[inline(always)]
    pub fn fifooie(&self) -> FIFOOIE_R {
        FIFOOIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc ADDRDONEIE"]
    #[inline(always)]
    pub fn addrdoneie(&self) -> ADDRDONEIE_R {
        ADDRDONEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CMDIE"]
    #[inline(always)]
    pub fn cmdie(&mut self) -> CMDIE_W<0> {
        CMDIE_W::new(self)
    }
    #[doc = "Bit 1 - desc FIFOEIE"]
    #[inline(always)]
    pub fn fifoeie(&mut self) -> FIFOEIE_W<1> {
        FIFOEIE_W::new(self)
    }
    #[doc = "Bit 2 - desc FIFOHIE"]
    #[inline(always)]
    pub fn fifohie(&mut self) -> FIFOHIE_W<2> {
        FIFOHIE_W::new(self)
    }
    #[doc = "Bit 3 - desc FIFOFIE"]
    #[inline(always)]
    pub fn fifofie(&mut self) -> FIFOFIE_W<3> {
        FIFOFIE_W::new(self)
    }
    #[doc = "Bit 4 - desc IDLEIE"]
    #[inline(always)]
    pub fn idleie(&mut self) -> IDLEIE_W<4> {
        IDLEIE_W::new(self)
    }
    #[doc = "Bit 5 - desc DATAWAITIE"]
    #[inline(always)]
    pub fn datawaitie(&mut self) -> DATAWAITIE_W<5> {
        DATAWAITIE_W::new(self)
    }
    #[doc = "Bit 6 - desc FIFOOIE"]
    #[inline(always)]
    pub fn fifooie(&mut self) -> FIFOOIE_W<6> {
        FIFOOIE_W::new(self)
    }
    #[doc = "Bit 7 - desc ADDRDONEIE"]
    #[inline(always)]
    pub fn addrdoneie(&mut self) -> ADDRDONEIE_W<7> {
        ADDRDONEIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
