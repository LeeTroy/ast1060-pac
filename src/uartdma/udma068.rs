#[doc = "Register `UDMA068` reader"]
pub type R = crate::R<Udma068Spec>;
#[doc = "Register `UDMA068` writer"]
pub type W = crate::W<Udma068Spec>;
#[doc = "Field `UART2TXBufBaseAddr` reader - UART2 TX buffer base address"]
pub type Uart2txbufBaseAddrR = crate::FieldReader<u16>;
#[doc = "Field `UART2TXBufBaseAddr` writer - UART2 TX buffer base address"]
pub type Uart2txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART2 TX buffer base address"]
    #[inline(always)]
    pub fn uart2txbuf_base_addr(&self) -> Uart2txbufBaseAddrR {
        Uart2txbufBaseAddrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART2 TX buffer base address"]
    #[inline(always)]
    pub fn uart2txbuf_base_addr(&mut self) -> Uart2txbufBaseAddrW<Udma068Spec> {
        Uart2txbufBaseAddrW::new(self, 0)
    }
}
#[doc = "UART2 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma068::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma068::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma068Spec;
impl crate::RegisterSpec for Udma068Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma068::R`](R) reader structure"]
impl crate::Readable for Udma068Spec {}
#[doc = "`write(|w| ..)` method takes [`udma068::W`](W) writer structure"]
impl crate::Writable for Udma068Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA068 to value 0"]
impl crate::Resettable for Udma068Spec {}
