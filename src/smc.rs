#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SMC NFC Configuration Register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x04 - SMC NFC Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x08 - SMC NFC Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x0c - SMC NFC Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x10 - SMC NFC Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x14 - SMC NFC Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x18 - SMC NFC Address Cycle Zero Register"]
    pub addr: crate::Reg<addr::ADDR_SPEC>,
    #[doc = "0x1c - SMC Bank Address Register"]
    pub bank: crate::Reg<bank::BANK_SPEC>,
    #[doc = "0x20 - SMC ECC Control Register"]
    pub ecc_ctrl: crate::Reg<ecc_ctrl::ECC_CTRL_SPEC>,
    #[doc = "0x24 - SMC ECC Mode Register"]
    pub ecc_md: crate::Reg<ecc_md::ECC_MD_SPEC>,
    #[doc = "0x28 - SMC ECC Status 1 Register"]
    pub ecc_sr1: crate::Reg<ecc_sr1::ECC_SR1_SPEC>,
    _reserved_11_ecc_pr0: [u8; 0x04],
    _reserved_12_ecc_pr1: [u8; 0x04],
    #[doc = "0x34 - SMC ECC status 2 Register"]
    pub ecc_sr2: crate::Reg<ecc_sr2::ECC_SR2_SPEC>,
    _reserved_14_ecc_pr2: [u8; 0x04],
    _reserved_15_ecc_pr3: [u8; 0x04],
    _reserved_16_ecc_pr4: [u8; 0x04],
    _reserved_17_ecc_pr5: [u8; 0x04],
    _reserved_18_ecc_pr6: [u8; 0x04],
    _reserved_19_ecc_pr7: [u8; 0x04],
    #[doc = "0x50 - SMC ECC parity 8 Register"]
    pub ecc_pr8: crate::Reg<ecc_pr8::ECC_PR8_SPEC>,
    #[doc = "0x54 - SMC ECC parity 9 Register"]
    pub ecc_pr9: crate::Reg<ecc_pr9::ECC_PR9_SPEC>,
    #[doc = "0x58 - SMC ECC parity 10 Register"]
    pub ecc_pr10: crate::Reg<ecc_pr10::ECC_PR10_SPEC>,
    #[doc = "0x5c - SMC ECC parity 11 Register"]
    pub ecc_pr11: crate::Reg<ecc_pr11::ECC_PR11_SPEC>,
    #[doc = "0x60 - SMC ECC parity 12 Register"]
    pub ecc_pr12: crate::Reg<ecc_pr12::ECC_PR12_SPEC>,
    #[doc = "0x64 - SMC ECC parity 13 Register"]
    pub ecc_pr13: crate::Reg<ecc_pr13::ECC_PR13_SPEC>,
    #[doc = "0x68 - SMC ECC parity 14 Register"]
    pub ecc_pr14: crate::Reg<ecc_pr14::ECC_PR14_SPEC>,
    #[doc = "0x6c - SMC ECC parity 15 Register"]
    pub ecc_pr15: crate::Reg<ecc_pr15::ECC_PR15_SPEC>,
    #[doc = "0x70 - SMC Setup Register (CS_number = 0)"]
    pub setup0: crate::Reg<setup0::SETUP0_SPEC>,
    #[doc = "0x74 - SMC Pulse Register (CS_number = 0)"]
    pub pulse0: crate::Reg<pulse0::PULSE0_SPEC>,
    #[doc = "0x78 - SMC Cycle Register (CS_number = 0)"]
    pub cycle0: crate::Reg<cycle0::CYCLE0_SPEC>,
    #[doc = "0x7c - SMC Timings Register (CS_number = 0)"]
    pub timings0: crate::Reg<timings0::TIMINGS0_SPEC>,
    #[doc = "0x80 - SMC Mode Register (CS_number = 0)"]
    pub mode0: crate::Reg<mode0::MODE0_SPEC>,
    #[doc = "0x84 - SMC Setup Register (CS_number = 1)"]
    pub setup1: crate::Reg<setup1::SETUP1_SPEC>,
    #[doc = "0x88 - SMC Pulse Register (CS_number = 1)"]
    pub pulse1: crate::Reg<pulse1::PULSE1_SPEC>,
    #[doc = "0x8c - SMC Cycle Register (CS_number = 1)"]
    pub cycle1: crate::Reg<cycle1::CYCLE1_SPEC>,
    #[doc = "0x90 - SMC Timings Register (CS_number = 1)"]
    pub timings1: crate::Reg<timings1::TIMINGS1_SPEC>,
    #[doc = "0x94 - SMC Mode Register (CS_number = 1)"]
    pub mode1: crate::Reg<mode1::MODE1_SPEC>,
    #[doc = "0x98 - SMC Setup Register (CS_number = 2)"]
    pub setup2: crate::Reg<setup2::SETUP2_SPEC>,
    #[doc = "0x9c - SMC Pulse Register (CS_number = 2)"]
    pub pulse2: crate::Reg<pulse2::PULSE2_SPEC>,
    #[doc = "0xa0 - SMC Cycle Register (CS_number = 2)"]
    pub cycle2: crate::Reg<cycle2::CYCLE2_SPEC>,
    #[doc = "0xa4 - SMC Timings Register (CS_number = 2)"]
    pub timings2: crate::Reg<timings2::TIMINGS2_SPEC>,
    #[doc = "0xa8 - SMC Mode Register (CS_number = 2)"]
    pub mode2: crate::Reg<mode2::MODE2_SPEC>,
    #[doc = "0xac - SMC Setup Register (CS_number = 3)"]
    pub setup3: crate::Reg<setup3::SETUP3_SPEC>,
    #[doc = "0xb0 - SMC Pulse Register (CS_number = 3)"]
    pub pulse3: crate::Reg<pulse3::PULSE3_SPEC>,
    #[doc = "0xb4 - SMC Cycle Register (CS_number = 3)"]
    pub cycle3: crate::Reg<cycle3::CYCLE3_SPEC>,
    #[doc = "0xb8 - SMC Timings Register (CS_number = 3)"]
    pub timings3: crate::Reg<timings3::TIMINGS3_SPEC>,
    #[doc = "0xbc - SMC Mode Register (CS_number = 3)"]
    pub mode3: crate::Reg<mode3::MODE3_SPEC>,
    #[doc = "0xc0 - SMC Setup Register (CS_number = 4)"]
    pub setup4: crate::Reg<setup4::SETUP4_SPEC>,
    #[doc = "0xc4 - SMC Pulse Register (CS_number = 4)"]
    pub pulse4: crate::Reg<pulse4::PULSE4_SPEC>,
    #[doc = "0xc8 - SMC Cycle Register (CS_number = 4)"]
    pub cycle4: crate::Reg<cycle4::CYCLE4_SPEC>,
    #[doc = "0xcc - SMC Timings Register (CS_number = 4)"]
    pub timings4: crate::Reg<timings4::TIMINGS4_SPEC>,
    #[doc = "0xd0 - SMC Mode Register (CS_number = 4)"]
    pub mode4: crate::Reg<mode4::MODE4_SPEC>,
    #[doc = "0xd4 - SMC Setup Register (CS_number = 5)"]
    pub setup5: crate::Reg<setup5::SETUP5_SPEC>,
    #[doc = "0xd8 - SMC Pulse Register (CS_number = 5)"]
    pub pulse5: crate::Reg<pulse5::PULSE5_SPEC>,
    #[doc = "0xdc - SMC Cycle Register (CS_number = 5)"]
    pub cycle5: crate::Reg<cycle5::CYCLE5_SPEC>,
    #[doc = "0xe0 - SMC Timings Register (CS_number = 5)"]
    pub timings5: crate::Reg<timings5::TIMINGS5_SPEC>,
    #[doc = "0xe4 - SMC Mode Register (CS_number = 5)"]
    pub mode5: crate::Reg<mode5::MODE5_SPEC>,
    #[doc = "0xe8 - SMC Setup Register (CS_number = 6)"]
    pub setup6: crate::Reg<setup6::SETUP6_SPEC>,
    #[doc = "0xec - SMC Pulse Register (CS_number = 6)"]
    pub pulse6: crate::Reg<pulse6::PULSE6_SPEC>,
    #[doc = "0xf0 - SMC Cycle Register (CS_number = 6)"]
    pub cycle6: crate::Reg<cycle6::CYCLE6_SPEC>,
    #[doc = "0xf4 - SMC Timings Register (CS_number = 6)"]
    pub timings6: crate::Reg<timings6::TIMINGS6_SPEC>,
    #[doc = "0xf8 - SMC Mode Register (CS_number = 6)"]
    pub mode6: crate::Reg<mode6::MODE6_SPEC>,
    #[doc = "0xfc - SMC Setup Register (CS_number = 7)"]
    pub setup7: crate::Reg<setup7::SETUP7_SPEC>,
    #[doc = "0x100 - SMC Pulse Register (CS_number = 7)"]
    pub pulse7: crate::Reg<pulse7::PULSE7_SPEC>,
    #[doc = "0x104 - SMC Cycle Register (CS_number = 7)"]
    pub cycle7: crate::Reg<cycle7::CYCLE7_SPEC>,
    #[doc = "0x108 - SMC Timings Register (CS_number = 7)"]
    pub timings7: crate::Reg<timings7::TIMINGS7_SPEC>,
    #[doc = "0x10c - SMC Mode Register (CS_number = 7)"]
    pub mode7: crate::Reg<mode7::MODE7_SPEC>,
    #[doc = "0x110 - SMC OCMS Register"]
    pub ocms: crate::Reg<ocms::OCMS_SPEC>,
    #[doc = "0x114 - SMC OCMS KEY1 Register"]
    pub key1: crate::Reg<key1::KEY1_SPEC>,
    #[doc = "0x118 - SMC OCMS KEY2 Register"]
    pub key2: crate::Reg<key2::KEY2_SPEC>,
    _reserved71: [u8; 0xc8],
    #[doc = "0x1e4 - Write Protection Control Register"]
    pub wpcr: crate::Reg<wpcr::WPCR_SPEC>,
    #[doc = "0x1e8 - Write Protection Status Register"]
    pub wpsr: crate::Reg<wpsr::WPSR_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x2c - SMC ECC Parity 0 Register"]
    #[inline(always)]
    pub fn w8bit_ecc_pr0_w8bit(
        &self,
    ) -> &crate::Reg<w8bit_ecc_pr0_w8bit::W8BIT_ECC_PR0_W8BIT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(44usize)
                as *const crate::Reg<w8bit_ecc_pr0_w8bit::W8BIT_ECC_PR0_W8BIT_SPEC>)
        }
    }
    #[doc = "0x2c - SMC ECC Parity 0 Register"]
    #[inline(always)]
    pub fn w9bit_ecc_pr0_w9bit(
        &self,
    ) -> &crate::Reg<w9bit_ecc_pr0_w9bit::W9BIT_ECC_PR0_W9BIT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(44usize)
                as *const crate::Reg<w9bit_ecc_pr0_w9bit::W9BIT_ECC_PR0_W9BIT_SPEC>)
        }
    }
    #[doc = "0x2c - SMC ECC Parity 0 Register"]
    #[inline(always)]
    pub fn ecc_pr0(&self) -> &crate::Reg<ecc_pr0::ECC_PR0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(44usize)
                as *const crate::Reg<ecc_pr0::ECC_PR0_SPEC>)
        }
    }
    #[doc = "0x30 - SMC ECC parity 1 Register"]
    #[inline(always)]
    pub fn w8bit_ecc_pr1_w8bit(
        &self,
    ) -> &crate::Reg<w8bit_ecc_pr1_w8bit::W8BIT_ECC_PR1_W8BIT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(48usize)
                as *const crate::Reg<w8bit_ecc_pr1_w8bit::W8BIT_ECC_PR1_W8BIT_SPEC>)
        }
    }
    #[doc = "0x30 - SMC ECC parity 1 Register"]
    #[inline(always)]
    pub fn w9bit_ecc_pr1_w9bit(
        &self,
    ) -> &crate::Reg<w9bit_ecc_pr1_w9bit::W9BIT_ECC_PR1_W9BIT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(48usize)
                as *const crate::Reg<w9bit_ecc_pr1_w9bit::W9BIT_ECC_PR1_W9BIT_SPEC>)
        }
    }
    #[doc = "0x30 - SMC ECC parity 1 Register"]
    #[inline(always)]
    pub fn ecc_pr1(&self) -> &crate::Reg<ecc_pr1::ECC_PR1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(48usize)
                as *const crate::Reg<ecc_pr1::ECC_PR1_SPEC>)
        }
    }
    #[doc = "0x38 - SMC ECC parity 2 Register"]
    #[inline(always)]
    pub fn w8bit_ecc_pr2_w8bit(
        &self,
    ) -> &crate::Reg<w8bit_ecc_pr2_w8bit::W8BIT_ECC_PR2_W8BIT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(56usize)
                as *const crate::Reg<w8bit_ecc_pr2_w8bit::W8BIT_ECC_PR2_W8BIT_SPEC>)
        }
    }
    #[doc = "0x38 - SMC ECC parity 2 Register"]
    #[inline(always)]
    pub fn ecc_pr2(&self) -> &crate::Reg<ecc_pr2::ECC_PR2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(56usize)
                as *const crate::Reg<ecc_pr2::ECC_PR2_SPEC>)
        }
    }
    #[doc = "0x3c - SMC ECC parity 3 Register"]
    #[inline(always)]
    pub fn w8bit_ecc_pr3_w8bit(
        &self,
    ) -> &crate::Reg<w8bit_ecc_pr3_w8bit::W8BIT_ECC_PR3_W8BIT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(60usize)
                as *const crate::Reg<w8bit_ecc_pr3_w8bit::W8BIT_ECC_PR3_W8BIT_SPEC>)
        }
    }
    #[doc = "0x3c - SMC ECC parity 3 Register"]
    #[inline(always)]
    pub fn ecc_pr3(&self) -> &crate::Reg<ecc_pr3::ECC_PR3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(60usize)
                as *const crate::Reg<ecc_pr3::ECC_PR3_SPEC>)
        }
    }
    #[doc = "0x40 - SMC ECC parity 4 Register"]
    #[inline(always)]
    pub fn w8bit_ecc_pr4_w8bit(
        &self,
    ) -> &crate::Reg<w8bit_ecc_pr4_w8bit::W8BIT_ECC_PR4_W8BIT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(64usize)
                as *const crate::Reg<w8bit_ecc_pr4_w8bit::W8BIT_ECC_PR4_W8BIT_SPEC>)
        }
    }
    #[doc = "0x40 - SMC ECC parity 4 Register"]
    #[inline(always)]
    pub fn ecc_pr4(&self) -> &crate::Reg<ecc_pr4::ECC_PR4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(64usize)
                as *const crate::Reg<ecc_pr4::ECC_PR4_SPEC>)
        }
    }
    #[doc = "0x44 - SMC ECC parity 5 Register"]
    #[inline(always)]
    pub fn w8bit_ecc_pr5_w8bit(
        &self,
    ) -> &crate::Reg<w8bit_ecc_pr5_w8bit::W8BIT_ECC_PR5_W8BIT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(68usize)
                as *const crate::Reg<w8bit_ecc_pr5_w8bit::W8BIT_ECC_PR5_W8BIT_SPEC>)
        }
    }
    #[doc = "0x44 - SMC ECC parity 5 Register"]
    #[inline(always)]
    pub fn ecc_pr5(&self) -> &crate::Reg<ecc_pr5::ECC_PR5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(68usize)
                as *const crate::Reg<ecc_pr5::ECC_PR5_SPEC>)
        }
    }
    #[doc = "0x48 - SMC ECC parity 6 Register"]
    #[inline(always)]
    pub fn w8bit_ecc_pr6_w8bit(
        &self,
    ) -> &crate::Reg<w8bit_ecc_pr6_w8bit::W8BIT_ECC_PR6_W8BIT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(72usize)
                as *const crate::Reg<w8bit_ecc_pr6_w8bit::W8BIT_ECC_PR6_W8BIT_SPEC>)
        }
    }
    #[doc = "0x48 - SMC ECC parity 6 Register"]
    #[inline(always)]
    pub fn ecc_pr6(&self) -> &crate::Reg<ecc_pr6::ECC_PR6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(72usize)
                as *const crate::Reg<ecc_pr6::ECC_PR6_SPEC>)
        }
    }
    #[doc = "0x4c - SMC ECC parity 7 Register"]
    #[inline(always)]
    pub fn w8bit_ecc_pr7_w8bit(
        &self,
    ) -> &crate::Reg<w8bit_ecc_pr7_w8bit::W8BIT_ECC_PR7_W8BIT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(76usize)
                as *const crate::Reg<w8bit_ecc_pr7_w8bit::W8BIT_ECC_PR7_W8BIT_SPEC>)
        }
    }
    #[doc = "0x4c - SMC ECC parity 7 Register"]
    #[inline(always)]
    pub fn ecc_pr7(&self) -> &crate::Reg<ecc_pr7::ECC_PR7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(76usize)
                as *const crate::Reg<ecc_pr7::ECC_PR7_SPEC>)
        }
    }
}
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "SMC NFC Configuration Register"]
pub mod cfg;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "SMC NFC Control Register"]
pub mod ctrl;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "SMC NFC Status Register"]
pub mod sr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "SMC NFC Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "SMC NFC Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "SMC NFC Interrupt Mask Register"]
pub mod imr;
#[doc = "ADDR register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "SMC NFC Address Cycle Zero Register"]
pub mod addr;
#[doc = "BANK register accessor: an alias for `Reg<BANK_SPEC>`"]
pub type BANK = crate::Reg<bank::BANK_SPEC>;
#[doc = "SMC Bank Address Register"]
pub mod bank;
#[doc = "ECC_CTRL register accessor: an alias for `Reg<ECC_CTRL_SPEC>`"]
pub type ECC_CTRL = crate::Reg<ecc_ctrl::ECC_CTRL_SPEC>;
#[doc = "SMC ECC Control Register"]
pub mod ecc_ctrl;
#[doc = "ECC_MD register accessor: an alias for `Reg<ECC_MD_SPEC>`"]
pub type ECC_MD = crate::Reg<ecc_md::ECC_MD_SPEC>;
#[doc = "SMC ECC Mode Register"]
pub mod ecc_md;
#[doc = "ECC_SR1 register accessor: an alias for `Reg<ECC_SR1_SPEC>`"]
pub type ECC_SR1 = crate::Reg<ecc_sr1::ECC_SR1_SPEC>;
#[doc = "SMC ECC Status 1 Register"]
pub mod ecc_sr1;
#[doc = "ECC_PR0 register accessor: an alias for `Reg<ECC_PR0_SPEC>`"]
pub type ECC_PR0 = crate::Reg<ecc_pr0::ECC_PR0_SPEC>;
#[doc = "SMC ECC Parity 0 Register"]
pub mod ecc_pr0;
#[doc = "W9BIT_ECC_PR0_W9BIT register accessor: an alias for `Reg<W9BIT_ECC_PR0_W9BIT_SPEC>`"]
pub type W9BIT_ECC_PR0_W9BIT = crate::Reg<w9bit_ecc_pr0_w9bit::W9BIT_ECC_PR0_W9BIT_SPEC>;
#[doc = "SMC ECC Parity 0 Register"]
pub mod w9bit_ecc_pr0_w9bit;
#[doc = "W8BIT_ECC_PR0_W8BIT register accessor: an alias for `Reg<W8BIT_ECC_PR0_W8BIT_SPEC>`"]
pub type W8BIT_ECC_PR0_W8BIT = crate::Reg<w8bit_ecc_pr0_w8bit::W8BIT_ECC_PR0_W8BIT_SPEC>;
#[doc = "SMC ECC Parity 0 Register"]
pub mod w8bit_ecc_pr0_w8bit;
#[doc = "ECC_PR1 register accessor: an alias for `Reg<ECC_PR1_SPEC>`"]
pub type ECC_PR1 = crate::Reg<ecc_pr1::ECC_PR1_SPEC>;
#[doc = "SMC ECC parity 1 Register"]
pub mod ecc_pr1;
#[doc = "W9BIT_ECC_PR1_W9BIT register accessor: an alias for `Reg<W9BIT_ECC_PR1_W9BIT_SPEC>`"]
pub type W9BIT_ECC_PR1_W9BIT = crate::Reg<w9bit_ecc_pr1_w9bit::W9BIT_ECC_PR1_W9BIT_SPEC>;
#[doc = "SMC ECC parity 1 Register"]
pub mod w9bit_ecc_pr1_w9bit;
#[doc = "W8BIT_ECC_PR1_W8BIT register accessor: an alias for `Reg<W8BIT_ECC_PR1_W8BIT_SPEC>`"]
pub type W8BIT_ECC_PR1_W8BIT = crate::Reg<w8bit_ecc_pr1_w8bit::W8BIT_ECC_PR1_W8BIT_SPEC>;
#[doc = "SMC ECC parity 1 Register"]
pub mod w8bit_ecc_pr1_w8bit;
#[doc = "ECC_SR2 register accessor: an alias for `Reg<ECC_SR2_SPEC>`"]
pub type ECC_SR2 = crate::Reg<ecc_sr2::ECC_SR2_SPEC>;
#[doc = "SMC ECC status 2 Register"]
pub mod ecc_sr2;
#[doc = "ECC_PR2 register accessor: an alias for `Reg<ECC_PR2_SPEC>`"]
pub type ECC_PR2 = crate::Reg<ecc_pr2::ECC_PR2_SPEC>;
#[doc = "SMC ECC parity 2 Register"]
pub mod ecc_pr2;
#[doc = "W8BIT_ECC_PR2_W8BIT register accessor: an alias for `Reg<W8BIT_ECC_PR2_W8BIT_SPEC>`"]
pub type W8BIT_ECC_PR2_W8BIT = crate::Reg<w8bit_ecc_pr2_w8bit::W8BIT_ECC_PR2_W8BIT_SPEC>;
#[doc = "SMC ECC parity 2 Register"]
pub mod w8bit_ecc_pr2_w8bit;
#[doc = "ECC_PR3 register accessor: an alias for `Reg<ECC_PR3_SPEC>`"]
pub type ECC_PR3 = crate::Reg<ecc_pr3::ECC_PR3_SPEC>;
#[doc = "SMC ECC parity 3 Register"]
pub mod ecc_pr3;
#[doc = "W8BIT_ECC_PR3_W8BIT register accessor: an alias for `Reg<W8BIT_ECC_PR3_W8BIT_SPEC>`"]
pub type W8BIT_ECC_PR3_W8BIT = crate::Reg<w8bit_ecc_pr3_w8bit::W8BIT_ECC_PR3_W8BIT_SPEC>;
#[doc = "SMC ECC parity 3 Register"]
pub mod w8bit_ecc_pr3_w8bit;
#[doc = "ECC_PR4 register accessor: an alias for `Reg<ECC_PR4_SPEC>`"]
pub type ECC_PR4 = crate::Reg<ecc_pr4::ECC_PR4_SPEC>;
#[doc = "SMC ECC parity 4 Register"]
pub mod ecc_pr4;
#[doc = "W8BIT_ECC_PR4_W8BIT register accessor: an alias for `Reg<W8BIT_ECC_PR4_W8BIT_SPEC>`"]
pub type W8BIT_ECC_PR4_W8BIT = crate::Reg<w8bit_ecc_pr4_w8bit::W8BIT_ECC_PR4_W8BIT_SPEC>;
#[doc = "SMC ECC parity 4 Register"]
pub mod w8bit_ecc_pr4_w8bit;
#[doc = "ECC_PR5 register accessor: an alias for `Reg<ECC_PR5_SPEC>`"]
pub type ECC_PR5 = crate::Reg<ecc_pr5::ECC_PR5_SPEC>;
#[doc = "SMC ECC parity 5 Register"]
pub mod ecc_pr5;
#[doc = "W8BIT_ECC_PR5_W8BIT register accessor: an alias for `Reg<W8BIT_ECC_PR5_W8BIT_SPEC>`"]
pub type W8BIT_ECC_PR5_W8BIT = crate::Reg<w8bit_ecc_pr5_w8bit::W8BIT_ECC_PR5_W8BIT_SPEC>;
#[doc = "SMC ECC parity 5 Register"]
pub mod w8bit_ecc_pr5_w8bit;
#[doc = "ECC_PR6 register accessor: an alias for `Reg<ECC_PR6_SPEC>`"]
pub type ECC_PR6 = crate::Reg<ecc_pr6::ECC_PR6_SPEC>;
#[doc = "SMC ECC parity 6 Register"]
pub mod ecc_pr6;
#[doc = "W8BIT_ECC_PR6_W8BIT register accessor: an alias for `Reg<W8BIT_ECC_PR6_W8BIT_SPEC>`"]
pub type W8BIT_ECC_PR6_W8BIT = crate::Reg<w8bit_ecc_pr6_w8bit::W8BIT_ECC_PR6_W8BIT_SPEC>;
#[doc = "SMC ECC parity 6 Register"]
pub mod w8bit_ecc_pr6_w8bit;
#[doc = "ECC_PR7 register accessor: an alias for `Reg<ECC_PR7_SPEC>`"]
pub type ECC_PR7 = crate::Reg<ecc_pr7::ECC_PR7_SPEC>;
#[doc = "SMC ECC parity 7 Register"]
pub mod ecc_pr7;
#[doc = "W8BIT_ECC_PR7_W8BIT register accessor: an alias for `Reg<W8BIT_ECC_PR7_W8BIT_SPEC>`"]
pub type W8BIT_ECC_PR7_W8BIT = crate::Reg<w8bit_ecc_pr7_w8bit::W8BIT_ECC_PR7_W8BIT_SPEC>;
#[doc = "SMC ECC parity 7 Register"]
pub mod w8bit_ecc_pr7_w8bit;
#[doc = "ECC_PR8 register accessor: an alias for `Reg<ECC_PR8_SPEC>`"]
pub type ECC_PR8 = crate::Reg<ecc_pr8::ECC_PR8_SPEC>;
#[doc = "SMC ECC parity 8 Register"]
pub mod ecc_pr8;
#[doc = "ECC_PR9 register accessor: an alias for `Reg<ECC_PR9_SPEC>`"]
pub type ECC_PR9 = crate::Reg<ecc_pr9::ECC_PR9_SPEC>;
#[doc = "SMC ECC parity 9 Register"]
pub mod ecc_pr9;
#[doc = "ECC_PR10 register accessor: an alias for `Reg<ECC_PR10_SPEC>`"]
pub type ECC_PR10 = crate::Reg<ecc_pr10::ECC_PR10_SPEC>;
#[doc = "SMC ECC parity 10 Register"]
pub mod ecc_pr10;
#[doc = "ECC_PR11 register accessor: an alias for `Reg<ECC_PR11_SPEC>`"]
pub type ECC_PR11 = crate::Reg<ecc_pr11::ECC_PR11_SPEC>;
#[doc = "SMC ECC parity 11 Register"]
pub mod ecc_pr11;
#[doc = "ECC_PR12 register accessor: an alias for `Reg<ECC_PR12_SPEC>`"]
pub type ECC_PR12 = crate::Reg<ecc_pr12::ECC_PR12_SPEC>;
#[doc = "SMC ECC parity 12 Register"]
pub mod ecc_pr12;
#[doc = "ECC_PR13 register accessor: an alias for `Reg<ECC_PR13_SPEC>`"]
pub type ECC_PR13 = crate::Reg<ecc_pr13::ECC_PR13_SPEC>;
#[doc = "SMC ECC parity 13 Register"]
pub mod ecc_pr13;
#[doc = "ECC_PR14 register accessor: an alias for `Reg<ECC_PR14_SPEC>`"]
pub type ECC_PR14 = crate::Reg<ecc_pr14::ECC_PR14_SPEC>;
#[doc = "SMC ECC parity 14 Register"]
pub mod ecc_pr14;
#[doc = "ECC_PR15 register accessor: an alias for `Reg<ECC_PR15_SPEC>`"]
pub type ECC_PR15 = crate::Reg<ecc_pr15::ECC_PR15_SPEC>;
#[doc = "SMC ECC parity 15 Register"]
pub mod ecc_pr15;
#[doc = "SETUP0 register accessor: an alias for `Reg<SETUP0_SPEC>`"]
pub type SETUP0 = crate::Reg<setup0::SETUP0_SPEC>;
#[doc = "SMC Setup Register (CS_number = 0)"]
pub mod setup0;
#[doc = "PULSE0 register accessor: an alias for `Reg<PULSE0_SPEC>`"]
pub type PULSE0 = crate::Reg<pulse0::PULSE0_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 0)"]
pub mod pulse0;
#[doc = "CYCLE0 register accessor: an alias for `Reg<CYCLE0_SPEC>`"]
pub type CYCLE0 = crate::Reg<cycle0::CYCLE0_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 0)"]
pub mod cycle0;
#[doc = "TIMINGS0 register accessor: an alias for `Reg<TIMINGS0_SPEC>`"]
pub type TIMINGS0 = crate::Reg<timings0::TIMINGS0_SPEC>;
#[doc = "SMC Timings Register (CS_number = 0)"]
pub mod timings0;
#[doc = "MODE0 register accessor: an alias for `Reg<MODE0_SPEC>`"]
pub type MODE0 = crate::Reg<mode0::MODE0_SPEC>;
#[doc = "SMC Mode Register (CS_number = 0)"]
pub mod mode0;
#[doc = "SETUP1 register accessor: an alias for `Reg<SETUP1_SPEC>`"]
pub type SETUP1 = crate::Reg<setup1::SETUP1_SPEC>;
#[doc = "SMC Setup Register (CS_number = 1)"]
pub mod setup1;
#[doc = "PULSE1 register accessor: an alias for `Reg<PULSE1_SPEC>`"]
pub type PULSE1 = crate::Reg<pulse1::PULSE1_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 1)"]
pub mod pulse1;
#[doc = "CYCLE1 register accessor: an alias for `Reg<CYCLE1_SPEC>`"]
pub type CYCLE1 = crate::Reg<cycle1::CYCLE1_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 1)"]
pub mod cycle1;
#[doc = "TIMINGS1 register accessor: an alias for `Reg<TIMINGS1_SPEC>`"]
pub type TIMINGS1 = crate::Reg<timings1::TIMINGS1_SPEC>;
#[doc = "SMC Timings Register (CS_number = 1)"]
pub mod timings1;
#[doc = "MODE1 register accessor: an alias for `Reg<MODE1_SPEC>`"]
pub type MODE1 = crate::Reg<mode1::MODE1_SPEC>;
#[doc = "SMC Mode Register (CS_number = 1)"]
pub mod mode1;
#[doc = "SETUP2 register accessor: an alias for `Reg<SETUP2_SPEC>`"]
pub type SETUP2 = crate::Reg<setup2::SETUP2_SPEC>;
#[doc = "SMC Setup Register (CS_number = 2)"]
pub mod setup2;
#[doc = "PULSE2 register accessor: an alias for `Reg<PULSE2_SPEC>`"]
pub type PULSE2 = crate::Reg<pulse2::PULSE2_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 2)"]
pub mod pulse2;
#[doc = "CYCLE2 register accessor: an alias for `Reg<CYCLE2_SPEC>`"]
pub type CYCLE2 = crate::Reg<cycle2::CYCLE2_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 2)"]
pub mod cycle2;
#[doc = "TIMINGS2 register accessor: an alias for `Reg<TIMINGS2_SPEC>`"]
pub type TIMINGS2 = crate::Reg<timings2::TIMINGS2_SPEC>;
#[doc = "SMC Timings Register (CS_number = 2)"]
pub mod timings2;
#[doc = "MODE2 register accessor: an alias for `Reg<MODE2_SPEC>`"]
pub type MODE2 = crate::Reg<mode2::MODE2_SPEC>;
#[doc = "SMC Mode Register (CS_number = 2)"]
pub mod mode2;
#[doc = "SETUP3 register accessor: an alias for `Reg<SETUP3_SPEC>`"]
pub type SETUP3 = crate::Reg<setup3::SETUP3_SPEC>;
#[doc = "SMC Setup Register (CS_number = 3)"]
pub mod setup3;
#[doc = "PULSE3 register accessor: an alias for `Reg<PULSE3_SPEC>`"]
pub type PULSE3 = crate::Reg<pulse3::PULSE3_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 3)"]
pub mod pulse3;
#[doc = "CYCLE3 register accessor: an alias for `Reg<CYCLE3_SPEC>`"]
pub type CYCLE3 = crate::Reg<cycle3::CYCLE3_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 3)"]
pub mod cycle3;
#[doc = "TIMINGS3 register accessor: an alias for `Reg<TIMINGS3_SPEC>`"]
pub type TIMINGS3 = crate::Reg<timings3::TIMINGS3_SPEC>;
#[doc = "SMC Timings Register (CS_number = 3)"]
pub mod timings3;
#[doc = "MODE3 register accessor: an alias for `Reg<MODE3_SPEC>`"]
pub type MODE3 = crate::Reg<mode3::MODE3_SPEC>;
#[doc = "SMC Mode Register (CS_number = 3)"]
pub mod mode3;
#[doc = "SETUP4 register accessor: an alias for `Reg<SETUP4_SPEC>`"]
pub type SETUP4 = crate::Reg<setup4::SETUP4_SPEC>;
#[doc = "SMC Setup Register (CS_number = 4)"]
pub mod setup4;
#[doc = "PULSE4 register accessor: an alias for `Reg<PULSE4_SPEC>`"]
pub type PULSE4 = crate::Reg<pulse4::PULSE4_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 4)"]
pub mod pulse4;
#[doc = "CYCLE4 register accessor: an alias for `Reg<CYCLE4_SPEC>`"]
pub type CYCLE4 = crate::Reg<cycle4::CYCLE4_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 4)"]
pub mod cycle4;
#[doc = "TIMINGS4 register accessor: an alias for `Reg<TIMINGS4_SPEC>`"]
pub type TIMINGS4 = crate::Reg<timings4::TIMINGS4_SPEC>;
#[doc = "SMC Timings Register (CS_number = 4)"]
pub mod timings4;
#[doc = "MODE4 register accessor: an alias for `Reg<MODE4_SPEC>`"]
pub type MODE4 = crate::Reg<mode4::MODE4_SPEC>;
#[doc = "SMC Mode Register (CS_number = 4)"]
pub mod mode4;
#[doc = "SETUP5 register accessor: an alias for `Reg<SETUP5_SPEC>`"]
pub type SETUP5 = crate::Reg<setup5::SETUP5_SPEC>;
#[doc = "SMC Setup Register (CS_number = 5)"]
pub mod setup5;
#[doc = "PULSE5 register accessor: an alias for `Reg<PULSE5_SPEC>`"]
pub type PULSE5 = crate::Reg<pulse5::PULSE5_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 5)"]
pub mod pulse5;
#[doc = "CYCLE5 register accessor: an alias for `Reg<CYCLE5_SPEC>`"]
pub type CYCLE5 = crate::Reg<cycle5::CYCLE5_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 5)"]
pub mod cycle5;
#[doc = "TIMINGS5 register accessor: an alias for `Reg<TIMINGS5_SPEC>`"]
pub type TIMINGS5 = crate::Reg<timings5::TIMINGS5_SPEC>;
#[doc = "SMC Timings Register (CS_number = 5)"]
pub mod timings5;
#[doc = "MODE5 register accessor: an alias for `Reg<MODE5_SPEC>`"]
pub type MODE5 = crate::Reg<mode5::MODE5_SPEC>;
#[doc = "SMC Mode Register (CS_number = 5)"]
pub mod mode5;
#[doc = "SETUP6 register accessor: an alias for `Reg<SETUP6_SPEC>`"]
pub type SETUP6 = crate::Reg<setup6::SETUP6_SPEC>;
#[doc = "SMC Setup Register (CS_number = 6)"]
pub mod setup6;
#[doc = "PULSE6 register accessor: an alias for `Reg<PULSE6_SPEC>`"]
pub type PULSE6 = crate::Reg<pulse6::PULSE6_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 6)"]
pub mod pulse6;
#[doc = "CYCLE6 register accessor: an alias for `Reg<CYCLE6_SPEC>`"]
pub type CYCLE6 = crate::Reg<cycle6::CYCLE6_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 6)"]
pub mod cycle6;
#[doc = "TIMINGS6 register accessor: an alias for `Reg<TIMINGS6_SPEC>`"]
pub type TIMINGS6 = crate::Reg<timings6::TIMINGS6_SPEC>;
#[doc = "SMC Timings Register (CS_number = 6)"]
pub mod timings6;
#[doc = "MODE6 register accessor: an alias for `Reg<MODE6_SPEC>`"]
pub type MODE6 = crate::Reg<mode6::MODE6_SPEC>;
#[doc = "SMC Mode Register (CS_number = 6)"]
pub mod mode6;
#[doc = "SETUP7 register accessor: an alias for `Reg<SETUP7_SPEC>`"]
pub type SETUP7 = crate::Reg<setup7::SETUP7_SPEC>;
#[doc = "SMC Setup Register (CS_number = 7)"]
pub mod setup7;
#[doc = "PULSE7 register accessor: an alias for `Reg<PULSE7_SPEC>`"]
pub type PULSE7 = crate::Reg<pulse7::PULSE7_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 7)"]
pub mod pulse7;
#[doc = "CYCLE7 register accessor: an alias for `Reg<CYCLE7_SPEC>`"]
pub type CYCLE7 = crate::Reg<cycle7::CYCLE7_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 7)"]
pub mod cycle7;
#[doc = "TIMINGS7 register accessor: an alias for `Reg<TIMINGS7_SPEC>`"]
pub type TIMINGS7 = crate::Reg<timings7::TIMINGS7_SPEC>;
#[doc = "SMC Timings Register (CS_number = 7)"]
pub mod timings7;
#[doc = "MODE7 register accessor: an alias for `Reg<MODE7_SPEC>`"]
pub type MODE7 = crate::Reg<mode7::MODE7_SPEC>;
#[doc = "SMC Mode Register (CS_number = 7)"]
pub mod mode7;
#[doc = "OCMS register accessor: an alias for `Reg<OCMS_SPEC>`"]
pub type OCMS = crate::Reg<ocms::OCMS_SPEC>;
#[doc = "SMC OCMS Register"]
pub mod ocms;
#[doc = "KEY1 register accessor: an alias for `Reg<KEY1_SPEC>`"]
pub type KEY1 = crate::Reg<key1::KEY1_SPEC>;
#[doc = "SMC OCMS KEY1 Register"]
pub mod key1;
#[doc = "KEY2 register accessor: an alias for `Reg<KEY2_SPEC>`"]
pub type KEY2 = crate::Reg<key2::KEY2_SPEC>;
#[doc = "SMC OCMS KEY2 Register"]
pub mod key2;
#[doc = "WPCR register accessor: an alias for `Reg<WPCR_SPEC>`"]
pub type WPCR = crate::Reg<wpcr::WPCR_SPEC>;
#[doc = "Write Protection Control Register"]
pub mod wpcr;
#[doc = "WPSR register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod wpsr;
