#[doc = "Register `OSS` writer"]
pub struct W(crate::W<OSS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSS_SPEC>;
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
impl From<crate::W<OSS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSSH0` writer - Output Selection Set for PWMH output of the channel 0"]
pub struct OSSH0_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSH0_W<'a> {
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
#[doc = "Field `OSSH1` writer - Output Selection Set for PWMH output of the channel 1"]
pub struct OSSH1_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSH1_W<'a> {
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
#[doc = "Field `OSSH2` writer - Output Selection Set for PWMH output of the channel 2"]
pub struct OSSH2_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSH2_W<'a> {
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
#[doc = "Field `OSSH3` writer - Output Selection Set for PWMH output of the channel 3"]
pub struct OSSH3_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSH3_W<'a> {
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
#[doc = "Field `OSSH4` writer - Output Selection Set for PWMH output of the channel 4"]
pub struct OSSH4_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSH4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `OSSH5` writer - Output Selection Set for PWMH output of the channel 5"]
pub struct OSSH5_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSH5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `OSSH6` writer - Output Selection Set for PWMH output of the channel 6"]
pub struct OSSH6_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSH6_W<'a> {
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
#[doc = "Field `OSSH7` writer - Output Selection Set for PWMH output of the channel 7"]
pub struct OSSH7_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSH7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `OSSL0` writer - Output Selection Set for PWML output of the channel 0"]
pub struct OSSL0_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSL0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `OSSL1` writer - Output Selection Set for PWML output of the channel 1"]
pub struct OSSL1_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSL1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `OSSL2` writer - Output Selection Set for PWML output of the channel 2"]
pub struct OSSL2_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSL2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `OSSL3` writer - Output Selection Set for PWML output of the channel 3"]
pub struct OSSL3_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSL3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `OSSL4` writer - Output Selection Set for PWML output of the channel 4"]
pub struct OSSL4_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSL4_W<'a> {
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
#[doc = "Field `OSSL5` writer - Output Selection Set for PWML output of the channel 5"]
pub struct OSSL5_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSL5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `OSSL6` writer - Output Selection Set for PWML output of the channel 6"]
pub struct OSSL6_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSL6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `OSSL7` writer - Output Selection Set for PWML output of the channel 7"]
pub struct OSSL7_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSL7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Output Selection Set for PWMH output of the channel 0"]
    #[inline(always)]
    pub fn ossh0(&mut self) -> OSSH0_W {
        OSSH0_W { w: self }
    }
    #[doc = "Bit 1 - Output Selection Set for PWMH output of the channel 1"]
    #[inline(always)]
    pub fn ossh1(&mut self) -> OSSH1_W {
        OSSH1_W { w: self }
    }
    #[doc = "Bit 2 - Output Selection Set for PWMH output of the channel 2"]
    #[inline(always)]
    pub fn ossh2(&mut self) -> OSSH2_W {
        OSSH2_W { w: self }
    }
    #[doc = "Bit 3 - Output Selection Set for PWMH output of the channel 3"]
    #[inline(always)]
    pub fn ossh3(&mut self) -> OSSH3_W {
        OSSH3_W { w: self }
    }
    #[doc = "Bit 4 - Output Selection Set for PWMH output of the channel 4"]
    #[inline(always)]
    pub fn ossh4(&mut self) -> OSSH4_W {
        OSSH4_W { w: self }
    }
    #[doc = "Bit 5 - Output Selection Set for PWMH output of the channel 5"]
    #[inline(always)]
    pub fn ossh5(&mut self) -> OSSH5_W {
        OSSH5_W { w: self }
    }
    #[doc = "Bit 6 - Output Selection Set for PWMH output of the channel 6"]
    #[inline(always)]
    pub fn ossh6(&mut self) -> OSSH6_W {
        OSSH6_W { w: self }
    }
    #[doc = "Bit 7 - Output Selection Set for PWMH output of the channel 7"]
    #[inline(always)]
    pub fn ossh7(&mut self) -> OSSH7_W {
        OSSH7_W { w: self }
    }
    #[doc = "Bit 16 - Output Selection Set for PWML output of the channel 0"]
    #[inline(always)]
    pub fn ossl0(&mut self) -> OSSL0_W {
        OSSL0_W { w: self }
    }
    #[doc = "Bit 17 - Output Selection Set for PWML output of the channel 1"]
    #[inline(always)]
    pub fn ossl1(&mut self) -> OSSL1_W {
        OSSL1_W { w: self }
    }
    #[doc = "Bit 18 - Output Selection Set for PWML output of the channel 2"]
    #[inline(always)]
    pub fn ossl2(&mut self) -> OSSL2_W {
        OSSL2_W { w: self }
    }
    #[doc = "Bit 19 - Output Selection Set for PWML output of the channel 3"]
    #[inline(always)]
    pub fn ossl3(&mut self) -> OSSL3_W {
        OSSL3_W { w: self }
    }
    #[doc = "Bit 20 - Output Selection Set for PWML output of the channel 4"]
    #[inline(always)]
    pub fn ossl4(&mut self) -> OSSL4_W {
        OSSL4_W { w: self }
    }
    #[doc = "Bit 21 - Output Selection Set for PWML output of the channel 5"]
    #[inline(always)]
    pub fn ossl5(&mut self) -> OSSL5_W {
        OSSL5_W { w: self }
    }
    #[doc = "Bit 22 - Output Selection Set for PWML output of the channel 6"]
    #[inline(always)]
    pub fn ossl6(&mut self) -> OSSL6_W {
        OSSL6_W { w: self }
    }
    #[doc = "Bit 23 - Output Selection Set for PWML output of the channel 7"]
    #[inline(always)]
    pub fn ossl7(&mut self) -> OSSL7_W {
        OSSL7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Output Selection Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oss](index.html) module"]
pub struct OSS_SPEC;
impl crate::RegisterSpec for OSS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [oss::W](W) writer structure"]
impl crate::Writable for OSS_SPEC {
    type Writer = W;
}
