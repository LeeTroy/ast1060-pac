#[doc = "Register `FMC080` reader"]
pub type R = crate::R<Fmc080Spec>;
#[doc = "Register `FMC080` writer"]
pub type W = crate::W<Fmc080Spec>;
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaenbl {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable DMA operation"]
    EnableDmaOperation = 1,
}
impl From<Dmaenbl> for bool {
    #[inline(always)]
    fn from(variant: Dmaenbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEnbl` reader - DMA Enable"]
pub type DmaenblR = crate::BitReader<Dmaenbl>;
impl DmaenblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaenbl {
        match self.bits {
            false => Dmaenbl::Disable,
            true => Dmaenbl::EnableDmaOperation,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dmaenbl::Disable
    }
    #[doc = "Enable DMA operation"]
    #[inline(always)]
    pub fn is_enable_dma_operation(&self) -> bool {
        *self == Dmaenbl::EnableDmaOperation
    }
}
#[doc = "Field `DMAEnbl` writer - DMA Enable"]
pub type DmaenblW<'a, REG> = crate::BitWriter<'a, REG, Dmaenbl>;
impl<'a, REG> DmaenblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaenbl::Disable)
    }
    #[doc = "Enable DMA operation"]
    #[inline(always)]
    pub fn enable_dma_operation(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaenbl::EnableDmaOperation)
    }
}
#[doc = "DMA Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmadirection {
    #[doc = "0: Read flash, move from flash to external memory"]
    ReadFlashMoveFromFlashToExternalMemory = 0,
    #[doc = "1: Write flash, move from external memory to flash"]
    WriteFlashMoveFromExternalMemoryToFlash = 1,
}
impl From<Dmadirection> for bool {
    #[inline(always)]
    fn from(variant: Dmadirection) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMADirection` reader - DMA Direction"]
pub type DmadirectionR = crate::BitReader<Dmadirection>;
impl DmadirectionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmadirection {
        match self.bits {
            false => Dmadirection::ReadFlashMoveFromFlashToExternalMemory,
            true => Dmadirection::WriteFlashMoveFromExternalMemoryToFlash,
        }
    }
    #[doc = "Read flash, move from flash to external memory"]
    #[inline(always)]
    pub fn is_read_flash_move_from_flash_to_external_memory(&self) -> bool {
        *self == Dmadirection::ReadFlashMoveFromFlashToExternalMemory
    }
    #[doc = "Write flash, move from external memory to flash"]
    #[inline(always)]
    pub fn is_write_flash_move_from_external_memory_to_flash(&self) -> bool {
        *self == Dmadirection::WriteFlashMoveFromExternalMemoryToFlash
    }
}
#[doc = "Field `DMADirection` writer - DMA Direction"]
pub type DmadirectionW<'a, REG> = crate::BitWriter<'a, REG, Dmadirection>;
impl<'a, REG> DmadirectionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read flash, move from flash to external memory"]
    #[inline(always)]
    pub fn read_flash_move_from_flash_to_external_memory(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadirection::ReadFlashMoveFromFlashToExternalMemory)
    }
    #[doc = "Write flash, move from external memory to flash"]
    #[inline(always)]
    pub fn write_flash_move_from_external_memory_to_flash(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadirection::WriteFlashMoveFromExternalMemoryToFlash)
    }
}
#[doc = "CheckSum Calculation Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckSumCalculationOnly {
    #[doc = "0: Normal DMA operation"]
    NormalDmaOperation = 0,
    #[doc = "1: Checksum accumulation only"]
    ChecksumAccumulationOnly = 1,
}
impl From<CheckSumCalculationOnly> for bool {
    #[inline(always)]
    fn from(variant: CheckSumCalculationOnly) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CheckSumCalculationOnly` reader - CheckSum Calculation Only"]
pub type CheckSumCalculationOnlyR = crate::BitReader<CheckSumCalculationOnly>;
impl CheckSumCalculationOnlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CheckSumCalculationOnly {
        match self.bits {
            false => CheckSumCalculationOnly::NormalDmaOperation,
            true => CheckSumCalculationOnly::ChecksumAccumulationOnly,
        }
    }
    #[doc = "Normal DMA operation"]
    #[inline(always)]
    pub fn is_normal_dma_operation(&self) -> bool {
        *self == CheckSumCalculationOnly::NormalDmaOperation
    }
    #[doc = "Checksum accumulation only"]
    #[inline(always)]
    pub fn is_checksum_accumulation_only(&self) -> bool {
        *self == CheckSumCalculationOnly::ChecksumAccumulationOnly
    }
}
#[doc = "Field `CheckSumCalculationOnly` writer - CheckSum Calculation Only"]
pub type CheckSumCalculationOnlyW<'a, REG> = crate::BitWriter<'a, REG, CheckSumCalculationOnly>;
impl<'a, REG> CheckSumCalculationOnlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal DMA operation"]
    #[inline(always)]
    pub fn normal_dma_operation(self) -> &'a mut crate::W<REG> {
        self.variant(CheckSumCalculationOnly::NormalDmaOperation)
    }
    #[doc = "Checksum accumulation only"]
    #[inline(always)]
    pub fn checksum_accumulation_only(self) -> &'a mut crate::W<REG> {
        self.variant(CheckSumCalculationOnly::ChecksumAccumulationOnly)
    }
}
#[doc = "Calibration Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalibrationMode {
    #[doc = "0: Normal mode"]
    NormalMode = 0,
    #[doc = "1: Calibration mode. At this mode, SPI clock rate and read delay cycle parameters"]
    CalibrationModeAtThisModeSpiClockRateAndReadDelayCycleParameters = 1,
}
impl From<CalibrationMode> for bool {
    #[inline(always)]
    fn from(variant: CalibrationMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CalibrationMode` reader - Calibration Mode"]
pub type CalibrationModeR = crate::BitReader<CalibrationMode>;
impl CalibrationModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CalibrationMode {
        match self.bits {
            false => CalibrationMode::NormalMode,
            true => {
                CalibrationMode::CalibrationModeAtThisModeSpiClockRateAndReadDelayCycleParameters
            }
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == CalibrationMode::NormalMode
    }
    #[doc = "Calibration mode. At this mode, SPI clock rate and read delay cycle parameters"]
    #[inline(always)]
    pub fn is_calibration_mode_at_this_mode_spi_clock_rate_and_read_delay_cycle_parameters(
        &self,
    ) -> bool {
        *self == CalibrationMode::CalibrationModeAtThisModeSpiClockRateAndReadDelayCycleParameters
    }
}
#[doc = "Field `CalibrationMode` writer - Calibration Mode"]
pub type CalibrationModeW<'a, REG> = crate::BitWriter<'a, REG, CalibrationMode>;
impl<'a, REG> CalibrationModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(CalibrationMode::NormalMode)
    }
    #[doc = "Calibration mode. At this mode, SPI clock rate and read delay cycle parameters"]
    #[inline(always)]
    pub fn calibration_mode_at_this_mode_spi_clock_rate_and_read_delay_cycle_parameters(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(
            CalibrationMode::CalibrationModeAtThisModeSpiClockRateAndReadDelayCycleParameters,
        )
    }
}
#[doc = "DMA Buffer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmabufferMode {
    #[doc = "0: Memory mode, transfer to/from DRAM/SRAM."]
    MemoryModeTransferTofromDramsram = 0,
    #[doc = "1: Buffer mode, transfer to/from DMA buffer. The DMA buffer works as FIFO mode for data transfer, and DMA will stop temporarily when FIFO is empty or full."]
    BufferModeTransferTofromDmaBufferTheDmaBufferWorksAsFifoModeForDataTransferAndDmaWillStopTemporarilyWhenFifoIsEmptyOrFull =
        1,
}
impl From<DmabufferMode> for bool {
    #[inline(always)]
    fn from(variant: DmabufferMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMABufferMode` reader - DMA Buffer Mode"]
pub type DmabufferModeR = crate::BitReader<DmabufferMode>;
impl DmabufferModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmabufferMode {
        match self . bits { false => DmabufferMode :: MemoryModeTransferTofromDramsram , true => DmabufferMode :: BufferModeTransferTofromDmaBufferTheDmaBufferWorksAsFifoModeForDataTransferAndDmaWillStopTemporarilyWhenFifoIsEmptyOrFull , }
    }
    #[doc = "Memory mode, transfer to/from DRAM/SRAM."]
    #[inline(always)]
    pub fn is_memory_mode_transfer_tofrom_dramsram(&self) -> bool {
        *self == DmabufferMode::MemoryModeTransferTofromDramsram
    }
    #[doc = "Buffer mode, transfer to/from DMA buffer. The DMA buffer works as FIFO mode for data transfer, and DMA will stop temporarily when FIFO is empty or full."]
    #[inline(always)]
    pub fn is_buffer_mode_transfer_tofrom_dma_buffer_the_dma_buffer_works_as_fifo_mode_for_data_transfer_and_dma_will_stop_temporarily_when_fifo_is_empty_or_full(
        &self,
    ) -> bool {
        * self == DmabufferMode :: BufferModeTransferTofromDmaBufferTheDmaBufferWorksAsFifoModeForDataTransferAndDmaWillStopTemporarilyWhenFifoIsEmptyOrFull
    }
}
#[doc = "Field `DMABufferMode` writer - DMA Buffer Mode"]
pub type DmabufferModeW<'a, REG> = crate::BitWriter<'a, REG, DmabufferMode>;
impl<'a, REG> DmabufferModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Memory mode, transfer to/from DRAM/SRAM."]
    #[inline(always)]
    pub fn memory_mode_transfer_tofrom_dramsram(self) -> &'a mut crate::W<REG> {
        self.variant(DmabufferMode::MemoryModeTransferTofromDramsram)
    }
    #[doc = "Buffer mode, transfer to/from DMA buffer. The DMA buffer works as FIFO mode for data transfer, and DMA will stop temporarily when FIFO is empty or full."]
    #[inline(always)]
    pub fn buffer_mode_transfer_tofrom_dma_buffer_the_dma_buffer_works_as_fifo_mode_for_data_transfer_and_dma_will_stop_temporarily_when_fifo_is_empty_or_full(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (DmabufferMode :: BufferModeTransferTofromDmaBufferTheDmaBufferWorksAsFifoModeForDataTransferAndDmaWillStopTemporarilyWhenFifoIsEmptyOrFull)
    }
}
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `SPIReadDataInputDelayCycleSetting` reader - SPI read data input delay cycle setting"]
pub type SpireadDataInputDelayCycleSettingR = crate::FieldReader;
#[doc = "Field `SPIReadDataInputDelayCycleSetting` writer - SPI read data input delay cycle setting"]
pub type SpireadDataInputDelayCycleSettingW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPIClkFrequencySetting` reader - SPI clock frequency setting"]
pub type SpiclkFrequencySettingR = crate::FieldReader;
#[doc = "Field `SPIClkFrequencySetting` writer - SPI clock frequency setting"]
pub type SpiclkFrequencySettingW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaenbl(&self) -> DmaenblR {
        DmaenblR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Direction"]
    #[inline(always)]
    pub fn dmadirection(&self) -> DmadirectionR {
        DmadirectionR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CheckSum Calculation Only"]
    #[inline(always)]
    pub fn check_sum_calculation_only(&self) -> CheckSumCalculationOnlyR {
        CheckSumCalculationOnlyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Calibration Mode"]
    #[inline(always)]
    pub fn calibration_mode(&self) -> CalibrationModeR {
        CalibrationModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Buffer Mode"]
    #[inline(always)]
    pub fn dmabuffer_mode(&self) -> DmabufferModeR {
        DmabufferModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:15 - SPI read data input delay cycle setting"]
    #[inline(always)]
    pub fn spiread_data_input_delay_cycle_setting(&self) -> SpireadDataInputDelayCycleSettingR {
        SpireadDataInputDelayCycleSettingR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - SPI clock frequency setting"]
    #[inline(always)]
    pub fn spiclk_frequency_setting(&self) -> SpiclkFrequencySettingR {
        SpiclkFrequencySettingR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaenbl(&mut self) -> DmaenblW<Fmc080Spec> {
        DmaenblW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Direction"]
    #[inline(always)]
    pub fn dmadirection(&mut self) -> DmadirectionW<Fmc080Spec> {
        DmadirectionW::new(self, 1)
    }
    #[doc = "Bit 2 - CheckSum Calculation Only"]
    #[inline(always)]
    pub fn check_sum_calculation_only(&mut self) -> CheckSumCalculationOnlyW<Fmc080Spec> {
        CheckSumCalculationOnlyW::new(self, 2)
    }
    #[doc = "Bit 3 - Calibration Mode"]
    #[inline(always)]
    pub fn calibration_mode(&mut self) -> CalibrationModeW<Fmc080Spec> {
        CalibrationModeW::new(self, 3)
    }
    #[doc = "Bit 4 - DMA Buffer Mode"]
    #[inline(always)]
    pub fn dmabuffer_mode(&mut self) -> DmabufferModeW<Fmc080Spec> {
        DmabufferModeW::new(self, 4)
    }
    #[doc = "Bits 8:15 - SPI read data input delay cycle setting"]
    #[inline(always)]
    pub fn spiread_data_input_delay_cycle_setting(
        &mut self,
    ) -> SpireadDataInputDelayCycleSettingW<Fmc080Spec> {
        SpireadDataInputDelayCycleSettingW::new(self, 8)
    }
    #[doc = "Bits 16:19 - SPI clock frequency setting"]
    #[inline(always)]
    pub fn spiclk_frequency_setting(&mut self) -> SpiclkFrequencySettingW<Fmc080Spec> {
        SpiclkFrequencySettingW::new(self, 16)
    }
}
#[doc = "DMA Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc080::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc080::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc080Spec;
impl crate::RegisterSpec for Fmc080Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc080::R`](R) reader structure"]
impl crate::Readable for Fmc080Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc080::W`](W) writer structure"]
impl crate::Writable for Fmc080Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC080 to value 0"]
impl crate::Resettable for Fmc080Spec {}
