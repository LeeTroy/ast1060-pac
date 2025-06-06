#[doc = "Register `GPIO524` reader"]
pub type R = crate::R<Gpio524Spec>;
#[doc = "Register `GPIO524` writer"]
pub type W = crate::W<Gpio524Spec>;
#[doc = "Field `PortSerialGPIOE70INTSensitivityType0Sel` reader - Port Serial GPIOE\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpioe70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOE70INTSensitivityType0Sel` writer - Port Serial GPIOE\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpioe70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOF70INTSensitivityType0Sel` reader - Port Serial GPIOF\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpiof70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOF70INTSensitivityType0Sel` writer - Port Serial GPIOF\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpiof70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOG70INTSensitivityType0Sel` reader - Port Serial GPIOG\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpiog70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOG70INTSensitivityType0Sel` writer - Port Serial GPIOG\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpiog70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOH70INTSensitivityType0Sel` reader - Port Serial GPIOH\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpioh70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOH70INTSensitivityType0Sel` writer - Port Serial GPIOH\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpioh70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOE\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpioe70intsensitivity_type0sel(
        &self,
    ) -> PortSerialGpioe70intsensitivityType0selR {
        PortSerialGpioe70intsensitivityType0selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOF\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpiof70intsensitivity_type0sel(
        &self,
    ) -> PortSerialGpiof70intsensitivityType0selR {
        PortSerialGpiof70intsensitivityType0selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOG\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpiog70intsensitivity_type0sel(
        &self,
    ) -> PortSerialGpiog70intsensitivityType0selR {
        PortSerialGpiog70intsensitivityType0selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOH\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpioh70intsensitivity_type0sel(
        &self,
    ) -> PortSerialGpioh70intsensitivityType0selR {
        PortSerialGpioh70intsensitivityType0selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOE\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpioe70intsensitivity_type0sel(
        &mut self,
    ) -> PortSerialGpioe70intsensitivityType0selW<Gpio524Spec> {
        PortSerialGpioe70intsensitivityType0selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOF\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpiof70intsensitivity_type0sel(
        &mut self,
    ) -> PortSerialGpiof70intsensitivityType0selW<Gpio524Spec> {
        PortSerialGpiof70intsensitivityType0selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOG\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpiog70intsensitivity_type0sel(
        &mut self,
    ) -> PortSerialGpiog70intsensitivityType0selW<Gpio524Spec> {
        PortSerialGpiog70intsensitivityType0selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOH\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpioh70intsensitivity_type0sel(
        &mut self,
    ) -> PortSerialGpioh70intsensitivityType0selW<Gpio524Spec> {
        PortSerialGpioh70intsensitivityType0selW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_E/F/G/H 1 Interrupt Sensitivity Type 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio524::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio524::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio524Spec;
impl crate::RegisterSpec for Gpio524Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio524::R`](R) reader structure"]
impl crate::Readable for Gpio524Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio524::W`](W) writer structure"]
impl crate::Writable for Gpio524Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO524 to value 0"]
impl crate::Resettable for Gpio524Spec {}
