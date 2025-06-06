#[doc = "Register `FMC088` reader"]
pub type R = crate::R<Fmc088Spec>;
#[doc = "Register `FMC088` writer"]
pub type W = crate::W<Fmc088Spec>;
#[doc = "Field `DRAMSideStartAddrMemoryModeOnly` reader - DRAM side start address (Memory mode only)"]
pub type DramsideStartAddrMemoryModeOnlyR = crate::FieldReader<u32>;
#[doc = "Field `DRAMSideStartAddrMemoryModeOnly` writer - DRAM side start address (Memory mode only)"]
pub type DramsideStartAddrMemoryModeOnlyW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - DRAM side start address (Memory mode only)"]
    #[inline(always)]
    pub fn dramside_start_addr_memory_mode_only(&self) -> DramsideStartAddrMemoryModeOnlyR {
        DramsideStartAddrMemoryModeOnlyR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - DRAM side start address (Memory mode only)"]
    #[inline(always)]
    pub fn dramside_start_addr_memory_mode_only(
        &mut self,
    ) -> DramsideStartAddrMemoryModeOnlyW<Fmc088Spec> {
        DramsideStartAddrMemoryModeOnlyW::new(self, 2)
    }
}
#[doc = "DMA DRAM/SRAM Side Address\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc088::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc088::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc088Spec;
impl crate::RegisterSpec for Fmc088Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc088::R`](R) reader structure"]
impl crate::Readable for Fmc088Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc088::W`](W) writer structure"]
impl crate::Writable for Fmc088Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC088 to value 0"]
impl crate::Resettable for Fmc088Spec {}
