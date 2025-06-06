#[doc = "Register `SCU6B0` reader"]
pub type R = crate::R<Scu6b0Spec>;
#[doc = "Register `SCU6B0` writer"]
pub type W = crate::W<Scu6b0Spec>;
#[doc = "Field `EnblSMBusMonitor1SDAOutFnPin` reader - Enable SMBus Monitor 1 SDA Out function pin"]
pub type EnblSmbusMonitor1sdaoutFnPinR = crate::BitReader;
#[doc = "Field `EnblSMBusMonitor1SDAOutFnPin` writer - Enable SMBus Monitor 1 SDA Out function pin"]
pub type EnblSmbusMonitor1sdaoutFnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSMBusMonitor4SCLInFnPin` reader - Enable SMBus Monitor 4 SCL In function pin"]
pub type EnblSmbusMonitor4sclinFnPinR = crate::BitReader;
#[doc = "Field `EnblSMBusMonitor4SCLInFnPin` writer - Enable SMBus Monitor 4 SCL In function pin"]
pub type EnblSmbusMonitor4sclinFnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSMBusMonitor4SDAInFnPin` reader - Enable SMBus Monitor 4 SDA In function pin"]
pub type EnblSmbusMonitor4sdainFnPinR = crate::BitReader;
#[doc = "Field `EnblSMBusMonitor4SDAInFnPin` writer - Enable SMBus Monitor 4 SDA In function pin"]
pub type EnblSmbusMonitor4sdainFnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSMBusMonitor4SCLOutFnPin` reader - Enable SMBus Monitor 4 SCL Out function pin"]
pub type EnblSmbusMonitor4scloutFnPinR = crate::BitReader;
#[doc = "Field `EnblSMBusMonitor4SCLOutFnPin` writer - Enable SMBus Monitor 4 SCL Out function pin"]
pub type EnblSmbusMonitor4scloutFnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSMBusMonitor4SDAOutFnPin` reader - Enable SMBus Monitor 4 SDA Out function pin"]
pub type EnblSmbusMonitor4sdaoutFnPinR = crate::BitReader;
#[doc = "Field `EnblSMBusMonitor4SDAOutFnPin` writer - Enable SMBus Monitor 4 SDA Out function pin"]
pub type EnblSmbusMonitor4sdaoutFnPinW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable SMBus Monitor 1 SDA Out function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor1sdaout_fn_pin(&self) -> EnblSmbusMonitor1sdaoutFnPinR {
        EnblSmbusMonitor1sdaoutFnPinR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable SMBus Monitor 4 SCL In function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor4sclin_fn_pin(&self) -> EnblSmbusMonitor4sclinFnPinR {
        EnblSmbusMonitor4sclinFnPinR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable SMBus Monitor 4 SDA In function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor4sdain_fn_pin(&self) -> EnblSmbusMonitor4sdainFnPinR {
        EnblSmbusMonitor4sdainFnPinR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable SMBus Monitor 4 SCL Out function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor4sclout_fn_pin(&self) -> EnblSmbusMonitor4scloutFnPinR {
        EnblSmbusMonitor4scloutFnPinR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable SMBus Monitor 4 SDA Out function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor4sdaout_fn_pin(&self) -> EnblSmbusMonitor4sdaoutFnPinR {
        EnblSmbusMonitor4sdaoutFnPinR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable SMBus Monitor 1 SDA Out function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor1sdaout_fn_pin(
        &mut self,
    ) -> EnblSmbusMonitor1sdaoutFnPinW<Scu6b0Spec> {
        EnblSmbusMonitor1sdaoutFnPinW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable SMBus Monitor 4 SCL In function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor4sclin_fn_pin(&mut self) -> EnblSmbusMonitor4sclinFnPinW<Scu6b0Spec> {
        EnblSmbusMonitor4sclinFnPinW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable SMBus Monitor 4 SDA In function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor4sdain_fn_pin(&mut self) -> EnblSmbusMonitor4sdainFnPinW<Scu6b0Spec> {
        EnblSmbusMonitor4sdainFnPinW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable SMBus Monitor 4 SCL Out function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor4sclout_fn_pin(
        &mut self,
    ) -> EnblSmbusMonitor4scloutFnPinW<Scu6b0Spec> {
        EnblSmbusMonitor4scloutFnPinW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable SMBus Monitor 4 SDA Out function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor4sdaout_fn_pin(
        &mut self,
    ) -> EnblSmbusMonitor4sdaoutFnPinW<Scu6b0Spec> {
        EnblSmbusMonitor4sdaoutFnPinW::new(self, 4)
    }
}
#[doc = "Multi-function Pin Control \\#25\n\nYou can [`read`](crate::Reg::read) this register and get [`scu6b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu6b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu6b0Spec;
impl crate::RegisterSpec for Scu6b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu6b0::R`](R) reader structure"]
impl crate::Readable for Scu6b0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu6b0::W`](W) writer structure"]
impl crate::Writable for Scu6b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU6B0 to value 0"]
impl crate::Resettable for Scu6b0Spec {}
