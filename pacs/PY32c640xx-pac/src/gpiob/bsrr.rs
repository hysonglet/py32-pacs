#[doc = "Register `BSRR` writer"]
pub struct W(crate::W<BSRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSRR_SPEC>;
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
impl From<crate::W<BSRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BS0` writer - Port x set bit y (y= 0..15)"]
pub type BS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, bool, O>;
#[doc = "Field `BS1` writer - Port x set bit y (y= 0..15)"]
pub type BS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, bool, O>;
#[doc = "Field `BS2` writer - Port x set bit y (y= 0..15)"]
pub type BS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, bool, O>;
#[doc = "Field `BS3` writer - Port x set bit y (y= 0..15)"]
pub type BS3_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, bool, O>;
#[doc = "Field `BS4` writer - Port x set bit y (y= 0..15)"]
pub type BS4_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, bool, O>;
#[doc = "Field `BS5` writer - Port x set bit y (y= 0..15)"]
pub type BS5_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, bool, O>;
#[doc = "Field `BS6` writer - Port x set bit y (y= 0..15)"]
pub type BS6_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, bool, O>;
#[doc = "Field `BS7` writer - Port x set bit y (y= 0..15)"]
pub type BS7_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, bool, O>;
#[doc = "Field `BS8` writer - Port x set bit y (y= 0..15)"]
pub type BS8_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, bool, O>;
#[doc = "Field `BR0` writer - Port x set bit y (y= 0..15)"]
pub type BR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, bool, O>;
#[doc = "Field `BR1` writer - Port x reset bit y (y = 0..15)"]
pub type BR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, bool, O>;
#[doc = "Field `BR2` writer - Port x reset bit y (y = 0..15)"]
pub type BR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, bool, O>;
#[doc = "Field `BR3` writer - Port x reset bit y (y = 0..15)"]
pub type BR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, bool, O>;
#[doc = "Field `BR4` writer - Port x reset bit y (y = 0..15)"]
pub type BR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, bool, O>;
#[doc = "Field `BR5` writer - Port x reset bit y (y = 0..15)"]
pub type BR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, bool, O>;
#[doc = "Field `BR6` writer - Port x reset bit y (y = 0..15)"]
pub type BR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, bool, O>;
#[doc = "Field `BR7` writer - Port x reset bit y (y = 0..15)"]
pub type BR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, bool, O>;
#[doc = "Field `BR8` writer - Port x reset bit y (y = 0..15)"]
pub type BR8_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs0(&mut self) -> BS0_W<0> {
        BS0_W::new(self)
    }
    #[doc = "Bit 1 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs1(&mut self) -> BS1_W<1> {
        BS1_W::new(self)
    }
    #[doc = "Bit 2 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs2(&mut self) -> BS2_W<2> {
        BS2_W::new(self)
    }
    #[doc = "Bit 3 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs3(&mut self) -> BS3_W<3> {
        BS3_W::new(self)
    }
    #[doc = "Bit 4 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs4(&mut self) -> BS4_W<4> {
        BS4_W::new(self)
    }
    #[doc = "Bit 5 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs5(&mut self) -> BS5_W<5> {
        BS5_W::new(self)
    }
    #[doc = "Bit 6 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs6(&mut self) -> BS6_W<6> {
        BS6_W::new(self)
    }
    #[doc = "Bit 7 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs7(&mut self) -> BS7_W<7> {
        BS7_W::new(self)
    }
    #[doc = "Bit 8 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs8(&mut self) -> BS8_W<8> {
        BS8_W::new(self)
    }
    #[doc = "Bit 16 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn br0(&mut self) -> BR0_W<16> {
        BR0_W::new(self)
    }
    #[doc = "Bit 17 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br1(&mut self) -> BR1_W<17> {
        BR1_W::new(self)
    }
    #[doc = "Bit 18 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br2(&mut self) -> BR2_W<18> {
        BR2_W::new(self)
    }
    #[doc = "Bit 19 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br3(&mut self) -> BR3_W<19> {
        BR3_W::new(self)
    }
    #[doc = "Bit 20 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br4(&mut self) -> BR4_W<20> {
        BR4_W::new(self)
    }
    #[doc = "Bit 21 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br5(&mut self) -> BR5_W<21> {
        BR5_W::new(self)
    }
    #[doc = "Bit 22 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br6(&mut self) -> BR6_W<22> {
        BR6_W::new(self)
    }
    #[doc = "Bit 23 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br7(&mut self) -> BR7_W<23> {
        BR7_W::new(self)
    }
    #[doc = "Bit 24 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br8(&mut self) -> BR8_W<24> {
        BR8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port bit set/reset register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsrr](index.html) module"]
pub struct BSRR_SPEC;
impl crate::RegisterSpec for BSRR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [bsrr::W](W) writer structure"]
impl crate::Writable for BSRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BSRR to value 0"]
impl crate::Resettable for BSRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
