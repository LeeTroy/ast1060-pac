#[doc = "Register `UDMA03C` reader"]
pub type R = crate::R<Udma03cSpec>;
#[doc = "Register `UDMA03C` writer"]
pub type W = crate::W<Udma03cSpec>;
#[doc = "Field `UARTRXDMAINTSts` reader - UART RX DMA interrupt status"]
pub type UartrxdmaintstsR = crate::FieldReader<u16>;
#[doc = "Field `UARTRXDMAINTSts` writer - UART RX DMA interrupt status"]
pub type UartrxdmaintstsW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `UARTRXDMAINTSts1` reader - UART RX DMA interrupt status"]
pub type Uartrxdmaintsts1R = crate::FieldReader;
#[doc = "Field `UARTRXDMAINTSts1` writer - UART RX DMA interrupt status"]
pub type Uartrxdmaintsts1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:11 - UART RX DMA interrupt status"]
    #[inline(always)]
    pub fn uartrxdmaintsts(&self) -> UartrxdmaintstsR {
        UartrxdmaintstsR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - UART RX DMA interrupt status"]
    #[inline(always)]
    pub fn uartrxdmaintsts1(&self) -> Uartrxdmaintsts1R {
        Uartrxdmaintsts1R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - UART RX DMA interrupt status"]
    #[inline(always)]
    pub fn uartrxdmaintsts(&mut self) -> UartrxdmaintstsW<Udma03cSpec> {
        UartrxdmaintstsW::new(self, 0)
    }
    #[doc = "Bits 0:3 - UART RX DMA interrupt status"]
    #[inline(always)]
    pub fn uartrxdmaintsts1(&mut self) -> Uartrxdmaintsts1W<Udma03cSpec> {
        Uartrxdmaintsts1W::new(self, 0)
    }
}
#[doc = "UART RX DMA interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`udma03c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma03c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma03cSpec;
impl crate::RegisterSpec for Udma03cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma03c::R`](R) reader structure"]
impl crate::Readable for Udma03cSpec {}
#[doc = "`write(|w| ..)` method takes [`udma03c::W`](W) writer structure"]
impl crate::Writable for Udma03cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA03C to value 0"]
impl crate::Resettable for Udma03cSpec {}
