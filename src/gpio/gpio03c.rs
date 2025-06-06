#[doc = "Register `GPIO03C` reader"]
pub type R = crate::R<Gpio03cSpec>;
#[doc = "Register `GPIO03C` writer"]
pub type W = crate::W<Gpio03cSpec>;
#[doc = "Field `PortGPIOE70WDTRstToleranceEnbl` reader - Port GPIOE\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpioe70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortGPIOE70WDTRstToleranceEnbl` writer - Port GPIOE\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpioe70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOF70WDTRstToleranceEnbl` reader - Port GPIOF\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpiof70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortGPIOF70WDTRstToleranceEnbl` writer - Port GPIOF\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpiof70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOG70WDTRstToleranceEnbl` reader - Port GPIOG\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpiog70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortGPIOG70WDTRstToleranceEnbl` writer - Port GPIOG\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpiog70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOH70WDTRstToleranceEnbl` reader - Port GPIOH\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpioh70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortGPIOH70WDTRstToleranceEnbl` writer - Port GPIOH\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpioh70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOE\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpioe70wdtrst_tolerance_enbl(&self) -> PortGpioe70wdtrstToleranceEnblR {
        PortGpioe70wdtrstToleranceEnblR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOF\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpiof70wdtrst_tolerance_enbl(&self) -> PortGpiof70wdtrstToleranceEnblR {
        PortGpiof70wdtrstToleranceEnblR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOG\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpiog70wdtrst_tolerance_enbl(&self) -> PortGpiog70wdtrstToleranceEnblR {
        PortGpiog70wdtrstToleranceEnblR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOH\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpioh70wdtrst_tolerance_enbl(&self) -> PortGpioh70wdtrstToleranceEnblR {
        PortGpioh70wdtrstToleranceEnblR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOE\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpioe70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortGpioe70wdtrstToleranceEnblW<Gpio03cSpec> {
        PortGpioe70wdtrstToleranceEnblW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOF\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpiof70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortGpiof70wdtrstToleranceEnblW<Gpio03cSpec> {
        PortGpiof70wdtrstToleranceEnblW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOG\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpiog70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortGpiog70wdtrstToleranceEnblW<Gpio03cSpec> {
        PortGpiog70wdtrstToleranceEnblW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOH\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpioh70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortGpioh70wdtrstToleranceEnblW<Gpio03cSpec> {
        PortGpioh70wdtrstToleranceEnblW::new(self, 24)
    }
}
#[doc = "GPIO\\_E/F/G/H Reset Tolerant Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio03c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio03c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio03cSpec;
impl crate::RegisterSpec for Gpio03cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio03c::R`](R) reader structure"]
impl crate::Readable for Gpio03cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio03c::W`](W) writer structure"]
impl crate::Writable for Gpio03cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO03C to value 0"]
impl crate::Resettable for Gpio03cSpec {}
