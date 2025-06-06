#[doc = "Register `GPIO098` reader"]
pub type R = crate::R<Gpio098Spec>;
#[doc = "Register `GPIO098` writer"]
pub type W = crate::W<Gpio098Spec>;
#[doc = "Field `PortGPIOI70INTEnbl` reader - Port GPIOI\\[7:0\\] interrupt enable"]
pub type PortGpioi70intenblR = crate::FieldReader;
#[doc = "Field `PortGPIOI70INTEnbl` writer - Port GPIOI\\[7:0\\] interrupt enable"]
pub type PortGpioi70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOJ70INTEnbl` reader - Port GPIOJ\\[7:0\\] interrupt enable"]
pub type PortGpioj70intenblR = crate::FieldReader;
#[doc = "Field `PortGPIOJ70INTEnbl` writer - Port GPIOJ\\[7:0\\] interrupt enable"]
pub type PortGpioj70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOK70INTEnbl` reader - Port GPIOK\\[7:0\\] interrupt enable"]
pub type PortGpiok70intenblR = crate::FieldReader;
#[doc = "Field `PortGPIOK70INTEnbl` writer - Port GPIOK\\[7:0\\] interrupt enable"]
pub type PortGpiok70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOL70INTEnbl` reader - Port GPIOL\\[7:0\\] interrupt enable"]
pub type PortGpiol70intenblR = crate::FieldReader;
#[doc = "Field `PortGPIOL70INTEnbl` writer - Port GPIOL\\[7:0\\] interrupt enable"]
pub type PortGpiol70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOI\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpioi70intenbl(&self) -> PortGpioi70intenblR {
        PortGpioi70intenblR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOJ\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpioj70intenbl(&self) -> PortGpioj70intenblR {
        PortGpioj70intenblR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOK\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpiok70intenbl(&self) -> PortGpiok70intenblR {
        PortGpiok70intenblR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOL\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpiol70intenbl(&self) -> PortGpiol70intenblR {
        PortGpiol70intenblR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOI\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpioi70intenbl(&mut self) -> PortGpioi70intenblW<Gpio098Spec> {
        PortGpioi70intenblW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOJ\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpioj70intenbl(&mut self) -> PortGpioj70intenblW<Gpio098Spec> {
        PortGpioj70intenblW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOK\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpiok70intenbl(&mut self) -> PortGpiok70intenblW<Gpio098Spec> {
        PortGpiok70intenblW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOL\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpiol70intenbl(&mut self) -> PortGpiol70intenblW<Gpio098Spec> {
        PortGpiol70intenblW::new(self, 24)
    }
}
#[doc = "GPIO\\_I/J/K/L Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio098::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio098::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio098Spec;
impl crate::RegisterSpec for Gpio098Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio098::R`](R) reader structure"]
impl crate::Readable for Gpio098Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio098::W`](W) writer structure"]
impl crate::Writable for Gpio098Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO098 to value 0"]
impl crate::Resettable for Gpio098Spec {}
