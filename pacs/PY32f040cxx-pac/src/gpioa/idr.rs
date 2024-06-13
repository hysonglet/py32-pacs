#[doc = "Register `IDR` reader"]
pub struct R(crate::R<IDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ID0` reader - Port input data (y=0-15)"]
pub type ID0_R = crate::BitReader<bool>;
#[doc = "Field `ID1` reader - Port input data (y=0-15)"]
pub type ID1_R = crate::BitReader<bool>;
#[doc = "Field `ID2` reader - Port input data (y=0-15)"]
pub type ID2_R = crate::BitReader<bool>;
#[doc = "Field `ID3` reader - Port input data (y=0-15)"]
pub type ID3_R = crate::BitReader<bool>;
#[doc = "Field `ID4` reader - Port input data (y=0-15)"]
pub type ID4_R = crate::BitReader<bool>;
#[doc = "Field `ID5` reader - Port input data (y=0-15)"]
pub type ID5_R = crate::BitReader<bool>;
#[doc = "Field `ID6` reader - Port input data (y=0-15)"]
pub type ID6_R = crate::BitReader<bool>;
#[doc = "Field `ID7` reader - Port input data (y=0-15)"]
pub type ID7_R = crate::BitReader<bool>;
#[doc = "Field `ID8` reader - Port input data (y=0-15)"]
pub type ID8_R = crate::BitReader<bool>;
#[doc = "Field `ID9` reader - Port input data (y=0-15)"]
pub type ID9_R = crate::BitReader<bool>;
#[doc = "Field `ID10` reader - Port input data (y=0-15)"]
pub type ID10_R = crate::BitReader<bool>;
#[doc = "Field `ID11` reader - Port input data (y=0-15)"]
pub type ID11_R = crate::BitReader<bool>;
#[doc = "Field `ID12` reader - Port input data (y=0-15)"]
pub type ID12_R = crate::BitReader<bool>;
#[doc = "Field `ID13` reader - Port input data (y=0-15)"]
pub type ID13_R = crate::BitReader<bool>;
#[doc = "Field `ID14` reader - Port input data (y=0-15)"]
pub type ID14_R = crate::BitReader<bool>;
#[doc = "Field `ID15` reader - Port input data (y=0-15)"]
pub type ID15_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Port input data (y=0-15)"]
    #[inline(always)]
    pub fn id0(&self) -> ID0_R {
        ID0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port input data (y=0-15)"]
    #[inline(always)]
    pub fn id1(&self) -> ID1_R {
        ID1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port input data (y=0-15)"]
    #[inline(always)]
    pub fn id2(&self) -> ID2_R {
        ID2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port input data (y=0-15)"]
    #[inline(always)]
    pub fn id3(&self) -> ID3_R {
        ID3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port input data (y=0-15)"]
    #[inline(always)]
    pub fn id4(&self) -> ID4_R {
        ID4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port input data (y=0-15)"]
    #[inline(always)]
    pub fn id5(&self) -> ID5_R {
        ID5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port input data (y=0-15)"]
    #[inline(always)]
    pub fn id6(&self) -> ID6_R {
        ID6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port input data (y=0-15)"]
    #[inline(always)]
    pub fn id7(&self) -> ID7_R {
        ID7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port input data (y=0-15)"]
    #[inline(always)]
    pub fn id8(&self) -> ID8_R {
        ID8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port input data (y=0-15)"]
    #[inline(always)]
    pub fn id9(&self) -> ID9_R {
        ID9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port input data (y=0-15)"]
    #[inline(always)]
    pub fn id10(&self) -> ID10_R {
        ID10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port input data (y=0-15)"]
    #[inline(always)]
    pub fn id11(&self) -> ID11_R {
        ID11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port input data (y=0-15)"]
    #[inline(always)]
    pub fn id12(&self) -> ID12_R {
        ID12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port input data (y=0-15)"]
    #[inline(always)]
    pub fn id13(&self) -> ID13_R {
        ID13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port input data (y=0-15)"]
    #[inline(always)]
    pub fn id14(&self) -> ID14_R {
        ID14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port input data (y=0-15)"]
    #[inline(always)]
    pub fn id15(&self) -> ID15_R {
        ID15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "GPIO port input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](index.html) module"]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idr::R](R) reader structure"]
impl crate::Readable for IDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
