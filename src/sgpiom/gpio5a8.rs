#[doc = "Register `GPIO5A8` reader"]
pub type R = crate::R<Gpio5a8Spec>;
#[doc = "Register `GPIO5A8` writer"]
pub type W = crate::W<Gpio5a8Spec>;
#[doc = "Field `PortSerialGPIOM70WDTRstToleranceEnbl` reader - Port Serial GPIOM\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpiom70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOM70WDTRstToleranceEnbl` writer - Port Serial GPIOM\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpiom70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPION70WDTRstToleranceEnbl` reader - Port Serial GPION\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpion70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortSerialGPION70WDTRstToleranceEnbl` writer - Port Serial GPION\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpion70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOO70WDTRstToleranceEnbl` reader - Port Serial GPIOO\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpioo70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOO70WDTRstToleranceEnbl` writer - Port Serial GPIOO\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpioo70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOP70WDTRstToleranceEnbl` reader - Port Serial GPIOP\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpiop70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOP70WDTRstToleranceEnbl` writer - Port Serial GPIOP\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpiop70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOM\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpiom70wdtrst_tolerance_enbl(
        &self,
    ) -> PortSerialGpiom70wdtrstToleranceEnblR {
        PortSerialGpiom70wdtrstToleranceEnblR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPION\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpion70wdtrst_tolerance_enbl(
        &self,
    ) -> PortSerialGpion70wdtrstToleranceEnblR {
        PortSerialGpion70wdtrstToleranceEnblR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOO\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpioo70wdtrst_tolerance_enbl(
        &self,
    ) -> PortSerialGpioo70wdtrstToleranceEnblR {
        PortSerialGpioo70wdtrstToleranceEnblR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOP\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpiop70wdtrst_tolerance_enbl(
        &self,
    ) -> PortSerialGpiop70wdtrstToleranceEnblR {
        PortSerialGpiop70wdtrstToleranceEnblR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOM\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpiom70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortSerialGpiom70wdtrstToleranceEnblW<Gpio5a8Spec> {
        PortSerialGpiom70wdtrstToleranceEnblW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPION\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpion70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortSerialGpion70wdtrstToleranceEnblW<Gpio5a8Spec> {
        PortSerialGpion70wdtrstToleranceEnblW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOO\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpioo70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortSerialGpioo70wdtrstToleranceEnblW<Gpio5a8Spec> {
        PortSerialGpioo70wdtrstToleranceEnblW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOP\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpiop70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortSerialGpiop70wdtrstToleranceEnblW<Gpio5a8Spec> {
        PortSerialGpiop70wdtrstToleranceEnblW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_M/N/O/P 1 Reset Tolerant Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio5a8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio5a8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio5a8Spec;
impl crate::RegisterSpec for Gpio5a8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio5a8::R`](R) reader structure"]
impl crate::Readable for Gpio5a8Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio5a8::W`](W) writer structure"]
impl crate::Writable for Gpio5a8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO5A8 to value 0"]
impl crate::Resettable for Gpio5a8Spec {}
