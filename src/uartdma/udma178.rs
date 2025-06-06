#[doc = "Register `UDMA178` reader"]
pub type R = crate::R<Udma178Spec>;
#[doc = "Register `UDMA178` writer"]
pub type W = crate::W<Udma178Spec>;
#[doc = "Field `UART10TXBufBaseAddr` reader - UART10 TX buffer base address"]
pub type Uart10txbufBaseAddrR = crate::FieldReader<u16>;
#[doc = "Field `UART10TXBufBaseAddr` writer - UART10 TX buffer base address"]
pub type Uart10txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART10 TX buffer base address"]
    #[inline(always)]
    pub fn uart10txbuf_base_addr(&self) -> Uart10txbufBaseAddrR {
        Uart10txbufBaseAddrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART10 TX buffer base address"]
    #[inline(always)]
    pub fn uart10txbuf_base_addr(&mut self) -> Uart10txbufBaseAddrW<Udma178Spec> {
        Uart10txbufBaseAddrW::new(self, 0)
    }
}
#[doc = "UART10 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma178::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma178::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma178Spec;
impl crate::RegisterSpec for Udma178Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma178::R`](R) reader structure"]
impl crate::Readable for Udma178Spec {}
#[doc = "`write(|w| ..)` method takes [`udma178::W`](W) writer structure"]
impl crate::Writable for Udma178Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA178 to value 0"]
impl crate::Resettable for Udma178Spec {}
