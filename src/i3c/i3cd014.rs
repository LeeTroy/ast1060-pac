#[doc = "Register `I3CD014` reader"]
pub type R = crate::R<I3cd014Spec>;
#[doc = "Register `I3CD014` writer"]
pub type W = crate::W<I3cd014Spec>;
#[doc = "Field `RxDataPort` reader - Receive Data Port"]
pub type RxDataPortR = crate::FieldReader<u32>;
#[doc = "Field `TxDataPort` reader - Transmit Data Port"]
pub type TxDataPortR = crate::FieldReader<u32>;
#[doc = "Field `TxDataPort` writer - Transmit Data Port"]
pub type TxDataPortW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive Data Port"]
    #[inline(always)]
    pub fn rx_data_port(&self) -> RxDataPortR {
        RxDataPortR::new(self.bits)
    }
    #[doc = "Bits 0:31 - Transmit Data Port"]
    #[inline(always)]
    pub fn tx_data_port(&self) -> TxDataPortR {
        TxDataPortR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Data Port"]
    #[inline(always)]
    pub fn tx_data_port(&mut self) -> TxDataPortW<I3cd014Spec> {
        TxDataPortW::new(self, 0)
    }
}
#[doc = "Receive/Transmit Data Port Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd014Spec;
impl crate::RegisterSpec for I3cd014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd014::R`](R) reader structure"]
impl crate::Readable for I3cd014Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd014::W`](W) writer structure"]
impl crate::Writable for I3cd014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD014 to value 0"]
impl crate::Resettable for I3cd014Spec {}
