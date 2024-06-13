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
#[doc = "Field `ADD` reader - desc ADD"]
pub type ADD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADD` writer - desc ADD"]
pub type ADD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 7, O>;
#[doc = "Field `UPDATE` reader - desc UPDATE"]
pub type UPDATE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_SUSPEND` reader - desc ENABLE_SUSPEND"]
pub type ENABLE_SUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_SUSPEND` writer - desc ENABLE_SUSPEND"]
pub type ENABLE_SUSPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SUSPEND_MODE` reader - desc SUSPEND_MODE"]
pub type SUSPEND_MODE_R = crate::BitReader<bool>;
#[doc = "Field `RESUME` reader - desc RESUME"]
pub type RESUME_R = crate::BitReader<bool>;
#[doc = "Field `RESUME` writer - desc RESUME"]
pub type RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RESET` reader - desc RESET"]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `ISO_UPDATE` reader - desc ISO_UPDATE"]
pub type ISO_UPDATE_R = crate::BitReader<bool>;
#[doc = "Field `ISO_UPDATE` writer - desc ISO_UPDATE"]
pub type ISO_UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - desc ADD"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - desc UPDATE"]
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc ENABLE_SUSPEND"]
    #[inline(always)]
    pub fn enable_suspend(&self) -> ENABLE_SUSPEND_R {
        ENABLE_SUSPEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc SUSPEND_MODE"]
    #[inline(always)]
    pub fn suspend_mode(&self) -> SUSPEND_MODE_R {
        SUSPEND_MODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc RESUME"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc RESET"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - desc ISO_UPDATE"]
    #[inline(always)]
    pub fn iso_update(&self) -> ISO_UPDATE_R {
        ISO_UPDATE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - desc ADD"]
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W<0> {
        ADD_W::new(self)
    }
    #[doc = "Bit 8 - desc ENABLE_SUSPEND"]
    #[inline(always)]
    pub fn enable_suspend(&mut self) -> ENABLE_SUSPEND_W<8> {
        ENABLE_SUSPEND_W::new(self)
    }
    #[doc = "Bit 10 - desc RESUME"]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W<10> {
        RESUME_W::new(self)
    }
    #[doc = "Bit 15 - desc ISO_UPDATE"]
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
#[doc = "desc CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
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
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
