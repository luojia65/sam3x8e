#[doc = "Register `MATRIX_MRCR` reader"]
pub struct R(crate::R<MATRIX_MRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MATRIX_MRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MATRIX_MRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MATRIX_MRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MATRIX_MRCR` writer"]
pub struct W(crate::W<MATRIX_MRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MATRIX_MRCR_SPEC>;
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
impl From<crate::W<MATRIX_MRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MATRIX_MRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCB0` reader - Remap Command Bit for AHB Master 0"]
pub struct RCB0_R(crate::FieldReader<bool, bool>);
impl RCB0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCB0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB0` writer - Remap Command Bit for AHB Master 0"]
pub struct RCB0_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `RCB1` reader - Remap Command Bit for AHB Master 1"]
pub struct RCB1_R(crate::FieldReader<bool, bool>);
impl RCB1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB1` writer - Remap Command Bit for AHB Master 1"]
pub struct RCB1_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RCB2` reader - Remap Command Bit for AHB Master 2"]
pub struct RCB2_R(crate::FieldReader<bool, bool>);
impl RCB2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB2` writer - Remap Command Bit for AHB Master 2"]
pub struct RCB2_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `RCB3` reader - Remap Command Bit for AHB Master 3"]
pub struct RCB3_R(crate::FieldReader<bool, bool>);
impl RCB3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCB3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCB3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB3` writer - Remap Command Bit for AHB Master 3"]
pub struct RCB3_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `RCB4` reader - Remap Command Bit for AHB Master 4"]
pub struct RCB4_R(crate::FieldReader<u8, u8>);
impl RCB4_R {
    pub(crate) fn new(bits: u8) -> Self {
        RCB4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCB4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB4` writer - Remap Command Bit for AHB Master 4"]
pub struct RCB4_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `RCB5` reader - Remap Command Bit for AHB Master 5"]
pub struct RCB5_R(crate::FieldReader<bool, bool>);
impl RCB5_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCB5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCB5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB5` writer - Remap Command Bit for AHB Master 5"]
pub struct RCB5_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Remap Command Bit for AHB Master 0"]
    #[inline(always)]
    pub fn rcb0(&self) -> RCB0_R {
        RCB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Remap Command Bit for AHB Master 1"]
    #[inline(always)]
    pub fn rcb1(&self) -> RCB1_R {
        RCB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Remap Command Bit for AHB Master 2"]
    #[inline(always)]
    pub fn rcb2(&self) -> RCB2_R {
        RCB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Remap Command Bit for AHB Master 3"]
    #[inline(always)]
    pub fn rcb3(&self) -> RCB3_R {
        RCB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Remap Command Bit for AHB Master 4"]
    #[inline(always)]
    pub fn rcb4(&self) -> RCB4_R {
        RCB4_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Remap Command Bit for AHB Master 5"]
    #[inline(always)]
    pub fn rcb5(&self) -> RCB5_R {
        RCB5_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remap Command Bit for AHB Master 0"]
    #[inline(always)]
    pub fn rcb0(&mut self) -> RCB0_W {
        RCB0_W { w: self }
    }
    #[doc = "Bit 1 - Remap Command Bit for AHB Master 1"]
    #[inline(always)]
    pub fn rcb1(&mut self) -> RCB1_W {
        RCB1_W { w: self }
    }
    #[doc = "Bit 2 - Remap Command Bit for AHB Master 2"]
    #[inline(always)]
    pub fn rcb2(&mut self) -> RCB2_W {
        RCB2_W { w: self }
    }
    #[doc = "Bit 3 - Remap Command Bit for AHB Master 3"]
    #[inline(always)]
    pub fn rcb3(&mut self) -> RCB3_W {
        RCB3_W { w: self }
    }
    #[doc = "Bits 4:5 - Remap Command Bit for AHB Master 4"]
    #[inline(always)]
    pub fn rcb4(&mut self) -> RCB4_W {
        RCB4_W { w: self }
    }
    #[doc = "Bit 6 - Remap Command Bit for AHB Master 5"]
    #[inline(always)]
    pub fn rcb5(&mut self) -> RCB5_W {
        RCB5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Remap Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_mrcr](index.html) module"]
pub struct MATRIX_MRCR_SPEC;
impl crate::RegisterSpec for MATRIX_MRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [matrix_mrcr::R](R) reader structure"]
impl crate::Readable for MATRIX_MRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [matrix_mrcr::W](W) writer structure"]
impl crate::Writable for MATRIX_MRCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MATRIX_MRCR to value 0"]
impl crate::Resettable for MATRIX_MRCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
