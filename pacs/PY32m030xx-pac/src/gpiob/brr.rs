#[doc = "Register `BRR` writer"]
pub struct W(crate::W<BRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRR_SPEC>;
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
impl From<crate::W<BRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BR0` writer - Port Reset bit"]
pub type BR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR1` writer - Port Reset bit"]
pub type BR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR2` writer - Port Reset bit"]
pub type BR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR3` writer - Port Reset bit"]
pub type BR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR4` writer - Port Reset bit"]
pub type BR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR5` writer - Port Reset bit"]
pub type BR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR6` writer - Port Reset bit"]
pub type BR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR7` writer - Port Reset bit"]
pub type BR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR8` writer - Port Reset bit"]
pub type BR8_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Port Reset bit"]
    #[inline(always)]
    pub fn br0(&mut self) -> BR0_W<0> {
        BR0_W::new(self)
    }
    #[doc = "Bit 1 - Port Reset bit"]
    #[inline(always)]
    pub fn br1(&mut self) -> BR1_W<1> {
        BR1_W::new(self)
    }
    #[doc = "Bit 2 - Port Reset bit"]
    #[inline(always)]
    pub fn br2(&mut self) -> BR2_W<2> {
        BR2_W::new(self)
    }
    #[doc = "Bit 3 - Port Reset bit"]
    #[inline(always)]
    pub fn br3(&mut self) -> BR3_W<3> {
        BR3_W::new(self)
    }
    #[doc = "Bit 4 - Port Reset bit"]
    #[inline(always)]
    pub fn br4(&mut self) -> BR4_W<4> {
        BR4_W::new(self)
    }
    #[doc = "Bit 5 - Port Reset bit"]
    #[inline(always)]
    pub fn br5(&mut self) -> BR5_W<5> {
        BR5_W::new(self)
    }
    #[doc = "Bit 6 - Port Reset bit"]
    #[inline(always)]
    pub fn br6(&mut self) -> BR6_W<6> {
        BR6_W::new(self)
    }
    #[doc = "Bit 7 - Port Reset bit"]
    #[inline(always)]
    pub fn br7(&mut self) -> BR7_W<7> {
        BR7_W::new(self)
    }
    #[doc = "Bit 8 - Port Reset bit"]
    #[inline(always)]
    pub fn br8(&mut self) -> BR8_W<8> {
        BR8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "port bit reset register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brr](index.html) module"]
pub struct BRR_SPEC;
impl crate::RegisterSpec for BRR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [brr::W](W) writer structure"]
impl crate::Writable for BRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
