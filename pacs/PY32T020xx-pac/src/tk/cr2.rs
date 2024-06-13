#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - "]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - "]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `RSTB` reader - "]
pub type RSTB_R = crate::BitReader<bool>;
#[doc = "Field `RSTB` writer - "]
pub type RSTB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `SWCTRL` reader - "]
pub type SWCTRL_R = crate::BitReader<bool>;
#[doc = "Field `SWCTRL` writer - "]
pub type SWCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `CMODCHRG` reader - "]
pub type CMODCHRG_R = crate::BitReader<bool>;
#[doc = "Field `CMODCHRG` writer - "]
pub type CMODCHRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `CMODDISCH` reader - "]
pub type CMODDISCH_R = crate::BitReader<bool>;
#[doc = "Field `CMODDISCH` writer - "]
pub type CMODDISCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `MIDACEN` reader - "]
pub type MIDACEN_R = crate::BitReader<bool>;
#[doc = "Field `MIDACEN` writer - "]
pub type MIDACEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `SIDACEN` reader - "]
pub type SIDACEN_R = crate::BitReader<bool>;
#[doc = "Field `SIDACEN` writer - "]
pub type SIDACEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `CMODVTINIT` reader - "]
pub type CMODVTINIT_R = crate::BitReader<bool>;
#[doc = "Field `CMODVTINIT` writer - "]
pub type CMODVTINIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `PWMM` reader - "]
pub type PWMM_R = crate::BitReader<bool>;
#[doc = "Field `PWMM` writer - "]
pub type PWMM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `CTRLSOURCE` reader - "]
pub type CTRLSOURCE_R = crate::BitReader<bool>;
#[doc = "Field `CTRLSOURCE` writer - "]
pub type CTRLSOURCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `LPABNORMAL` reader - "]
pub type LPABNORMAL_R = crate::BitReader<bool>;
#[doc = "Field `LPABNORMAL` writer - "]
pub type LPABNORMAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `CSA_DISCH_SEL` reader - "]
pub type CSA_DISCH_SEL_R = crate::BitReader<bool>;
#[doc = "Field `CSA_DISCH_SEL` writer - "]
pub type CSA_DISCH_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `SHORTSW_NUM` reader - "]
pub type SHORTSW_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHORTSW_NUM` writer - "]
pub type SHORTSW_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `LPEN` reader - "]
pub type LPEN_R = crate::BitReader<bool>;
#[doc = "Field `LPEN` writer - "]
pub type LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `KVREF` reader - "]
pub type KVREF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KVREF` writer - "]
pub type KVREF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `KVREF_CSA_TARGET` reader - "]
pub type KVREF_CSA_TARGET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KVREF_CSA_TARGET` writer - "]
pub type KVREF_CSA_TARGET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rstb(&self) -> RSTB_R {
        RSTB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn swctrl(&self) -> SWCTRL_R {
        SWCTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cmodchrg(&self) -> CMODCHRG_R {
        CMODCHRG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cmoddisch(&self) -> CMODDISCH_R {
        CMODDISCH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn midacen(&self) -> MIDACEN_R {
        MIDACEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sidacen(&self) -> SIDACEN_R {
        SIDACEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cmodvtinit(&self) -> CMODVTINIT_R {
        CMODVTINIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pwmm(&self) -> PWMM_R {
        PWMM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ctrlsource(&self) -> CTRLSOURCE_R {
        CTRLSOURCE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn lpabnormal(&self) -> LPABNORMAL_R {
        LPABNORMAL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn csa_disch_sel(&self) -> CSA_DISCH_SEL_R {
        CSA_DISCH_SEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn shortsw_num(&self) -> SHORTSW_NUM_R {
        SHORTSW_NUM_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn lpen(&self) -> LPEN_R {
        LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn kvref(&self) -> KVREF_R {
        KVREF_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn kvref_csa_target(&self) -> KVREF_CSA_TARGET_R {
        KVREF_CSA_TARGET_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rstb(&mut self) -> RSTB_W<1> {
        RSTB_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn swctrl(&mut self) -> SWCTRL_W<2> {
        SWCTRL_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cmodchrg(&mut self) -> CMODCHRG_W<3> {
        CMODCHRG_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cmoddisch(&mut self) -> CMODDISCH_W<4> {
        CMODDISCH_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn midacen(&mut self) -> MIDACEN_W<5> {
        MIDACEN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sidacen(&mut self) -> SIDACEN_W<6> {
        SIDACEN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cmodvtinit(&mut self) -> CMODVTINIT_W<7> {
        CMODVTINIT_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pwmm(&mut self) -> PWMM_W<8> {
        PWMM_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ctrlsource(&mut self) -> CTRLSOURCE_W<9> {
        CTRLSOURCE_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn lpabnormal(&mut self) -> LPABNORMAL_W<10> {
        LPABNORMAL_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn csa_disch_sel(&mut self) -> CSA_DISCH_SEL_W<11> {
        CSA_DISCH_SEL_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn shortsw_num(&mut self) -> SHORTSW_NUM_W<12> {
        SHORTSW_NUM_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn lpen(&mut self) -> LPEN_W<14> {
        LPEN_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn kvref(&mut self) -> KVREF_W<16> {
        KVREF_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn kvref_csa_target(&mut self) -> KVREF_CSA_TARGET_W<18> {
        KVREF_CSA_TARGET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0x02"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
