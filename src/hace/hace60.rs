#[doc = "Register `HACE60` reader"]
pub type R = crate::R<Hace60Spec>;
#[doc = "Register `HACE60` writer"]
pub type W = crate::W<Hace60Spec>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `CmdQueueDataFormat` reader - Command Queue Data Format"]
pub type CmdQueueDataFormatR = crate::BitReader;
#[doc = "Field `CmdQueueDataFormat` writer - Command Queue Data Format"]
pub type CmdQueueDataFormatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DefeatureEnblAESDESCTRModeWith64bitCounter` reader - defeatureEnable AES/DES CTR Mode with 64-bit Counter"]
pub type DefeatureEnblAesdesctrmodeWith64bitCounterR = crate::BitReader;
#[doc = "Field `DefeatureEnblAESDESCTRModeWith64bitCounter` writer - defeatureEnable AES/DES CTR Mode with 64-bit Counter"]
pub type DefeatureEnblAesdesctrmodeWith64bitCounterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrRegDataFromCmdQueueData` reader - Enable Write Register Data from Command Queue Data"]
pub type EnblWrRegDataFromCmdQueueDataR = crate::BitReader;
#[doc = "Field `EnblWrRegDataFromCmdQueueData` writer - Enable Write Register Data from Command Queue Data"]
pub type EnblWrRegDataFromCmdQueueDataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblDRAMCmdQueueBuffer` reader - Enable DRAM Command Queue Buffer"]
pub type EnblDramcmdQueueBufferR = crate::BitReader;
#[doc = "Field `EnblDRAMCmdQueueBuffer` writer - Enable DRAM Command Queue Buffer"]
pub type EnblDramcmdQueueBufferW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:27 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 28 - Command Queue Data Format"]
    #[inline(always)]
    pub fn cmd_queue_data_format(&self) -> CmdQueueDataFormatR {
        CmdQueueDataFormatR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - defeatureEnable AES/DES CTR Mode with 64-bit Counter"]
    #[inline(always)]
    pub fn defeature_enbl_aesdesctrmode_with64bit_counter(
        &self,
    ) -> DefeatureEnblAesdesctrmodeWith64bitCounterR {
        DefeatureEnblAesdesctrmodeWith64bitCounterR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Write Register Data from Command Queue Data"]
    #[inline(always)]
    pub fn enbl_wr_reg_data_from_cmd_queue_data(&self) -> EnblWrRegDataFromCmdQueueDataR {
        EnblWrRegDataFromCmdQueueDataR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable DRAM Command Queue Buffer"]
    #[inline(always)]
    pub fn enbl_dramcmd_queue_buffer(&self) -> EnblDramcmdQueueBufferR {
        EnblDramcmdQueueBufferR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Command Queue Data Format"]
    #[inline(always)]
    pub fn cmd_queue_data_format(&mut self) -> CmdQueueDataFormatW<Hace60Spec> {
        CmdQueueDataFormatW::new(self, 28)
    }
    #[doc = "Bit 29 - defeatureEnable AES/DES CTR Mode with 64-bit Counter"]
    #[inline(always)]
    pub fn defeature_enbl_aesdesctrmode_with64bit_counter(
        &mut self,
    ) -> DefeatureEnblAesdesctrmodeWith64bitCounterW<Hace60Spec> {
        DefeatureEnblAesdesctrmodeWith64bitCounterW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Write Register Data from Command Queue Data"]
    #[inline(always)]
    pub fn enbl_wr_reg_data_from_cmd_queue_data(
        &mut self,
    ) -> EnblWrRegDataFromCmdQueueDataW<Hace60Spec> {
        EnblWrRegDataFromCmdQueueDataW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable DRAM Command Queue Buffer"]
    #[inline(always)]
    pub fn enbl_dramcmd_queue_buffer(&mut self) -> EnblDramcmdQueueBufferW<Hace60Spec> {
        EnblDramcmdQueueBufferW::new(self, 31)
    }
}
#[doc = "HAC Engine Feature Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace60::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace60::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hace60Spec;
impl crate::RegisterSpec for Hace60Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hace60::R`](R) reader structure"]
impl crate::Readable for Hace60Spec {}
#[doc = "`write(|w| ..)` method takes [`hace60::W`](W) writer structure"]
impl crate::Writable for Hace60Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HACE60 to value 0"]
impl crate::Resettable for Hace60Spec {}
