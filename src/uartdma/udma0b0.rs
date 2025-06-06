#[doc = "Register `UDMA0B0` reader"]
pub type R = crate::R<Udma0b0Spec>;
#[doc = "Register `UDMA0B0` writer"]
pub type W = crate::W<Udma0b0Spec>;
#[doc = "Field `UART4RXReadPointer` reader - UART4 RX read pointer"]
pub type Uart4rxreadPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART4RXReadPointer` writer - UART4 RX read pointer"]
pub type Uart4rxreadPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART4 RX read pointer"]
    #[inline(always)]
    pub fn uart4rxread_pointer(&self) -> Uart4rxreadPointerR {
        Uart4rxreadPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART4 RX read pointer"]
    #[inline(always)]
    pub fn uart4rxread_pointer(&mut self) -> Uart4rxreadPointerW<Udma0b0Spec> {
        Uart4rxreadPointerW::new(self, 0)
    }
}
#[doc = "UART4 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma0b0Spec;
impl crate::RegisterSpec for Udma0b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma0b0::R`](R) reader structure"]
impl crate::Readable for Udma0b0Spec {}
#[doc = "`write(|w| ..)` method takes [`udma0b0::W`](W) writer structure"]
impl crate::Writable for Udma0b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA0B0 to value 0"]
impl crate::Resettable for Udma0b0Spec {}
