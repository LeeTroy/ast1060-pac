#[doc = "Register `GPIO59C` reader"]
pub type R = crate::R<Gpio59cSpec>;
#[doc = "Register `GPIO59C` writer"]
pub type W = crate::W<Gpio59cSpec>;
#[doc = "Field `PortSerialGPIOM70INTSensitivityType1Sel` reader - Port Serial GPIOM\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpiom70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOM70INTSensitivityType1Sel` writer - Port Serial GPIOM\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpiom70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPION70INTSensitivityType1Sel` reader - Port Serial GPION\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpion70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortSerialGPION70INTSensitivityType1Sel` writer - Port Serial GPION\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpion70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOO70INTSensitivityType1Sel` reader - Port Serial GPIOO\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpioo70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOO70INTSensitivityType1Sel` writer - Port Serial GPIOO\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpioo70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOP70INTSensitivityType1Sel` reader - Port Serial GPIOP\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpiop70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOP70INTSensitivityType1Sel` writer - Port Serial GPIOP\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpiop70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOM\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpiom70intsensitivity_type1sel(
        &self,
    ) -> PortSerialGpiom70intsensitivityType1selR {
        PortSerialGpiom70intsensitivityType1selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPION\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpion70intsensitivity_type1sel(
        &self,
    ) -> PortSerialGpion70intsensitivityType1selR {
        PortSerialGpion70intsensitivityType1selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOO\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpioo70intsensitivity_type1sel(
        &self,
    ) -> PortSerialGpioo70intsensitivityType1selR {
        PortSerialGpioo70intsensitivityType1selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOP\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpiop70intsensitivity_type1sel(
        &self,
    ) -> PortSerialGpiop70intsensitivityType1selR {
        PortSerialGpiop70intsensitivityType1selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOM\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpiom70intsensitivity_type1sel(
        &mut self,
    ) -> PortSerialGpiom70intsensitivityType1selW<Gpio59cSpec> {
        PortSerialGpiom70intsensitivityType1selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPION\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpion70intsensitivity_type1sel(
        &mut self,
    ) -> PortSerialGpion70intsensitivityType1selW<Gpio59cSpec> {
        PortSerialGpion70intsensitivityType1selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOO\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpioo70intsensitivity_type1sel(
        &mut self,
    ) -> PortSerialGpioo70intsensitivityType1selW<Gpio59cSpec> {
        PortSerialGpioo70intsensitivityType1selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOP\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpiop70intsensitivity_type1sel(
        &mut self,
    ) -> PortSerialGpiop70intsensitivityType1selW<Gpio59cSpec> {
        PortSerialGpiop70intsensitivityType1selW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_M/N/O/P 1 Interrupt Sensitivity Type 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio59c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio59c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio59cSpec;
impl crate::RegisterSpec for Gpio59cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio59c::R`](R) reader structure"]
impl crate::Readable for Gpio59cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio59c::W`](W) writer structure"]
impl crate::Writable for Gpio59cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO59C to value 0"]
impl crate::Resettable for Gpio59cSpec {}
