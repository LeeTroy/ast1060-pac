#[doc = "Register `SCU438` reader"]
pub type R = crate::R<Scu438Spec>;
#[doc = "Register `SCU438` writer"]
pub type W = crate::W<Scu438Spec>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `EnblGPIOPassthrough0Debounce` reader - Enable GPIO Passthrough 0 de-bounce"]
pub type EnblGpiopassthrough0debounceR = crate::BitReader;
#[doc = "Field `EnblGPIOPassthrough0Debounce` writer - Enable GPIO Passthrough 0 de-bounce"]
pub type EnblGpiopassthrough0debounceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIOPassthrough1Debounce` reader - Enable GPIO Passthrough 1 de-bounce"]
pub type EnblGpiopassthrough1debounceR = crate::BitReader;
#[doc = "Field `EnblGPIOPassthrough1Debounce` writer - Enable GPIO Passthrough 1 de-bounce"]
pub type EnblGpiopassthrough1debounceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIOPassthrough2Debounce` reader - Enable GPIO Passthrough 2 de-bounce"]
pub type EnblGpiopassthrough2debounceR = crate::BitReader;
#[doc = "Field `EnblGPIOPassthrough2Debounce` writer - Enable GPIO Passthrough 2 de-bounce"]
pub type EnblGpiopassthrough2debounceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIOPassthrough3Debounce` reader - Enable GPIO Passthrough 3 de-bounce"]
pub type EnblGpiopassthrough3debounceR = crate::BitReader;
#[doc = "Field `EnblGPIOPassthrough3Debounce` writer - Enable GPIO Passthrough 3 de-bounce"]
pub type EnblGpiopassthrough3debounceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Enable GPIO Passthrough 0 de-bounce"]
    #[inline(always)]
    pub fn enbl_gpiopassthrough0debounce(&self) -> EnblGpiopassthrough0debounceR {
        EnblGpiopassthrough0debounceR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO Passthrough 1 de-bounce"]
    #[inline(always)]
    pub fn enbl_gpiopassthrough1debounce(&self) -> EnblGpiopassthrough1debounceR {
        EnblGpiopassthrough1debounceR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO Passthrough 2 de-bounce"]
    #[inline(always)]
    pub fn enbl_gpiopassthrough2debounce(&self) -> EnblGpiopassthrough2debounceR {
        EnblGpiopassthrough2debounceR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO Passthrough 3 de-bounce"]
    #[inline(always)]
    pub fn enbl_gpiopassthrough3debounce(&self) -> EnblGpiopassthrough3debounceR {
        EnblGpiopassthrough3debounceR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu438Spec> {
        Reserved1W::new(self, 0)
    }
    #[doc = "Bit 24 - Enable GPIO Passthrough 0 de-bounce"]
    #[inline(always)]
    pub fn enbl_gpiopassthrough0debounce(&mut self) -> EnblGpiopassthrough0debounceW<Scu438Spec> {
        EnblGpiopassthrough0debounceW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO Passthrough 1 de-bounce"]
    #[inline(always)]
    pub fn enbl_gpiopassthrough1debounce(&mut self) -> EnblGpiopassthrough1debounceW<Scu438Spec> {
        EnblGpiopassthrough1debounceW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO Passthrough 2 de-bounce"]
    #[inline(always)]
    pub fn enbl_gpiopassthrough2debounce(&mut self) -> EnblGpiopassthrough2debounceW<Scu438Spec> {
        EnblGpiopassthrough2debounceW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO Passthrough 3 de-bounce"]
    #[inline(always)]
    pub fn enbl_gpiopassthrough3debounce(&mut self) -> EnblGpiopassthrough3debounceW<Scu438Spec> {
        EnblGpiopassthrough3debounceW::new(self, 27)
    }
}
#[doc = "Multi-function Pin Control \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`scu438::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu438::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu438Spec;
impl crate::RegisterSpec for Scu438Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu438::R`](R) reader structure"]
impl crate::Readable for Scu438Spec {}
#[doc = "`write(|w| ..)` method takes [`scu438::W`](W) writer structure"]
impl crate::Writable for Scu438Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU438 to value 0x3800"]
impl crate::Resettable for Scu438Spec {
    const RESET_VALUE: u32 = 0x3800;
}
