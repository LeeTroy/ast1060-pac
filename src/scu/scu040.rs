#[doc = "Register `SCU040` reader"]
pub type R = crate::R<Scu040Spec>;
#[doc = "Register `SCU040` writer"]
pub type W = crate::W<Scu040Spec>;
#[doc = "Field `EnblRstSRAMCtrlWhenFullChipWDResetOccur` reader - Enable reset SRAM controller when full chip Watchdog reset occur"]
pub type EnblRstSramctrlWhenFullChipWdresetOccurR = crate::BitReader;
#[doc = "Field `EnblRstSRAMCtrlWhenFullChipWDResetOccur` writer - Enable reset SRAM controller when full chip Watchdog reset occur"]
pub type EnblRstSramctrlWhenFullChipWdresetOccurW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable reset SRAM controller when full chip Watchdog reset occur"]
    #[inline(always)]
    pub fn enbl_rst_sramctrl_when_full_chip_wdreset_occur(
        &self,
    ) -> EnblRstSramctrlWhenFullChipWdresetOccurR {
        EnblRstSramctrlWhenFullChipWdresetOccurR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable reset SRAM controller when full chip Watchdog reset occur"]
    #[inline(always)]
    pub fn enbl_rst_sramctrl_when_full_chip_wdreset_occur(
        &mut self,
    ) -> EnblRstSramctrlWhenFullChipWdresetOccurW<Scu040Spec> {
        EnblRstSramctrlWhenFullChipWdresetOccurW::new(self, 0)
    }
}
#[doc = "System Reset Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu040::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu040::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu040Spec;
impl crate::RegisterSpec for Scu040Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu040::R`](R) reader structure"]
impl crate::Readable for Scu040Spec {}
#[doc = "`write(|w| ..)` method takes [`scu040::W`](W) writer structure"]
impl crate::Writable for Scu040Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU040 to value 0xffc3_fed8"]
impl crate::Resettable for Scu040Spec {
    const RESET_VALUE: u32 = 0xffc3_fed8;
}
