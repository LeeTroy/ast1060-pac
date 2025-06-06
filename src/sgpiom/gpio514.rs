#[doc = "Register `GPIO514` reader"]
pub type R = crate::R<Gpio514Spec>;
#[doc = "Register `GPIO514` writer"]
pub type W = crate::W<Gpio514Spec>;
#[doc = "Field `PortSerialGPIOA70INTStsReg` reader - Port Serial GPIOA\\[7:0\\] interrupt status register"]
pub type PortSerialGpioa70intstsRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOA70INTStsReg` writer - Port Serial GPIOA\\[7:0\\] interrupt status register"]
pub type PortSerialGpioa70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOB70INTStsReg` reader - Port Serial GPIOB\\[7:0\\] interrupt status register"]
pub type PortSerialGpiob70intstsRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOB70INTStsReg` writer - Port Serial GPIOB\\[7:0\\] interrupt status register"]
pub type PortSerialGpiob70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOC70INTStsReg` reader - Port Serial GPIOC\\[7:0\\] interrupt status register"]
pub type PortSerialGpioc70intstsRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOC70INTStsReg` writer - Port Serial GPIOC\\[7:0\\] interrupt status register"]
pub type PortSerialGpioc70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOD70INTStsReg` reader - Port Serial GPIOD\\[7:0\\] interrupt status register"]
pub type PortSerialGpiod70intstsRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOD70INTStsReg` writer - Port Serial GPIOD\\[7:0\\] interrupt status register"]
pub type PortSerialGpiod70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOA\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpioa70intsts_reg(&self) -> PortSerialGpioa70intstsRegR {
        PortSerialGpioa70intstsRegR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOB\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpiob70intsts_reg(&self) -> PortSerialGpiob70intstsRegR {
        PortSerialGpiob70intstsRegR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOC\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpioc70intsts_reg(&self) -> PortSerialGpioc70intstsRegR {
        PortSerialGpioc70intstsRegR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOD\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpiod70intsts_reg(&self) -> PortSerialGpiod70intstsRegR {
        PortSerialGpiod70intstsRegR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOA\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpioa70intsts_reg(&mut self) -> PortSerialGpioa70intstsRegW<Gpio514Spec> {
        PortSerialGpioa70intstsRegW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOB\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpiob70intsts_reg(&mut self) -> PortSerialGpiob70intstsRegW<Gpio514Spec> {
        PortSerialGpiob70intstsRegW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOC\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpioc70intsts_reg(&mut self) -> PortSerialGpioc70intstsRegW<Gpio514Spec> {
        PortSerialGpioc70intstsRegW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOD\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpiod70intsts_reg(&mut self) -> PortSerialGpiod70intstsRegW<Gpio514Spec> {
        PortSerialGpiod70intstsRegW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_A/B/C/D 1 Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio514::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio514::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio514Spec;
impl crate::RegisterSpec for Gpio514Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio514::R`](R) reader structure"]
impl crate::Readable for Gpio514Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio514::W`](W) writer structure"]
impl crate::Writable for Gpio514Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO514 to value 0"]
impl crate::Resettable for Gpio514Spec {}
