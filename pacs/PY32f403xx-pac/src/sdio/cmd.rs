#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDINDEX` reader - desc CMDINDEX"]
pub type CMDINDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDINDEX` writer - desc CMDINDEX"]
pub type CMDINDEX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD_SPEC, u8, u8, 6, O>;
#[doc = "Field `WAITRESP` reader - desc WAITRESP"]
pub type WAITRESP_R = crate::BitReader<bool>;
#[doc = "Field `WAITRESP` writer - desc WAITRESP"]
pub type WAITRESP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `RESPLEN` reader - desc RESPLEN"]
pub type RESPLEN_R = crate::BitReader<bool>;
#[doc = "Field `RESPLEN` writer - desc RESPLEN"]
pub type RESPLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `CHECKRESPCRC` reader - desc CHECKRESPCRC"]
pub type CHECKRESPCRC_R = crate::BitReader<bool>;
#[doc = "Field `CHECKRESPCRC` writer - desc CHECKRESPCRC"]
pub type CHECKRESPCRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `DEXPECT` reader - desc DEXPECT"]
pub type DEXPECT_R = crate::BitReader<bool>;
#[doc = "Field `DEXPECT` writer - desc DEXPECT"]
pub type DEXPECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `DIR` reader - desc DIR"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - desc DIR"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `DTMODE` reader - desc DTMODE"]
pub type DTMODE_R = crate::BitReader<bool>;
#[doc = "Field `DTMODE` writer - desc DTMODE"]
pub type DTMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `AUTOSTOP` reader - desc AUTOSTOP"]
pub type AUTOSTOP_R = crate::BitReader<bool>;
#[doc = "Field `AUTOSTOP` writer - desc AUTOSTOP"]
pub type AUTOSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `WAITPEND` reader - desc WAITPEND"]
pub type WAITPEND_R = crate::BitReader<bool>;
#[doc = "Field `WAITPEND` writer - desc WAITPEND"]
pub type WAITPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `ABORTCMD` reader - desc ABORTCMD"]
pub type ABORTCMD_R = crate::BitReader<bool>;
#[doc = "Field `ABORTCMD` writer - desc ABORTCMD"]
pub type ABORTCMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `AUTOINIT` reader - desc AUTOINIT"]
pub type AUTOINIT_R = crate::BitReader<bool>;
#[doc = "Field `AUTOINIT` writer - desc AUTOINIT"]
pub type AUTOINIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `REGSYNC` reader - desc REGSYNC"]
pub type REGSYNC_R = crate::BitReader<bool>;
#[doc = "Field `REGSYNC` writer - desc REGSYNC"]
pub type REGSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `ATACMD` reader - desc ATACMD"]
pub type ATACMD_R = crate::BitReader<bool>;
#[doc = "Field `ATACMD` writer - desc ATACMD"]
pub type ATACMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `IEN` reader - desc IEN"]
pub type IEN_R = crate::BitReader<bool>;
#[doc = "Field `IEN` writer - desc IEN"]
pub type IEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `BOOTEN` reader - desc BOOTEN"]
pub type BOOTEN_R = crate::BitReader<bool>;
#[doc = "Field `BOOTEN` writer - desc BOOTEN"]
pub type BOOTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `BOOTACK` reader - desc BOOTACK"]
pub type BOOTACK_R = crate::BitReader<bool>;
#[doc = "Field `BOOTACK` writer - desc BOOTACK"]
pub type BOOTACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `BOOTDIS` reader - desc BOOTDIS"]
pub type BOOTDIS_R = crate::BitReader<bool>;
#[doc = "Field `BOOTDIS` writer - desc BOOTDIS"]
pub type BOOTDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `BOOTMODE` reader - desc BOOTMODE"]
pub type BOOTMODE_R = crate::BitReader<bool>;
#[doc = "Field `BOOTMODE` writer - desc BOOTMODE"]
pub type BOOTMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `STARTCMD` reader - desc STARTCMD"]
pub type STARTCMD_R = crate::BitReader<bool>;
#[doc = "Field `STARTCMD` writer - desc STARTCMD"]
pub type STARTCMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - desc CMDINDEX"]
    #[inline(always)]
    pub fn cmdindex(&self) -> CMDINDEX_R {
        CMDINDEX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - desc WAITRESP"]
    #[inline(always)]
    pub fn waitresp(&self) -> WAITRESP_R {
        WAITRESP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc RESPLEN"]
    #[inline(always)]
    pub fn resplen(&self) -> RESPLEN_R {
        RESPLEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc CHECKRESPCRC"]
    #[inline(always)]
    pub fn checkrespcrc(&self) -> CHECKRESPCRC_R {
        CHECKRESPCRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc DEXPECT"]
    #[inline(always)]
    pub fn dexpect(&self) -> DEXPECT_R {
        DEXPECT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc DTMODE"]
    #[inline(always)]
    pub fn dtmode(&self) -> DTMODE_R {
        DTMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc AUTOSTOP"]
    #[inline(always)]
    pub fn autostop(&self) -> AUTOSTOP_R {
        AUTOSTOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc WAITPEND"]
    #[inline(always)]
    pub fn waitpend(&self) -> WAITPEND_R {
        WAITPEND_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc ABORTCMD"]
    #[inline(always)]
    pub fn abortcmd(&self) -> ABORTCMD_R {
        ABORTCMD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc AUTOINIT"]
    #[inline(always)]
    pub fn autoinit(&self) -> AUTOINIT_R {
        AUTOINIT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - desc REGSYNC"]
    #[inline(always)]
    pub fn regsync(&self) -> REGSYNC_R {
        REGSYNC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc ATACMD"]
    #[inline(always)]
    pub fn atacmd(&self) -> ATACMD_R {
        ATACMD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc IEN"]
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - desc BOOTEN"]
    #[inline(always)]
    pub fn booten(&self) -> BOOTEN_R {
        BOOTEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc BOOTACK"]
    #[inline(always)]
    pub fn bootack(&self) -> BOOTACK_R {
        BOOTACK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - desc BOOTDIS"]
    #[inline(always)]
    pub fn bootdis(&self) -> BOOTDIS_R {
        BOOTDIS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - desc BOOTMODE"]
    #[inline(always)]
    pub fn bootmode(&self) -> BOOTMODE_R {
        BOOTMODE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - desc STARTCMD"]
    #[inline(always)]
    pub fn startcmd(&self) -> STARTCMD_R {
        STARTCMD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - desc CMDINDEX"]
    #[inline(always)]
    pub fn cmdindex(&mut self) -> CMDINDEX_W<0> {
        CMDINDEX_W::new(self)
    }
    #[doc = "Bit 6 - desc WAITRESP"]
    #[inline(always)]
    pub fn waitresp(&mut self) -> WAITRESP_W<6> {
        WAITRESP_W::new(self)
    }
    #[doc = "Bit 7 - desc RESPLEN"]
    #[inline(always)]
    pub fn resplen(&mut self) -> RESPLEN_W<7> {
        RESPLEN_W::new(self)
    }
    #[doc = "Bit 8 - desc CHECKRESPCRC"]
    #[inline(always)]
    pub fn checkrespcrc(&mut self) -> CHECKRESPCRC_W<8> {
        CHECKRESPCRC_W::new(self)
    }
    #[doc = "Bit 9 - desc DEXPECT"]
    #[inline(always)]
    pub fn dexpect(&mut self) -> DEXPECT_W<9> {
        DEXPECT_W::new(self)
    }
    #[doc = "Bit 10 - desc DIR"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<10> {
        DIR_W::new(self)
    }
    #[doc = "Bit 11 - desc DTMODE"]
    #[inline(always)]
    pub fn dtmode(&mut self) -> DTMODE_W<11> {
        DTMODE_W::new(self)
    }
    #[doc = "Bit 12 - desc AUTOSTOP"]
    #[inline(always)]
    pub fn autostop(&mut self) -> AUTOSTOP_W<12> {
        AUTOSTOP_W::new(self)
    }
    #[doc = "Bit 13 - desc WAITPEND"]
    #[inline(always)]
    pub fn waitpend(&mut self) -> WAITPEND_W<13> {
        WAITPEND_W::new(self)
    }
    #[doc = "Bit 14 - desc ABORTCMD"]
    #[inline(always)]
    pub fn abortcmd(&mut self) -> ABORTCMD_W<14> {
        ABORTCMD_W::new(self)
    }
    #[doc = "Bit 15 - desc AUTOINIT"]
    #[inline(always)]
    pub fn autoinit(&mut self) -> AUTOINIT_W<15> {
        AUTOINIT_W::new(self)
    }
    #[doc = "Bit 21 - desc REGSYNC"]
    #[inline(always)]
    pub fn regsync(&mut self) -> REGSYNC_W<21> {
        REGSYNC_W::new(self)
    }
    #[doc = "Bit 22 - desc ATACMD"]
    #[inline(always)]
    pub fn atacmd(&mut self) -> ATACMD_W<22> {
        ATACMD_W::new(self)
    }
    #[doc = "Bit 23 - desc IEN"]
    #[inline(always)]
    pub fn ien(&mut self) -> IEN_W<23> {
        IEN_W::new(self)
    }
    #[doc = "Bit 24 - desc BOOTEN"]
    #[inline(always)]
    pub fn booten(&mut self) -> BOOTEN_W<24> {
        BOOTEN_W::new(self)
    }
    #[doc = "Bit 25 - desc BOOTACK"]
    #[inline(always)]
    pub fn bootack(&mut self) -> BOOTACK_W<25> {
        BOOTACK_W::new(self)
    }
    #[doc = "Bit 26 - desc BOOTDIS"]
    #[inline(always)]
    pub fn bootdis(&mut self) -> BOOTDIS_W<26> {
        BOOTDIS_W::new(self)
    }
    #[doc = "Bit 27 - desc BOOTMODE"]
    #[inline(always)]
    pub fn bootmode(&mut self) -> BOOTMODE_W<27> {
        BOOTMODE_W::new(self)
    }
    #[doc = "Bit 31 - desc STARTCMD"]
    #[inline(always)]
    pub fn startcmd(&mut self) -> STARTCMD_W<31> {
        STARTCMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CMD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
