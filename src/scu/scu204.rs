#[doc = "Register `SCU204` reader"]
pub type R = crate::R<Scu204Spec>;
#[doc = "Register `SCU204` writer"]
pub type W = crate::W<Scu204Spec>;
#[doc = "Field `HPLLBandwidthAdjustmentBit15` reader - 10]"]
pub type HpllbandwidthAdjustmentBit15R = crate::FieldReader<u16>;
#[doc = "Field `HPLLBandwidthAdjustmentBit15` writer - 10]"]
pub type HpllbandwidthAdjustmentBit15W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `Reserved1` reader - Reserved(0)"]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:11 - 10]"]
    #[inline(always)]
    pub fn hpllbandwidth_adjustment_bit15(&self) -> HpllbandwidthAdjustmentBit15R {
        HpllbandwidthAdjustmentBit15R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:30 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 12) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:11 - 10]"]
    #[inline(always)]
    pub fn hpllbandwidth_adjustment_bit15(&mut self) -> HpllbandwidthAdjustmentBit15W<Scu204Spec> {
        HpllbandwidthAdjustmentBit15W::new(self, 0)
    }
}
#[doc = "Extended H-PLL Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu204::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu204::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu204Spec;
impl crate::RegisterSpec for Scu204Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu204::R`](R) reader structure"]
impl crate::Readable for Scu204Spec {}
#[doc = "`write(|w| ..)` method takes [`scu204::W`](W) writer structure"]
impl crate::Writable for Scu204Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU204 to value 0x31"]
impl crate::Resettable for Scu204Spec {
    const RESET_VALUE: u32 = 0x31;
}
