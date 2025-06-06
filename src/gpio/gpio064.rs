#[doc = "Register `GPIO064` reader"]
pub type R = crate::R<Gpio064Spec>;
#[doc = "Register `GPIO064` writer"]
pub type W = crate::W<Gpio064Spec>;
#[doc = "Field `PortGPIOA70CmdSource1` reader - Port GPIOA\\[7:0\\] Command Source 1"]
pub type PortGpioa70cmdSource1R = crate::BitReader;
#[doc = "Field `PortGPIOA70CmdSource1` writer - Port GPIOA\\[7:0\\] Command Source 1"]
pub type PortGpioa70cmdSource1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `PortGPIOB70CmdSource1` reader - Port GPIOB\\[7:0\\] Command Source 1"]
pub type PortGpiob70cmdSource1R = crate::BitReader;
#[doc = "Field `PortGPIOB70CmdSource1` writer - Port GPIOB\\[7:0\\] Command Source 1"]
pub type PortGpiob70cmdSource1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `PortGPIOC70CmdSource1` reader - Port GPIOC\\[7:0\\] Command Source 1"]
pub type PortGpioc70cmdSource1R = crate::BitReader;
#[doc = "Field `PortGPIOC70CmdSource1` writer - Port GPIOC\\[7:0\\] Command Source 1"]
pub type PortGpioc70cmdSource1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `PortGPIOD70CmdSource1` reader - Port GPIOD\\[7:0\\] Command Source 1"]
pub type PortGpiod70cmdSource1R = crate::BitReader;
#[doc = "Field `PortGPIOD70CmdSource1` writer - Port GPIOD\\[7:0\\] Command Source 1"]
pub type PortGpiod70cmdSource1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port GPIOA\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpioa70cmd_source1(&self) -> PortGpioa70cmdSource1R {
        PortGpioa70cmdSource1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Port GPIOB\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpiob70cmd_source1(&self) -> PortGpiob70cmdSource1R {
        PortGpiob70cmdSource1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Port GPIOC\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpioc70cmd_source1(&self) -> PortGpioc70cmdSource1R {
        PortGpioc70cmdSource1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Port GPIOD\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpiod70cmd_source1(&self) -> PortGpiod70cmdSource1R {
        PortGpiod70cmdSource1R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port GPIOA\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpioa70cmd_source1(&mut self) -> PortGpioa70cmdSource1W<Gpio064Spec> {
        PortGpioa70cmdSource1W::new(self, 0)
    }
    #[doc = "Bit 8 - Port GPIOB\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpiob70cmd_source1(&mut self) -> PortGpiob70cmdSource1W<Gpio064Spec> {
        PortGpiob70cmdSource1W::new(self, 8)
    }
    #[doc = "Bit 16 - Port GPIOC\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpioc70cmd_source1(&mut self) -> PortGpioc70cmdSource1W<Gpio064Spec> {
        PortGpioc70cmdSource1W::new(self, 16)
    }
    #[doc = "Bit 24 - Port GPIOD\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpiod70cmd_source1(&mut self) -> PortGpiod70cmdSource1W<Gpio064Spec> {
        PortGpiod70cmdSource1W::new(self, 24)
    }
}
#[doc = "GPIO\\_A/B/C/D Command Source 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio064::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio064::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio064Spec;
impl crate::RegisterSpec for Gpio064Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio064::R`](R) reader structure"]
impl crate::Readable for Gpio064Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio064::W`](W) writer structure"]
impl crate::Writable for Gpio064Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO064 to value 0"]
impl crate::Resettable for Gpio064Spec {}
