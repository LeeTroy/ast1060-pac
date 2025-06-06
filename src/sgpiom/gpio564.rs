#[doc = "Register `GPIO564` reader"]
pub type R = crate::R<Gpio564Spec>;
#[doc = "Register `GPIO564` writer"]
pub type W = crate::W<Gpio564Spec>;
#[doc = "Field `PortGPIOM70InputMask` reader - Port GPIOM\\[7:0\\] input mask"]
pub type PortGpiom70inputMaskR = crate::FieldReader;
#[doc = "Field `PortGPIOM70InputMask` writer - Port GPIOM\\[7:0\\] input mask"]
pub type PortGpiom70inputMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPION70InputMask` reader - Port GPION\\[7:0\\] input mask"]
pub type PortGpion70inputMaskR = crate::FieldReader;
#[doc = "Field `PortGPION70InputMask` writer - Port GPION\\[7:0\\] input mask"]
pub type PortGpion70inputMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOO70InputMask` reader - Port GPIOO\\[7:0\\] input mask"]
pub type PortGpioo70inputMaskR = crate::FieldReader;
#[doc = "Field `PortGPIOO70InputMask` writer - Port GPIOO\\[7:0\\] input mask"]
pub type PortGpioo70inputMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOP70InputMask` reader - Port GPIOP\\[7:0\\] input mask"]
pub type PortGpiop70inputMaskR = crate::FieldReader;
#[doc = "Field `PortGPIOP70InputMask` writer - Port GPIOP\\[7:0\\] input mask"]
pub type PortGpiop70inputMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOM\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpiom70input_mask(&self) -> PortGpiom70inputMaskR {
        PortGpiom70inputMaskR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPION\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpion70input_mask(&self) -> PortGpion70inputMaskR {
        PortGpion70inputMaskR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOO\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpioo70input_mask(&self) -> PortGpioo70inputMaskR {
        PortGpioo70inputMaskR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOP\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpiop70input_mask(&self) -> PortGpiop70inputMaskR {
        PortGpiop70inputMaskR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOM\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpiom70input_mask(&mut self) -> PortGpiom70inputMaskW<Gpio564Spec> {
        PortGpiom70inputMaskW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPION\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpion70input_mask(&mut self) -> PortGpion70inputMaskW<Gpio564Spec> {
        PortGpion70inputMaskW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOO\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpioo70input_mask(&mut self) -> PortGpioo70inputMaskW<Gpio564Spec> {
        PortGpioo70inputMaskW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOP\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpiop70input_mask(&mut self) -> PortGpiop70inputMaskW<Gpio564Spec> {
        PortGpiop70inputMaskW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_M/N/O/P Input Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio564::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio564::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio564Spec;
impl crate::RegisterSpec for Gpio564Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio564::R`](R) reader structure"]
impl crate::Readable for Gpio564Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio564::W`](W) writer structure"]
impl crate::Writable for Gpio564Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO564 to value 0"]
impl crate::Resettable for Gpio564Spec {}
