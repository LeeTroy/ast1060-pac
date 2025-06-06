#[doc = "Register `FMC084` reader"]
pub type R = crate::R<Fmc084Spec>;
#[doc = "Register `FMC084` writer"]
pub type W = crate::W<Fmc084Spec>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `FlashSideStartAddr` reader - Flash side start address"]
pub type FlashSideStartAddrR = crate::FieldReader<u32>;
#[doc = "Field `FlashSideStartAddr` writer - Flash side start address"]
pub type FlashSideStartAddrW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:27 - Flash side start address"]
    #[inline(always)]
    pub fn flash_side_start_addr(&self) -> FlashSideStartAddrR {
        FlashSideStartAddrR::new((self.bits >> 2) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:27 - Flash side start address"]
    #[inline(always)]
    pub fn flash_side_start_addr(&mut self) -> FlashSideStartAddrW<Fmc084Spec> {
        FlashSideStartAddrW::new(self, 2)
    }
}
#[doc = "DMA Flash Side Address\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc084::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc084::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc084Spec;
impl crate::RegisterSpec for Fmc084Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc084::R`](R) reader structure"]
impl crate::Readable for Fmc084Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc084::W`](W) writer structure"]
impl crate::Writable for Fmc084Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC084 to value 0"]
impl crate::Resettable for Fmc084Spec {}
