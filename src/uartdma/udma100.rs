#[doc = "Register `UDMA100` reader"]
pub type R = crate::R<Udma100Spec>;
#[doc = "Register `UDMA100` writer"]
pub type W = crate::W<Udma100Spec>;
#[doc = "Field `UART7TXReadPointer` reader - UART7 TX read pointer"]
pub type Uart7txreadPointerR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART7 TX read pointer"]
    #[inline(always)]
    pub fn uart7txread_pointer(&self) -> Uart7txreadPointerR {
        Uart7txreadPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "UART7 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma100::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma100::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma100Spec;
impl crate::RegisterSpec for Udma100Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma100::R`](R) reader structure"]
impl crate::Readable for Udma100Spec {}
#[doc = "`write(|w| ..)` method takes [`udma100::W`](W) writer structure"]
impl crate::Writable for Udma100Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA100 to value 0"]
impl crate::Resettable for Udma100Spec {}
