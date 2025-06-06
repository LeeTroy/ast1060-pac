#[doc = "Register `SCU69C` reader"]
pub type R = crate::R<Scu69cSpec>;
#[doc = "Register `SCU69C` writer"]
pub type W = crate::W<Scu69cSpec>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader<u16>;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `EnblQSPIMonitor2RSTInFnPin` reader - Enable QSPI Monitor 2 RST In function pin"]
pub type EnblQspimonitor2rstinFnPinR = crate::BitReader;
#[doc = "Field `EnblQSPIMonitor2RSTInFnPin` writer - Enable QSPI Monitor 2 RST In function pin"]
pub type EnblQspimonitor2rstinFnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblQSPIMonitor4RSTInFnPin` reader - Enable QSPI Monitor 4 RST In function pin"]
pub type EnblQspimonitor4rstinFnPinR = crate::BitReader;
#[doc = "Field `EnblQSPIMonitor4RSTInFnPin` writer - Enable QSPI Monitor 4 RST In function pin"]
pub type EnblQspimonitor4rstinFnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EnblSMBusMonitor2SCLInFnPin` reader - Enable SMBus Monitor 2 SCL In function pin"]
pub type EnblSmbusMonitor2sclinFnPinR = crate::BitReader;
#[doc = "Field `EnblSMBusMonitor2SCLInFnPin` writer - Enable SMBus Monitor 2 SCL In function pin"]
pub type EnblSmbusMonitor2sclinFnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSMBusMonitor2SDAInFnPin` reader - Enable SMBus Monitor 2 SDA In function pin"]
pub type EnblSmbusMonitor2sdainFnPinR = crate::BitReader;
#[doc = "Field `EnblSMBusMonitor2SDAInFnPin` writer - Enable SMBus Monitor 2 SDA In function pin"]
pub type EnblSmbusMonitor2sdainFnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSMBusMonitor2SCLOutFnPin` reader - Enable SMBus Monitor 2 SCL Out function pin"]
pub type EnblSmbusMonitor2scloutFnPinR = crate::BitReader;
#[doc = "Field `EnblSMBusMonitor2SCLOutFnPin` writer - Enable SMBus Monitor 2 SCL Out function pin"]
pub type EnblSmbusMonitor2scloutFnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSMBusMonitor2SDAOutFnPin` reader - Enable SMBus Monitor 2 SDA Out function pin"]
pub type EnblSmbusMonitor2sdaoutFnPinR = crate::BitReader;
#[doc = "Field `EnblSMBusMonitor2SDAOutFnPin` writer - Enable SMBus Monitor 2 SDA Out function pin"]
pub type EnblSmbusMonitor2sdaoutFnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSMBusMonitor1SCLInFnPin` reader - Enable SMBus Monitor 1 SCL In function pin"]
pub type EnblSmbusMonitor1sclinFnPinR = crate::BitReader;
#[doc = "Field `EnblSMBusMonitor1SCLInFnPin` writer - Enable SMBus Monitor 1 SCL In function pin"]
pub type EnblSmbusMonitor1sclinFnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSMBusMonitor1SDAInFnPin` reader - Enable SMBus Monitor 1 SDA In function pin"]
pub type EnblSmbusMonitor1sdainFnPinR = crate::BitReader;
#[doc = "Field `EnblSMBusMonitor1SDAInFnPin` writer - Enable SMBus Monitor 1 SDA In function pin"]
pub type EnblSmbusMonitor1sdainFnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSMBusMonitor1SCLOutFnPin` reader - Enable SMBus Monitor 1 SCL Out function pin"]
pub type EnblSmbusMonitor1scloutFnPinR = crate::BitReader;
#[doc = "Field `EnblSMBusMonitor1SCLOutFnPin` writer - Enable SMBus Monitor 1 SCL Out function pin"]
pub type EnblSmbusMonitor1scloutFnPinW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - Enable QSPI Monitor 2 RST In function pin"]
    #[inline(always)]
    pub fn enbl_qspimonitor2rstin_fn_pin(&self) -> EnblQspimonitor2rstinFnPinR {
        EnblQspimonitor2rstinFnPinR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable QSPI Monitor 4 RST In function pin"]
    #[inline(always)]
    pub fn enbl_qspimonitor4rstin_fn_pin(&self) -> EnblQspimonitor4rstinFnPinR {
        EnblQspimonitor4rstinFnPinR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:19 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bit 20 - Enable SMBus Monitor 2 SCL In function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor2sclin_fn_pin(&self) -> EnblSmbusMonitor2sclinFnPinR {
        EnblSmbusMonitor2sclinFnPinR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable SMBus Monitor 2 SDA In function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor2sdain_fn_pin(&self) -> EnblSmbusMonitor2sdainFnPinR {
        EnblSmbusMonitor2sdainFnPinR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable SMBus Monitor 2 SCL Out function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor2sclout_fn_pin(&self) -> EnblSmbusMonitor2scloutFnPinR {
        EnblSmbusMonitor2scloutFnPinR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable SMBus Monitor 2 SDA Out function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor2sdaout_fn_pin(&self) -> EnblSmbusMonitor2sdaoutFnPinR {
        EnblSmbusMonitor2sdaoutFnPinR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable SMBus Monitor 1 SCL In function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor1sclin_fn_pin(&self) -> EnblSmbusMonitor1sclinFnPinR {
        EnblSmbusMonitor1sclinFnPinR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable SMBus Monitor 1 SDA In function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor1sdain_fn_pin(&self) -> EnblSmbusMonitor1sdainFnPinR {
        EnblSmbusMonitor1sdainFnPinR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable SMBus Monitor 1 SCL Out function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor1sclout_fn_pin(&self) -> EnblSmbusMonitor1scloutFnPinR {
        EnblSmbusMonitor1scloutFnPinR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Scu69cSpec> {
        Reserved3W::new(self, 0)
    }
    #[doc = "Bit 9 - Enable QSPI Monitor 2 RST In function pin"]
    #[inline(always)]
    pub fn enbl_qspimonitor2rstin_fn_pin(&mut self) -> EnblQspimonitor2rstinFnPinW<Scu69cSpec> {
        EnblQspimonitor2rstinFnPinW::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Scu69cSpec> {
        Reserved2W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable QSPI Monitor 4 RST In function pin"]
    #[inline(always)]
    pub fn enbl_qspimonitor4rstin_fn_pin(&mut self) -> EnblQspimonitor4rstinFnPinW<Scu69cSpec> {
        EnblQspimonitor4rstinFnPinW::new(self, 11)
    }
    #[doc = "Bits 12:19 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu69cSpec> {
        Reserved1W::new(self, 12)
    }
    #[doc = "Bit 20 - Enable SMBus Monitor 2 SCL In function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor2sclin_fn_pin(&mut self) -> EnblSmbusMonitor2sclinFnPinW<Scu69cSpec> {
        EnblSmbusMonitor2sclinFnPinW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable SMBus Monitor 2 SDA In function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor2sdain_fn_pin(&mut self) -> EnblSmbusMonitor2sdainFnPinW<Scu69cSpec> {
        EnblSmbusMonitor2sdainFnPinW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable SMBus Monitor 2 SCL Out function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor2sclout_fn_pin(
        &mut self,
    ) -> EnblSmbusMonitor2scloutFnPinW<Scu69cSpec> {
        EnblSmbusMonitor2scloutFnPinW::new(self, 22)
    }
    #[doc = "Bit 23 - Enable SMBus Monitor 2 SDA Out function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor2sdaout_fn_pin(
        &mut self,
    ) -> EnblSmbusMonitor2sdaoutFnPinW<Scu69cSpec> {
        EnblSmbusMonitor2sdaoutFnPinW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable SMBus Monitor 1 SCL In function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor1sclin_fn_pin(&mut self) -> EnblSmbusMonitor1sclinFnPinW<Scu69cSpec> {
        EnblSmbusMonitor1sclinFnPinW::new(self, 24)
    }
    #[doc = "Bit 30 - Enable SMBus Monitor 1 SDA In function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor1sdain_fn_pin(&mut self) -> EnblSmbusMonitor1sdainFnPinW<Scu69cSpec> {
        EnblSmbusMonitor1sdainFnPinW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable SMBus Monitor 1 SCL Out function pin"]
    #[inline(always)]
    pub fn enbl_smbus_monitor1sclout_fn_pin(
        &mut self,
    ) -> EnblSmbusMonitor1scloutFnPinW<Scu69cSpec> {
        EnblSmbusMonitor1scloutFnPinW::new(self, 31)
    }
}
#[doc = "Multi-function Pin Control \\#24\n\nYou can [`read`](crate::Reg::read) this register and get [`scu69c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu69c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu69cSpec;
impl crate::RegisterSpec for Scu69cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu69c::R`](R) reader structure"]
impl crate::Readable for Scu69cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu69c::W`](W) writer structure"]
impl crate::Writable for Scu69cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU69C to value 0"]
impl crate::Resettable for Scu69cSpec {}
