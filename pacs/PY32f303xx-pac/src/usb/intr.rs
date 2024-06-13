#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SUSPEND` reader - desc SUSPEND"]
pub type SUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `RESUME` reader - desc RESUME"]
pub type RESUME_R = crate::BitReader<bool>;
#[doc = "Field `RESET` reader - desc RESET"]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `SOF` reader - desc SOF"]
pub type SOF_R = crate::BitReader<bool>;
#[doc = "Field `EP1OUT` reader - desc EP1OUT"]
pub type EP1OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP2OUT` reader - desc EP2OUT"]
pub type EP2OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP3OUT` reader - desc EP3OUT"]
pub type EP3OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP4OUT` reader - desc EP4OUT"]
pub type EP4OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP5OUT` reader - desc EP5OUT"]
pub type EP5OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP6OUT` reader - desc EP4OUT"]
pub type EP6OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP7OUT` reader - desc EP4OUT"]
pub type EP7OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP0` reader - desc EP0"]
pub type EP0_R = crate::BitReader<bool>;
#[doc = "Field `EP1IN` reader - desc EP1IN"]
pub type EP1IN_R = crate::BitReader<bool>;
#[doc = "Field `EP2IN` reader - desc EP2IN"]
pub type EP2IN_R = crate::BitReader<bool>;
#[doc = "Field `EP3IN` reader - desc EP3IN"]
pub type EP3IN_R = crate::BitReader<bool>;
#[doc = "Field `EP4IN` reader - desc EP4IN"]
pub type EP4IN_R = crate::BitReader<bool>;
#[doc = "Field `EP5IN` reader - desc EP5IN"]
pub type EP5IN_R = crate::BitReader<bool>;
#[doc = "Field `EP6IN` reader - desc EP6IN"]
pub type EP6IN_R = crate::BitReader<bool>;
#[doc = "Field `EP7IN` reader - desc EP7IN"]
pub type EP7IN_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - desc SUSPEND"]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RESUME"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc RESET"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc SOF"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - desc EP1OUT"]
    #[inline(always)]
    pub fn ep1out(&self) -> EP1OUT_R {
        EP1OUT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc EP2OUT"]
    #[inline(always)]
    pub fn ep2out(&self) -> EP2OUT_R {
        EP2OUT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc EP3OUT"]
    #[inline(always)]
    pub fn ep3out(&self) -> EP3OUT_R {
        EP3OUT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc EP4OUT"]
    #[inline(always)]
    pub fn ep4out(&self) -> EP4OUT_R {
        EP4OUT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc EP5OUT"]
    #[inline(always)]
    pub fn ep5out(&self) -> EP5OUT_R {
        EP5OUT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc EP4OUT"]
    #[inline(always)]
    pub fn ep6out(&self) -> EP6OUT_R {
        EP6OUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc EP4OUT"]
    #[inline(always)]
    pub fn ep7out(&self) -> EP7OUT_R {
        EP7OUT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc EP0"]
    #[inline(always)]
    pub fn ep0(&self) -> EP0_R {
        EP0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc EP1IN"]
    #[inline(always)]
    pub fn ep1in(&self) -> EP1IN_R {
        EP1IN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc EP2IN"]
    #[inline(always)]
    pub fn ep2in(&self) -> EP2IN_R {
        EP2IN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc EP3IN"]
    #[inline(always)]
    pub fn ep3in(&self) -> EP3IN_R {
        EP3IN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc EP4IN"]
    #[inline(always)]
    pub fn ep4in(&self) -> EP4IN_R {
        EP4IN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc EP5IN"]
    #[inline(always)]
    pub fn ep5in(&self) -> EP5IN_R {
        EP5IN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc EP6IN"]
    #[inline(always)]
    pub fn ep6in(&self) -> EP6IN_R {
        EP6IN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc EP7IN"]
    #[inline(always)]
    pub fn ep7in(&self) -> EP7IN_R {
        EP7IN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "INTR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
