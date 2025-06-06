#[doc = "Register `GPIO50C` reader"]
pub type R = crate::R<Gpio50cSpec>;
#[doc = "Register `GPIO50C` writer"]
pub type W = crate::W<Gpio50cSpec>;
#[doc = "Field `PortSerialGPIOA70INTSensitivityType1Sel` reader - Port Serial GPIOA\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpioa70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOA70INTSensitivityType1Sel` writer - Port Serial GPIOA\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpioa70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOB70INTSensitivityType1Sel` reader - Port Serial GPIOB\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpiob70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOB70INTSensitivityType1Sel` writer - Port Serial GPIOB\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpiob70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOC70INTSensitivityType1Sel` reader - Port Serial GPIOC\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpioc70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOC70INTSensitivityType1Sel` writer - Port Serial GPIOC\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpioc70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOD70INTSensitivityType1Sel` reader - Port Serial GPIOD\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpiod70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOD70INTSensitivityType1Sel` writer - Port Serial GPIOD\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpiod70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOA\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpioa70intsensitivity_type1sel(
        &self,
    ) -> PortSerialGpioa70intsensitivityType1selR {
        PortSerialGpioa70intsensitivityType1selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOB\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpiob70intsensitivity_type1sel(
        &self,
    ) -> PortSerialGpiob70intsensitivityType1selR {
        PortSerialGpiob70intsensitivityType1selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOC\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpioc70intsensitivity_type1sel(
        &self,
    ) -> PortSerialGpioc70intsensitivityType1selR {
        PortSerialGpioc70intsensitivityType1selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOD\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpiod70intsensitivity_type1sel(
        &self,
    ) -> PortSerialGpiod70intsensitivityType1selR {
        PortSerialGpiod70intsensitivityType1selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOA\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpioa70intsensitivity_type1sel(
        &mut self,
    ) -> PortSerialGpioa70intsensitivityType1selW<Gpio50cSpec> {
        PortSerialGpioa70intsensitivityType1selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOB\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpiob70intsensitivity_type1sel(
        &mut self,
    ) -> PortSerialGpiob70intsensitivityType1selW<Gpio50cSpec> {
        PortSerialGpiob70intsensitivityType1selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOC\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpioc70intsensitivity_type1sel(
        &mut self,
    ) -> PortSerialGpioc70intsensitivityType1selW<Gpio50cSpec> {
        PortSerialGpioc70intsensitivityType1selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOD\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpiod70intsensitivity_type1sel(
        &mut self,
    ) -> PortSerialGpiod70intsensitivityType1selW<Gpio50cSpec> {
        PortSerialGpiod70intsensitivityType1selW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_A/B/C/D 1 Interrupt Sensitivity Type 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio50c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio50c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio50cSpec;
impl crate::RegisterSpec for Gpio50cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio50c::R`](R) reader structure"]
impl crate::Readable for Gpio50cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio50c::W`](W) writer structure"]
impl crate::Writable for Gpio50cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO50C to value 0"]
impl crate::Resettable for Gpio50cSpec {}
