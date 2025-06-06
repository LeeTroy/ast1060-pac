#[doc = "Register `GPIO0F4` reader"]
pub type R = crate::R<Gpio0f4Spec>;
#[doc = "Register `GPIO0F4` writer"]
pub type W = crate::W<Gpio0f4Spec>;
#[doc = "Field `PortGPIOM70INTSensitivityType2Sel` reader - Port GPIOM\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpiom70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortGPIOM70INTSensitivityType2Sel` writer - Port GPIOM\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpiom70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPION70INTSensitivityType2Sel` reader - Port GPION\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpion70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortGPION70INTSensitivityType2Sel` writer - Port GPION\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpion70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOO70INTSensitivityType2Sel` reader - Port GPIOO\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpioo70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortGPIOO70INTSensitivityType2Sel` writer - Port GPIOO\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpioo70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOP70INTSensitivityType2Sel` reader - Port GPIOP\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpiop70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortGPIOP70INTSensitivityType2Sel` writer - Port GPIOP\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpiop70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOM\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpiom70intsensitivity_type2sel(&self) -> PortGpiom70intsensitivityType2selR {
        PortGpiom70intsensitivityType2selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPION\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpion70intsensitivity_type2sel(&self) -> PortGpion70intsensitivityType2selR {
        PortGpion70intsensitivityType2selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOO\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpioo70intsensitivity_type2sel(&self) -> PortGpioo70intsensitivityType2selR {
        PortGpioo70intsensitivityType2selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOP\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpiop70intsensitivity_type2sel(&self) -> PortGpiop70intsensitivityType2selR {
        PortGpiop70intsensitivityType2selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOM\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpiom70intsensitivity_type2sel(
        &mut self,
    ) -> PortGpiom70intsensitivityType2selW<Gpio0f4Spec> {
        PortGpiom70intsensitivityType2selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPION\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpion70intsensitivity_type2sel(
        &mut self,
    ) -> PortGpion70intsensitivityType2selW<Gpio0f4Spec> {
        PortGpion70intsensitivityType2selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOO\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpioo70intsensitivity_type2sel(
        &mut self,
    ) -> PortGpioo70intsensitivityType2selW<Gpio0f4Spec> {
        PortGpioo70intsensitivityType2selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOP\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpiop70intsensitivity_type2sel(
        &mut self,
    ) -> PortGpiop70intsensitivityType2selW<Gpio0f4Spec> {
        PortGpiop70intsensitivityType2selW::new(self, 24)
    }
}
#[doc = "GPIO\\_M/N/O/P Interrupt Sensitivity Type 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0f4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0f4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0f4Spec;
impl crate::RegisterSpec for Gpio0f4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0f4::R`](R) reader structure"]
impl crate::Readable for Gpio0f4Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio0f4::W`](W) writer structure"]
impl crate::Writable for Gpio0f4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO0F4 to value 0"]
impl crate::Resettable for Gpio0f4Spec {}
