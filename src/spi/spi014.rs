#[doc = "Register `SPI014` reader"]
pub type R = crate::R<Spi014Spec>;
#[doc = "Register `SPI014` writer"]
pub type W = crate::W<Spi014Spec>;
#[doc = "SPICMDMODECommand Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SpicmdmodecmdMode1 {
    #[doc = "0: Auto-Read (0x03/0x13 + Address + Read data \\[1/2/3/4 bytes\\])"]
    AutoRead0x030x13_Address_ReadData1234Bytes = 0,
    #[doc = "1: Normal-Read (CMD + Address + Read data \\[1/2/3/4 bytes\\])"]
    NormalReadCmd_Address_ReadData1234Bytes = 1,
    #[doc = "2: Normal-Write (CMD + Address + Write data \\[1/2/3/4 bytes\\])"]
    NormalWriteCmd_Address_WriteData1234Bytes = 2,
    #[doc = "3: User-Mode (Read/write data \\[1/2/3/4 bytes\\])"]
    UserModeReadwriteData1234Bytes = 3,
}
impl From<SpicmdmodecmdMode1> for u8 {
    #[inline(always)]
    fn from(variant: SpicmdmodecmdMode1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SpicmdmodecmdMode1 {
    type Ux = u8;
}
impl crate::IsEnum for SpicmdmodecmdMode1 {}
#[doc = "Field `SPICMDMODECmdMode1` reader - SPICMDMODECommand Mode"]
pub type SpicmdmodecmdMode1R = crate::FieldReader<SpicmdmodecmdMode1>;
impl SpicmdmodecmdMode1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpicmdmodecmdMode1 {
        match self.bits {
            0 => SpicmdmodecmdMode1::AutoRead0x030x13_Address_ReadData1234Bytes,
            1 => SpicmdmodecmdMode1::NormalReadCmd_Address_ReadData1234Bytes,
            2 => SpicmdmodecmdMode1::NormalWriteCmd_Address_WriteData1234Bytes,
            3 => SpicmdmodecmdMode1::UserModeReadwriteData1234Bytes,
            _ => unreachable!(),
        }
    }
    #[doc = "Auto-Read (0x03/0x13 + Address + Read data \\[1/2/3/4 bytes\\])"]
    #[inline(always)]
    pub fn is_auto_read_0x030x13__address__read_data_1234_bytes(&self) -> bool {
        *self == SpicmdmodecmdMode1::AutoRead0x030x13_Address_ReadData1234Bytes
    }
    #[doc = "Normal-Read (CMD + Address + Read data \\[1/2/3/4 bytes\\])"]
    #[inline(always)]
    pub fn is_normal_read_cmd__address__read_data_1234_bytes(&self) -> bool {
        *self == SpicmdmodecmdMode1::NormalReadCmd_Address_ReadData1234Bytes
    }
    #[doc = "Normal-Write (CMD + Address + Write data \\[1/2/3/4 bytes\\])"]
    #[inline(always)]
    pub fn is_normal_write_cmd__address__write_data_1234_bytes(&self) -> bool {
        *self == SpicmdmodecmdMode1::NormalWriteCmd_Address_WriteData1234Bytes
    }
    #[doc = "User-Mode (Read/write data \\[1/2/3/4 bytes\\])"]
    #[inline(always)]
    pub fn is_user_mode_readwrite_data_1234_bytes(&self) -> bool {
        *self == SpicmdmodecmdMode1::UserModeReadwriteData1234Bytes
    }
}
#[doc = "Field `SPICMDMODECmdMode1` writer - SPICMDMODECommand Mode"]
pub type SpicmdmodecmdMode1W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, SpicmdmodecmdMode1, crate::Safe>;
impl<'a, REG> SpicmdmodecmdMode1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Auto-Read (0x03/0x13 + Address + Read data \\[1/2/3/4 bytes\\])"]
    #[inline(always)]
    pub fn auto_read_0x030x13__address__read_data_1234_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(SpicmdmodecmdMode1::AutoRead0x030x13_Address_ReadData1234Bytes)
    }
    #[doc = "Normal-Read (CMD + Address + Read data \\[1/2/3/4 bytes\\])"]
    #[inline(always)]
    pub fn normal_read_cmd__address__read_data_1234_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(SpicmdmodecmdMode1::NormalReadCmd_Address_ReadData1234Bytes)
    }
    #[doc = "Normal-Write (CMD + Address + Write data \\[1/2/3/4 bytes\\])"]
    #[inline(always)]
    pub fn normal_write_cmd__address__write_data_1234_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(SpicmdmodecmdMode1::NormalWriteCmd_Address_WriteData1234Bytes)
    }
    #[doc = "User-Mode (Read/write data \\[1/2/3/4 bytes\\])"]
    #[inline(always)]
    pub fn user_mode_readwrite_data_1234_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(SpicmdmodecmdMode1::UserModeReadwriteData1234Bytes)
    }
}
#[doc = "Field `CEStopActiveCtrl1` reader - CE# Stop Active Control"]
pub type CestopActiveCtrl1R = crate::BitReader;
#[doc = "Field `CEStopActiveCtrl1` writer - CE# Stop Active Control"]
pub type CestopActiveCtrl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Reserved Enable dual data input mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReservedEnblDualDataInputMode1 {
    #[doc = "0: 28] = \"010\" mode."]
    _28_010Mode = 0,
}
impl From<ReservedEnblDualDataInputMode1> for bool {
    #[inline(always)]
    fn from(variant: ReservedEnblDualDataInputMode1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ReservedEnblDualDataInputMode1` reader - Reserved Enable dual data input mode"]
pub type ReservedEnblDualDataInputMode1R = crate::BitReader<ReservedEnblDualDataInputMode1>;
impl ReservedEnblDualDataInputMode1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ReservedEnblDualDataInputMode1> {
        match self.bits {
            false => Some(ReservedEnblDualDataInputMode1::_28_010Mode),
            _ => None,
        }
    }
    #[doc = "28] = \"010\" mode."]
    #[inline(always)]
    pub fn is_28__010_mode(&self) -> bool {
        *self == ReservedEnblDualDataInputMode1::_28_010Mode
    }
}
#[doc = "Field `ReservedEnblDualDataInputMode1` writer - Reserved Enable dual data input mode"]
pub type ReservedEnblDualDataInputMode1W<'a, REG> =
    crate::BitWriter<'a, REG, ReservedEnblDualDataInputMode1>;
impl<'a, REG> ReservedEnblDualDataInputMode1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "28] = \"010\" mode."]
    #[inline(always)]
    pub fn _28__010_mode(self) -> &'a mut crate::W<REG> {
        self.variant(ReservedEnblDualDataInputMode1::_28_010Mode)
    }
}
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "MSB/LSB first control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsblsbfirstCtrl1 {
    #[doc = "0: MSB First \\htextA{(default for boot code)"]
    MsbFirstHtextAdefaultForBootCode = 0,
    #[doc = "1: LSB First"]
    LsbFirst = 1,
}
impl From<MsblsbfirstCtrl1> for bool {
    #[inline(always)]
    fn from(variant: MsblsbfirstCtrl1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSBLSBFirstCtrl1` reader - MSB/LSB first control"]
pub type MsblsbfirstCtrl1R = crate::BitReader<MsblsbfirstCtrl1>;
impl MsblsbfirstCtrl1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MsblsbfirstCtrl1 {
        match self.bits {
            false => MsblsbfirstCtrl1::MsbFirstHtextAdefaultForBootCode,
            true => MsblsbfirstCtrl1::LsbFirst,
        }
    }
    #[doc = "MSB First \\htextA{(default for boot code)"]
    #[inline(always)]
    pub fn is_msb_first_htext_adefault_for_boot_code(&self) -> bool {
        *self == MsblsbfirstCtrl1::MsbFirstHtextAdefaultForBootCode
    }
    #[doc = "LSB First"]
    #[inline(always)]
    pub fn is_lsb_first(&self) -> bool {
        *self == MsblsbfirstCtrl1::LsbFirst
    }
}
#[doc = "Field `MSBLSBFirstCtrl1` writer - MSB/LSB first control"]
pub type MsblsbfirstCtrl1W<'a, REG> = crate::BitWriter<'a, REG, MsblsbfirstCtrl1>;
impl<'a, REG> MsblsbfirstCtrl1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSB First \\htextA{(default for boot code)"]
    #[inline(always)]
    pub fn msb_first_htext_adefault_for_boot_code(self) -> &'a mut crate::W<REG> {
        self.variant(MsblsbfirstCtrl1::MsbFirstHtextAdefaultForBootCode)
    }
    #[doc = "LSB First"]
    #[inline(always)]
    pub fn lsb_first(self) -> &'a mut crate::W<REG> {
        self.variant(MsblsbfirstCtrl1::LsbFirst)
    }
}
#[doc = "Field `DummyCyclesBeforeDataForNormalReadCmdLowBits1` reader - Dummy cycles before data for Normal-Read command (low bits)"]
pub type DummyCyclesBeforeDataForNormalReadCmdLowBits1R = crate::FieldReader;
#[doc = "Field `DummyCyclesBeforeDataForNormalReadCmdLowBits1` writer - Dummy cycles before data for Normal-Read command (low bits)"]
pub type DummyCyclesBeforeDataForNormalReadCmdLowBits1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "SPI clock frequency selection (t-CK)\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SpiclkFrequencySelTck1 {
    #[doc = "0: baseclk + (16 * t-HCLK)"]
    Baseclk_16_Thclk = 0,
    #[doc = "1: baseclk + (14 * t-HCLK)"]
    Baseclk_14_Thclk = 1,
    #[doc = "2: baseclk + (12 * t-HCLK)"]
    Baseclk_12_Thclk = 2,
    #[doc = "3: baseclk + (10 * t-HCLK)"]
    Baseclk_10_Thclk = 3,
    #[doc = "4: baseclk + (~8 * t-HCLK)\\regdefmark"]
    Baseclk_8_Thclkregdefmark = 4,
    #[doc = "5: baseclk + (~6 * t-HCLK)"]
    Baseclk_6_Thclk = 5,
    #[doc = "6: baseclk + (~4 * t-HCLK)"]
    Baseclk_4_Thclk = 6,
    #[doc = "7: baseclk + (~2 * t-HCLK)"]
    Baseclk_2_Thclk = 7,
    #[doc = "8: baseclk + (15 * t-HCLK)"]
    Baseclk_15_Thclk = 8,
    #[doc = "9: baseclk + (13 * t-HCLK)"]
    Baseclk_13_Thclk = 9,
    #[doc = "10: baseclk + (11 * t-HCLK)"]
    Baseclk_11_Thclk = 10,
    #[doc = "11: baseclk + (~9 * t-HCLK)"]
    Baseclk_9_Thclk = 11,
    #[doc = "12: baseclk + (~7 * t-HCLK)"]
    Baseclk_7_Thclk = 12,
    #[doc = "13: baseclk + (~5 * t-HCLK)"]
    Baseclk_5_Thclk = 13,
    #[doc = "14: baseclk + (~3 * t-HCLK)"]
    Baseclk_3_Thclk = 14,
    #[doc = "15: baseclk + (~1 * t-HCLK) (only valid for baseclk selection not equal to 0)"]
    Baseclk_1_ThclkOnlyValidForBaseclkSelectionNotEqualTo0 = 15,
}
impl From<SpiclkFrequencySelTck1> for u8 {
    #[inline(always)]
    fn from(variant: SpiclkFrequencySelTck1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SpiclkFrequencySelTck1 {
    type Ux = u8;
}
impl crate::IsEnum for SpiclkFrequencySelTck1 {}
#[doc = "Field `SPIClkFrequencySelTCK1` reader - SPI clock frequency selection (t-CK)"]
pub type SpiclkFrequencySelTck1R = crate::FieldReader<SpiclkFrequencySelTck1>;
impl SpiclkFrequencySelTck1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpiclkFrequencySelTck1 {
        match self.bits {
            0 => SpiclkFrequencySelTck1::Baseclk_16_Thclk,
            1 => SpiclkFrequencySelTck1::Baseclk_14_Thclk,
            2 => SpiclkFrequencySelTck1::Baseclk_12_Thclk,
            3 => SpiclkFrequencySelTck1::Baseclk_10_Thclk,
            4 => SpiclkFrequencySelTck1::Baseclk_8_Thclkregdefmark,
            5 => SpiclkFrequencySelTck1::Baseclk_6_Thclk,
            6 => SpiclkFrequencySelTck1::Baseclk_4_Thclk,
            7 => SpiclkFrequencySelTck1::Baseclk_2_Thclk,
            8 => SpiclkFrequencySelTck1::Baseclk_15_Thclk,
            9 => SpiclkFrequencySelTck1::Baseclk_13_Thclk,
            10 => SpiclkFrequencySelTck1::Baseclk_11_Thclk,
            11 => SpiclkFrequencySelTck1::Baseclk_9_Thclk,
            12 => SpiclkFrequencySelTck1::Baseclk_7_Thclk,
            13 => SpiclkFrequencySelTck1::Baseclk_5_Thclk,
            14 => SpiclkFrequencySelTck1::Baseclk_3_Thclk,
            15 => SpiclkFrequencySelTck1::Baseclk_1_ThclkOnlyValidForBaseclkSelectionNotEqualTo0,
            _ => unreachable!(),
        }
    }
    #[doc = "baseclk + (16 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__16__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck1::Baseclk_16_Thclk
    }
    #[doc = "baseclk + (14 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__14__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck1::Baseclk_14_Thclk
    }
    #[doc = "baseclk + (12 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__12__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck1::Baseclk_12_Thclk
    }
    #[doc = "baseclk + (10 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__10__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck1::Baseclk_10_Thclk
    }
    #[doc = "baseclk + (~8 * t-HCLK)\\regdefmark"]
    #[inline(always)]
    pub fn is_baseclk__8__thclkregdefmark(&self) -> bool {
        *self == SpiclkFrequencySelTck1::Baseclk_8_Thclkregdefmark
    }
    #[doc = "baseclk + (~6 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__6__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck1::Baseclk_6_Thclk
    }
    #[doc = "baseclk + (~4 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__4__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck1::Baseclk_4_Thclk
    }
    #[doc = "baseclk + (~2 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__2__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck1::Baseclk_2_Thclk
    }
    #[doc = "baseclk + (15 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__15__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck1::Baseclk_15_Thclk
    }
    #[doc = "baseclk + (13 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__13__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck1::Baseclk_13_Thclk
    }
    #[doc = "baseclk + (11 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__11__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck1::Baseclk_11_Thclk
    }
    #[doc = "baseclk + (~9 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__9__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck1::Baseclk_9_Thclk
    }
    #[doc = "baseclk + (~7 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__7__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck1::Baseclk_7_Thclk
    }
    #[doc = "baseclk + (~5 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__5__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck1::Baseclk_5_Thclk
    }
    #[doc = "baseclk + (~3 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__3__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck1::Baseclk_3_Thclk
    }
    #[doc = "baseclk + (~1 * t-HCLK) (only valid for baseclk selection not equal to 0)"]
    #[inline(always)]
    pub fn is_baseclk__1__thclk_only_valid_for_baseclk_selection_not_equal_to_0(&self) -> bool {
        *self == SpiclkFrequencySelTck1::Baseclk_1_ThclkOnlyValidForBaseclkSelectionNotEqualTo0
    }
}
#[doc = "Field `SPIClkFrequencySelTCK1` writer - SPI clock frequency selection (t-CK)"]
pub type SpiclkFrequencySelTck1W<'a, REG> =
    crate::FieldWriter<'a, REG, 4, SpiclkFrequencySelTck1, crate::Safe>;
impl<'a, REG> SpiclkFrequencySelTck1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "baseclk + (16 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__16__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck1::Baseclk_16_Thclk)
    }
    #[doc = "baseclk + (14 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__14__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck1::Baseclk_14_Thclk)
    }
    #[doc = "baseclk + (12 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__12__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck1::Baseclk_12_Thclk)
    }
    #[doc = "baseclk + (10 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__10__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck1::Baseclk_10_Thclk)
    }
    #[doc = "baseclk + (~8 * t-HCLK)\\regdefmark"]
    #[inline(always)]
    pub fn baseclk__8__thclkregdefmark(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck1::Baseclk_8_Thclkregdefmark)
    }
    #[doc = "baseclk + (~6 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__6__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck1::Baseclk_6_Thclk)
    }
    #[doc = "baseclk + (~4 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__4__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck1::Baseclk_4_Thclk)
    }
    #[doc = "baseclk + (~2 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__2__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck1::Baseclk_2_Thclk)
    }
    #[doc = "baseclk + (15 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__15__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck1::Baseclk_15_Thclk)
    }
    #[doc = "baseclk + (13 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__13__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck1::Baseclk_13_Thclk)
    }
    #[doc = "baseclk + (11 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__11__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck1::Baseclk_11_Thclk)
    }
    #[doc = "baseclk + (~9 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__9__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck1::Baseclk_9_Thclk)
    }
    #[doc = "baseclk + (~7 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__7__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck1::Baseclk_7_Thclk)
    }
    #[doc = "baseclk + (~5 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__5__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck1::Baseclk_5_Thclk)
    }
    #[doc = "baseclk + (~3 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__3__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck1::Baseclk_3_Thclk)
    }
    #[doc = "baseclk + (~1 * t-HCLK) (only valid for baseclk selection not equal to 0)"]
    #[inline(always)]
    pub fn baseclk__1__thclk_only_valid_for_baseclk_selection_not_equal_to_0(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck1::Baseclk_1_ThclkOnlyValidForBaseclkSelectionNotEqualTo0)
    }
}
#[doc = "Disable SPI flash read/write command merge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisSpiflashReadwrCmdMerge1 {
    #[doc = "0: Enable"]
    Enable = 0,
    #[doc = "1: Disable (with performance penalty)"]
    DisableWithPerformancePenalty = 1,
}
impl From<DisSpiflashReadwrCmdMerge1> for bool {
    #[inline(always)]
    fn from(variant: DisSpiflashReadwrCmdMerge1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DisSPIFlashReadwrCmdMerge1` reader - Disable SPI flash read/write command merge"]
pub type DisSpiflashReadwrCmdMerge1R = crate::BitReader<DisSpiflashReadwrCmdMerge1>;
impl DisSpiflashReadwrCmdMerge1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DisSpiflashReadwrCmdMerge1 {
        match self.bits {
            false => DisSpiflashReadwrCmdMerge1::Enable,
            true => DisSpiflashReadwrCmdMerge1::DisableWithPerformancePenalty,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DisSpiflashReadwrCmdMerge1::Enable
    }
    #[doc = "Disable (with performance penalty)"]
    #[inline(always)]
    pub fn is_disable_with_performance_penalty(&self) -> bool {
        *self == DisSpiflashReadwrCmdMerge1::DisableWithPerformancePenalty
    }
}
#[doc = "Field `DisSPIFlashReadwrCmdMerge1` writer - Disable SPI flash read/write command merge"]
pub type DisSpiflashReadwrCmdMerge1W<'a, REG> =
    crate::BitWriter<'a, REG, DisSpiflashReadwrCmdMerge1>;
impl<'a, REG> DisSpiflashReadwrCmdMerge1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DisSpiflashReadwrCmdMerge1::Enable)
    }
    #[doc = "Disable (with performance penalty)"]
    #[inline(always)]
    pub fn disable_with_performance_penalty(self) -> &'a mut crate::W<REG> {
        self.variant(DisSpiflashReadwrCmdMerge1::DisableWithPerformancePenalty)
    }
}
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DummyCyclesBeforeDataForNormalReadCmdHighBits1` reader - Dummy cycles before data for Normal-Read command (high bits)"]
pub type DummyCyclesBeforeDataForNormalReadCmdHighBits1R = crate::BitReader;
#[doc = "Field `DummyCyclesBeforeDataForNormalReadCmdHighBits1` writer - Dummy cycles before data for Normal-Read command (high bits)"]
pub type DummyCyclesBeforeDataForNormalReadCmdHighBits1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Dummy cycle command output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DummyCycleCmdOutput1 {
    #[doc = "0: dummy cycle no command output"]
    DummyCycleNoCommandOutput = 0,
    #[doc = "1: first dummy cycle has command output"]
    FirstDummyCycleHasCommandOutput = 1,
}
impl From<DummyCycleCmdOutput1> for bool {
    #[inline(always)]
    fn from(variant: DummyCycleCmdOutput1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DummyCycleCmdOutput1` reader - Dummy cycle command output"]
pub type DummyCycleCmdOutput1R = crate::BitReader<DummyCycleCmdOutput1>;
impl DummyCycleCmdOutput1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DummyCycleCmdOutput1 {
        match self.bits {
            false => DummyCycleCmdOutput1::DummyCycleNoCommandOutput,
            true => DummyCycleCmdOutput1::FirstDummyCycleHasCommandOutput,
        }
    }
    #[doc = "dummy cycle no command output"]
    #[inline(always)]
    pub fn is_dummy_cycle_no_command_output(&self) -> bool {
        *self == DummyCycleCmdOutput1::DummyCycleNoCommandOutput
    }
    #[doc = "first dummy cycle has command output"]
    #[inline(always)]
    pub fn is_first_dummy_cycle_has_command_output(&self) -> bool {
        *self == DummyCycleCmdOutput1::FirstDummyCycleHasCommandOutput
    }
}
#[doc = "Field `DummyCycleCmdOutput1` writer - Dummy cycle command output"]
pub type DummyCycleCmdOutput1W<'a, REG> = crate::BitWriter<'a, REG, DummyCycleCmdOutput1>;
impl<'a, REG> DummyCycleCmdOutput1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "dummy cycle no command output"]
    #[inline(always)]
    pub fn dummy_cycle_no_command_output(self) -> &'a mut crate::W<REG> {
        self.variant(DummyCycleCmdOutput1::DummyCycleNoCommandOutput)
    }
    #[doc = "first dummy cycle has command output"]
    #[inline(always)]
    pub fn first_dummy_cycle_has_command_output(self) -> &'a mut crate::W<REG> {
        self.variant(DummyCycleCmdOutput1::FirstDummyCycleHasCommandOutput)
    }
}
#[doc = "Field `SPICmd1` reader - SPI Command"]
pub type Spicmd1R = crate::FieldReader;
#[doc = "Field `SPICmd1` writer - SPI Command"]
pub type Spicmd1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "SPI base clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SpibaseClkSel1 {
    #[doc = "0: baseclk = 0 * t-HCLK"]
    Baseclk_0_Thclk = 0,
    #[doc = "1: baseclk = 16 * t-HCLK"]
    Baseclk_16_Thclk = 1,
    #[doc = "2: baseclk = 32 * t-HCLK"]
    Baseclk_32_Thclk = 2,
    #[doc = "3: baseclk = 48 * t-HCLK"]
    Baseclk_48_Thclk = 3,
    #[doc = "15: baseclk = 240 * t-HCLK"]
    Baseclk_240_Thclk = 15,
}
impl From<SpibaseClkSel1> for u8 {
    #[inline(always)]
    fn from(variant: SpibaseClkSel1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SpibaseClkSel1 {
    type Ux = u8;
}
impl crate::IsEnum for SpibaseClkSel1 {}
#[doc = "Field `SPIBaseClkSel1` reader - SPI base clock selection"]
pub type SpibaseClkSel1R = crate::FieldReader<SpibaseClkSel1>;
impl SpibaseClkSel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SpibaseClkSel1> {
        match self.bits {
            0 => Some(SpibaseClkSel1::Baseclk_0_Thclk),
            1 => Some(SpibaseClkSel1::Baseclk_16_Thclk),
            2 => Some(SpibaseClkSel1::Baseclk_32_Thclk),
            3 => Some(SpibaseClkSel1::Baseclk_48_Thclk),
            15 => Some(SpibaseClkSel1::Baseclk_240_Thclk),
            _ => None,
        }
    }
    #[doc = "baseclk = 0 * t-HCLK"]
    #[inline(always)]
    pub fn is_baseclk__0___thclk(&self) -> bool {
        *self == SpibaseClkSel1::Baseclk_0_Thclk
    }
    #[doc = "baseclk = 16 * t-HCLK"]
    #[inline(always)]
    pub fn is_baseclk__16__thclk(&self) -> bool {
        *self == SpibaseClkSel1::Baseclk_16_Thclk
    }
    #[doc = "baseclk = 32 * t-HCLK"]
    #[inline(always)]
    pub fn is_baseclk__32__thclk(&self) -> bool {
        *self == SpibaseClkSel1::Baseclk_32_Thclk
    }
    #[doc = "baseclk = 48 * t-HCLK"]
    #[inline(always)]
    pub fn is_baseclk__48__thclk(&self) -> bool {
        *self == SpibaseClkSel1::Baseclk_48_Thclk
    }
    #[doc = "baseclk = 240 * t-HCLK"]
    #[inline(always)]
    pub fn is_baseclk__240__thclk(&self) -> bool {
        *self == SpibaseClkSel1::Baseclk_240_Thclk
    }
}
#[doc = "Field `SPIBaseClkSel1` writer - SPI base clock selection"]
pub type SpibaseClkSel1W<'a, REG> = crate::FieldWriter<'a, REG, 4, SpibaseClkSel1>;
impl<'a, REG> SpibaseClkSel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "baseclk = 0 * t-HCLK"]
    #[inline(always)]
    pub fn baseclk__0___thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpibaseClkSel1::Baseclk_0_Thclk)
    }
    #[doc = "baseclk = 16 * t-HCLK"]
    #[inline(always)]
    pub fn baseclk__16__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpibaseClkSel1::Baseclk_16_Thclk)
    }
    #[doc = "baseclk = 32 * t-HCLK"]
    #[inline(always)]
    pub fn baseclk__32__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpibaseClkSel1::Baseclk_32_Thclk)
    }
    #[doc = "baseclk = 48 * t-HCLK"]
    #[inline(always)]
    pub fn baseclk__48__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpibaseClkSel1::Baseclk_48_Thclk)
    }
    #[doc = "baseclk = 240 * t-HCLK"]
    #[inline(always)]
    pub fn baseclk__240__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpibaseClkSel1::Baseclk_240_Thclk)
    }
}
#[doc = "IO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iomode1 {
    #[doc = "0: single bit."]
    SingleBit = 0,
    #[doc = "2: dual bit read/write, data cycle only."]
    DualBitReadwriteDataCycleOnly = 2,
    #[doc = "3: dual bit read/write, including address and dummy byte cycle."]
    DualBitReadwriteIncludingAddressAndDummyByteCycle = 3,
    #[doc = "4: quad bit read/write, data cycle only."]
    QuadBitReadwriteDataCycleOnly = 4,
    #[doc = "5: quad bit read/write, including address and dummy byte cycle."]
    QuadBitReadwriteIncludingAddressAndDummyByteCycle = 5,
    #[doc = "8: QPI mode, quad bit on command/address/data cycles."]
    QpiModeQuadBitOnCommandaddressdataCycles = 8,
}
impl From<Iomode1> for u8 {
    #[inline(always)]
    fn from(variant: Iomode1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iomode1 {
    type Ux = u8;
}
impl crate::IsEnum for Iomode1 {}
#[doc = "Field `IOMode1` reader - IO Mode"]
pub type Iomode1R = crate::FieldReader<Iomode1>;
impl Iomode1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Iomode1> {
        match self.bits {
            0 => Some(Iomode1::SingleBit),
            2 => Some(Iomode1::DualBitReadwriteDataCycleOnly),
            3 => Some(Iomode1::DualBitReadwriteIncludingAddressAndDummyByteCycle),
            4 => Some(Iomode1::QuadBitReadwriteDataCycleOnly),
            5 => Some(Iomode1::QuadBitReadwriteIncludingAddressAndDummyByteCycle),
            8 => Some(Iomode1::QpiModeQuadBitOnCommandaddressdataCycles),
            _ => None,
        }
    }
    #[doc = "single bit."]
    #[inline(always)]
    pub fn is_single_bit(&self) -> bool {
        *self == Iomode1::SingleBit
    }
    #[doc = "dual bit read/write, data cycle only."]
    #[inline(always)]
    pub fn is_dual_bit_readwrite_data_cycle_only(&self) -> bool {
        *self == Iomode1::DualBitReadwriteDataCycleOnly
    }
    #[doc = "dual bit read/write, including address and dummy byte cycle."]
    #[inline(always)]
    pub fn is_dual_bit_readwrite_including_address_and_dummy_byte_cycle(&self) -> bool {
        *self == Iomode1::DualBitReadwriteIncludingAddressAndDummyByteCycle
    }
    #[doc = "quad bit read/write, data cycle only."]
    #[inline(always)]
    pub fn is_quad_bit_readwrite_data_cycle_only(&self) -> bool {
        *self == Iomode1::QuadBitReadwriteDataCycleOnly
    }
    #[doc = "quad bit read/write, including address and dummy byte cycle."]
    #[inline(always)]
    pub fn is_quad_bit_readwrite_including_address_and_dummy_byte_cycle(&self) -> bool {
        *self == Iomode1::QuadBitReadwriteIncludingAddressAndDummyByteCycle
    }
    #[doc = "QPI mode, quad bit on command/address/data cycles."]
    #[inline(always)]
    pub fn is_qpi_mode_quad_bit_on_commandaddressdata_cycles(&self) -> bool {
        *self == Iomode1::QpiModeQuadBitOnCommandaddressdataCycles
    }
}
#[doc = "Field `IOMode1` writer - IO Mode"]
pub type Iomode1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Iomode1>;
impl<'a, REG> Iomode1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "single bit."]
    #[inline(always)]
    pub fn single_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Iomode1::SingleBit)
    }
    #[doc = "dual bit read/write, data cycle only."]
    #[inline(always)]
    pub fn dual_bit_readwrite_data_cycle_only(self) -> &'a mut crate::W<REG> {
        self.variant(Iomode1::DualBitReadwriteDataCycleOnly)
    }
    #[doc = "dual bit read/write, including address and dummy byte cycle."]
    #[inline(always)]
    pub fn dual_bit_readwrite_including_address_and_dummy_byte_cycle(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(Iomode1::DualBitReadwriteIncludingAddressAndDummyByteCycle)
    }
    #[doc = "quad bit read/write, data cycle only."]
    #[inline(always)]
    pub fn quad_bit_readwrite_data_cycle_only(self) -> &'a mut crate::W<REG> {
        self.variant(Iomode1::QuadBitReadwriteDataCycleOnly)
    }
    #[doc = "quad bit read/write, including address and dummy byte cycle."]
    #[inline(always)]
    pub fn quad_bit_readwrite_including_address_and_dummy_byte_cycle(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(Iomode1::QuadBitReadwriteIncludingAddressAndDummyByteCycle)
    }
    #[doc = "QPI mode, quad bit on command/address/data cycles."]
    #[inline(always)]
    pub fn qpi_mode_quad_bit_on_commandaddressdata_cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Iomode1::QpiModeQuadBitOnCommandaddressdataCycles)
    }
}
impl R {
    #[doc = "Bits 0:1 - SPICMDMODECommand Mode"]
    #[inline(always)]
    pub fn spicmdmodecmd_mode1(&self) -> SpicmdmodecmdMode1R {
        SpicmdmodecmdMode1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - CE# Stop Active Control"]
    #[inline(always)]
    pub fn cestop_active_ctrl1(&self) -> CestopActiveCtrl1R {
        CestopActiveCtrl1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved Enable dual data input mode"]
    #[inline(always)]
    pub fn reserved_enbl_dual_data_input_mode1(&self) -> ReservedEnblDualDataInputMode1R {
        ReservedEnblDualDataInputMode1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MSB/LSB first control"]
    #[inline(always)]
    pub fn msblsbfirst_ctrl1(&self) -> MsblsbfirstCtrl1R {
        MsblsbfirstCtrl1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Dummy cycles before data for Normal-Read command (low bits)"]
    #[inline(always)]
    pub fn dummy_cycles_before_data_for_normal_read_cmd_low_bits1(
        &self,
    ) -> DummyCyclesBeforeDataForNormalReadCmdLowBits1R {
        DummyCyclesBeforeDataForNormalReadCmdLowBits1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - SPI clock frequency selection (t-CK)"]
    #[inline(always)]
    pub fn spiclk_frequency_sel_tck1(&self) -> SpiclkFrequencySelTck1R {
        SpiclkFrequencySelTck1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Disable SPI flash read/write command merge"]
    #[inline(always)]
    pub fn dis_spiflash_readwr_cmd_merge1(&self) -> DisSpiflashReadwrCmdMerge1R {
        DisSpiflashReadwrCmdMerge1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Dummy cycles before data for Normal-Read command (high bits)"]
    #[inline(always)]
    pub fn dummy_cycles_before_data_for_normal_read_cmd_high_bits1(
        &self,
    ) -> DummyCyclesBeforeDataForNormalReadCmdHighBits1R {
        DummyCyclesBeforeDataForNormalReadCmdHighBits1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Dummy cycle command output"]
    #[inline(always)]
    pub fn dummy_cycle_cmd_output1(&self) -> DummyCycleCmdOutput1R {
        DummyCycleCmdOutput1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - SPI Command"]
    #[inline(always)]
    pub fn spicmd1(&self) -> Spicmd1R {
        Spicmd1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - SPI base clock selection"]
    #[inline(always)]
    pub fn spibase_clk_sel1(&self) -> SpibaseClkSel1R {
        SpibaseClkSel1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - IO Mode"]
    #[inline(always)]
    pub fn iomode1(&self) -> Iomode1R {
        Iomode1R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPICMDMODECommand Mode"]
    #[inline(always)]
    pub fn spicmdmodecmd_mode1(&mut self) -> SpicmdmodecmdMode1W<Spi014Spec> {
        SpicmdmodecmdMode1W::new(self, 0)
    }
    #[doc = "Bit 2 - CE# Stop Active Control"]
    #[inline(always)]
    pub fn cestop_active_ctrl1(&mut self) -> CestopActiveCtrl1W<Spi014Spec> {
        CestopActiveCtrl1W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved Enable dual data input mode"]
    #[inline(always)]
    pub fn reserved_enbl_dual_data_input_mode1(
        &mut self,
    ) -> ReservedEnblDualDataInputMode1W<Spi014Spec> {
        ReservedEnblDualDataInputMode1W::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Spi014Spec> {
        Reserved3W::new(self, 4)
    }
    #[doc = "Bit 5 - MSB/LSB first control"]
    #[inline(always)]
    pub fn msblsbfirst_ctrl1(&mut self) -> MsblsbfirstCtrl1W<Spi014Spec> {
        MsblsbfirstCtrl1W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Dummy cycles before data for Normal-Read command (low bits)"]
    #[inline(always)]
    pub fn dummy_cycles_before_data_for_normal_read_cmd_low_bits1(
        &mut self,
    ) -> DummyCyclesBeforeDataForNormalReadCmdLowBits1W<Spi014Spec> {
        DummyCyclesBeforeDataForNormalReadCmdLowBits1W::new(self, 6)
    }
    #[doc = "Bits 8:11 - SPI clock frequency selection (t-CK)"]
    #[inline(always)]
    pub fn spiclk_frequency_sel_tck1(&mut self) -> SpiclkFrequencySelTck1W<Spi014Spec> {
        SpiclkFrequencySelTck1W::new(self, 8)
    }
    #[doc = "Bit 12 - Disable SPI flash read/write command merge"]
    #[inline(always)]
    pub fn dis_spiflash_readwr_cmd_merge1(&mut self) -> DisSpiflashReadwrCmdMerge1W<Spi014Spec> {
        DisSpiflashReadwrCmdMerge1W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Spi014Spec> {
        Reserved1W::new(self, 13)
    }
    #[doc = "Bit 14 - Dummy cycles before data for Normal-Read command (high bits)"]
    #[inline(always)]
    pub fn dummy_cycles_before_data_for_normal_read_cmd_high_bits1(
        &mut self,
    ) -> DummyCyclesBeforeDataForNormalReadCmdHighBits1W<Spi014Spec> {
        DummyCyclesBeforeDataForNormalReadCmdHighBits1W::new(self, 14)
    }
    #[doc = "Bit 15 - Dummy cycle command output"]
    #[inline(always)]
    pub fn dummy_cycle_cmd_output1(&mut self) -> DummyCycleCmdOutput1W<Spi014Spec> {
        DummyCycleCmdOutput1W::new(self, 15)
    }
    #[doc = "Bits 16:23 - SPI Command"]
    #[inline(always)]
    pub fn spicmd1(&mut self) -> Spicmd1W<Spi014Spec> {
        Spicmd1W::new(self, 16)
    }
    #[doc = "Bits 24:27 - SPI base clock selection"]
    #[inline(always)]
    pub fn spibase_clk_sel1(&mut self) -> SpibaseClkSel1W<Spi014Spec> {
        SpibaseClkSel1W::new(self, 24)
    }
    #[doc = "Bits 28:31 - IO Mode"]
    #[inline(always)]
    pub fn iomode1(&mut self) -> Iomode1W<Spi014Spec> {
        Iomode1W::new(self, 28)
    }
}
#[doc = "CE1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi014Spec;
impl crate::RegisterSpec for Spi014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi014::R`](R) reader structure"]
impl crate::Readable for Spi014Spec {}
#[doc = "`write(|w| ..)` method takes [`spi014::W`](W) writer structure"]
impl crate::Writable for Spi014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI014 to value 0x0400"]
impl crate::Resettable for Spi014Spec {
    const RESET_VALUE: u32 = 0x0400;
}
