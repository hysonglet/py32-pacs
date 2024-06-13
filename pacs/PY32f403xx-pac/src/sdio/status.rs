#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXWMARK` reader - desc RXWMARK"]
pub type RXWMARK_R = crate::BitReader<bool>;
#[doc = "Field `TXWMARK` reader - desc TXWMARK"]
pub type TXWMARK_R = crate::BitReader<bool>;
#[doc = "Field `FIFOE` reader - desc FIFOE"]
pub type FIFOE_R = crate::BitReader<bool>;
#[doc = "Field `FIFOF` reader - desc FIFOF"]
pub type FIFOF_R = crate::BitReader<bool>;
#[doc = "Field `CMDFSM` reader - desc CMDFSM"]
pub type CMDFSM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CARDPRESENT` reader - desc CARDPRESENT"]
pub type CARDPRESENT_R = crate::BitReader<bool>;
#[doc = "Field `CARDBSY` reader - desc CARDBSY"]
pub type CARDBSY_R = crate::BitReader<bool>;
#[doc = "Field `FIFOCNT` reader - desc FIFOCNT"]
pub type FIFOCNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - desc RXWMARK"]
    #[inline(always)]
    pub fn rxwmark(&self) -> RXWMARK_R {
        RXWMARK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TXWMARK"]
    #[inline(always)]
    pub fn txwmark(&self) -> TXWMARK_R {
        TXWMARK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc FIFOE"]
    #[inline(always)]
    pub fn fifoe(&self) -> FIFOE_R {
        FIFOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc FIFOF"]
    #[inline(always)]
    pub fn fifof(&self) -> FIFOF_R {
        FIFOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - desc CMDFSM"]
    #[inline(always)]
    pub fn cmdfsm(&self) -> CMDFSM_R {
        CMDFSM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - desc CARDPRESENT"]
    #[inline(always)]
    pub fn cardpresent(&self) -> CARDPRESENT_R {
        CARDPRESENT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc CARDBSY"]
    #[inline(always)]
    pub fn cardbsy(&self) -> CARDBSY_R {
        CARDBSY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 17:29 - desc FIFOCNT"]
    #[inline(always)]
    pub fn fifocnt(&self) -> FIFOCNT_R {
        FIFOCNT_R::new(((self.bits >> 17) & 0x1fff) as u16)
    }
}
#[doc = "desc STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0x06"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}
