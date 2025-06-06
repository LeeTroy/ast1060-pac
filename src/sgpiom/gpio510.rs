#[doc = "Register `GPIO510` reader"]
pub type R = crate::R<Gpio510Spec>;
#[doc = "Register `GPIO510` writer"]
pub type W = crate::W<Gpio510Spec>;
#[doc = "Field `PortSerialGPIOA70INTSensitivityType2Sel` reader - Port Serial GPIOA\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpioa70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOA70INTSensitivityType2Sel` writer - Port Serial GPIOA\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpioa70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOB70INTSensitivityType2Sel` reader - Port Serial GPIOB\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpiob70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOB70INTSensitivityType2Sel` writer - Port Serial GPIOB\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpiob70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOC70INTSensitivityType2Sel` reader - Port Serial GPIOC\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpioc70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOC70INTSensitivityType2Sel` writer - Port Serial GPIOC\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpioc70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOD70INTSensitivityType2Sel` reader - Port Serial GPIOD\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpiod70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOD70INTSensitivityType2Sel` writer - Port Serial GPIOD\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpiod70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOA\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpioa70intsensitivity_type2sel(
        &self,
    ) -> PortSerialGpioa70intsensitivityType2selR {
        PortSerialGpioa70intsensitivityType2selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOB\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpiob70intsensitivity_type2sel(
        &self,
    ) -> PortSerialGpiob70intsensitivityType2selR {
        PortSerialGpiob70intsensitivityType2selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOC\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpioc70intsensitivity_type2sel(
        &self,
    ) -> PortSerialGpioc70intsensitivityType2selR {
        PortSerialGpioc70intsensitivityType2selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOD\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpiod70intsensitivity_type2sel(
        &self,
    ) -> PortSerialGpiod70intsensitivityType2selR {
        PortSerialGpiod70intsensitivityType2selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOA\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpioa70intsensitivity_type2sel(
        &mut self,
    ) -> PortSerialGpioa70intsensitivityType2selW<Gpio510Spec> {
        PortSerialGpioa70intsensitivityType2selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOB\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpiob70intsensitivity_type2sel(
        &mut self,
    ) -> PortSerialGpiob70intsensitivityType2selW<Gpio510Spec> {
        PortSerialGpiob70intsensitivityType2selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOC\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpioc70intsensitivity_type2sel(
        &mut self,
    ) -> PortSerialGpioc70intsensitivityType2selW<Gpio510Spec> {
        PortSerialGpioc70intsensitivityType2selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOD\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpiod70intsensitivity_type2sel(
        &mut self,
    ) -> PortSerialGpiod70intsensitivityType2selW<Gpio510Spec> {
        PortSerialGpiod70intsensitivityType2selW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_A/B/C/D 1 Interrupt Sensitivity Type 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio510::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio510::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio510Spec;
impl crate::RegisterSpec for Gpio510Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio510::R`](R) reader structure"]
impl crate::Readable for Gpio510Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio510::W`](W) writer structure"]
impl crate::Writable for Gpio510Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO510 to value 0"]
impl crate::Resettable for Gpio510Spec {}
