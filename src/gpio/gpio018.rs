#[doc = "Register `GPIO018` reader"]
pub type R = crate::R<Gpio018Spec>;
#[doc = "Register `GPIO018` writer"]
pub type W = crate::W<Gpio018Spec>;
#[doc = "Field `PortGPIOA70INTStsReg` reader - Port GPIOA\\[7:0\\] interrupt status register"]
pub type PortGpioa70intstsRegR = crate::FieldReader;
#[doc = "Field `PortGPIOA70INTStsReg` writer - Port GPIOA\\[7:0\\] interrupt status register"]
pub type PortGpioa70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOB70INTStsReg` reader - Port GPIOB\\[7:0\\] interrupt status register"]
pub type PortGpiob70intstsRegR = crate::FieldReader;
#[doc = "Field `PortGPIOB70INTStsReg` writer - Port GPIOB\\[7:0\\] interrupt status register"]
pub type PortGpiob70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOC70INTStsReg` reader - Port GPIOC\\[7:0\\] interrupt status register"]
pub type PortGpioc70intstsRegR = crate::FieldReader;
#[doc = "Field `PortGPIOC70INTStsReg` writer - Port GPIOC\\[7:0\\] interrupt status register"]
pub type PortGpioc70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOD70INTStsReg` reader - Port GPIOD\\[7:0\\] interrupt status register"]
pub type PortGpiod70intstsRegR = crate::FieldReader;
#[doc = "Field `PortGPIOD70INTStsReg` writer - Port GPIOD\\[7:0\\] interrupt status register"]
pub type PortGpiod70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOA\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpioa70intsts_reg(&self) -> PortGpioa70intstsRegR {
        PortGpioa70intstsRegR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOB\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpiob70intsts_reg(&self) -> PortGpiob70intstsRegR {
        PortGpiob70intstsRegR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOC\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpioc70intsts_reg(&self) -> PortGpioc70intstsRegR {
        PortGpioc70intstsRegR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOD\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpiod70intsts_reg(&self) -> PortGpiod70intstsRegR {
        PortGpiod70intstsRegR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOA\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpioa70intsts_reg(&mut self) -> PortGpioa70intstsRegW<Gpio018Spec> {
        PortGpioa70intstsRegW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOB\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpiob70intsts_reg(&mut self) -> PortGpiob70intstsRegW<Gpio018Spec> {
        PortGpiob70intstsRegW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOC\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpioc70intsts_reg(&mut self) -> PortGpioc70intstsRegW<Gpio018Spec> {
        PortGpioc70intstsRegW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOD\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpiod70intsts_reg(&mut self) -> PortGpiod70intstsRegW<Gpio018Spec> {
        PortGpiod70intstsRegW::new(self, 24)
    }
}
#[doc = "GPIO\\_A/B/C/D Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio018::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio018::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio018Spec;
impl crate::RegisterSpec for Gpio018Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio018::R`](R) reader structure"]
impl crate::Readable for Gpio018Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio018::W`](W) writer structure"]
impl crate::Writable for Gpio018Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO018 to value 0"]
impl crate::Resettable for Gpio018Spec {}
