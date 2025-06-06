#[doc = "Register `UDMA020` reader"]
pub type R = crate::R<Udma020Spec>;
#[doc = "Register `UDMA020` writer"]
pub type W = crate::W<Udma020Spec>;
#[doc = "Field `UARTTXDMARst` reader - UART TX DMA reset"]
pub type UarttxdmarstR = crate::FieldReader<u16>;
#[doc = "Field `UARTTXDMARst` writer - UART TX DMA reset"]
pub type UarttxdmarstW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `UARTTXDMARst1` reader - UART TX DMA reset"]
pub type Uarttxdmarst1R = crate::FieldReader;
#[doc = "Field `UARTTXDMARst1` writer - UART TX DMA reset"]
pub type Uarttxdmarst1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:11 - UART TX DMA reset"]
    #[inline(always)]
    pub fn uarttxdmarst(&self) -> UarttxdmarstR {
        UarttxdmarstR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - UART TX DMA reset"]
    #[inline(always)]
    pub fn uarttxdmarst1(&self) -> Uarttxdmarst1R {
        Uarttxdmarst1R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - UART TX DMA reset"]
    #[inline(always)]
    pub fn uarttxdmarst(&mut self) -> UarttxdmarstW<Udma020Spec> {
        UarttxdmarstW::new(self, 0)
    }
    #[doc = "Bits 0:3 - UART TX DMA reset"]
    #[inline(always)]
    pub fn uarttxdmarst1(&mut self) -> Uarttxdmarst1W<Udma020Spec> {
        Uarttxdmarst1W::new(self, 0)
    }
}
#[doc = "UART TX DMA reset\n\nYou can [`read`](crate::Reg::read) this register and get [`udma020::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma020::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma020Spec;
impl crate::RegisterSpec for Udma020Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma020::R`](R) reader structure"]
impl crate::Readable for Udma020Spec {}
#[doc = "`write(|w| ..)` method takes [`udma020::W`](W) writer structure"]
impl crate::Writable for Udma020Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA020 to value 0"]
impl crate::Resettable for Udma020Spec {}
