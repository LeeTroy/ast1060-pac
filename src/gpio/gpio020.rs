#[doc = "Register `GPIO020` reader"]
pub type R = crate::R<Gpio020Spec>;
#[doc = "Register `GPIO020` writer"]
pub type W = crate::W<Gpio020Spec>;
#[doc = "Field `PortGPIOE70DataReg` reader - Port GPIOE\\[7:0\\] data register"]
pub type PortGpioe70dataRegR = crate::FieldReader;
#[doc = "Field `PortGPIOE70DataReg` writer - Port GPIOE\\[7:0\\] data register"]
pub type PortGpioe70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOF70DataReg` reader - Port GPIOF\\[7:0\\] data register"]
pub type PortGpiof70dataRegR = crate::FieldReader;
#[doc = "Field `PortGPIOF70DataReg` writer - Port GPIOF\\[7:0\\] data register"]
pub type PortGpiof70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOG70DataReg` reader - Port GPIOG\\[7:0\\] data register"]
pub type PortGpiog70dataRegR = crate::FieldReader;
#[doc = "Field `PortGPIOG70DataReg` writer - Port GPIOG\\[7:0\\] data register"]
pub type PortGpiog70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOH70DataReg` reader - Port GPIOH\\[7:0\\] data register"]
pub type PortGpioh70dataRegR = crate::FieldReader;
#[doc = "Field `PortGPIOH70DataReg` writer - Port GPIOH\\[7:0\\] data register"]
pub type PortGpioh70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOE\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpioe70data_reg(&self) -> PortGpioe70dataRegR {
        PortGpioe70dataRegR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOF\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpiof70data_reg(&self) -> PortGpiof70dataRegR {
        PortGpiof70dataRegR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOG\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpiog70data_reg(&self) -> PortGpiog70dataRegR {
        PortGpiog70dataRegR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOH\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpioh70data_reg(&self) -> PortGpioh70dataRegR {
        PortGpioh70dataRegR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOE\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpioe70data_reg(&mut self) -> PortGpioe70dataRegW<Gpio020Spec> {
        PortGpioe70dataRegW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOF\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpiof70data_reg(&mut self) -> PortGpiof70dataRegW<Gpio020Spec> {
        PortGpiof70dataRegW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOG\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpiog70data_reg(&mut self) -> PortGpiog70dataRegW<Gpio020Spec> {
        PortGpiog70dataRegW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOH\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpioh70data_reg(&mut self) -> PortGpioh70dataRegW<Gpio020Spec> {
        PortGpioh70dataRegW::new(self, 24)
    }
}
#[doc = "GPIO\\_E/F/G/H Data Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio020::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio020::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio020Spec;
impl crate::RegisterSpec for Gpio020Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio020::R`](R) reader structure"]
impl crate::Readable for Gpio020Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio020::W`](W) writer structure"]
impl crate::Writable for Gpio020Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO020 to value 0"]
impl crate::Resettable for Gpio020Spec {}
