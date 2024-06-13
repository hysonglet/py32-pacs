#[doc = "Register `CR0` reader"]
pub struct R(crate::R<CR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR0` writer"]
pub struct W(crate::W<CR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR0_SPEC>;
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
impl From<crate::W<CR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OP1OEN1` reader - OP1OEN1"]
pub type OP1OEN1_R = crate::BitReader<bool>;
#[doc = "Field `OP1OEN1` writer - OP1OEN1"]
pub type OP1OEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `OP2OEN1` reader - OP2OEN1"]
pub type OP2OEN1_R = crate::BitReader<bool>;
#[doc = "Field `OP2OEN1` writer - OP2OEN1"]
pub type OP2OEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - OP1OEN1"]
    #[inline(always)]
    pub fn op1oen1(&self) -> OP1OEN1_R {
        OP1OEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - OP2OEN1"]
    #[inline(always)]
    pub fn op2oen1(&self) -> OP2OEN1_R {
        OP2OEN1_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - OP1OEN1"]
    #[inline(always)]
    pub fn op1oen1(&mut self) -> OP1OEN1_W<1> {
        OP1OEN1_W::new(self)
    }
    #[doc = "Bit 6 - OP2OEN1"]
    #[inline(always)]
    pub fn op2oen1(&mut self) -> OP2OEN1_W<6> {
        OP2OEN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CR0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr0](index.html) module"]
pub struct CR0_SPEC;
impl crate::RegisterSpec for CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr0::R](R) reader structure"]
impl crate::Readable for CR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr0::W](W) writer structure"]
impl crate::Writable for CR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR0 to value 0xffff_ffff"]
impl crate::Resettable for CR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
