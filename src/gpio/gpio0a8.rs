#[doc = "Register `GPIO0A8` reader"]
pub type R = crate::R<Gpio0a8Spec>;
#[doc = "Register `GPIO0A8` writer"]
pub type W = crate::W<Gpio0a8Spec>;
#[doc = "Field `PortGPIOI70INTStsReg` reader - Port GPIOI\\[7:0\\] interrupt status register"]
pub type PortGpioi70intstsRegR = crate::FieldReader;
#[doc = "Field `PortGPIOI70INTStsReg` writer - Port GPIOI\\[7:0\\] interrupt status register"]
pub type PortGpioi70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOJ70INTStsReg` reader - Port GPIOJ\\[7:0\\] interrupt status register"]
pub type PortGpioj70intstsRegR = crate::FieldReader;
#[doc = "Field `PortGPIOJ70INTStsReg` writer - Port GPIOJ\\[7:0\\] interrupt status register"]
pub type PortGpioj70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOK70INTStsReg` reader - Port GPIOK\\[7:0\\] interrupt status register"]
pub type PortGpiok70intstsRegR = crate::FieldReader;
#[doc = "Field `PortGPIOK70INTStsReg` writer - Port GPIOK\\[7:0\\] interrupt status register"]
pub type PortGpiok70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOL70INTStsReg` reader - Port GPIOL\\[7:0\\] interrupt status register"]
pub type PortGpiol70intstsRegR = crate::FieldReader;
#[doc = "Field `PortGPIOL70INTStsReg` writer - Port GPIOL\\[7:0\\] interrupt status register"]
pub type PortGpiol70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOI\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpioi70intsts_reg(&self) -> PortGpioi70intstsRegR {
        PortGpioi70intstsRegR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOJ\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpioj70intsts_reg(&self) -> PortGpioj70intstsRegR {
        PortGpioj70intstsRegR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOK\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpiok70intsts_reg(&self) -> PortGpiok70intstsRegR {
        PortGpiok70intstsRegR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOL\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpiol70intsts_reg(&self) -> PortGpiol70intstsRegR {
        PortGpiol70intstsRegR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOI\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpioi70intsts_reg(&mut self) -> PortGpioi70intstsRegW<Gpio0a8Spec> {
        PortGpioi70intstsRegW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOJ\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpioj70intsts_reg(&mut self) -> PortGpioj70intstsRegW<Gpio0a8Spec> {
        PortGpioj70intstsRegW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOK\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpiok70intsts_reg(&mut self) -> PortGpiok70intstsRegW<Gpio0a8Spec> {
        PortGpiok70intstsRegW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOL\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpiol70intsts_reg(&mut self) -> PortGpiol70intstsRegW<Gpio0a8Spec> {
        PortGpiol70intstsRegW::new(self, 24)
    }
}
#[doc = "GPIO\\_I/J/K/L Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0a8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0a8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0a8Spec;
impl crate::RegisterSpec for Gpio0a8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0a8::R`](R) reader structure"]
impl crate::Readable for Gpio0a8Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio0a8::W`](W) writer structure"]
impl crate::Writable for Gpio0a8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO0A8 to value 0"]
impl crate::Resettable for Gpio0a8Spec {}
