#[doc = "Register `SCUA14` reader"]
pub type R = crate::R<Scua14Spec>;
#[doc = "Register `SCUA14` writer"]
pub type W = crate::W<Scua14Spec>;
#[doc = "Field `SRAMBaseAddrForCM4F` reader - SRAM base address for CM4F"]
pub type SrambaseAddrForCm4fR = crate::FieldReader<u32>;
#[doc = "Field `SRAMBaseAddrForCM4F` writer - SRAM base address for CM4F"]
pub type SrambaseAddrForCm4fW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SRAM base address for CM4F"]
    #[inline(always)]
    pub fn srambase_addr_for_cm4f(&self) -> SrambaseAddrForCm4fR {
        SrambaseAddrForCm4fR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SRAM base address for CM4F"]
    #[inline(always)]
    pub fn srambase_addr_for_cm4f(&mut self) -> SrambaseAddrForCm4fW<Scua14Spec> {
        SrambaseAddrForCm4fW::new(self, 0)
    }
}
#[doc = "CM4F Memory Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scua14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scua14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scua14Spec;
impl crate::RegisterSpec for Scua14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scua14::R`](R) reader structure"]
impl crate::Readable for Scua14Spec {}
#[doc = "`write(|w| ..)` method takes [`scua14::W`](W) writer structure"]
impl crate::Writable for Scua14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUA14 to value 0"]
impl crate::Resettable for Scua14Spec {}
