#[doc = "Register `SCU434` reader"]
pub type R = crate::R<Scu434Spec>;
#[doc = "Register `SCU434` writer"]
pub type W = crate::W<Scu434Spec>;
#[doc = "Field `EnblGPIU0FnPin` reader - Enable GPIU0 function pin"]
pub type EnblGpiu0fnPinR = crate::BitReader;
#[doc = "Field `EnblGPIU0FnPin` writer - Enable GPIU0 function pin"]
pub type EnblGpiu0fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIU1FnPin` reader - Enable GPIU1 function pin"]
pub type EnblGpiu1fnPinR = crate::BitReader;
#[doc = "Field `EnblGPIU1FnPin` writer - Enable GPIU1 function pin"]
pub type EnblGpiu1fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIU2FnPin` reader - Enable GPIU2 function pin"]
pub type EnblGpiu2fnPinR = crate::BitReader;
#[doc = "Field `EnblGPIU2FnPin` writer - Enable GPIU2 function pin"]
pub type EnblGpiu2fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIU3FnPin` reader - Enable GPIU3 function pin"]
pub type EnblGpiu3fnPinR = crate::BitReader;
#[doc = "Field `EnblGPIU3FnPin` writer - Enable GPIU3 function pin"]
pub type EnblGpiu3fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIU4FnPin` reader - Enable GPIU4 function pin"]
pub type EnblGpiu4fnPinR = crate::BitReader;
#[doc = "Field `EnblGPIU4FnPin` writer - Enable GPIU4 function pin"]
pub type EnblGpiu4fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIU5FnPin` reader - Enable GPIU5 function pin"]
pub type EnblGpiu5fnPinR = crate::BitReader;
#[doc = "Field `EnblGPIU5FnPin` writer - Enable GPIU5 function pin"]
pub type EnblGpiu5fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIU6FnPin` reader - Enable GPIU6 function pin"]
pub type EnblGpiu6fnPinR = crate::BitReader;
#[doc = "Field `EnblGPIU6FnPin` writer - Enable GPIU6 function pin"]
pub type EnblGpiu6fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIU7FnPin` reader - Enable GPIU7 function pin"]
pub type EnblGpiu7fnPinR = crate::BitReader;
#[doc = "Field `EnblGPIU7FnPin` writer - Enable GPIU7 function pin"]
pub type EnblGpiu7fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIU0 function pin"]
    #[inline(always)]
    pub fn enbl_gpiu0fn_pin(&self) -> EnblGpiu0fnPinR {
        EnblGpiu0fnPinR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIU1 function pin"]
    #[inline(always)]
    pub fn enbl_gpiu1fn_pin(&self) -> EnblGpiu1fnPinR {
        EnblGpiu1fnPinR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIU2 function pin"]
    #[inline(always)]
    pub fn enbl_gpiu2fn_pin(&self) -> EnblGpiu2fnPinR {
        EnblGpiu2fnPinR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIU3 function pin"]
    #[inline(always)]
    pub fn enbl_gpiu3fn_pin(&self) -> EnblGpiu3fnPinR {
        EnblGpiu3fnPinR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable GPIU4 function pin"]
    #[inline(always)]
    pub fn enbl_gpiu4fn_pin(&self) -> EnblGpiu4fnPinR {
        EnblGpiu4fnPinR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable GPIU5 function pin"]
    #[inline(always)]
    pub fn enbl_gpiu5fn_pin(&self) -> EnblGpiu5fnPinR {
        EnblGpiu5fnPinR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable GPIU6 function pin"]
    #[inline(always)]
    pub fn enbl_gpiu6fn_pin(&self) -> EnblGpiu6fnPinR {
        EnblGpiu6fnPinR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable GPIU7 function pin"]
    #[inline(always)]
    pub fn enbl_gpiu7fn_pin(&self) -> EnblGpiu7fnPinR {
        EnblGpiu7fnPinR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIU0 function pin"]
    #[inline(always)]
    pub fn enbl_gpiu0fn_pin(&mut self) -> EnblGpiu0fnPinW<Scu434Spec> {
        EnblGpiu0fnPinW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIU1 function pin"]
    #[inline(always)]
    pub fn enbl_gpiu1fn_pin(&mut self) -> EnblGpiu1fnPinW<Scu434Spec> {
        EnblGpiu1fnPinW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIU2 function pin"]
    #[inline(always)]
    pub fn enbl_gpiu2fn_pin(&mut self) -> EnblGpiu2fnPinW<Scu434Spec> {
        EnblGpiu2fnPinW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIU3 function pin"]
    #[inline(always)]
    pub fn enbl_gpiu3fn_pin(&mut self) -> EnblGpiu3fnPinW<Scu434Spec> {
        EnblGpiu3fnPinW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable GPIU4 function pin"]
    #[inline(always)]
    pub fn enbl_gpiu4fn_pin(&mut self) -> EnblGpiu4fnPinW<Scu434Spec> {
        EnblGpiu4fnPinW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable GPIU5 function pin"]
    #[inline(always)]
    pub fn enbl_gpiu5fn_pin(&mut self) -> EnblGpiu5fnPinW<Scu434Spec> {
        EnblGpiu5fnPinW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable GPIU6 function pin"]
    #[inline(always)]
    pub fn enbl_gpiu6fn_pin(&mut self) -> EnblGpiu6fnPinW<Scu434Spec> {
        EnblGpiu6fnPinW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable GPIU7 function pin"]
    #[inline(always)]
    pub fn enbl_gpiu7fn_pin(&mut self) -> EnblGpiu7fnPinW<Scu434Spec> {
        EnblGpiu7fnPinW::new(self, 7)
    }
}
#[doc = "Multi-function Pin Control \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`scu434::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu434::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu434Spec;
impl crate::RegisterSpec for Scu434Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu434::R`](R) reader structure"]
impl crate::Readable for Scu434Spec {}
#[doc = "`write(|w| ..)` method takes [`scu434::W`](W) writer structure"]
impl crate::Writable for Scu434Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU434 to value 0x00ff_0000"]
impl crate::Resettable for Scu434Spec {
    const RESET_VALUE: u32 = 0x00ff_0000;
}
