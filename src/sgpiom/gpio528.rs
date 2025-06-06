#[doc = "Register `GPIO528` reader"]
pub type R = crate::R<Gpio528Spec>;
#[doc = "Register `GPIO528` writer"]
pub type W = crate::W<Gpio528Spec>;
#[doc = "Field `PortSerialGPIOE70INTSensitivityType1Sel` reader - Port Serial GPIOE\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpioe70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOE70INTSensitivityType1Sel` writer - Port Serial GPIOE\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpioe70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOF70INTSensitivityType1Sel` reader - Port Serial GPIOF\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpiof70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOF70INTSensitivityType1Sel` writer - Port Serial GPIOF\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpiof70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOG70INTSensitivityType1Sel` reader - Port Serial GPIOG\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpiog70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOG70INTSensitivityType1Sel` writer - Port Serial GPIOG\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpiog70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOH70INTSensitivityType1Sel` reader - Port Serial GPIOH\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpioh70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOH70INTSensitivityType1Sel` writer - Port Serial GPIOH\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpioh70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOE\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpioe70intsensitivity_type1sel(
        &self,
    ) -> PortSerialGpioe70intsensitivityType1selR {
        PortSerialGpioe70intsensitivityType1selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOF\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpiof70intsensitivity_type1sel(
        &self,
    ) -> PortSerialGpiof70intsensitivityType1selR {
        PortSerialGpiof70intsensitivityType1selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOG\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpiog70intsensitivity_type1sel(
        &self,
    ) -> PortSerialGpiog70intsensitivityType1selR {
        PortSerialGpiog70intsensitivityType1selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOH\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpioh70intsensitivity_type1sel(
        &self,
    ) -> PortSerialGpioh70intsensitivityType1selR {
        PortSerialGpioh70intsensitivityType1selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOE\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpioe70intsensitivity_type1sel(
        &mut self,
    ) -> PortSerialGpioe70intsensitivityType1selW<Gpio528Spec> {
        PortSerialGpioe70intsensitivityType1selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOF\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpiof70intsensitivity_type1sel(
        &mut self,
    ) -> PortSerialGpiof70intsensitivityType1selW<Gpio528Spec> {
        PortSerialGpiof70intsensitivityType1selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOG\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpiog70intsensitivity_type1sel(
        &mut self,
    ) -> PortSerialGpiog70intsensitivityType1selW<Gpio528Spec> {
        PortSerialGpiog70intsensitivityType1selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOH\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpioh70intsensitivity_type1sel(
        &mut self,
    ) -> PortSerialGpioh70intsensitivityType1selW<Gpio528Spec> {
        PortSerialGpioh70intsensitivityType1selW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_E/F/G/H 1 Interrupt Sensitivity Type 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio528::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio528::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio528Spec;
impl crate::RegisterSpec for Gpio528Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio528::R`](R) reader structure"]
impl crate::Readable for Gpio528Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio528::W`](W) writer structure"]
impl crate::Writable for Gpio528Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO528 to value 0"]
impl crate::Resettable for Gpio528Spec {}
