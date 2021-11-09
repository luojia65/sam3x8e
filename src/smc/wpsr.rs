#[doc = "Register `WPSR` reader"]
pub struct R(crate::R<WPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WP_VS` reader - Write Protection Violation Status"]
pub struct WP_VS_R(crate::FieldReader<u8, u8>);
impl WP_VS_R {
    pub(crate) fn new(bits: u8) -> Self {
        WP_VS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WP_VS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP_VSRC` reader - Write Protection Violation Source"]
pub struct WP_VSRC_R(crate::FieldReader<u16, u16>);
impl WP_VSRC_R {
    pub(crate) fn new(bits: u16) -> Self {
        WP_VSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WP_VSRC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Write Protection Violation Status"]
    #[inline(always)]
    pub fn wp_vs(&self) -> WP_VS_R {
        WP_VS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - Write Protection Violation Source"]
    #[inline(always)]
    pub fn wp_vsrc(&self) -> WP_VSRC_R {
        WP_VSRC_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
#[doc = "Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsr](index.html) module"]
pub struct WPSR_SPEC;
impl crate::RegisterSpec for WPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpsr::R](R) reader structure"]
impl crate::Readable for WPSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WPSR to value 0"]
impl crate::Resettable for WPSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
