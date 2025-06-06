#[doc = "Register `GPIO520` reader"]
pub type R = crate::R<Gpio520Spec>;
#[doc = "Register `GPIO520` writer"]
pub type W = crate::W<Gpio520Spec>;
#[doc = "Field `PortSerialGPIOE70INTEnbl` reader - Port Serial GPIOE\\[7:0\\] interrupt enable"]
pub type PortSerialGpioe70intenblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOE70INTEnbl` writer - Port Serial GPIOE\\[7:0\\] interrupt enable"]
pub type PortSerialGpioe70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOF70INTEnbl` reader - Port Serial GPIOF\\[7:0\\] interrupt enable"]
pub type PortSerialGpiof70intenblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOF70INTEnbl` writer - Port Serial GPIOF\\[7:0\\] interrupt enable"]
pub type PortSerialGpiof70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOG70INTEnbl` reader - Port Serial GPIOG\\[7:0\\] interrupt enable"]
pub type PortSerialGpiog70intenblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOG70INTEnbl` writer - Port Serial GPIOG\\[7:0\\] interrupt enable"]
pub type PortSerialGpiog70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOH70INTEnbl` reader - Port Serial GPIOH\\[7:0\\] interrupt enable"]
pub type PortSerialGpioh70intenblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOH70INTEnbl` writer - Port Serial GPIOH\\[7:0\\] interrupt enable"]
pub type PortSerialGpioh70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOE\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpioe70intenbl(&self) -> PortSerialGpioe70intenblR {
        PortSerialGpioe70intenblR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOF\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpiof70intenbl(&self) -> PortSerialGpiof70intenblR {
        PortSerialGpiof70intenblR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOG\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpiog70intenbl(&self) -> PortSerialGpiog70intenblR {
        PortSerialGpiog70intenblR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOH\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpioh70intenbl(&self) -> PortSerialGpioh70intenblR {
        PortSerialGpioh70intenblR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOE\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpioe70intenbl(&mut self) -> PortSerialGpioe70intenblW<Gpio520Spec> {
        PortSerialGpioe70intenblW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOF\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpiof70intenbl(&mut self) -> PortSerialGpiof70intenblW<Gpio520Spec> {
        PortSerialGpiof70intenblW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOG\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpiog70intenbl(&mut self) -> PortSerialGpiog70intenblW<Gpio520Spec> {
        PortSerialGpiog70intenblW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOH\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpioh70intenbl(&mut self) -> PortSerialGpioh70intenblW<Gpio520Spec> {
        PortSerialGpioh70intenblW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_E/F/G/H 1 Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio520::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio520::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio520Spec;
impl crate::RegisterSpec for Gpio520Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio520::R`](R) reader structure"]
impl crate::Readable for Gpio520Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio520::W`](W) writer structure"]
impl crate::Writable for Gpio520Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO520 to value 0"]
impl crate::Resettable for Gpio520Spec {}
