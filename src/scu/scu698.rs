#[doc = "Register `SCU698` reader"]
pub type R = crate::R<Scu698Spec>;
#[doc = "Register `SCU698` writer"]
pub type W = crate::W<Scu698Spec>;
#[doc = "Field `EnblSMBusMonitor3SCLInFnPin` reader - Enable SMBus Monitor 3 SCL In function pin"]
pub type EnblSmbusMonitor3sclinFnPinR = crate::BitReader;
#[doc = "Field `EnblSMBusMonitor3SCLInFnPin` writer - Enable SMBus Monitor 3 SCL In function pin"]
pub type EnblSmbusMonitor3sclinFnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSMBusMonitor3SDAInFnPin` reader - Enable SMBus Monitor 3 SDA In function pin"]
pub type EnblSmbusMonitor3sdainFnPinR = crate::BitReader;
#[doc = "Field `EnblSMBusMonitor3SDAInFnPin` writer - Enable SMBus Monitor 3 SDA In function pin"]
pub type EnblSmbusMonitor3sdainFnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSMBusMonitor3SCLOutFnPin` reader - Enable SMBus Monitor 3 SCL Out function pin"]
pub type EnblSmbusMonitor3scloutFnPinR = crate::BitReader;
#[doc = "Field `EnblSMBusMonitor3SCLOutFnPin` writer - Enable SMBus Monitor 3 SCL Out function pin"]
pub type EnblSmbusMonitor3scloutFnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSMBusMonitor3SDAOutFnPin` reader - Enable SMBus Monitor 3 SDA Out function pin"]
pub type EnblSmbusMonitor3sdaoutFnPinR = crate::BitReader;
#[doc = "Field `EnblSMBusMonitor3SDAOutFnPin` writer - Enable SMBus Monitor 3 SDA Out function pin"]
pub type EnblSmbusMonitor3sdaoutFnPinW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 28 - Enable SMBus Monitor 3 SCL In function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor3sclin_fn_pin(&self) -> EnblSmbusMonitor3sclinFnPinR {
        EnblSmbusMonitor3sclinFnPinR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable SMBus Monitor 3 SDA In function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor3sdain_fn_pin(&self) -> EnblSmbusMonitor3sdainFnPinR {
        EnblSmbusMonitor3sdainFnPinR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable SMBus Monitor 3 SCL Out function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor3sclout_fn_pin(&self) -> EnblSmbusMonitor3scloutFnPinR {
        EnblSmbusMonitor3scloutFnPinR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable SMBus Monitor 3 SDA Out function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor3sdaout_fn_pin(&self) -> EnblSmbusMonitor3sdaoutFnPinR {
        EnblSmbusMonitor3sdaoutFnPinR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Enable SMBus Monitor 3 SCL In function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor3sclin_fn_pin(&mut self) -> EnblSmbusMonitor3sclinFnPinW<Scu698Spec> {
        EnblSmbusMonitor3sclinFnPinW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable SMBus Monitor 3 SDA In function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor3sdain_fn_pin(&mut self) -> EnblSmbusMonitor3sdainFnPinW<Scu698Spec> {
        EnblSmbusMonitor3sdainFnPinW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable SMBus Monitor 3 SCL Out function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor3sclout_fn_pin(
        &mut self,
    ) -> EnblSmbusMonitor3scloutFnPinW<Scu698Spec> {
        EnblSmbusMonitor3scloutFnPinW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable SMBus Monitor 3 SDA Out function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor3sdaout_fn_pin(
        &mut self,
    ) -> EnblSmbusMonitor3sdaoutFnPinW<Scu698Spec> {
        EnblSmbusMonitor3sdaoutFnPinW::new(self, 31)
    }
}
#[doc = "Multi-function Pin Control \\#23\n\nYou can [`read`](crate::Reg::read) this register and get [`scu698::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu698::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu698Spec;
impl crate::RegisterSpec for Scu698Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu698::R`](R) reader structure"]
impl crate::Readable for Scu698Spec {}
#[doc = "`write(|w| ..)` method takes [`scu698::W`](W) writer structure"]
impl crate::Writable for Scu698Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU698 to value 0"]
impl crate::Resettable for Scu698Spec {}
