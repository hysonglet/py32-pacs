#[doc = "Register `TTCR` reader"]
pub struct R(crate::R<TTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTCR` writer"]
pub struct W(crate::W<TTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTCR_SPEC>;
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
impl From<crate::W<TTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TTPTR` reader - desc TTPTR"]
pub type TTPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTPTR` writer - desc TTPTR"]
pub type TTPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTCR_SPEC, u8, u8, 6, O>;
#[doc = "Field `TTYPE` reader - desc TTYPE"]
pub type TTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTYPE` writer - desc TTYPE"]
pub type TTYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `TEW` reader - desc TEW"]
pub type TEW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEW` writer - desc TEW"]
pub type TEW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `TBPTR` reader - desc TBPTR"]
pub type TBPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TBPTR` writer - desc TBPTR"]
pub type TBPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTCR_SPEC, u8, u8, 6, O>;
#[doc = "Field `TBF` reader - desc TBF"]
pub type TBF_R = crate::BitReader<bool>;
#[doc = "Field `TBF` writer - desc TBF"]
pub type TBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTCR_SPEC, bool, O>;
#[doc = "Field `TBE` reader - desc TBE"]
pub type TBE_R = crate::BitReader<bool>;
#[doc = "Field `TBE` writer - desc TBE"]
pub type TBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTCR_SPEC, bool, O>;
#[doc = "Field `TTEN` reader - desc TTEN"]
pub type TTEN_R = crate::BitReader<bool>;
#[doc = "Field `TTEN` writer - desc TTEN"]
pub type TTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTCR_SPEC, bool, O>;
#[doc = "Field `T_PRESC` reader - desc T_PRESC"]
pub type T_PRESC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_PRESC` writer - desc T_PRESC"]
pub type T_PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:5 - desc TTPTR"]
    #[inline(always)]
    pub fn ttptr(&self) -> TTPTR_R {
        TTPTR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:10 - desc TTYPE"]
    #[inline(always)]
    pub fn ttype(&self) -> TTYPE_R {
        TTYPE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:15 - desc TEW"]
    #[inline(always)]
    pub fn tew(&self) -> TEW_R {
        TEW_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - desc TBPTR"]
    #[inline(always)]
    pub fn tbptr(&self) -> TBPTR_R {
        TBPTR_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - desc TBF"]
    #[inline(always)]
    pub fn tbf(&self) -> TBF_R {
        TBF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc TBE"]
    #[inline(always)]
    pub fn tbe(&self) -> TBE_R {
        TBE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - desc TTEN"]
    #[inline(always)]
    pub fn tten(&self) -> TTEN_R {
        TTEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - desc T_PRESC"]
    #[inline(always)]
    pub fn t_presc(&self) -> T_PRESC_R {
        T_PRESC_R::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - desc TTPTR"]
    #[inline(always)]
    pub fn ttptr(&mut self) -> TTPTR_W<0> {
        TTPTR_W::new(self)
    }
    #[doc = "Bits 8:10 - desc TTYPE"]
    #[inline(always)]
    pub fn ttype(&mut self) -> TTYPE_W<8> {
        TTYPE_W::new(self)
    }
    #[doc = "Bits 12:15 - desc TEW"]
    #[inline(always)]
    pub fn tew(&mut self) -> TEW_W<12> {
        TEW_W::new(self)
    }
    #[doc = "Bits 16:21 - desc TBPTR"]
    #[inline(always)]
    pub fn tbptr(&mut self) -> TBPTR_W<16> {
        TBPTR_W::new(self)
    }
    #[doc = "Bit 22 - desc TBF"]
    #[inline(always)]
    pub fn tbf(&mut self) -> TBF_W<22> {
        TBF_W::new(self)
    }
    #[doc = "Bit 23 - desc TBE"]
    #[inline(always)]
    pub fn tbe(&mut self) -> TBE_W<23> {
        TBE_W::new(self)
    }
    #[doc = "Bit 24 - desc TTEN"]
    #[inline(always)]
    pub fn tten(&mut self) -> TTEN_W<24> {
        TTEN_W::new(self)
    }
    #[doc = "Bits 25:26 - desc T_PRESC"]
    #[inline(always)]
    pub fn t_presc(&mut self) -> T_PRESC_W<25> {
        T_PRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc TTCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttcr](index.html) module"]
pub struct TTCR_SPEC;
impl crate::RegisterSpec for TTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttcr::R](R) reader structure"]
impl crate::Readable for TTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ttcr::W](W) writer structure"]
impl crate::Writable for TTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TTCR to value 0"]
impl crate::Resettable for TTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
