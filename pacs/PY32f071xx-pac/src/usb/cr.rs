#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADD` reader - ADD"]
pub type ADD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADD` writer - ADD"]
pub type ADD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 7, O>;
#[doc = "Field `UPDATE` reader - UPDATE"]
pub type UPDATE_R = crate::BitReader<bool>;
#[doc = "Field `UPDATE` writer - UPDATE"]
pub type UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `Enable_Suspend` reader - Enable_Suspend"]
pub type ENABLE_SUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `Enable_Suspend` writer - Enable_Suspend"]
pub type ENABLE_SUSPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `Suspend_Mode` reader - Suspend_Mode"]
pub type SUSPEND_MODE_R = crate::BitReader<bool>;
#[doc = "Field `Suspend_Mode` writer - Suspend_Mode"]
pub type SUSPEND_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `Resume` reader - Resume"]
pub type RESUME_R = crate::BitReader<bool>;
#[doc = "Field `Resume` writer - Resume"]
pub type RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `Reset` reader - Reset"]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `Reset` writer - Reset"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ISO_Update` reader - ISO_Update"]
pub type ISO_UPDATE_R = crate::BitReader<bool>;
#[doc = "Field `ISO_Update` writer - ISO_Update"]
pub type ISO_UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - ADD"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - UPDATE"]
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable_Suspend"]
    #[inline(always)]
    pub fn enable_suspend(&self) -> ENABLE_SUSPEND_R {
        ENABLE_SUSPEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Suspend_Mode"]
    #[inline(always)]
    pub fn suspend_mode(&self) -> SUSPEND_MODE_R {
        SUSPEND_MODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Resume"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reset"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - ISO_Update"]
    #[inline(always)]
    pub fn iso_update(&self) -> ISO_UPDATE_R {
        ISO_UPDATE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - ADD"]
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W<0> {
        ADD_W::new(self)
    }
    #[doc = "Bit 7 - UPDATE"]
    #[inline(always)]
    pub fn update(&mut self) -> UPDATE_W<7> {
        UPDATE_W::new(self)
    }
    #[doc = "Bit 8 - Enable_Suspend"]
    #[inline(always)]
    pub fn enable_suspend(&mut self) -> ENABLE_SUSPEND_W<8> {
        ENABLE_SUSPEND_W::new(self)
    }
    #[doc = "Bit 9 - Suspend_Mode"]
    #[inline(always)]
    pub fn suspend_mode(&mut self) -> SUSPEND_MODE_W<9> {
        SUSPEND_MODE_W::new(self)
    }
    #[doc = "Bit 10 - Resume"]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W<10> {
        RESUME_W::new(self)
    }
    #[doc = "Bit 11 - Reset"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<11> {
        RESET_W::new(self)
    }
    #[doc = "Bit 15 - ISO_Update"]
    #[inline(always)]
    pub fn iso_update(&mut self) -> ISO_UPDATE_W<15> {
        ISO_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0xffff_ffff"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
