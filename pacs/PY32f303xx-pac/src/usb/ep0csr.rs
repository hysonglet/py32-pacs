#[doc = "Register `EP0CSR` writer"]
pub struct W(crate::W<EP0CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP0CSR_SPEC>;
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
impl From<crate::W<EP0CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP0CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPKTRDY` writer - desc INPKTRDY"]
pub type INPKTRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0CSR_SPEC, bool, O>;
#[doc = "Field `SENTSTALL` writer - desc SENTSTALL"]
pub type SENTSTALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0CSR_SPEC, bool, O>;
#[doc = "Field `DATAEND` writer - desc DATAEND"]
pub type DATAEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0CSR_SPEC, bool, O>;
#[doc = "Field `SENDSTALL` writer - desc SENDSTALL"]
pub type SENDSTALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0CSR_SPEC, bool, O>;
#[doc = "Field `SERVICEDOUTPKTRDY` writer - desc SERVICEDOUTPKTRDY"]
pub type SERVICEDOUTPKTRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0CSR_SPEC, bool, O>;
#[doc = "Field `SERVICEDSETUPEND` writer - desc SERVICEDSETUPEND"]
pub type SERVICEDSETUPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0CSR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 1 - desc INPKTRDY"]
    #[inline(always)]
    pub fn inpktrdy(&mut self) -> INPKTRDY_W<1> {
        INPKTRDY_W::new(self)
    }
    #[doc = "Bit 2 - desc SENTSTALL"]
    #[inline(always)]
    pub fn sentstall(&mut self) -> SENTSTALL_W<2> {
        SENTSTALL_W::new(self)
    }
    #[doc = "Bit 3 - desc DATAEND"]
    #[inline(always)]
    pub fn dataend(&mut self) -> DATAEND_W<3> {
        DATAEND_W::new(self)
    }
    #[doc = "Bit 5 - desc SENDSTALL"]
    #[inline(always)]
    pub fn sendstall(&mut self) -> SENDSTALL_W<5> {
        SENDSTALL_W::new(self)
    }
    #[doc = "Bit 6 - desc SERVICEDOUTPKTRDY"]
    #[inline(always)]
    pub fn servicedoutpktrdy(&mut self) -> SERVICEDOUTPKTRDY_W<6> {
        SERVICEDOUTPKTRDY_W::new(self)
    }
    #[doc = "Bit 7 - desc SERVICEDSETUPEND"]
    #[inline(always)]
    pub fn servicedsetupend(&mut self) -> SERVICEDSETUPEND_W<7> {
        SERVICEDSETUPEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc EP0CSR\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep0csr](index.html) module"]
pub struct EP0CSR_SPEC;
impl crate::RegisterSpec for EP0CSR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ep0csr::W](W) writer structure"]
impl crate::Writable for EP0CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EP0CSR to value 0"]
impl crate::Resettable for EP0CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
