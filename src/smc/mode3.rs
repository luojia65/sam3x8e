#[doc = "Register `MODE3` reader"]
pub struct R(crate::R<MODE3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE3` writer"]
pub struct W(crate::W<MODE3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE3_SPEC>;
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
impl From<crate::W<MODE3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selection of the Control Signal for Read Operation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_MODE_A {
    #[doc = "0: The Read operation is controlled by the NCS signal."]
    NCS_CTRL = 0,
    #[doc = "1: The Read operation is controlled by the NRD signal."]
    NRD_CTRL = 1,
}
impl From<READ_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: READ_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ_MODE` reader - Selection of the Control Signal for Read Operation"]
pub struct READ_MODE_R(crate::FieldReader<bool, READ_MODE_A>);
impl READ_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        READ_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_MODE_A {
        match self.bits {
            false => READ_MODE_A::NCS_CTRL,
            true => READ_MODE_A::NRD_CTRL,
        }
    }
    #[doc = "Checks if the value of the field is `NCS_CTRL`"]
    #[inline(always)]
    pub fn is_ncs_ctrl(&self) -> bool {
        **self == READ_MODE_A::NCS_CTRL
    }
    #[doc = "Checks if the value of the field is `NRD_CTRL`"]
    #[inline(always)]
    pub fn is_nrd_ctrl(&self) -> bool {
        **self == READ_MODE_A::NRD_CTRL
    }
}
impl core::ops::Deref for READ_MODE_R {
    type Target = crate::FieldReader<bool, READ_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READ_MODE` writer - Selection of the Control Signal for Read Operation"]
pub struct READ_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READ_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The Read operation is controlled by the NCS signal."]
    #[inline(always)]
    pub fn ncs_ctrl(self) -> &'a mut W {
        self.variant(READ_MODE_A::NCS_CTRL)
    }
    #[doc = "The Read operation is controlled by the NRD signal."]
    #[inline(always)]
    pub fn nrd_ctrl(self) -> &'a mut W {
        self.variant(READ_MODE_A::NRD_CTRL)
    }
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
#[doc = "Selection of the Control Signal for Write Operation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_MODE_A {
    #[doc = "0: The Write operation is controller by the NCS signal."]
    NCS_CTRL = 0,
    #[doc = "1: The Write operation is controlled by the NWE signal."]
    NWE_CTRL = 1,
}
impl From<WRITE_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: WRITE_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE_MODE` reader - Selection of the Control Signal for Write Operation"]
pub struct WRITE_MODE_R(crate::FieldReader<bool, WRITE_MODE_A>);
impl WRITE_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRITE_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITE_MODE_A {
        match self.bits {
            false => WRITE_MODE_A::NCS_CTRL,
            true => WRITE_MODE_A::NWE_CTRL,
        }
    }
    #[doc = "Checks if the value of the field is `NCS_CTRL`"]
    #[inline(always)]
    pub fn is_ncs_ctrl(&self) -> bool {
        **self == WRITE_MODE_A::NCS_CTRL
    }
    #[doc = "Checks if the value of the field is `NWE_CTRL`"]
    #[inline(always)]
    pub fn is_nwe_ctrl(&self) -> bool {
        **self == WRITE_MODE_A::NWE_CTRL
    }
}
impl core::ops::Deref for WRITE_MODE_R {
    type Target = crate::FieldReader<bool, WRITE_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRITE_MODE` writer - Selection of the Control Signal for Write Operation"]
pub struct WRITE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRITE_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The Write operation is controller by the NCS signal."]
    #[inline(always)]
    pub fn ncs_ctrl(self) -> &'a mut W {
        self.variant(WRITE_MODE_A::NCS_CTRL)
    }
    #[doc = "The Write operation is controlled by the NWE signal."]
    #[inline(always)]
    pub fn nwe_ctrl(self) -> &'a mut W {
        self.variant(WRITE_MODE_A::NWE_CTRL)
    }
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
#[doc = "NWAIT Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXNW_MODE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "2: Frozen Mode"]
    FROZEN = 2,
    #[doc = "3: Ready Mode"]
    READY = 3,
}
impl From<EXNW_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EXNW_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXNW_MODE` reader - NWAIT Mode"]
pub struct EXNW_MODE_R(crate::FieldReader<u8, EXNW_MODE_A>);
impl EXNW_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXNW_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXNW_MODE_A> {
        match self.bits {
            0 => Some(EXNW_MODE_A::DISABLED),
            2 => Some(EXNW_MODE_A::FROZEN),
            3 => Some(EXNW_MODE_A::READY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EXNW_MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `FROZEN`"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        **self == EXNW_MODE_A::FROZEN
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == EXNW_MODE_A::READY
    }
}
impl core::ops::Deref for EXNW_MODE_R {
    type Target = crate::FieldReader<u8, EXNW_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXNW_MODE` writer - NWAIT Mode"]
pub struct EXNW_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXNW_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXNW_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXNW_MODE_A::DISABLED)
    }
    #[doc = "Frozen Mode"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(EXNW_MODE_A::FROZEN)
    }
    #[doc = "Ready Mode"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(EXNW_MODE_A::READY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `BAT` reader - Byte Access Type"]
pub struct BAT_R(crate::FieldReader<bool, bool>);
impl BAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        BAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAT` writer - Byte Access Type"]
pub struct BAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Data Bus Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBW_A {
    #[doc = "0: 8-bit bus"]
    BIT_8 = 0,
    #[doc = "1: 16-bit bus"]
    BIT_16 = 1,
}
impl From<DBW_A> for bool {
    #[inline(always)]
    fn from(variant: DBW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBW` reader - Data Bus Width"]
pub struct DBW_R(crate::FieldReader<bool, DBW_A>);
impl DBW_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBW_A {
        match self.bits {
            false => DBW_A::BIT_8,
            true => DBW_A::BIT_16,
        }
    }
    #[doc = "Checks if the value of the field is `BIT_8`"]
    #[inline(always)]
    pub fn is_bit_8(&self) -> bool {
        **self == DBW_A::BIT_8
    }
    #[doc = "Checks if the value of the field is `BIT_16`"]
    #[inline(always)]
    pub fn is_bit_16(&self) -> bool {
        **self == DBW_A::BIT_16
    }
}
impl core::ops::Deref for DBW_R {
    type Target = crate::FieldReader<bool, DBW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBW` writer - Data Bus Width"]
pub struct DBW_W<'a> {
    w: &'a mut W,
}
impl<'a> DBW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "8-bit bus"]
    #[inline(always)]
    pub fn bit_8(self) -> &'a mut W {
        self.variant(DBW_A::BIT_8)
    }
    #[doc = "16-bit bus"]
    #[inline(always)]
    pub fn bit_16(self) -> &'a mut W {
        self.variant(DBW_A::BIT_16)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `TDF_CYCLES` reader - Data Float Time"]
pub struct TDF_CYCLES_R(crate::FieldReader<u8, u8>);
impl TDF_CYCLES_R {
    pub(crate) fn new(bits: u8) -> Self {
        TDF_CYCLES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDF_CYCLES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDF_CYCLES` writer - Data Float Time"]
pub struct TDF_CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> TDF_CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `TDF_MODE` reader - TDF Optimization"]
pub struct TDF_MODE_R(crate::FieldReader<bool, bool>);
impl TDF_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDF_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDF_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDF_MODE` writer - TDF Optimization"]
pub struct TDF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDF_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Selection of the Control Signal for Read Operation"]
    #[inline(always)]
    pub fn read_mode(&self) -> READ_MODE_R {
        READ_MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Selection of the Control Signal for Write Operation"]
    #[inline(always)]
    pub fn write_mode(&self) -> WRITE_MODE_R {
        WRITE_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    pub fn exnw_mode(&self) -> EXNW_MODE_R {
        EXNW_MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Byte Access Type"]
    #[inline(always)]
    pub fn bat(&self) -> BAT_R {
        BAT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline(always)]
    pub fn tdf_cycles(&self) -> TDF_CYCLES_R {
        TDF_CYCLES_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline(always)]
    pub fn tdf_mode(&self) -> TDF_MODE_R {
        TDF_MODE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selection of the Control Signal for Read Operation"]
    #[inline(always)]
    pub fn read_mode(&mut self) -> READ_MODE_W {
        READ_MODE_W { w: self }
    }
    #[doc = "Bit 1 - Selection of the Control Signal for Write Operation"]
    #[inline(always)]
    pub fn write_mode(&mut self) -> WRITE_MODE_W {
        WRITE_MODE_W { w: self }
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    pub fn exnw_mode(&mut self) -> EXNW_MODE_W {
        EXNW_MODE_W { w: self }
    }
    #[doc = "Bit 8 - Byte Access Type"]
    #[inline(always)]
    pub fn bat(&mut self) -> BAT_W {
        BAT_W { w: self }
    }
    #[doc = "Bit 12 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&mut self) -> DBW_W {
        DBW_W { w: self }
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline(always)]
    pub fn tdf_cycles(&mut self) -> TDF_CYCLES_W {
        TDF_CYCLES_W { w: self }
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline(always)]
    pub fn tdf_mode(&mut self) -> TDF_MODE_W {
        TDF_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC Mode Register (CS_number = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode3](index.html) module"]
pub struct MODE3_SPEC;
impl crate::RegisterSpec for MODE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode3::R](R) reader structure"]
impl crate::Readable for MODE3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode3::W](W) writer structure"]
impl crate::Writable for MODE3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODE3 to value 0x1000_0003"]
impl crate::Resettable for MODE3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000_0003
    }
}
