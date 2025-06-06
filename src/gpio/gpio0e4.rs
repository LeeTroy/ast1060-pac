#[doc = "Register `GPIO0E4` reader"]
pub type R = crate::R<Gpio0e4Spec>;
#[doc = "Register `GPIO0E4` writer"]
pub type W = crate::W<Gpio0e4Spec>;
#[doc = "Field `PortGPIOM70CmdSource1` reader - Port GPIOM\\[7:0\\] Command Source 1"]
pub type PortGpiom70cmdSource1R = crate::BitReader;
#[doc = "Field `PortGPIOM70CmdSource1` writer - Port GPIOM\\[7:0\\] Command Source 1"]
pub type PortGpiom70cmdSource1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `PortGPION70CmdSource1` reader - Port GPION\\[7:0\\] Command Source 1"]
pub type PortGpion70cmdSource1R = crate::BitReader;
#[doc = "Field `PortGPION70CmdSource1` writer - Port GPION\\[7:0\\] Command Source 1"]
pub type PortGpion70cmdSource1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `PortGPIOO70CmdSource1` reader - Port GPIOO\\[7:0\\] Command Source 1"]
pub type PortGpioo70cmdSource1R = crate::BitReader;
#[doc = "Field `PortGPIOO70CmdSource1` writer - Port GPIOO\\[7:0\\] Command Source 1"]
pub type PortGpioo70cmdSource1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `PortGPIOP70CmdSource1` reader - Port GPIOP\\[7:0\\] Command Source 1"]
pub type PortGpiop70cmdSource1R = crate::BitReader;
#[doc = "Field `PortGPIOP70CmdSource1` writer - Port GPIOP\\[7:0\\] Command Source 1"]
pub type PortGpiop70cmdSource1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port GPIOM\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpiom70cmd_source1(&self) -> PortGpiom70cmdSource1R {
        PortGpiom70cmdSource1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Port GPION\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpion70cmd_source1(&self) -> PortGpion70cmdSource1R {
        PortGpion70cmdSource1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Port GPIOO\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpioo70cmd_source1(&self) -> PortGpioo70cmdSource1R {
        PortGpioo70cmdSource1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Port GPIOP\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpiop70cmd_source1(&self) -> PortGpiop70cmdSource1R {
        PortGpiop70cmdSource1R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port GPIOM\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpiom70cmd_source1(&mut self) -> PortGpiom70cmdSource1W<Gpio0e4Spec> {
        PortGpiom70cmdSource1W::new(self, 0)
    }
    #[doc = "Bit 8 - Port GPION\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpion70cmd_source1(&mut self) -> PortGpion70cmdSource1W<Gpio0e4Spec> {
        PortGpion70cmdSource1W::new(self, 8)
    }
    #[doc = "Bit 16 - Port GPIOO\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpioo70cmd_source1(&mut self) -> PortGpioo70cmdSource1W<Gpio0e4Spec> {
        PortGpioo70cmdSource1W::new(self, 16)
    }
    #[doc = "Bit 24 - Port GPIOP\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpiop70cmd_source1(&mut self) -> PortGpiop70cmdSource1W<Gpio0e4Spec> {
        PortGpiop70cmdSource1W::new(self, 24)
    }
}
#[doc = "GPIO\\_M/N/O/P Command Source 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0e4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0e4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0e4Spec;
impl crate::RegisterSpec for Gpio0e4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0e4::R`](R) reader structure"]
impl crate::Readable for Gpio0e4Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio0e4::W`](W) writer structure"]
impl crate::Writable for Gpio0e4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO0E4 to value 0"]
impl crate::Resettable for Gpio0e4Spec {}
