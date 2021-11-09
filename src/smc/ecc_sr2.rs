#[doc = "Register `ECC_SR2` reader"]
pub struct R(crate::R<ECC_SR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECC_SR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECC_SR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECC_SR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RECERR8` reader - Recoverable Error in the page between the 2048th and the 2303rd bytes"]
pub struct RECERR8_R(crate::FieldReader<bool, bool>);
impl RECERR8_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECERR8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECERR8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCERR8` reader - ECC Error in the page between the 2048th and the 2303rd bytes"]
pub struct ECCERR8_R(crate::FieldReader<bool, bool>);
impl ECCERR8_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCERR8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCERR8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULERR8` reader - Multiple Error in the page between the 2048th and the 2303rd bytes"]
pub struct MULERR8_R(crate::FieldReader<bool, bool>);
impl MULERR8_R {
    pub(crate) fn new(bits: bool) -> Self {
        MULERR8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MULERR8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECERR9` reader - Recoverable Error in the page between the 2304th and the 2559th bytes"]
pub struct RECERR9_R(crate::FieldReader<bool, bool>);
impl RECERR9_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECERR9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECERR9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCERR9` reader - ECC Error in the page between the 2304th and the 2559th bytes"]
pub struct ECCERR9_R(crate::FieldReader<bool, bool>);
impl ECCERR9_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCERR9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCERR9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULERR9` reader - Multiple Error in the page between the 2304th and the 2559th bytes"]
pub struct MULERR9_R(crate::FieldReader<bool, bool>);
impl MULERR9_R {
    pub(crate) fn new(bits: bool) -> Self {
        MULERR9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MULERR9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECERR10` reader - Recoverable Error in the page between the 2560th and the 2815th bytes"]
pub struct RECERR10_R(crate::FieldReader<bool, bool>);
impl RECERR10_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECERR10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECERR10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCERR10` reader - ECC Error in the page between the 2560th and the 2815th bytes"]
pub struct ECCERR10_R(crate::FieldReader<bool, bool>);
impl ECCERR10_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCERR10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCERR10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULERR10` reader - Multiple Error in the page between the 2560th and the 2815th bytes"]
pub struct MULERR10_R(crate::FieldReader<bool, bool>);
impl MULERR10_R {
    pub(crate) fn new(bits: bool) -> Self {
        MULERR10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MULERR10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECERR11` reader - Recoverable Error in the page between the 2816th and the 3071st bytes"]
pub struct RECERR11_R(crate::FieldReader<bool, bool>);
impl RECERR11_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECERR11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECERR11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCERR11` reader - ECC Error in the page between the 2816th and the 3071st bytes"]
pub struct ECCERR11_R(crate::FieldReader<bool, bool>);
impl ECCERR11_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCERR11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCERR11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULERR11` reader - Multiple Error in the page between the 2816th and the 3071st bytes"]
pub struct MULERR11_R(crate::FieldReader<bool, bool>);
impl MULERR11_R {
    pub(crate) fn new(bits: bool) -> Self {
        MULERR11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MULERR11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECERR12` reader - Recoverable Error in the page between the 3072nd and the 3327th bytes"]
pub struct RECERR12_R(crate::FieldReader<bool, bool>);
impl RECERR12_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECERR12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECERR12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCERR12` reader - ECC Error in the page between the 3072nd and the 3327th bytes"]
pub struct ECCERR12_R(crate::FieldReader<bool, bool>);
impl ECCERR12_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCERR12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCERR12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULERR12` reader - Multiple Error in the page between the 3072nd and the 3327th bytes"]
pub struct MULERR12_R(crate::FieldReader<bool, bool>);
impl MULERR12_R {
    pub(crate) fn new(bits: bool) -> Self {
        MULERR12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MULERR12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECERR13` reader - Recoverable Error in the page between the 3328th and the 3583rd bytes"]
pub struct RECERR13_R(crate::FieldReader<bool, bool>);
impl RECERR13_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECERR13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECERR13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCERR13` reader - ECC Error in the page between the 3328th and the 3583rd bytes"]
pub struct ECCERR13_R(crate::FieldReader<bool, bool>);
impl ECCERR13_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCERR13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCERR13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULERR13` reader - Multiple Error in the page between the 3328th and the 3583rd bytes"]
pub struct MULERR13_R(crate::FieldReader<bool, bool>);
impl MULERR13_R {
    pub(crate) fn new(bits: bool) -> Self {
        MULERR13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MULERR13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECERR14` reader - Recoverable Error in the page between the 3584th and the 3839th bytes"]
pub struct RECERR14_R(crate::FieldReader<bool, bool>);
impl RECERR14_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECERR14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECERR14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCERR14` reader - ECC Error in the page between the 3584th and the 3839th bytes"]
pub struct ECCERR14_R(crate::FieldReader<bool, bool>);
impl ECCERR14_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCERR14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCERR14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULERR14` reader - Multiple Error in the page between the 3584th and the 3839th bytes"]
pub struct MULERR14_R(crate::FieldReader<bool, bool>);
impl MULERR14_R {
    pub(crate) fn new(bits: bool) -> Self {
        MULERR14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MULERR14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECERR15` reader - Recoverable Error in the page between the 3840th and the 4095th bytes"]
pub struct RECERR15_R(crate::FieldReader<bool, bool>);
impl RECERR15_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECERR15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECERR15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCERR15` reader - ECC Error in the page between the 3840th and the 4095th bytes"]
pub struct ECCERR15_R(crate::FieldReader<bool, bool>);
impl ECCERR15_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCERR15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCERR15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULERR15` reader - Multiple Error in the page between the 3840th and the 4095th bytes"]
pub struct MULERR15_R(crate::FieldReader<bool, bool>);
impl MULERR15_R {
    pub(crate) fn new(bits: bool) -> Self {
        MULERR15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MULERR15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Recoverable Error in the page between the 2048th and the 2303rd bytes"]
    #[inline(always)]
    pub fn recerr8(&self) -> RECERR8_R {
        RECERR8_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ECC Error in the page between the 2048th and the 2303rd bytes"]
    #[inline(always)]
    pub fn eccerr8(&self) -> ECCERR8_R {
        ECCERR8_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Multiple Error in the page between the 2048th and the 2303rd bytes"]
    #[inline(always)]
    pub fn mulerr8(&self) -> MULERR8_R {
        MULERR8_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Recoverable Error in the page between the 2304th and the 2559th bytes"]
    #[inline(always)]
    pub fn recerr9(&self) -> RECERR9_R {
        RECERR9_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ECC Error in the page between the 2304th and the 2559th bytes"]
    #[inline(always)]
    pub fn eccerr9(&self) -> ECCERR9_R {
        ECCERR9_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Multiple Error in the page between the 2304th and the 2559th bytes"]
    #[inline(always)]
    pub fn mulerr9(&self) -> MULERR9_R {
        MULERR9_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Recoverable Error in the page between the 2560th and the 2815th bytes"]
    #[inline(always)]
    pub fn recerr10(&self) -> RECERR10_R {
        RECERR10_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ECC Error in the page between the 2560th and the 2815th bytes"]
    #[inline(always)]
    pub fn eccerr10(&self) -> ECCERR10_R {
        ECCERR10_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Multiple Error in the page between the 2560th and the 2815th bytes"]
    #[inline(always)]
    pub fn mulerr10(&self) -> MULERR10_R {
        MULERR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Recoverable Error in the page between the 2816th and the 3071st bytes"]
    #[inline(always)]
    pub fn recerr11(&self) -> RECERR11_R {
        RECERR11_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ECC Error in the page between the 2816th and the 3071st bytes"]
    #[inline(always)]
    pub fn eccerr11(&self) -> ECCERR11_R {
        ECCERR11_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Multiple Error in the page between the 2816th and the 3071st bytes"]
    #[inline(always)]
    pub fn mulerr11(&self) -> MULERR11_R {
        MULERR11_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Recoverable Error in the page between the 3072nd and the 3327th bytes"]
    #[inline(always)]
    pub fn recerr12(&self) -> RECERR12_R {
        RECERR12_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ECC Error in the page between the 3072nd and the 3327th bytes"]
    #[inline(always)]
    pub fn eccerr12(&self) -> ECCERR12_R {
        ECCERR12_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Multiple Error in the page between the 3072nd and the 3327th bytes"]
    #[inline(always)]
    pub fn mulerr12(&self) -> MULERR12_R {
        MULERR12_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Recoverable Error in the page between the 3328th and the 3583rd bytes"]
    #[inline(always)]
    pub fn recerr13(&self) -> RECERR13_R {
        RECERR13_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ECC Error in the page between the 3328th and the 3583rd bytes"]
    #[inline(always)]
    pub fn eccerr13(&self) -> ECCERR13_R {
        ECCERR13_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Multiple Error in the page between the 3328th and the 3583rd bytes"]
    #[inline(always)]
    pub fn mulerr13(&self) -> MULERR13_R {
        MULERR13_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Recoverable Error in the page between the 3584th and the 3839th bytes"]
    #[inline(always)]
    pub fn recerr14(&self) -> RECERR14_R {
        RECERR14_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - ECC Error in the page between the 3584th and the 3839th bytes"]
    #[inline(always)]
    pub fn eccerr14(&self) -> ECCERR14_R {
        ECCERR14_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Multiple Error in the page between the 3584th and the 3839th bytes"]
    #[inline(always)]
    pub fn mulerr14(&self) -> MULERR14_R {
        MULERR14_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Recoverable Error in the page between the 3840th and the 4095th bytes"]
    #[inline(always)]
    pub fn recerr15(&self) -> RECERR15_R {
        RECERR15_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - ECC Error in the page between the 3840th and the 4095th bytes"]
    #[inline(always)]
    pub fn eccerr15(&self) -> ECCERR15_R {
        ECCERR15_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Multiple Error in the page between the 3840th and the 4095th bytes"]
    #[inline(always)]
    pub fn mulerr15(&self) -> MULERR15_R {
        MULERR15_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
#[doc = "SMC ECC status 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_sr2](index.html) module"]
pub struct ECC_SR2_SPEC;
impl crate::RegisterSpec for ECC_SR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecc_sr2::R](R) reader structure"]
impl crate::Readable for ECC_SR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ECC_SR2 to value 0"]
impl crate::Resettable for ECC_SR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
